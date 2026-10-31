//! Markdown reduced to its readable text for compact, single-line displays.

use pulldown_cmark::{Event, Options, Parser, TagEnd};

/// Returns the text of `markdown` without syntax, with blocks and line breaks as single spaces.
///
/// Used where Markdown is shown like any other short text, for example in table cells.
pub fn markdown_plain_text(markdown: &str) -> String {
    let mut text = String::with_capacity(markdown.len());
    let parser = Parser::new_ext(
        markdown,
        Options::ENABLE_TABLES | Options::ENABLE_STRIKETHROUGH,
    );
    for event in parser {
        match event {
            Event::Text(part) | Event::Code(part) | Event::Html(part) | Event::InlineHtml(part) => {
                text.push_str(&part);
            }
            Event::SoftBreak
            | Event::HardBreak
            | Event::Rule
            | Event::End(
                TagEnd::Paragraph
                | TagEnd::Heading(_)
                | TagEnd::Item
                | TagEnd::CodeBlock
                | TagEnd::TableCell
                | TagEnd::TableRow,
            ) => text.push(' '),
            _ => {}
        }
    }
    text.split_whitespace().collect::<Vec<_>>().join(" ")
}

#[cfg(test)]
mod tests {
    use super::markdown_plain_text;

    /// Syntax is dropped and blocks become single spaces.
    #[test]
    fn strips_markdown_syntax() {
        let markdown =
            "Title\n=====\n\nSome **bold** and *italic* text.\n\n* One\n* Two\n\n> Quote\n\n`code`";
        assert_eq!(
            markdown_plain_text(markdown),
            "Title Some bold and italic text. One Two Quote code"
        );
    }
}
