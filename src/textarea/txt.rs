use leptos::ev;
use leptos::prelude::*;
use wasm_bindgen::JsCast;
use web_sys::HtmlElement;

use crate::Size;

/// Multiline text field with the same sizing and focus treatment as [`Input`](crate::Input).
#[component]
pub fn Textarea(
    /// Current textarea value.
    #[prop(optional, into)]
    value: MaybeProp<String>,
    /// Placeholder text shown when the value is empty.
    #[prop(optional, into)]
    placeholder: MaybeProp<String>,
    /// Optional textarea name for form submission.
    #[prop(optional, into)]
    name: Option<String>,
    /// Optional textarea id.
    #[prop(optional, into)]
    id: Option<String>,
    /// Shared sizing token aligned with buttons and inputs.
    #[prop(optional)]
    size: Size,
    /// Number of visible text rows.
    #[prop(optional, default = 4)]
    rows: u32,
    /// Disables the textarea and prevents user interaction.
    #[prop(optional)]
    disabled: bool,
    /// Marks the textarea as read-only.
    #[prop(optional)]
    readonly: bool,
    /// Marks the textarea as invalid for styling and accessibility.
    #[prop(optional)]
    invalid: bool,
    /// Marks the field as required.
    #[prop(optional)]
    required: bool,
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
    // Compose the shared shell classes once so the rendered element follows the same modifier
    // contract as the other form controls in the library.
    let mut classes = vec!["birei-textarea", size.textarea_class_name()];

    if disabled {
        classes.push("birei-textarea--disabled");
    }
    if readonly {
        classes.push("birei-textarea--readonly");
    }
    if invalid {
        classes.push("birei-textarea--invalid");
    }
    if let Some(class) = class.as_deref() {
        classes.push(class);
    }

    let class_name = classes.join(" ");
    // The focus underline animation is driven entirely by a CSS custom property that tracks where
    // the user pressed inside the control shell.
    let line_style = RwSignal::new(String::from("--birei-textarea-line-origin: 50%;"));
    let handle_pointer_down = move |event: ev::PointerEvent| {
        if let Some(target) = event
            .current_target()
            .and_then(|target| target.dyn_into::<HtmlElement>().ok())
        {
            let rect = target.get_bounding_client_rect();
            let x = f64::from(event.client_x()) - rect.left();
            line_style.set(format!("--birei-textarea-line-origin: {x}px;"));
        }
    };

    view! {
        <div
            class=class_name
            style=move || line_style.get()
            on:pointerdown=handle_pointer_down
        >
            <textarea
                class="birei-textarea__field"
                id=id
                name=name
                prop:value=move || value.get()
                placeholder=move || placeholder.get()
                rows=rows
                disabled=disabled
                readonly=readonly
                required=required
                aria-invalid=move || if invalid { "true" } else { "false" }
                on:input=move |event| {
                    // Forward native events unchanged so controlled parents can decide how to store
                    // and validate textarea content.
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
                    // Focus/blur are forwarded separately so forms can hook analytics or validation
                    // without intercepting input/change events.
                    if let Some(on_focus) = on_focus.as_ref() {
                        on_focus.run(event);
                    }
                }
                on:blur=move |event| {
                    if let Some(on_blur) = on_blur.as_ref() {
                        on_blur.run(event);
                    }
                }
            ></textarea>
        </div>
    }
}
