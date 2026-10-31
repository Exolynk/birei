use leptos::html;
use leptos::prelude::*;
use wasm_bindgen::JsCast;
use web_sys::HtmlElement;

pub(super) fn find_command_item_element(
    list: &HtmlElement,
    item_index: usize,
) -> Option<HtmlElement> {
    list.query_selector(&format!(r#"[data-command-index="{item_index}"]"#))
        .ok()
        .flatten()
        .and_then(|element| element.dyn_into::<HtmlElement>().ok())
}

pub(super) fn reset_command_scroll(list_ref: &NodeRef<html::Div>) {
    let list_ref = *list_ref;
    request_animation_frame(move || {
        if let Some(list) = list_ref.try_get_untracked().flatten() {
            list.set_scroll_top(0);
        }
    });
}

pub(super) fn sync_command_scroll(list: &HtmlElement, item: &HtmlElement) {
    let item_top = item.offset_top();
    let item_bottom = item_top + item.offset_height();
    let view_top = list.scroll_top();
    let view_bottom = view_top + list.client_height();
    let padding = 8;

    if item_top - padding < view_top {
        list.set_scroll_top((item_top - padding).max(0));
    } else if item_bottom + padding > view_bottom {
        list.set_scroll_top(item_bottom + padding - list.client_height());
    }
}
