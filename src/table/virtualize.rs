/// Translate the current scroll window into an overscanned row range to render.
pub(crate) fn visible_range(
    row_count: usize,
    row_height: f64,
    overscan: usize,
    scroll_top: f64,
    viewport_height: f64,
) -> (usize, usize) {
    if row_count == 0 {
        return (0, 0);
    }

    let start = ((scroll_top / row_height).floor() as isize - overscan as isize).max(0) as usize;
    let end =
        (((scroll_top + viewport_height) / row_height).ceil() as usize + overscan).min(row_count);
    (start, end)
}

/// Request more rows only when the viewport is approaching the current tail of loaded data.
pub(crate) fn should_load_more(
    row_count: usize,
    visible_end: usize,
    threshold: usize,
    has_more: bool,
    is_loading: bool,
) -> bool {
    if row_count == 0 || !has_more || is_loading {
        return false;
    }

    visible_end >= row_count.saturating_sub(threshold)
}
