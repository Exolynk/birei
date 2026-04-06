use super::super::service::CodeLanguageService;
use super::super::types::{
    CodeCompletionItem, CodeCompletionKind, CodeCursor, CodeSelection, CompletionRequest,
    CompletionResponse, HighlightRequest, HighlightResponse, HighlightSpan, IndentAction,
    IndentRequest, IndentResponse, LocalBoxFuture, TextEdit,
};

/// Bundled HTML implementation used by the editor demo and component docs.
#[derive(Clone, Copy, Debug, Default)]
pub struct HtmlCodeLanguageService;

impl CodeLanguageService for HtmlCodeLanguageService {
    fn language_id(&self) -> &'static str {
        "html"
    }

    fn highlight<'a>(&'a self, req: HighlightRequest<'a>) -> LocalBoxFuture<'a, HighlightResponse> {
        Box::pin(async move {
            HighlightResponse {
                spans: highlight_html(req.text),
            }
        })
    }

    fn complete<'a>(
        &'a self,
        req: CompletionRequest<'a>,
    ) -> LocalBoxFuture<'a, CompletionResponse> {
        Box::pin(async move { complete_html(req.text, req.cursor, req.selection) })
    }

    fn indent<'a>(&'a self, req: IndentRequest<'a>) -> LocalBoxFuture<'a, IndentResponse> {
        Box::pin(async move {
            let edit = match req.action {
                IndentAction::NewLine => indent_after_newline(req.text, &req.selection),
                _ => None,
            };
            IndentResponse { edit }
        })
    }
}

const HTML_TAGS: &[&str] = &[
    "a", "article", "aside", "button", "body", "div", "footer", "form", "h1", "h2", "h3", "head",
    "header", "html", "img", "input", "label", "li", "link", "main", "meta", "nav", "ol", "option",
    "p", "script", "section", "select", "span", "style", "table", "tbody", "td", "textarea", "th",
    "thead", "title", "tr", "ul",
];

const HTML_VOID_TAGS: &[&str] = &[
    "area", "base", "br", "col", "embed", "hr", "img", "input", "link", "meta", "param", "source",
    "track", "wbr",
];

const HTML_ATTRIBUTES: &[&str] = &[
    "alt",
    "aria-label",
    "class",
    "content",
    "data-state",
    "disabled",
    "for",
    "href",
    "id",
    "name",
    "placeholder",
    "rel",
    "role",
    "src",
    "style",
    "tabindex",
    "target",
    "title",
    "type",
    "value",
];

/// Lightweight HTML tokenizer for syntax-highlight spans.
fn highlight_html(text: &str) -> Vec<HighlightSpan> {
    let bytes = text.as_bytes();
    let mut spans = Vec::new();
    let mut index = 0;

    while index < bytes.len() {
        if !text.is_char_boundary(index) {
            index = next_char_boundary(text, index);
            continue;
        }

        if text[index..].starts_with("<!--") {
            let end = text[index + 4..]
                .find("-->")
                .map(|offset| index + 4 + offset + 3)
                .unwrap_or(bytes.len());
            spans.push(HighlightSpan {
                range: index..end,
                class_name: "birei-code-editor__token--comment",
            });
            index = end;
            continue;
        }

        if bytes[index] == b'<' {
            let end = find_tag_end(text, index).unwrap_or(bytes.len());
            highlight_tag(&text[index..end], index, &mut spans);
            index = end;
            continue;
        }

        index = next_char_boundary(text, index);
    }

    spans
}

/// Finds the end of a tag while respecting quoted attribute values.
fn find_tag_end(text: &str, start: usize) -> Option<usize> {
    let bytes = text.as_bytes();
    let mut index = start + 1;
    let mut quote = None::<u8>;

    while index < bytes.len() {
        let byte = bytes[index];
        if let Some(current) = quote {
            if byte == current {
                quote = None;
            }
        } else if byte == b'\'' || byte == b'"' {
            quote = Some(byte);
        } else if byte == b'>' {
            return Some(index + 1);
        }
        index += 1;
    }

    None
}

/// Splits one tag into semantic token spans for punctuation, tag name,
/// attributes, operators, and quoted strings.
fn highlight_tag(tag: &str, base: usize, spans: &mut Vec<HighlightSpan>) {
    let bytes = tag.as_bytes();
    if bytes.is_empty() {
        return;
    }

    spans.push(HighlightSpan {
        range: base..base + 1,
        class_name: "birei-code-editor__token--punctuation",
    });

    let mut index = 1;
    if bytes.get(index) == Some(&b'/') {
        spans.push(HighlightSpan {
            range: base + index..base + index + 1,
            class_name: "birei-code-editor__token--punctuation",
        });
        index += 1;
    }

    while index < bytes.len() && bytes[index].is_ascii_whitespace() {
        index += 1;
    }

    let name_start = index;
    while index < bytes.len() && is_name_char(bytes[index]) {
        index += 1;
    }
    if name_start < index {
        spans.push(HighlightSpan {
            range: base + name_start..base + index,
            class_name: "birei-code-editor__token--tag",
        });
    }

    while index < bytes.len() {
        if bytes[index] == b'>' {
            spans.push(HighlightSpan {
                range: base + index..base + index + 1,
                class_name: "birei-code-editor__token--punctuation",
            });
            break;
        }

        if bytes[index] == b'/' && bytes.get(index + 1) == Some(&b'>') {
            spans.push(HighlightSpan {
                range: base + index..base + index + 2,
                class_name: "birei-code-editor__token--punctuation",
            });
            break;
        }

        if bytes[index].is_ascii_whitespace() {
            index += 1;
            continue;
        }

        let attr_start = index;
        while index < bytes.len() && is_name_char(bytes[index]) {
            index += 1;
        }
        if attr_start == index {
            index += 1;
            continue;
        }

        spans.push(HighlightSpan {
            range: base + attr_start..base + index,
            class_name: "birei-code-editor__token--attribute",
        });

        while index < bytes.len() && bytes[index].is_ascii_whitespace() {
            index += 1;
        }

        if bytes.get(index) == Some(&b'=') {
            spans.push(HighlightSpan {
                range: base + index..base + index + 1,
                class_name: "birei-code-editor__token--operator",
            });
            index += 1;
        }

        while index < bytes.len() && bytes[index].is_ascii_whitespace() {
            index += 1;
        }

        if let Some(quote) = bytes
            .get(index)
            .copied()
            .filter(|byte| *byte == b'\'' || *byte == b'"')
        {
            let string_start = index;
            index += 1;
            while index < bytes.len() && bytes[index] != quote {
                index += 1;
            }
            if index < bytes.len() {
                index += 1;
            }
            spans.push(HighlightSpan {
                range: base + string_start..base + index,
                class_name: "birei-code-editor__token--string",
            });
        }
    }
}

/// Provides tag or attribute completions based on the cursor's current HTML
/// context. Outside of a tag, no popup should be shown.
fn complete_html(text: &str, cursor: CodeCursor, selection: CodeSelection) -> CompletionResponse {
    if selection.start != selection.end {
        return CompletionResponse::default();
    }

    let offset = cursor.offset.min(text.len());
    let prefix_start = find_name_start(text, offset);
    let prefix = &text[prefix_start..offset];

    if is_in_tag_context(text, offset) {
        let before_prefix = &text[..prefix_start];
        let after_lt = before_prefix.rsplit('<').next().unwrap_or_default();
        let preferred_closing_tag = nearest_unclosed_tag(text, offset)
            .filter(|tag| !has_immediate_closing_tag_ahead(text, offset, tag));
        let items = if after_lt.trim_start().starts_with('/') {
            complete_from_tags(prefix, true, preferred_closing_tag.as_deref())
        } else if after_lt.contains(char::is_whitespace) {
            complete_from_attributes(prefix)
        } else {
            complete_from_tags(prefix, false, preferred_closing_tag.as_deref())
        };

        CompletionResponse {
            items,
            replace: Some(prefix_start..offset),
        }
    } else {
        CompletionResponse::default()
    }
}

/// Builds tag completion items and optionally promotes the nearest unclosed
/// tag so closing the current structure feels natural.
fn complete_from_tags(
    prefix: &str,
    closing: bool,
    preferred_tag: Option<&str>,
) -> Vec<CodeCompletionItem> {
    let prefix_lower = prefix.to_ascii_lowercase();
    let mut items: Vec<_> = HTML_TAGS
        .iter()
        .filter(|tag| tag.starts_with(&prefix_lower))
        .map(|tag| CodeCompletionItem {
            label: (*tag).to_owned(),
            detail: Some(String::from("HTML tag")),
            insert_text: format!("{tag}>"),
            kind: CodeCompletionKind::Tag,
        })
        .collect();

    if let Some(preferred_tag) = preferred_tag {
        if preferred_tag.starts_with(&prefix_lower) {
            if closing {
                if let Some(index) = items.iter().position(|item| item.label == preferred_tag) {
                    let mut preferred = items.remove(index);
                    preferred.detail = Some(String::from("Close open tag"));
                    items.insert(0, preferred);
                } else {
                    items.insert(
                        0,
                        CodeCompletionItem {
                            label: preferred_tag.to_owned(),
                            detail: Some(String::from("Close open tag")),
                            insert_text: format!("{preferred_tag}>"),
                            kind: CodeCompletionKind::Tag,
                        },
                    );
                }
            } else {
                items.insert(
                    0,
                    CodeCompletionItem {
                        label: format!("</{preferred_tag}>"),
                        detail: Some(String::from("Close open tag")),
                        insert_text: format!("/{preferred_tag}>"),
                        kind: CodeCompletionKind::Tag,
                    },
                );
            }
        }
    }

    items.truncate(8);
    items
}

/// Offers a small curated attribute list once the cursor is inside a tag body.
fn complete_from_attributes(prefix: &str) -> Vec<CodeCompletionItem> {
    let prefix_lower = prefix.to_ascii_lowercase();
    HTML_ATTRIBUTES
        .iter()
        .filter(|attribute| attribute.starts_with(&prefix_lower))
        .take(8)
        .map(|attribute| CodeCompletionItem {
            label: (*attribute).to_owned(),
            detail: Some(String::from("HTML attribute")),
            insert_text: format!(r#"{attribute}="""#),
            kind: CodeCompletionKind::Attribute,
        })
        .collect()
}

/// Carries indentation onto the next line and adds one extra level after an
/// opening tag that has not already been closed inline.
fn indent_after_newline(text: &str, selection: &CodeSelection) -> Option<TextEdit> {
    if selection.start != selection.end {
        return None;
    }

    let cursor = selection.start.min(text.len());
    let line_start = text[..cursor]
        .rfind('\n')
        .map(|index| index + 1)
        .unwrap_or(0);
    let previous_line = &text[line_start..cursor];
    let indent: String = previous_line
        .chars()
        .take_while(|ch| *ch == ' ' || *ch == '\t')
        .collect();

    let trimmed = previous_line.trim_end();
    let extra_indent =
        if trimmed.ends_with('>') && !trimmed.ends_with("/>") && !trimmed.starts_with("</") {
            if let Some(tag_name) = open_tag_name(trimmed) {
                if !trimmed.contains(&format!("</{tag_name}>")) {
                    "  "
                } else {
                    ""
                }
            } else {
                ""
            }
        } else {
            ""
        };

    Some(TextEdit {
        range: cursor..cursor,
        replacement: format!("\n{indent}{extra_indent}"),
        cursor: Some(cursor + 1 + indent.len() + extra_indent.len()),
    })
}

/// Extracts the opening tag name from a single trimmed line segment.
fn open_tag_name(line: &str) -> Option<&str> {
    let start = line.rfind('<')?;
    let tail = &line[start + 1..];
    if tail.starts_with('/') {
        return None;
    }
    let end = tail
        .find(|ch: char| ch.is_whitespace() || ch == '>' || ch == '/')
        .unwrap_or(tail.len());
    let name = &tail[..end];
    if name.is_empty() {
        None
    } else {
        Some(name)
    }
}

/// Checks whether the cursor currently sits between an unmatched `<` and `>`.
fn is_in_tag_context(text: &str, offset: usize) -> bool {
    let before = &text[..offset];
    let last_open = before.rfind('<');
    let last_close = before.rfind('>');
    matches!((last_open, last_close), (Some(open), Some(close)) if open > close)
        || matches!((last_open, last_close), (Some(_), None))
}

/// Walks the document up to the cursor and returns the innermost still-open
/// non-void tag, which is used to promote helpful closing suggestions.
fn nearest_unclosed_tag(text: &str, offset: usize) -> Option<String> {
    let mut index = 0;
    let safe_offset = offset.min(text.len());
    let mut stack = Vec::<String>::new();

    while index < safe_offset {
        if !text.is_char_boundary(index) {
            index = next_char_boundary(text, index);
            continue;
        }

        if text[index..safe_offset].starts_with("<!--") {
            let end = text[index + 4..safe_offset]
                .find("-->")
                .map(|delta| index + 4 + delta + 3)
                .unwrap_or(safe_offset);
            index = end;
            continue;
        }

        if text.as_bytes()[index] != b'<' {
            index = next_char_boundary(text, index);
            continue;
        }

        let Some(end) = find_tag_end(text, index).map(|value| value.min(safe_offset)) else {
            break;
        };
        let tag = &text[index..end];
        if let Some(tag_name) = parsed_tag_name(tag) {
            if is_closing_tag(tag) {
                if let Some(position) = stack.iter().rposition(|open| open == tag_name) {
                    stack.truncate(position);
                }
            } else if !is_self_closing_tag(tag) && !is_void_tag(tag_name) {
                stack.push(tag_name.to_owned());
            }
        }
        index = end;
    }

    stack.pop()
}

/// Advances to the next UTF-8 character boundary after `index`.
fn next_char_boundary(text: &str, index: usize) -> usize {
    text.get(index..)
        .and_then(|tail| tail.chars().next().map(|ch| index + ch.len_utf8()))
        .unwrap_or(text.len())
}

/// Parses the tag name out of an already isolated tag snippet.
fn parsed_tag_name(tag: &str) -> Option<&str> {
    let bytes = tag.as_bytes();
    if bytes.first() != Some(&b'<') {
        return None;
    }

    let mut index = 1;
    if bytes.get(index) == Some(&b'/') {
        index += 1;
    }
    while index < bytes.len() && bytes[index].is_ascii_whitespace() {
        index += 1;
    }

    let start = index;
    while index < bytes.len() && is_name_char(bytes[index]) {
        index += 1;
    }

    (start < index).then(|| &tag[start..index])
}

/// Distinguishes `</tag>` from opening tags.
fn is_closing_tag(tag: &str) -> bool {
    tag.as_bytes().get(1) == Some(&b'/')
}

/// Recognizes explicit self-closing syntax like `<input />`.
fn is_self_closing_tag(tag: &str) -> bool {
    tag.trim_end().ends_with("/>")
}

/// Filters out HTML tags that never require an explicit closing tag.
fn is_void_tag(tag_name: &str) -> bool {
    HTML_VOID_TAGS.contains(&tag_name)
}

/// Suppresses the promoted closing suggestion when the following source code
/// already starts with that same closing tag.
fn has_immediate_closing_tag_ahead(text: &str, offset: usize, tag_name: &str) -> bool {
    let remainder = text[offset.min(text.len())..].trim_start();
    remainder.starts_with(&format!("</{tag_name}"))
}

/// Expands left from the cursor to find the beginning of the current name-like
/// token used for replacement during completion.
fn find_name_start(text: &str, offset: usize) -> usize {
    let bytes = text.as_bytes();
    let mut index = offset.min(bytes.len());
    while index > 0 && is_name_char(bytes[index - 1]) {
        index -= 1;
    }
    index
}

/// Shared HTML name-character predicate for tags and attributes.
fn is_name_char(byte: u8) -> bool {
    byte.is_ascii_alphanumeric() || matches!(byte, b'-' | b'_' | b':')
}
