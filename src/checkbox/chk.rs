use leptos::ev;
use leptos::prelude::*;
use web_sys::HtmlInputElement;

use crate::Size;

/// Native checkbox with animated checkmark and inline descriptive content.
#[component]
pub fn Checkbox(
    /// Descriptive content rendered to the right of the checkbox control.
    children: Children,
    /// Current checked state.
    #[prop(optional, into)]
    checked: MaybeProp<bool>,
    /// Optional form field name.
    #[prop(optional, into)]
    name: Option<String>,
    /// Optional input id.
    #[prop(optional, into)]
    id: Option<String>,
    /// Optional submitted value when checked.
    #[prop(optional, into)]
    value: Option<String>,
    /// Accessible label used when no external label is associated with the checkbox.
    #[prop(optional, into)]
    aria_label: Option<String>,
    /// Shared sizing token aligned with the rest of the form controls.
    #[prop(optional)]
    size: Size,
    /// Disables the checkbox and prevents interaction.
    #[prop(optional)]
    disabled: bool,
    /// Marks the checkbox as invalid for styling and accessibility.
    #[prop(optional)]
    invalid: bool,
    /// Marks the field as required.
    #[prop(optional)]
    required: bool,
    /// Additional CSS class names applied to the root element.
    #[prop(optional, into)]
    class: Option<String>,
    /// Checked-state callback for controlled usage.
    #[prop(optional)]
    on_checked_change: Option<Callback<bool>>,
    /// Input event handler.
    #[prop(optional)]
    on_input: Option<Callback<ev::Event>>,
    /// Change event handler.
    #[prop(optional)]
    on_change: Option<Callback<ev::Event>>,
    /// Focus event handler.
    #[prop(optional)]
    on_focus: Option<Callback<ev::FocusEvent>>,
    /// Blur event handler.
    #[prop(optional)]
    on_blur: Option<Callback<ev::FocusEvent>>,
) -> impl IntoView {
    let mut classes = vec!["birei-checkbox", size.checkbox_class_name()];

    if disabled {
        classes.push("birei-checkbox--disabled");
    }
    if invalid {
        classes.push("birei-checkbox--invalid");
    }
    if let Some(class) = class.as_deref() {
        classes.push(class);
    }

    let handle_input = move |event: ev::Event| {
        if let Some(on_input) = on_input.as_ref() {
            on_input.run(event);
        }
    };

    let handle_change = move |event: ev::Event| {
        let is_checked = event_target::<HtmlInputElement>(&event).checked();

        if let Some(on_checked_change) = on_checked_change.as_ref() {
            on_checked_change.run(is_checked);
        }
        if let Some(on_change) = on_change.as_ref() {
            on_change.run(event);
        }
    };

    view! {
        <div class=classes.join(" ")>
            <input
                class="birei-checkbox__native"
                id=id
                type="checkbox"
                name=name
                value=value
                aria-label=aria_label
                prop:checked=move || checked.get().unwrap_or(false)
                disabled=disabled
                required=required
                aria-invalid=move || if invalid { "true" } else { "false" }
                on:input=handle_input
                on:change=handle_change
                on:focus=move |event| {
                    if let Some(on_focus) = on_focus.as_ref() {
                        on_focus.run(event);
                    }
                }
                on:blur=move |event| {
                    if let Some(on_blur) = on_blur.as_ref() {
                        on_blur.run(event);
                    }
                }
            />
            <span class="birei-checkbox__control" aria-hidden="true">
                <svg
                    class="birei-checkbox__check"
                    viewBox="0 0 16 16"
                    fill="none"
                    xmlns="http://www.w3.org/2000/svg"
                >
                    <path
                        d="M3.5 8.25L6.5 11.25L12.5 5.25"
                        pathLength="1"
                    />
                </svg>
            </span>
            <span class="birei-checkbox__label">{children()}</span>
        </div>
    }
}
