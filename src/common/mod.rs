// Shared enums and floating-layout helpers live here so components can reuse
// the same sizing and popup behavior without circular dependencies.
mod button_variant;
mod popup;
mod size;

pub use button_variant::ButtonVariant;
pub use popup::TooltipPlacement;
pub(crate) use popup::{
    measure_floating_popup_layout, measure_tooltip_layout, select_menu_theme_style,
    FloatingPopupLayout, FloatingTooltipLayout, FLOATING_POPUP_EDGE_PADDING,
};
pub use size::Size;
