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

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum TooltipPlacement {
    #[default]
    Top,
    Bottom,
    Left,
    Right,
}

#[derive(Clone, Default)]
pub(crate) struct FloatingTooltipLayout {
    pub(crate) top: f64,
    pub(crate) left: f64,
    pub(crate) placement: TooltipPlacement,
}

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

pub(crate) fn measure_tooltip_layout(
    anchor_rect: &DomRect,
    tooltip_width: f64,
    tooltip_height: f64,
    preferred_placement: TooltipPlacement,
) -> FloatingTooltipLayout {
    let Some(window) = web_sys::window() else {
        return FloatingTooltipLayout {
            top: anchor_rect.top(),
            left: anchor_rect.left(),
            placement: preferred_placement,
        };
    };

    let viewport_width = window
        .inner_width()
        .ok()
        .and_then(|value| value.as_f64())
        .unwrap_or(anchor_rect.right() + tooltip_width);
    let viewport_height = window
        .inner_height()
        .ok()
        .and_then(|value| value.as_f64())
        .unwrap_or(anchor_rect.bottom() + tooltip_height);
    let gap = 8.0_f64;
    let viewport_padding = 8.0_f64;

    let available_above = anchor_rect.top() - viewport_padding;
    let available_below = viewport_height - anchor_rect.bottom() - viewport_padding;
    let available_left = anchor_rect.left() - viewport_padding;
    let available_right = viewport_width - anchor_rect.right() - viewport_padding;

    let placement = match preferred_placement {
        TooltipPlacement::Top
            if available_above < tooltip_height + gap && available_below > available_above =>
        {
            TooltipPlacement::Bottom
        }
        TooltipPlacement::Bottom
            if available_below < tooltip_height + gap && available_above > available_below =>
        {
            TooltipPlacement::Top
        }
        TooltipPlacement::Left
            if available_left < tooltip_width + gap && available_right > available_left =>
        {
            TooltipPlacement::Right
        }
        TooltipPlacement::Right
            if available_right < tooltip_width + gap && available_left > available_right =>
        {
            TooltipPlacement::Left
        }
        other => other,
    };

    let unclamped_left = match placement {
        TooltipPlacement::Top | TooltipPlacement::Bottom => {
            anchor_rect.left() + (anchor_rect.width() - tooltip_width) / 2.0
        }
        TooltipPlacement::Left => anchor_rect.left() - tooltip_width - gap,
        TooltipPlacement::Right => anchor_rect.right() + gap,
    };
    let unclamped_top = match placement {
        TooltipPlacement::Top => anchor_rect.top() - tooltip_height - gap,
        TooltipPlacement::Bottom => anchor_rect.bottom() + gap,
        TooltipPlacement::Left | TooltipPlacement::Right => {
            anchor_rect.top() + (anchor_rect.height() - tooltip_height) / 2.0
        }
    };

    FloatingTooltipLayout {
        top: unclamped_top.clamp(
            viewport_padding,
            (viewport_height - tooltip_height - viewport_padding).max(viewport_padding),
        ),
        left: unclamped_left.clamp(
            viewport_padding,
            (viewport_width - tooltip_width - viewport_padding).max(viewport_padding),
        ),
        placement,
    }
}
