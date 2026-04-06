use web_sys::HtmlElement;

/// Keeps the active completion option inside the popup viewport with a small
/// edge padding so keyboard navigation does not feel cramped.
pub(crate) fn sync_completion_scroll(list: &HtmlElement, option: &HtmlElement) {
    let edge_padding = 6;
    let option_top = option.offset_top();
    let option_bottom = option_top + option.offset_height();
    let view_top = list.scroll_top();
    let view_bottom = view_top + list.client_height();

    if option_top - edge_padding < view_top {
        list.set_scroll_top((option_top - edge_padding).max(0));
    } else if option_bottom + edge_padding > view_bottom {
        list.set_scroll_top(option_bottom + edge_padding - list.client_height());
    }
}
