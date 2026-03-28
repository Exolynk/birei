use web_sys::{CssStyleDeclaration, DomRect};

#[derive(Clone, Default)]
pub(crate) struct FloatingPopupLayout {
    pub(crate) top: f64,
    pub(crate) left: f64,
    pub(crate) width: f64,
    pub(crate) max_height: f64,
    pub(crate) open_upward: bool,
}

pub(crate) const FLOATING_POPUP_EDGE_PADDING: i32 = 8;

pub(crate) fn measure_floating_popup_layout(rect: &DomRect) -> FloatingPopupLayout {
    let Some(window) = web_sys::window() else {
        return FloatingPopupLayout::default();
    };

    let viewport_height = window
        .inner_height()
        .ok()
        .and_then(|value| value.as_f64())
        .unwrap_or(rect.bottom() + 320.0);
    let gap = 7.2_f64;
    let viewport_padding = 8.0_f64;
    let available_below = (viewport_height - rect.bottom() - gap - viewport_padding).max(0.0);
    let available_above = (rect.top() - gap - viewport_padding).max(0.0);
    let open_upward = available_below < 192.0 && available_above > available_below;
    let max_height = if open_upward {
        available_above.min(256.0)
    } else {
        available_below.min(256.0)
    }
    .max(96.0);

    FloatingPopupLayout {
        top: if open_upward {
            rect.top() - gap
        } else {
            rect.bottom() + gap
        },
        left: rect.left(),
        width: rect.width(),
        max_height,
        open_upward,
    }
}

pub(crate) fn select_menu_theme_style(computed_style: &CssStyleDeclaration) -> String {
    format!(
        "--birei-select-menu-bg: {}; --birei-select-menu-border: {}; --birei-select-option-hover: {}; --birei-select-option-selected: {}; --birei-select-placeholder: {}; --birei-select-scrollbar-thumb: {}; --birei-select-scrollbar-thumb-hover: {};",
        computed_style
            .get_property_value("--birei-select-menu-bg")
            .unwrap_or_default(),
        computed_style
            .get_property_value("--birei-select-menu-border")
            .unwrap_or_default(),
        computed_style
            .get_property_value("--birei-select-option-hover")
            .unwrap_or_default(),
        computed_style
            .get_property_value("--birei-select-option-selected")
            .unwrap_or_default(),
        computed_style
            .get_property_value("--birei-select-placeholder")
            .unwrap_or_default(),
        computed_style
            .get_property_value("--birei-select-scrollbar-thumb")
            .unwrap_or_default(),
        computed_style
            .get_property_value("--birei-select-scrollbar-thumb-hover")
            .unwrap_or_default(),
    )
}

pub(crate) fn dropdown_menu_theme_style(computed_style: &CssStyleDeclaration) -> String {
    format!(
        "--birei-dropdown-menu-bg: {}; --birei-dropdown-menu-border: {}; --birei-dropdown-item-hover: {}; --birei-dropdown-item-active: {}; --birei-dropdown-scrollbar-thumb: {}; --birei-dropdown-scrollbar-thumb-hover: {};",
        computed_style
            .get_property_value("--birei-dropdown-menu-bg")
            .unwrap_or_default(),
        computed_style
            .get_property_value("--birei-dropdown-menu-border")
            .unwrap_or_default(),
        computed_style
            .get_property_value("--birei-dropdown-item-hover")
            .unwrap_or_default(),
        computed_style
            .get_property_value("--birei-dropdown-item-active")
            .unwrap_or_default(),
        computed_style
            .get_property_value("--birei-dropdown-scrollbar-thumb")
            .unwrap_or_default(),
        computed_style
            .get_property_value("--birei-dropdown-scrollbar-thumb-hover")
            .unwrap_or_default(),
    )
}
