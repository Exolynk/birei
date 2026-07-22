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

#[cfg(test)]
mod tests {
    use super::{should_load_more, visible_range};

    /// Verifies the range includes an overscan buffer without exceeding loaded rows.
    #[test]
    fn visible_range_is_bounded_and_overscanned() {
        assert_eq!(visible_range(100, 48.0, 2, 240.0, 144.0), (3, 10));
    }

    /// Verifies loading begins only when the virtual range approaches an available tail.
    #[test]
    fn load_more_requires_an_available_nearby_tail() {
        assert!(should_load_more(100, 96, 6, true, false));
        assert!(!should_load_more(100, 93, 6, true, false));
        assert!(!should_load_more(100, 100, 6, false, false));
        assert!(!should_load_more(100, 100, 6, true, true));
    }
}
