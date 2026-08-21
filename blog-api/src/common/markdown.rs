use comrak::{
    adapters::{HeadingAdapter, HeadingMeta},
    markdown_to_html_with_plugins,
    nodes::Sourcepos,
    Options, Plugins,
};
use regex::Regex;
use std::io::{self, Write};
pub struct MarkdownParser;
pub const DEFAULT_EXCERPT_MAX_CHARS: usize = 160;

impl MarkdownParser {
    pub fn parser_html(markdown: String) -> String {
        let adapter = CustomHeadingAdapter::new();
        let markdown = preprocess_markdown(markdown);
        let mut options = Options::default();
        options.render.unsafe_ = true;
        let mut plugins = Plugins::default();
        plugins.render.heading_adapter = Some(&adapter);
        let html = markdown_to_html_with_plugins(markdown.as_str(), &options, &plugins);
        // 服务端白名单消毒：在允许 raw HTML（unsafe_）的前提下只放行受控标签/属性，
        // 作为前端 v-safe-html 之外的第二道防线（script/事件属性/javascript: 等在此被剥除）
        sanitize_html(&html)
    }

    pub fn description_or_excerpt(description: &str, markdown: &str) -> String {
        let description = description.trim();
        if !description.is_empty() {
            return description.to_string();
        }
        Self::plain_text_excerpt(markdown, DEFAULT_EXCERPT_MAX_CHARS)
    }

    pub fn plain_text_excerpt(markdown: &str, max_chars: usize) -> String {
        truncate_text(&markdown_to_plain_text(markdown), max_chars)
    }
}

/// 白名单配置：覆盖 comrak 输出的全部标签/属性，外加文档化的 meting-js 自定义元素；
/// class/id 全局放行（tocbot 锚点、Prism 代码高亮、@@/%% 黑幕样式依赖它们）
fn sanitize_html(html: &str) -> String {
    let mut builder = ammonia::Builder::default();
    builder
        .add_tags(&["input", "meting-js"])
        // script/style 整体移除（默认仅清空内容、保留空标签）
        .rm_tags(&["script", "style"])
        .add_generic_attributes(&["class", "id"])
        .add_tag_attributes("input", &["type", "checked", "disabled"])
        .add_tag_attributes("h1", &["data-search-include"])
        .add_tag_attributes("h2", &["data-search-include"])
        .add_tag_attributes("h3", &["data-search-include"])
        .add_tag_attributes("h4", &["data-search-include"])
        .add_tag_attributes("h5", &["data-search-include"])
        .add_tag_attributes("h6", &["data-search-include"])
        .add_tag_attributes(
            "meting-js",
            &[
                "server",
                "type",
                "id",
                "theme",
                "autoplay",
                "volume",
                "mutex",
                "listmaxheight",
                "preload",
                "loop",
                "mini",
                "fixed",
                "order",
                "storage",
            ],
        );
    builder.clean(html).to_string()
}

#[cfg(test)]
mod tests {

    use comrak::{markdown_to_html_with_plugins, Options, Plugins};

    use super::{CustomHeadingAdapter, MarkdownParser, DEFAULT_EXCERPT_MAX_CHARS};

    #[test]
    fn test_markdown() {
        let adapter = CustomHeadingAdapter;
        let mut options = Options::default();
        let mut plugins = Plugins::default();
        plugins.render.heading_adapter = Some(&adapter);

        print_html(
            "Some text.\n\n## Please hide me from search\n\nSome other text",
            &options,
            &plugins,
        );
        print_html(
            "Some text.\n\n### Here is some `code`\n\nSome other text",
            &options,
            &plugins,
        );
        print_html(
            "Some text.\n\n### Here is some **bold** text and some *italicized* text\n\nSome other text",
            &options,
            &plugins
        );
        options.render.sourcepos = true;
        print_html("# Here is a [link](/)", &options, &plugins);
    }

    fn print_html(document: &str, options: &Options, plugins: &Plugins) {
        let html = markdown_to_html_with_plugins(document, options, plugins);
        println!("{}", html);
    }

    #[test]
    fn sanitizes_script_and_event_attributes() {
        let html = MarkdownParser::parser_html(
            "<script>alert(1)</script> <img src=x onerror=alert(1)> [x](javascript:alert(1))"
                .to_string(),
        );
        assert!(!html.contains("<script"), "script 应被移除: {}", html);
        assert!(!html.contains("onerror"), "事件属性应被移除: {}", html);
        assert!(
            !html.contains("<a href=\"javascript:"),
            "javascript: 不应成链: {}",
            html
        );
        // 正常链接不受影响
        let html = MarkdownParser::parser_html("[x](https://example.com)".to_string());
        assert!(
            html.contains("<a href=\"https://example.com\""),
            "正常链接应保留: {}",
            html
        );
    }

    #[test]
    fn keeps_heimu_spans_and_code_language_classes() {
        let html =
            MarkdownParser::parser_html("@@隐藏@@\n\n```rust\nfn main() {}\n```".to_string());
        assert!(html.contains("m-text-heimu"), "黑幕 span 应保留: {}", html);
        assert!(
            html.contains("language-rust"),
            "代码语言 class 应保留: {}",
            html
        );
    }

    #[test]
    fn keeps_heading_ids_and_search_include() {
        let html = MarkdownParser::parser_html("## Please hide me from search".to_string());
        assert!(html.contains("id=\""), "标题 id 应保留: {}", html);
        assert!(
            html.contains("data-search-include=\"false\""),
            "data-search-include 应保留: {}",
            html
        );
    }

    #[test]
    fn keeps_meting_element() {
        let html = MarkdownParser::parser_html(
            "<meting-js server=\"netease\" type=\"song\" id=\"123\"></meting-js>".to_string(),
        );
        assert!(html.contains("meting-js"), "meting-js 应保留: {}", html);
        assert!(
            html.contains("server=\"netease\""),
            "meting-js 属性应保留: {}",
            html
        );
    }

    #[test]
    fn creates_plain_text_excerpt_from_markdown() {
        let excerpt = MarkdownParser::plain_text_excerpt(
            r#"
# 标题

![封面](https://example.com/a.png)

这是第一段，包含 **加粗**、[链接文字](https://example.com) 和 `inline_code`。

```rust
fn main() {
    println!("code should not leak");
}
```

<script>alert(1)</script><p>第二段 HTML 文本。</p>
"#,
            DEFAULT_EXCERPT_MAX_CHARS,
        );

        assert!(excerpt.contains("标题"));
        assert!(excerpt.contains("链接文字"));
        assert!(excerpt.contains("inline_code"));
        assert!(excerpt.contains("第二段 HTML 文本"));
        assert!(!excerpt.contains("封面"));
        assert!(!excerpt.contains("println"));
        assert!(!excerpt.contains("<p>"));
    }

    #[test]
    fn truncates_excerpt_to_configured_length() {
        let markdown = "一".repeat(DEFAULT_EXCERPT_MAX_CHARS + 10);
        let excerpt = MarkdownParser::plain_text_excerpt(&markdown, DEFAULT_EXCERPT_MAX_CHARS);

        assert_eq!(excerpt.chars().count(), DEFAULT_EXCERPT_MAX_CHARS);
        assert!(excerpt.ends_with('…'));
    }

    #[test]
    fn keeps_manual_description_when_present() {
        let description =
            MarkdownParser::description_or_excerpt("  手写摘要  ", "正文内容正文内容正文内容");

        assert_eq!(description, "手写摘要");
    }
}

struct CustomHeadingAdapter;

impl CustomHeadingAdapter {
    fn new() -> Self {
        Self
    }
}

impl HeadingAdapter for CustomHeadingAdapter {
    fn enter(
        &self,
        output: &mut dyn Write,
        heading: &HeadingMeta,
        sourcepos: Option<Sourcepos>,
    ) -> io::Result<()> {
        let id = slug::slugify(&heading.content);

        let search_include = !&heading.content.contains("hide");

        write!(output, "<h{}", heading.level)?;

        if let Some(sourcepos) = sourcepos {
            write!(output, " data-sourcepos=\"{}\"", sourcepos)?;
        }

        write!(
            output,
            " id=\"{}\" data-search-include=\"{}\">",
            id, search_include
        )
    }

    fn exit(&self, output: &mut dyn Write, heading: &HeadingMeta) -> io::Result<()> {
        write!(output, "</h{}>", heading.level)
    }
}

fn preprocess_markdown(markdown: String) -> String {
    let heimu_re = Regex::new(r"(?s)@@(.*?)@@").unwrap();
    let cover_re = Regex::new(r"(?s)%%(.*?)%%").unwrap();
    let mut in_code_block = false;
    let mut result = Vec::with_capacity(markdown.lines().count());
    // 逐行处理：围栏外才应用 @@/%% 替换，代码块内容原样保留
    for line in markdown.lines() {
        // 行首含 ``` 或 ~~~ 即切换围栏状态（围栏可带语言标记，行首匹配即可）
        let trimmed = line.trim_start();
        if trimmed.starts_with("```") || trimmed.starts_with("~~~") {
            in_code_block = !in_code_block;
            result.push(line.to_string());
            continue;
        }
        if in_code_block {
            result.push(line.to_string());
            continue;
        }
        let replaced = heimu_re.replace_all(line, r#"<span class="m-text-heimu">$1</span>"#);
        let replaced = cover_re.replace_all(&replaced, r#"<span class="m-text-cover">$1</span>"#);
        result.push(replaced.into_owned());
    }
    // 使用 \n 连接，保持与原输入行结构一致（末尾不额外加分号）
    result.join("\n")
}

fn markdown_to_plain_text(markdown: &str) -> String {
    let mut text = markdown.to_string();
    let replacements = [
        (r"(?s)\A\s*---\s*\n.*?\n---\s*", " "),
        (r"(?s)```.*?```", " "),
        (r"(?s)~~~.*?~~~", " "),
        (r"(?s)<!--.*?-->", " "),
        (r"(?is)<script[^>]*>.*?</script>", " "),
        (r"(?is)<style[^>]*>.*?</style>", " "),
        (r"!\[[^\]]*\]\([^)]+\)", " "),
        (r"\[([^\]]+)\]\([^)]+\)", "$1"),
        (r"\[([^\]]+)\]\[[^\]]*\]", "$1"),
        (r"(?s)<[^>]+>", " "),
        (r"`([^`]*)`", "$1"),
        (r"(?m)^\s*\|?[\s:-]+\|[\s|:-]*$", " "),
        (r"(?m)^\s{0,3}#{1,6}\s*", ""),
        (r"(?m)^\s*>\s?", ""),
        (r"(?m)^\s*([-+*]|\d+[.)])\s+", ""),
        (r"[*~#]", ""),
        (r"\|", " "),
    ];
    for (pattern, replacement) in replacements {
        text = Regex::new(pattern)
            .unwrap()
            .replace_all(&text, replacement)
            .into_owned();
    }
    text = text
        .replace("&nbsp;", " ")
        .replace("&amp;", "&")
        .replace("&lt;", "<")
        .replace("&gt;", ">")
        .replace("&quot;", "\"")
        .replace("&#39;", "'");
    Regex::new(r"\s+")
        .unwrap()
        .replace_all(&text, " ")
        .trim()
        .to_string()
}

fn truncate_text(text: &str, max_chars: usize) -> String {
    if max_chars == 0 {
        return String::new();
    }
    if text.chars().count() <= max_chars {
        return text.to_string();
    }
    let mut excerpt = text
        .chars()
        .take(max_chars.saturating_sub(1))
        .collect::<String>()
        .trim_end()
        .trim_end_matches(['.', ',', ';', ':', '，', '。', '；', '：'])
        .to_string();
    excerpt.push('…');
    excerpt
}
