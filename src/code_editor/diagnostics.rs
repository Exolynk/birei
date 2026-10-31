use leptos::prelude::*;
use wasm_bindgen::JsCast;
use web_sys::{DomRect, Element, HtmlElement};

use super::text::{severity_name, visible_range};
use super::types::{CodeDiagnostic, CodeDiagnosticSeverity};

/// Diagnostics under the mouse pointer and where their popup is anchored.
#[derive(Clone, Debug, PartialEq)]
pub(super) struct DiagnosticHover {
    /// Severity and message of every diagnostic at the hovered place.
    pub(super) messages: Vec<(CodeDiagnosticSeverity, String)>,
    /// Left edge of the hovered underline in viewport pixels.
    pub(super) left: f64,
    /// Top edge of the hovered underline in viewport pixels.
    pub(super) top: f64,
    /// Bottom edge of the hovered underline in viewport pixels.
    pub(super) bottom: f64,
}

/// Finds the diagnostics whose underline lies under the pointer at `x`/`y`.
///
/// The underlines are rendered below the textarea, so the pointer is tested against their
/// boxes instead of relying on pointer events. All diagnostics overlapping the hovered
/// underline are reported, also those hidden by an overlapping underline.
pub(super) fn hovered_diagnostics(
    content: &HtmlElement,
    x: f64,
    y: f64,
    text: &str,
    diagnostics: &[CodeDiagnostic],
) -> Option<DiagnosticHover> {
    let marks = content.query_selector_all("[data-diagnostic]").ok()?;
    let (index, rect) = (0..marks.length())
        .filter_map(|position| marks.item(position))
        .filter_map(|node| node.dyn_into::<Element>().ok())
        .find_map(|mark| {
            let rect = hit_rect(&mark, x, y)?;
            let index = mark
                .get_attribute("data-diagnostic")?
                .parse::<usize>()
                .ok()?;
            Some((index, rect))
        })?;
    let hovered = visible_range(text, &diagnostics.get(index)?.range);
    let messages = diagnostics
        .iter()
        .filter(|diagnostic| {
            let (start, end) = visible_range(text, &diagnostic.range);
            start < hovered.1 && hovered.0 < end
        })
        .map(|diagnostic| (diagnostic.severity, diagnostic.message.clone()))
        .collect();
    Some(DiagnosticHover {
        messages,
        left: rect.left(),
        top: rect.top(),
        bottom: rect.bottom(),
    })
}

/// Returns the box of `mark` when it contains the point, with a little room below for the
/// wave. Marks never span lines, so one box describes them.
fn hit_rect(mark: &Element, x: f64, y: f64) -> Option<DomRect> {
    let rect = mark.get_bounding_client_rect();
    (x >= rect.left() && x <= rect.right() && y >= rect.top() && y <= rect.bottom() + 3.0)
        .then_some(rect)
}

/// Popup listing the diagnostics at the hovered underline, styled like the completion popup.
#[component]
pub(super) fn DiagnosticPopup(hover: RwSignal<Option<DiagnosticHover>>) -> impl IntoView {
    let style = move || {
        let Some(hover) = hover.try_get().flatten() else {
            return String::new();
        };
        let viewport_height = web_sys::window()
            .and_then(|window| window.inner_height().ok())
            .and_then(|height| height.as_f64())
            .unwrap_or(f64::MAX);
        // Opens above the underline when there is little room left below it.
        if viewport_height - hover.bottom < 160.0 {
            format!(
                "left: {}px; top: {}px; transform: translateY(-100%);",
                hover.left,
                hover.top - 4.0
            )
        } else {
            format!("left: {}px; top: {}px;", hover.left, hover.bottom + 4.0)
        }
    };

    view! {
        <div class="birei-code-editor__diagnostic-popup" role="tooltip" style=style>
            {move || {
                hover
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
        </div>
    }
}
