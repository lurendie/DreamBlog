use actix_web::{get, web, HttpResponse};
use sea_orm::{ColumnTrait, EntityTrait, QueryFilter, QueryOrder};

use crate::{
    app::AppState,
    entity::blog,
    error::{AppError, DataBaseError},
};

#[get("/sitemap.xml")]
pub async fn sitemap(app: web::Data<AppState>) -> Result<HttpResponse, AppError> {
    let db = app.get_mysql_pool();
    let server_config = crate::app::CONFIG.get_server_config();
    let site_url = normalize_site_url(&server_config.view_url);

    let blogs = blog::Entity::find()
        .filter(blog::Column::IsPublished.eq(true))
        .order_by_desc(blog::Column::UpdateTime)
        .all(db)
        .await
        .map_err(DataBaseError::from)?;

    let mut entries = vec![
        SitemapEntry::new("/", Some("daily"), Some("1.0"), None),
        SitemapEntry::new("/home", Some("daily"), Some("1.0"), None),
        SitemapEntry::new("/archives", Some("weekly"), Some("0.8"), None),
        SitemapEntry::new("/moments", Some("weekly"), Some("0.7"), None),
        SitemapEntry::new("/friends", Some("weekly"), Some("0.6"), None),
        SitemapEntry::new("/about", Some("monthly"), Some("0.7"), None),
    ];

    for item in blogs {
        entries.push(SitemapEntry::new(
            format!("/blog/{}", item.id),
            Some("monthly"),
            Some("0.8"),
            Some(item.update_time.format("%Y-%m-%dT%H:%M:%S").to_string()),
        ));
    }

    let xml = render_sitemap(&site_url, &entries);
    Ok(HttpResponse::Ok()
        .content_type("application/xml; charset=utf-8")
        .body(xml))
}

#[get("/robots.txt")]
pub async fn robots() -> Result<HttpResponse, AppError> {
    let server_config = crate::app::CONFIG.get_server_config();
    let site_url = normalize_site_url(&server_config.view_url);
    let sitemap_url = format!("{}/blog/sitemap.xml", normalize_site_url(&server_config.cms_url));
    let body = render_robots(&site_url, &sitemap_url);

    Ok(HttpResponse::Ok()
        .content_type("text/plain; charset=utf-8")
        .body(body))
}

struct SitemapEntry {
    path: String,
    changefreq: Option<&'static str>,
    priority: Option<&'static str>,
    lastmod: Option<String>,
}

impl SitemapEntry {
    fn new(
        path: impl Into<String>,
        changefreq: Option<&'static str>,
        priority: Option<&'static str>,
        lastmod: Option<String>,
    ) -> Self {
        Self {
            path: path.into(),
            changefreq,
            priority,
            lastmod,
        }
    }
}

fn normalize_site_url(url: &str) -> String {
    url.trim_end_matches('/').to_string()
}

fn join_url(site_url: &str, path: &str) -> String {
    if path == "/" {
        return format!("{site_url}/");
    }
    format!("{site_url}{path}")
}

fn escape_xml(value: &str) -> String {
    value
        .replace('&', "&amp;")
        .replace('<', "&lt;")
        .replace('>', "&gt;")
        .replace('"', "&quot;")
        .replace('\'', "&apos;")
}

fn render_sitemap(site_url: &str, entries: &[SitemapEntry]) -> String {
    let mut xml = String::from(
        r#"<?xml version="1.0" encoding="UTF-8"?>
<urlset xmlns="http://www.sitemaps.org/schemas/sitemap/0.9">
"#,
    );

    for entry in entries {
        xml.push_str("  <url>\n");
        xml.push_str(&format!(
            "    <loc>{}</loc>\n",
            escape_xml(&join_url(site_url, &entry.path))
        ));
        if let Some(lastmod) = &entry.lastmod {
            xml.push_str(&format!("    <lastmod>{}</lastmod>\n", escape_xml(lastmod)));
        }
        if let Some(changefreq) = entry.changefreq {
            xml.push_str(&format!("    <changefreq>{changefreq}</changefreq>\n"));
        }
        if let Some(priority) = entry.priority {
            xml.push_str(&format!("    <priority>{priority}</priority>\n"));
        }
        xml.push_str("  </url>\n");
    }

    xml.push_str("</urlset>\n");
    xml
}

fn render_robots(site_url: &str, sitemap_url: &str) -> String {
    let login_url = format!("{}/login", normalize_site_url(site_url));
    format!(
        "User-agent: *\nAllow: /\nDisallow: {login_url}\nSitemap: {sitemap_url}\n"
    )
}
