use leptos::prelude::*;

use super::btn_types::ButtonGroupContext;
use crate::{ButtonVariant, Size};

/// Groups related buttons and shares common button props with its children.
///
/// A group can provide inherited [`ButtonVariant`], [`Size`], and
/// disabled state values to nested [`Button`](super::Button) components.
#[component]
pub fn ButtonGroup(
    /// Buttons or other content rendered inside the group container.
    children: Children,
    /// Shared button variant inherited by child buttons unless overridden.
    #[prop(optional)]
    variant: Option<ButtonVariant>,
    /// Shared button size inherited by child buttons unless overridden.
    #[prop(optional)]
    size: Option<Size>,
    /// Shared disabled state inherited by child buttons unless overridden.
    #[prop(optional)]
    disabled: Option<bool>,
    /// Stacks buttons vertically instead of horizontally.
    #[prop(optional)]
    vertical: bool,
    /// Additional CSS class names applied to the group container.
    #[prop(optional, into)]
    class: Option<String>,
) -> impl IntoView {
    provide_context(ButtonGroupContext {
        variant,
        size,
        disabled,
    });

    let mut classes = vec!["birei-button-group"];

    if vertical {
        classes.push("birei-button-group--vertical");
    }
    if let Some(class) = class.as_deref() {
        classes.push(class);
    }

    view! {
        <div class=classes.join(" ") role="group">
            {children()}
        </div>
    }
}
