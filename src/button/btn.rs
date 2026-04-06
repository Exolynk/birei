use leptos::ev;
use leptos::prelude::*;
use wasm_bindgen::JsCast;
use web_sys::HtmlElement;

use super::btn_types::{ButtonGroupContext, ButtonType};
use crate::{ButtonVariant, Size};

/// Triggers an action or submits a form.
///
/// `Button` provides a small typed API over a native HTML `<button>` element.
/// Visual styling is controlled through [`ButtonVariant`] and [`Size`],
/// while shared defaults can be inherited from a surrounding
/// [`ButtonGroup`](super::ButtonGroup).
#[component]
pub fn Button(
    /// Content rendered inside the button.
    children: Children,
    /// Visual treatment of the button.
    #[prop(optional)]
    variant: Option<ButtonVariant>,
    /// Size of the button.
    #[prop(optional)]
    size: Option<Size>,
    /// Native HTML `type` attribute for the underlying `<button>`.
    #[prop(optional)]
    button_type: ButtonType,
    /// Disables the button and prevents user interaction.
    #[prop(optional)]
    disabled: Option<bool>,
    /// Expands the button to the full width of its container.
    #[prop(optional)]
    block: bool,
    /// Uses a pill-shaped border radius.
    #[prop(optional)]
    round: bool,
    /// Forces the button into a circular shape.
    #[prop(optional)]
    circle: bool,
    /// Additional CSS class names applied to the root element.
    #[prop(optional, into)]
    class: Option<String>,
    /// Click handler for the underlying button element.
    #[prop(optional)]
    on_click: Option<Callback<ev::MouseEvent>>,
) -> impl IntoView {
    // Group context provides shared defaults so button sets can stay concise.
    let group = use_context::<ButtonGroupContext>();
    let variant = variant
        .or_else(|| group.and_then(|group| group.variant))
        .unwrap_or_default();
    let size = size
        .or_else(|| group.and_then(|group| group.size))
        .unwrap_or_default();
    let disabled = disabled
        .or_else(|| group.and_then(|group| group.disabled))
        .unwrap_or(false);

    // Class assembly mirrors the visual state matrix used by the stylesheet.
    let mut classes = vec![
        "birei-button",
        variant.class_name(),
        size.button_class_name(),
    ];

    if block {
        classes.push("birei-button--block");
    }
    if round {
        classes.push("birei-button--round");
    }
    if circle {
        classes.push("birei-button--circle");
    }
    if disabled {
        classes.push("birei-button--disabled");
    }
    if let Some(class) = class.as_deref() {
        classes.push(class);
    }

    let class_name = classes.join(" ");
    // Ripple state is encoded into CSS variables plus an alternating phase
    // class so repeated clicks always restart the animation.
    let ripple_style = RwSignal::new(String::from(
        "--birei-ripple-x: 50%; --birei-ripple-y: 50%; --birei-ripple-size: 0px;",
    ));
    let ripple_phase = RwSignal::new(None::<bool>);
    let button_class = move || {
        let mut classes = class_name.clone();

        if let Some(phase) = ripple_phase.get() {
            classes.push(' ');
            classes.push_str(if phase {
                "birei-button--ripple-a"
            } else {
                "birei-button--ripple-b"
            });
        }

        classes
    };
    // Click handling both positions the visual ripple and forwards the user
    // callback on the same browser event.
    let handle_click = move |event: ev::MouseEvent| {
        if let Some(target) = event
            .current_target()
            .and_then(|target| target.dyn_into::<HtmlElement>().ok())
        {
            let rect = target.get_bounding_client_rect();
            let x = f64::from(event.client_x()) - rect.left();
            let y = f64::from(event.client_y()) - rect.top();
            let size = rect.width().max(rect.height()) * 1.35;

            ripple_style.set(format!(
                "--birei-ripple-x: {x}px; --birei-ripple-y: {y}px; --birei-ripple-size: {size}px;"
            ));
            ripple_phase.update(|phase| {
                *phase = Some(!phase.unwrap_or(false));
            });
        }

        if let Some(on_click) = on_click.as_ref() {
            on_click.run(event);
        }
    };

    view! {
        <button
            type=button_type.as_str()
            class=button_class
            style=move || ripple_style.get()
            disabled=disabled
            on:click=handle_click
        >
            {children()}
        </button>
    }
}
