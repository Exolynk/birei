use leptos::{html, prelude::*};
use wasm_bindgen::JsCast;
use web_sys::HtmlElement;

use super::menu::SELECT_OPTION_ROW_HEIGHT;
use crate::common::{
    measure_floating_popup_layout, FloatingPopupLayout, FLOATING_POPUP_EDGE_PADDING,
};

/// Narrowest menu width in rem, so options of narrow fields such as table cells stay readable.
const SELECT_MENU_MIN_WIDTH_REM: f64 = 14.0;
/// Width kept free for the `+N` chip when tags do not fit on one line.
const SELECT_MORE_CHIP_RESERVE: f64 = 40.0;
/// Narrowest width at which the first tag stays readable next to the `+N` chip.
const SELECT_MIN_VISIBLE_TAG: f64 = 48.0;

/// Updates the portaled menu position from the trigger surface.
pub(super) fn update_menu_layout(
    surface_ref: &NodeRef<html::Div>,
    menu_layout: RwSignal<FloatingPopupLayout>,
) {
    let Some(surface) = surface_ref.try_get_untracked().flatten() else {
        return;
    };
    let mut layout = measure_floating_popup_layout(&surface.get_bounding_client_rect());
    widen_menu_layout(&mut layout);
    let _ = menu_layout.try_set(layout);
}

/// Keeps the active popup option inside the visible scroll area.
pub(super) fn sync_menu_scroll_to_index(menu: &HtmlElement, option_index: usize) {
    let option_top = (option_index as f64 * SELECT_OPTION_ROW_HEIGHT) as i32;
    let option_bottom = option_top + SELECT_OPTION_ROW_HEIGHT as i32;
    let view_top = menu.scroll_top();
    let view_bottom = view_top + menu.client_height();

    if option_top - FLOATING_POPUP_EDGE_PADDING < view_top {
        menu.set_scroll_top((option_top - FLOATING_POPUP_EDGE_PADDING).max(0));
    } else if option_bottom + FLOATING_POPUP_EDGE_PADDING > view_bottom {
        menu.set_scroll_top(option_bottom + FLOATING_POPUP_EDGE_PADDING - menu.client_height());
    }
}

/// Widens a menu narrower than the minimum menu width and keeps it inside the viewport.
pub(super) fn widen_menu_layout(layout: &mut FloatingPopupLayout) {
    let Some(window) = web_sys::window() else {
        return;
    };
    let rem = window
        .document()
        .and_then(|document| document.document_element())
        .and_then(|root| window.get_computed_style(&root).ok().flatten())
        .and_then(|style| style.get_property_value("font-size").ok())
        .and_then(|size| size.trim_end_matches("px").parse::<f64>().ok())
        .unwrap_or(16.0);
    let min_width = SELECT_MENU_MIN_WIDTH_REM * rem;
    if layout.width >= min_width {
        return;
    }
    layout.width = min_width;
    let padding = f64::from(FLOATING_POPUP_EDGE_PADDING);
    if let Some(viewport_width) = window.inner_width().ok().and_then(|width| width.as_f64()) {
        layout.left = layout
            .left
            .min(viewport_width - padding - min_width)
            .max(padding);
    }
}

/// Moves an open menu left when its rendered box, including padding and border, reaches past
/// the right edge of the viewport.
pub(super) fn keep_menu_in_viewport(
    menu_ref: NodeRef<html::Div>,
    menu_layout: RwSignal<FloatingPopupLayout>,
) {
    let Some(menu) = menu_ref.try_get_untracked().flatten() else {
        return;
    };
    let Some(viewport_width) = web_sys::window()
        .and_then(|window| window.document())
        .and_then(|document| document.document_element())
        .map(|root| f64::from(root.client_width()))
    else {
        return;
    };
    let padding = f64::from(FLOATING_POPUP_EDGE_PADDING);
    let overflow = menu.get_bounding_client_rect().right() - (viewport_width - padding);
    if overflow > 0.5 {
        let _ =
            menu_layout.try_update(|layout| layout.left = (layout.left - overflow).max(padding));
    }
}

/// Hides the selected tags that do not fit on a single line of the control and records their
/// labels for the `+N` chip. The first tag stays visible unless the control is too narrow for
/// it and the chip; a control whose tags wrap onto further lines never overflows and hides none.
pub(super) fn fit_selected_tags(
    control_ref: NodeRef<html::Span>,
    hidden_tag_labels: RwSignal<Vec<String>>,
) {
    let Some(control) = control_ref.try_get_untracked().flatten() else {
        return;
    };
    let Ok(nodes) = control.query_selector_all(":scope > .birei-tag") else {
        return;
    };
    let tags = (0..nodes.length())
        .filter_map(|index| nodes.item(index))
        .filter_map(|node| node.dyn_into::<HtmlElement>().ok())
        .collect::<Vec<_>>();
    for tag in &tags {
        let _ = tag.remove_attribute("hidden");
    }
    // Measure the tags at their natural width, so a shrunk first tag does not make room for
    // the following ones.
    let _ = control.set_attribute("data-measuring", "");

    let mut hidden = Vec::new();
    if control.scroll_width() > control.client_width() {
        let rect = control.get_bounding_client_rect();
        let limit = rect.right() - SELECT_MORE_CHIP_RESERVE;
        let first_hidden = if rect.width() < SELECT_MIN_VISIBLE_TAG + SELECT_MORE_CHIP_RESERVE {
            Some(0)
        } else {
            tags.iter()
                .skip(1)
                .position(|tag| tag.get_bounding_client_rect().right() > limit)
                .map(|index| index + 1)
        };
        if let Some(first) = first_hidden {
            for tag in tags.iter().skip(first) {
                let _ = tag.set_attribute("hidden", "");
                let label = tag
                    .first_element_child()
                    .and_then(|label| label.text_content())
                    .unwrap_or_default();
                hidden.push(label);
            }
        }
    }
    let _ = control.remove_attribute("data-measuring");
    if hidden_tag_labels.try_get_untracked().as_ref() != Some(&hidden) {
        let _ = hidden_tag_labels.try_set(hidden);
    }
}
