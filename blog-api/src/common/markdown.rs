use comrak::{
    adapters::{HeadingAdapter, HeadingMeta},
    markdown_to_html_with_plugins,
    nodes::Sourcepos,
    Options, Plugins,
};
use regex::Regex;
use std::io::{self, Write};
pub struct MarkdownParser;
impl MarkdownParser {
    pub fn parser_html(markdown: String) -> String {
        let adapter = CustomHeadingAdapter::new();
        let markdown = preprocess_markdown(markdown);
        let mut options = Options::default();
        options.render.unsafe_ = true;
        let mut plugins = Plugins::default();
        plugins.render.heading_adapter = Some(&adapter);
        markdown_to_html_with_plugins(markdown.as_str(), &options, &plugins)
    }
}

#[cfg(test)]
mod tests {

    use comrak::{markdown_to_html_with_plugins, Options, Plugins};

    use super::CustomHeadingAdapter;

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
