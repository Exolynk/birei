use leptos::ev;
use leptos::prelude::*;
use std::sync::atomic::{AtomicUsize, Ordering};
use wasm_bindgen::JsCast;
use web_sys::HtmlElement;

use super::{InputAutocomplete, InputType};
use crate::{Label, Size};

static NEXT_INPUT_ID: AtomicUsize = AtomicUsize::new(1);

/// Text input with optional prefix/suffix content and animated focus line.
#[component]
pub fn Input(
    /// Current input value.
    #[prop(optional, into)]
    value: MaybeProp<String>,
    /// Placeholder text shown when the value is empty.
    #[prop(optional, into)]
    placeholder: MaybeProp<String>,
    /// Optional input name for form submission.
    #[prop(optional, into)]
    name: Option<String>,
    /// Optional input id.
    #[prop(optional, into)]
    id: Option<String>,
    /// Optional label shown above the input field.
    #[prop(optional, into)]
    label: Option<String>,
    /// Native HTML autocomplete attribute.
    #[prop(optional, into)]
    autocomplete: Option<InputAutocomplete>,
    /// Native HTML input type.
    #[prop(optional)]
    input_type: InputType,
    /// Shared sizing token aligned with button sizes.
    #[prop(optional)]
    size: Size,
    /// Disables the input and prevents user interaction.
    #[prop(optional)]
    disabled: bool,
    /// Marks the input as read-only.
    #[prop(optional)]
    readonly: bool,
    /// Marks the input as invalid for styling and accessibility.
    #[prop(optional)]
    invalid: bool,
    /// Marks the field as required and renders an asterisk in the label.
    #[prop(optional)]
    required: bool,
    /// Prefix content rendered before the field.
    #[prop(optional, into)]
    prefix: Option<ViewFn>,
    /// Suffix content rendered after the field.
    #[prop(optional, into)]
    suffix: Option<ViewFn>,
    /// Additional CSS class names applied to the root element.
    #[prop(optional, into)]
    class: Option<String>,
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
    let mut classes = vec!["birei-input", size.input_class_name()];

    if prefix.is_some() {
        classes.push("birei-input--with-prefix");
    }
    if suffix.is_some() {
        classes.push("birei-input--with-suffix");
    }
    if disabled {
        classes.push("birei-input--disabled");
    }
    if readonly {
        classes.push("birei-input--readonly");
    }
    if invalid {
        classes.push("birei-input--invalid");
    }
    if let Some(class) = class.as_deref() {
        classes.push(class);
    }

    let class_name = classes.join(" ");
    let input_id = id.unwrap_or_else(|| {
        format!(
            "birei-input-{}",
            NEXT_INPUT_ID.fetch_add(1, Ordering::Relaxed)
        )
    });
    let line_style = RwSignal::new(String::from("--birei-input-line-origin: 50%;"));
    let handle_pointer_down = move |event: ev::PointerEvent| {
        if let Some(target) = event
            .current_target()
            .and_then(|target| target.dyn_into::<HtmlElement>().ok())
        {
            let rect = target.get_bounding_client_rect();
            let x = f64::from(event.client_x()) - rect.left();
            line_style.set(format!("--birei-input-line-origin: {x}px;"));
        }
    };
    view! {
        <div class="birei-input-root">
            {label.as_ref().map(|label| {
                view! { <Label text=label.clone() for_id=input_id.clone() required=required/> }
            })}
            <div
                class=class_name
                style=move || line_style.get()
                on:pointerdown=handle_pointer_down
            >
                {prefix.as_ref().map(|prefix| {
                    view! {
                        <span class="birei-input__affix birei-input__affix--prefix">
                            {prefix.run()}
                        </span>
                    }
                })}
                <span class="birei-input__control">
                    <input
                        class="birei-input__field"
                        id=input_id.clone()
                        type=input_type.as_str()
                        name=name
                        autocomplete=autocomplete.map(InputAutocomplete::as_str)
                        prop:value=move || value.get()
                        placeholder=move || placeholder.get()
                        disabled=disabled
                        readonly=readonly
                        required=required
                        aria-invalid=move || if invalid { "true" } else { "false" }
                        on:input=move |event| {
                            if let Some(on_input) = on_input.as_ref() {
                                on_input.run(event);
                            }
                        }
                        on:change=move |event| {
                            if let Some(on_change) = on_change.as_ref() {
                                on_change.run(event);
                            }
                        }
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
                    <span class="birei-input__line" aria-hidden="true"></span>
                </span>
                {suffix.as_ref().map(|suffix| {
                    view! {
                        <span class="birei-input__affix birei-input__affix--suffix">
                            {suffix.run()}
                        </span>
                    }
                })}
            </div>
        </div>
    }
}
