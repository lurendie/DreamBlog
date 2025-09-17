use comrak::{
    adapters::{HeadingAdapter, HeadingMeta},
    markdown_to_html_with_plugins,
    nodes::Sourcepos,
    Options, Plugins,
};
use std::io::{self, Write};
pub struct MarkdownParser;
impl MarkdownParser {
    pub fn parser_html(markdown: String) -> String {
        let adapter = CustomHeadingAdapter::new();
        let options = Options::default();
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
