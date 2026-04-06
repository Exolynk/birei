use web_sys::HtmlTextAreaElement;

use super::types::{CodeCursor, CodeSelection, HighlightSpan, TextEdit};

/// Reads the current DOM selection from the textarea into the editor's
/// serializable selection type.
pub(crate) fn current_selection(textarea: &HtmlTextAreaElement) -> CodeSelection {
    let start = textarea.selection_start().ok().flatten().unwrap_or(0) as usize;
    let end = textarea
        .selection_end()
        .ok()
        .flatten()
        .unwrap_or(start as u32) as usize;
    CodeSelection { start, end }
}

/// Converts a byte offset into user-facing line/column data for services.
pub(crate) fn cursor_from_text(text: &str, offset: usize) -> CodeCursor {
    let safe_offset = offset.min(text.len());
    let before = &text[..safe_offset];
    let line = before.bytes().filter(|byte| *byte == b'\n').count() + 1;
    let column = before
        .rsplit('\n')
        .next()
        .unwrap_or_default()
        .chars()
        .count()
        + 1;
    CodeCursor {
        offset: safe_offset,
        line,
        column,
    }
}

/// Indents or outdents every line touched by the current selection.
pub(crate) fn indent_selection(
    text: &str,
    selection: CodeSelection,
    indent: &str,
    shift: bool,
) -> TextEdit {
    let start = line_start(text, selection.start);
    let end = selection.end.min(text.len());
    let line_end = text[end..]
        .find('\n')
        .map(|index| end + index)
        .unwrap_or(text.len());
    let block = &text[start..line_end];
    let mut updated = String::new();

    for (index, line) in block.split('\n').enumerate() {
        if index > 0 {
            updated.push('\n');
        }
        if shift {
            updated.push_str(line.strip_prefix(indent).unwrap_or(line));
        } else {
            updated.push_str(indent);
            updated.push_str(line);
        }
    }

    let cursor = if shift {
        selection.end.saturating_sub(indent.len())
    } else {
        selection.end + indent.len()
    };

    let next_cursor = cursor.min(start + updated.len());

    TextEdit {
        range: start..line_end,
        replacement: updated,
        cursor: Some(next_cursor),
    }
}

/// Removes one indentation unit from the current line when shift-tab is used
/// without a multi-line selection.
pub(crate) fn outdent_at_cursor(text: &str, cursor: usize, indent: &str) -> TextEdit {
    let start = line_start(text, cursor);
    let line = &text[start..cursor.min(text.len())];
    let removed = line
        .chars()
        .take_while(|ch| *ch == ' ')
        .count()
        .min(indent.len());
    TextEdit {
        range: start..start + removed,
        replacement: String::new(),
        cursor: Some(cursor.saturating_sub(removed)),
    }
}

/// Renders escaped text plus highlight spans into the HTML consumed by the
/// mirrored highlight layer.
pub(crate) fn render_highlight_html(text: &str, spans: &[HighlightSpan]) -> String {
    let mut html = String::new();
    let mut cursor = 0usize;

    for span in spans.iter().filter(|span| span.range.start < span.range.end) {
        let start = span.range.start.min(text.len());
        let end = span.range.end.min(text.len());
        if start < cursor || end <= start {
            continue;
        }
        html.push_str(&escape_html(&text[cursor..start]));
        html.push_str(r#"<span class=""#);
        html.push_str(span.class_name);
        html.push_str(r#"">"#);
        html.push_str(&escape_html(&text[start..end]));
        html.push_str("</span>");
        cursor = end;
    }

    html.push_str(&escape_html(&text[cursor..]));
    if text.ends_with('\n') {
        html.push(' ');
    }
    html
}

/// Escapes raw text for the hidden measurement layer and converts whitespace
/// into browser-visible layout tokens.
pub(crate) fn escape_html_with_breaks(text: &str) -> String {
    let mut escaped = escape_html(text);
    if escaped.ends_with('\n') {
        escaped.push(' ');
    }
    escaped = escaped.replace('\t', "&nbsp;&nbsp;");
    escaped = escaped.replace(' ', "&nbsp;");
    escaped = escaped.replace('\n', "<br>");
    escaped
}

/// Finds the first offset of the current line for indentation edits.
fn line_start(text: &str, offset: usize) -> usize {
    text[..offset.min(text.len())]
        .rfind('\n')
        .map(|index| index + 1)
        .unwrap_or(0)
}

/// Minimal HTML escaping used by both highlight and measurement rendering.
fn escape_html(text: &str) -> String {
    let mut escaped = String::with_capacity(text.len());
    for ch in text.chars() {
        match ch {
            '&' => escaped.push_str("&amp;"),
            '<' => escaped.push_str("&lt;"),
            '>' => escaped.push_str("&gt;"),
            '"' => escaped.push_str("&quot;"),
            '\'' => escaped.push_str("&#39;"),
            _ => escaped.push(ch),
        }
    }
    escaped
}
