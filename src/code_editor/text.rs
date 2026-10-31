use web_sys::HtmlTextAreaElement;

use super::types::{CodeCursor, CodeSelection, HighlightSpan, TextEdit};

/// Reads the current DOM selection from the textarea into the editor's
/// serializable selection type.
pub(crate) fn current_selection(textarea: &HtmlTextAreaElement) -> CodeSelection {
    let text = textarea.value();
    let start_utf16 = textarea.selection_start().ok().flatten().unwrap_or(0) as usize;
    let end_utf16 = textarea
        .selection_end()
        .ok()
        .flatten()
        .unwrap_or(start_utf16 as u32) as usize;
    let start = utf16_offset_to_byte_index(&text, start_utf16);
    let end = utf16_offset_to_byte_index(&text, end_utf16);
    CodeSelection { start, end }
}

/// Converts a byte offset into user-facing line/column data for services.
pub(crate) fn cursor_from_text(text: &str, offset: usize) -> CodeCursor {
    let safe_offset = clamp_byte_offset(text, offset);
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
    let end = clamp_byte_offset(text, selection.end);
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
    let safe_cursor = clamp_byte_offset(text, cursor);
    let line = &text[start..safe_cursor];
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

/// Renders highlighted text as one HTML fragment per line for the mirrored overlay.
///
/// Spans crossing a line break are split, so every line is self-contained and the overlay
/// only has to replace the lines that changed. A trailing newline yields an empty last line,
/// matching the textarea.
pub(crate) fn render_highlight_lines(text: &str, spans: &[HighlightSpan]) -> Vec<String> {
    let mut spans = spans
        .iter()
        .filter(|span| span.range.start < span.range.end)
        .map(|span| {
            (
                clamp_byte_offset(text, span.range.start),
                clamp_byte_offset(text, span.range.end),
                span.class_name,
            )
        })
        .peekable();
    let mut lines = Vec::new();
    let mut line_start = 0usize;

    for line in text.split('\n') {
        let line_end = line_start + line.len();
        let mut html = String::new();
        let mut cursor = line_start;
        // Spans are ordered; a span reaching into the next line stays queued for it.
        while let Some(&(start, end, class_name)) = spans.peek() {
            if start >= line_end {
                break;
            }
            let from = start.max(cursor);
            let to = end.min(line_end);
            if from < to {
                html.push_str(&escape_html(&text[cursor..from]));
                html.push_str(r#"<span class=""#);
                html.push_str(class_name);
                html.push_str(r#"">"#);
                html.push_str(&escape_html(&text[from..to]));
                html.push_str("</span>");
                cursor = to;
            }
            if end > line_end {
                break;
            }
            spans.next();
        }
        html.push_str(&escape_html(&text[cursor..line_end]));
        lines.push(html);
        line_start = line_end + 1;
    }

    lines
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
    text[..clamp_byte_offset(text, offset)]
        .rfind('\n')
        .map(|index| index + 1)
        .unwrap_or(0)
}

/// Converts an internal byte offset into the UTF-16 code-unit offset expected
/// by browser textarea selection APIs.
pub(crate) fn byte_index_to_utf16_offset(text: &str, byte_index: usize) -> u32 {
    let safe_index = clamp_byte_offset(text, byte_index);
    text[..safe_index].encode_utf16().count() as u32
}

/// Converts a browser UTF-16 textarea selection offset into a valid Rust byte
/// offset so all subsequent string slicing stays on char boundaries.
fn utf16_offset_to_byte_index(text: &str, utf16_offset: usize) -> usize {
    let mut utf16_units = 0usize;

    for (byte_index, ch) in text.char_indices() {
        if utf16_units >= utf16_offset {
            return byte_index;
        }
        utf16_units += ch.len_utf16();
        if utf16_units > utf16_offset {
            return byte_index;
        }
    }

    text.len()
}

/// Clamps an arbitrary byte offset onto the nearest valid UTF-8 character
/// boundary at or before the requested index.
fn clamp_byte_offset(text: &str, offset: usize) -> usize {
    let mut safe_offset = offset.min(text.len());
    while safe_offset > 0 && !text.is_char_boundary(safe_offset) {
        safe_offset -= 1;
    }
    safe_offset
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

#[cfg(test)]
mod tests {
    use super::render_highlight_lines;
    use crate::code_editor::HighlightSpan;

    /// Each line is rendered on its own, spans crossing lines are split.
    #[test]
    fn splits_spans_at_line_breaks() {
        let text = "a /* x\ny */ b\n";
        let spans = [HighlightSpan {
            range: 2..11,
            class_name: "c",
        }];
        assert_eq!(
            render_highlight_lines(text, &spans),
            vec![
                r#"a <span class="c">/* x</span>"#.to_string(),
                r#"<span class="c">y */</span> b"#.to_string(),
                String::new(),
            ]
        );
    }

    /// Text is escaped and lines without spans stay plain.
    #[test]
    fn escapes_plain_lines() {
        assert_eq!(
            render_highlight_lines("<a>\n&", &[]),
            vec!["&lt;a&gt;".to_string(), "&amp;".to_string()]
        );
    }
}
