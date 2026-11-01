use wasm_bindgen::JsCast;
use web_sys::{DomRect, Element, HtmlElement};

use super::text::visible_range;
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
