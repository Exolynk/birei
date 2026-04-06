use web_sys::KeyboardEvent;

/// Keys that should never trigger a completion refresh on `keyup`.
/// They either belong to popup interaction or are pure modifier keys.
pub(crate) fn should_skip_completion_refresh(key: &str) -> bool {
    matches!(
        key,
        "Enter" | "Escape" | "Tab" | "Shift" | "Control" | "Alt" | "Meta"
    )
}

/// Matches the platform-standard undo shortcut.
pub(crate) fn is_undo_shortcut(event: &KeyboardEvent) -> bool {
    (event.meta_key() || event.ctrl_key())
        && !event.shift_key()
        && event.key().eq_ignore_ascii_case("z")
}

/// Matches common redo shortcuts across macOS and Windows/Linux.
pub(crate) fn is_redo_shortcut(event: &KeyboardEvent) -> bool {
    (event.meta_key() || event.ctrl_key())
        && ((event.shift_key() && event.key().eq_ignore_ascii_case("z"))
            || (!event.shift_key() && event.key().eq_ignore_ascii_case("y")))
}
