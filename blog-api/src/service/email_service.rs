use chrono::Local;
use sea_orm::DatabaseConnection;

use crate::{
    app::CONFIG,
    common::{EmailServer, EmailType, GuestReply, OwenrComment},
    entity::comment,
    error::EmailServerError,
    model::CommentDTO,
    service::BlogService,
};

pub struct EmailService;

impl EmailService {
    pub async fn get_title_and_url(
        db: &DatabaseConnection,
        model: &comment::Model,
    ) -> Result<(String, String), EmailServerError> {
        let view_url = CONFIG.get_server_config().view_url;
        let mut post_url = format!("{}", view_url);
        let mut title = String::new();
        match model.page {
            0 => {
                let blog_title = match BlogService::find_blog_id_and_title(
                    db,
                    model.blog_id.unwrap_or_default(),
                )
                .await
                {
                    Ok(blog_title) => blog_title,
                    Err(e) => {
                        return Err(EmailServerError::Custom(e.to_string()));
                    }
                };
                post_url.push_str(blog_title.id.to_string().as_str());
                title.push_str(blog_title.title.as_str());
            }
            1 => {
                post_url.push_str("about");
                title.push_str("关于我");
            }
            _ => {
                post_url.push_str("friends");
                title.push_str("友情链接");
            }
        };
        Ok((title, post_url))
    }
    pub async fn send_guest_email(
        db: &DatabaseConnection,
        model: comment::Model,
        parent_model: CommentDTO,
    ) -> Result<(), EmailServerError> {
        let (title, post_url) = Self::get_title_and_url(db, &model).await?;
        //找出父评论 发送邮件给父评论
        let email_type =
            EmailType::get_type(model.is_notice, matches!(model.parent_comment_id, -1))?;
        let guest = GuestReply::new(
            title,
            post_url,
            parent_model.nickname,
            parent_model.content,
            model.nickname.to_string(),
            model.content,
            Local::now().naive_local(),
        );
        EmailServer::send_email(guest, email_type, model.email.as_str()).await?;
        Ok(())
    }

    pub async fn send_owenr_email(
        model: comment::Model,
        db: &DatabaseConnection,
        owenr_email:String,
    ) -> Result<(), EmailServerError> {
        let manage_url = CONFIG.get_server_config().cms_url;
        let (title, post_url) = Self::get_title_and_url(db, &model).await?;
        let owenr = OwenrComment::new(
            title,
            post_url,
            model.nickname,
            model.content,
            Local::now(),
            model.ip.unwrap_or_default(),
            model.email.clone(),
            "正常".to_string(),
            manage_url,
        );
        let email_type =
            EmailType::get_type(model.is_notice, matches!(model.parent_comment_id, -1))?;
        EmailServer::send_email(owenr, email_type, owenr_email.as_str()).await?;
        Ok(())
    }
}
