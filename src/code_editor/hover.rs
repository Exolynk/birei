use leptos::prelude::*;
use std::time::Duration;
use wasm_bindgen::JsCast;
use web_sys::{DomRect, Element, HtmlElement, Node};

use super::diagnostics::DiagnosticHover;
use super::text::{escape_html_with_breaks, severity_name};

/// Pause of the mouse pointer over a word before its description is requested.
pub(super) const MOUSE_HOVER_DELAY: Duration = Duration::from_millis(800);

/// Pause of the caret in a word before its description is requested.
pub(super) const CARET_HOVER_DELAY: Duration = Duration::from_millis(1500);

/// What triggered a description request.
#[derive(Clone, Copy, Debug, PartialEq)]
pub(super) enum HoverTrigger {
    /// The mouse pointer rests at these viewport coordinates.
    Mouse { x: f64, y: f64 },
    /// The caret rests at its current position.
    Caret,
}

/// Description of the code element at a position and where its popup is anchored.
#[derive(Clone, Debug, PartialEq)]
pub(super) struct DescriptionHover {
    /// Short signature or category shown above the description.
    pub(super) title: Option<String>,
    /// Longer explanation of the element.
    pub(super) documentation: String,
    /// What showed the description, so moving the other input keeps it open.
    pub(super) trigger: HoverTrigger,
    /// Left edge of the described element in viewport pixels.
    pub(super) left: f64,
    /// Right edge of the described element in viewport pixels.
    pub(super) right: f64,
    /// Top edge of the described element in viewport pixels.
    pub(super) top: f64,
    /// Bottom edge of the described element in viewport pixels.
    pub(super) bottom: f64,
}

impl DescriptionHover {
    /// Whether the viewport point lies on the described element.
    pub(super) fn contains(&self, x: f64, y: f64) -> bool {
        x >= self.left && x <= self.right && y >= self.top && y <= self.bottom + 3.0
    }
}

/// Finds the text byte offset under the viewport point and the box of that character.
///
/// The highlight overlay renders the text with the same layout as the textarea, one element
/// per line, so the character is found from the boxes of the overlay's text.
pub(super) fn offset_at_point(
    content: &HtmlElement,
    text: &str,
    x: f64,
    y: f64,
) -> Option<(usize, DomRect)> {
    let lines = content.children();
    let (line_index, line) = (0..lines.length())
        .filter_map(|index| Some((index as usize, lines.item(index)?)))
        .find(|(_, line)| {
            let rect = line.get_bounding_client_rect();
            y >= rect.top() && y < rect.bottom()
        })?;
    let line_start: usize = text
        .split_inclusive('\n')
        .take(line_index)
        .map(str::len)
        .sum();
    let line_text = text[line_start.min(text.len())..]
        .split('\n')
        .next()
        .unwrap_or_default();
    let document = web_sys::window()?.document()?;
    let range = document.create_range().ok()?;
    let mut char_index = 0usize;
    for node in text_nodes(&line) {
        let length = node
            .text_content()
            .unwrap_or_default()
            .encode_utf16()
            .count() as u32;
        for unit in 0..length {
            range.set_start(&node, unit).ok()?;
            range.set_end(&node, unit + 1).ok()?;
            let rect = range.get_bounding_client_rect();
            if x >= rect.left() && x < rect.right() && rect.width() > 0.0 {
                let byte = utf16_to_byte(line_text, char_index);
                return Some((line_start + byte, rect));
            }
            char_index += 1;
        }
    }
    None
}

/// Returns the text nodes below `element` in document order.
fn text_nodes(element: &Element) -> Vec<Node> {
    let mut nodes = Vec::new();
    let mut pending = vec![element.clone().unchecked_into::<Node>()];
    while let Some(node) = pending.pop() {
        if node.node_type() == Node::TEXT_NODE {
            nodes.push(node);
            continue;
        }
        let children = node.child_nodes();
        for index in (0..children.length()).rev() {
            if let Some(child) = children.item(index) {
                pending.push(child);
            }
        }
    }
    nodes
}

/// Converts a UTF-16 unit index of `line` into a byte offset.
fn utf16_to_byte(line: &str, units: usize) -> usize {
    let mut counted = 0usize;
    for (byte, character) in line.char_indices() {
        if counted >= units {
            return byte;
        }
        counted += character.len_utf16();
    }
    line.len()
}

/// Measures the box of the text range `start..end` in the hidden mirror of the textarea.
pub(super) fn measure_range(
    measure_content: &HtmlElement,
    text: &str,
    start: usize,
    end: usize,
) -> Option<DomRect> {
    let start = floor_char_boundary(text, start);
    let end = floor_char_boundary(text, end.max(start));
    measure_content.set_inner_html(&format!(
        r#"{}<span class="birei-code-editor__measure-caret">{}</span>"#,
        escape_html_with_breaks(&text[..start]),
        escape_html_with_breaks(&text[start..end]),
    ));
    let marker = measure_content
        .query_selector(".birei-code-editor__measure-caret")
        .ok()??;
    Some(marker.get_bounding_client_rect())
}

/// Moves `offset` back onto a character boundary of `text`.
fn floor_char_boundary(text: &str, offset: usize) -> usize {
    let mut offset = offset.min(text.len());
    while !text.is_char_boundary(offset) {
        offset -= 1;
    }
    offset
}

/// Popup with the diagnostics and the description at the hovered place.
///
/// It is anchored at the diagnostics when there are any, otherwise at the described element.
#[component]
pub(super) fn HoverPopup(
    diagnostics: RwSignal<Option<DiagnosticHover>>,
    description: RwSignal<Option<DescriptionHover>>,
) -> impl IntoView {
    let style = move || {
        let anchor = diagnostics
            .try_get()
            .flatten()
            .map(|hover| (hover.left, hover.top, hover.bottom))
            .or_else(|| {
                description
                    .try_get()
                    .flatten()
                    .map(|hover| (hover.left, hover.top, hover.bottom))
            });
        let Some((left, top, bottom)) = anchor else {
            return String::new();
        };
        let viewport_height = web_sys::window()
            .and_then(|window| window.inner_height().ok())
            .and_then(|height| height.as_f64())
            .unwrap_or(f64::MAX);
        // Opens above the element when there is little room left below it.
        if viewport_height - bottom < 200.0 {
            format!(
                "left: {left}px; top: {}px; transform: translateY(-100%);",
                top - 4.0
            )
        } else {
            format!("left: {left}px; top: {}px;", bottom + 4.0)
        }
    };

    view! {
        <div class="birei-code-editor__diagnostic-popup" role="tooltip" style=style>
            {move || {
                diagnostics
                    .try_get()
                    .flatten()
                    .map(|hover| hover.messages)
                    .unwrap_or_default()
                    .into_iter()
                    .map(|(severity, message)| {
                        view! {
                            <p class=format!(
                                "birei-code-editor__diagnostic-message birei-code-editor__diagnostic-message--{}",
                                severity_name(severity)
                            )>
                                {message}
                            </p>
                        }
                    })
                    .collect_view()
            }}
            {move || {
                description.try_get().flatten().map(|hover| {
                    view! {
                        <div class="birei-code-editor__description">
                            {hover.title.map(|title| view! {
                                <p class="birei-code-editor__description-title">{title}</p>
                            })}
                            <p class="birei-code-editor__description-text">{hover.documentation}</p>
                        </div>
                    }
                })
            }}
        </div>
    }
}
