use leptos::ev;
use leptos::html;
use leptos::prelude::*;
use web_sys::HtmlInputElement;

use crate::{Button, ButtonVariant, Icon, Input, Size};

/// Color input composed from the shared text input shell plus a native color picker.
#[component]
pub fn ColorInput(
    /// Current hex color value.
    #[prop(optional, into)]
    value: MaybeProp<String>,
    /// Placeholder text shown when the value is empty.
    #[prop(optional, into)]
    placeholder: MaybeProp<String>,
    /// Optional input name for form submission.
    #[prop(optional, into)]
    name: Option<String>,
    /// Optional input id for the visible text field.
    #[prop(optional, into)]
    id: Option<String>,
    /// Shared sizing token aligned with the rest of the form controls.
    #[prop(optional)]
    size: Size,
    /// Disables the field and picker trigger.
    #[prop(optional)]
    disabled: bool,
    /// Marks the field as read-only.
    #[prop(optional)]
    readonly: bool,
    /// Marks the field as invalid for styling and accessibility.
    #[prop(optional)]
    invalid: bool,
    /// Marks the field as required.
    #[prop(optional)]
    required: bool,
    /// Additional CSS class names applied to the root element.
    #[prop(optional, into)]
    class: Option<String>,
    /// Value change callback for controlled usage.
    #[prop(optional)]
    on_value_change: Option<Callback<String>>,
    /// Input event handler for the visible text field.
    #[prop(optional)]
    on_input: Option<Callback<ev::Event>>,
    /// Change event handler for the visible text field.
    #[prop(optional)]
    on_change: Option<Callback<ev::Event>>,
) -> impl IntoView {
    // Two hidden native color inputs are used: one for the preview swatch and
    // one for the explicit trigger button. Both feed the same controlled value.
    let preview_picker_ref = NodeRef::<html::Input>::new();
    let trigger_picker_ref = NodeRef::<html::Input>::new();
    let text_input_id = id.clone().unwrap_or_default();
    // The visible text field remains the source of truth while the native
    // picker stays synchronized through normalized hex values.
    let current_value = move || value.get().unwrap_or_default();
    let normalized_color = move || normalize_hex_color(&current_value());
    let preview_color = move || {
        normalized_color()
            .map(|color| color.preview_css)
            .unwrap_or("#ffffff".into())
    };
    let picker_value = move || {
        normalized_color()
            .map(|color| color.picker_hex)
            .unwrap_or("#ffffff".into())
    };
    let auto_invalid = Memo::new(move |_| {
        let current = current_value();
        invalid || (!current.trim().is_empty() && normalized_color().is_none())
    });

    // Preview clicks open the native picker without exposing the browser's
    // default color-input chrome directly.
    let open_preview_picker = move |_| {
        if disabled || readonly {
            return;
        }

        if let Some(input) = preview_picker_ref.get_untracked() {
            input.click();
        }
    };

    // The explicit palette trigger opens the same native picker affordance
    // from the field suffix button.
    let open_trigger_picker = move |_| {
        if disabled || readonly {
            return;
        }

        if let Some(input) = trigger_picker_ref.get_untracked() {
            input.click();
        }
    };

    // Text entry fans out to the controlled value callback plus any raw input
    // listener supplied by the consumer.
    let handle_text_input = move |event: ev::Event| {
        let next = event_target::<HtmlInputElement>(&event).value();

        if let Some(on_value_change) = on_value_change.as_ref() {
            on_value_change.run(next);
        }
        if let Some(on_input) = on_input.as_ref() {
            on_input.run(event);
        }
    };

    // Change events mirror the same controlled update path for consumers that
    // only listen to committed changes.
    let handle_text_change = move |event: ev::Event| {
        let next = event_target::<HtmlInputElement>(&event).value();

        if let Some(on_value_change) = on_value_change.as_ref() {
            on_value_change.run(next);
        }
        if let Some(on_change) = on_change.as_ref() {
            on_change.run(event);
        }
    };

    // Picker changes are normalized as plain string values and forwarded
    // through the shared value-change callback.
    let handle_picker_input = move |event: ev::Event| {
        let next = event_target::<HtmlInputElement>(&event).value();

        if let Some(on_value_change) = on_value_change.as_ref() {
            on_value_change.run(next);
        }
    };

    // The wrapper only carries an optional external hook class; all stateful
    // appearance comes from the shared input shell.
    let mut classes = vec!["birei-color-input"];
    if let Some(class) = class.as_deref() {
        classes.push(class);
    }

    view! {
        <div class=classes.join(" ")>
            <input
                node_ref=preview_picker_ref
                class="birei-color-input__native birei-color-input__native--preview"
                type="color"
                tabindex="-1"
                disabled=disabled || readonly
                prop:value=picker_value
                on:input=handle_picker_input
                on:change=handle_picker_input
            />
            <input
                node_ref=trigger_picker_ref
                class="birei-color-input__native birei-color-input__native--trigger"
                type="color"
                name=name
                tabindex="-1"
                disabled=disabled || readonly
                prop:value=picker_value
                on:input=handle_picker_input
                on:change=handle_picker_input
            />
            <Input
                id=text_input_id
                value=value
                placeholder=placeholder
                size=size
                disabled=disabled
                readonly=readonly
                invalid=auto_invalid
                required=required
                class="birei-color-input__field"
                on_input=Callback::new(handle_text_input)
                on_change=Callback::new(handle_text_change)
                prefix=move || {
                    view! {
                        <button
                            type="button"
                            class="birei-color-input__preview"
                            style=move || format!("--birei-color-input-preview: {};", preview_color())
                            aria-label="Open color picker"
                            disabled=disabled || readonly
                            on:click=open_preview_picker
                        >
                            <span class="birei-color-input__preview-swatch" aria-hidden="true"></span>
                        </button>
                    }
                }
                suffix=move || {
                    view! {
                        <Button
                            size=Size::Small
                            variant=ButtonVariant::Transparent
                            disabled=disabled || readonly
                            on_click=Callback::new(open_trigger_picker)
                            class="birei-color-input__trigger"
                        >
                            <Icon name="palette" label="Open color palette"/>
                        </Button>
                    }
                }
            />
        </div>
    }
}

/// Parsed color data separates the browser picker hex value from the preview
/// CSS string, which may need alpha expressed as `rgba(...)`.
struct ParsedColor {
    preview_css: String,
    picker_hex: String,
}

/// Accepts common short and long hex formats and converts them into the
/// preview/picker values needed by the composed control.
fn normalize_hex_color(value: &str) -> Option<ParsedColor> {
    let trimmed = value.trim();
    let hex = trimmed.strip_prefix('#').unwrap_or(trimmed);

    if hex.len() == 3 && hex.chars().all(|char| char.is_ascii_hexdigit()) {
        let expanded = hex
            .chars()
            .flat_map(|char| [char, char])
            .collect::<String>();
        let picker_hex = format!("#{}", expanded.to_ascii_lowercase());
        return Some(ParsedColor {
            preview_css: picker_hex.clone(),
            picker_hex,
        });
    }

    if hex.len() == 4 && hex.chars().all(|char| char.is_ascii_hexdigit()) {
        let expanded = hex
            .chars()
            .flat_map(|char| [char, char])
            .collect::<String>()
            .to_ascii_lowercase();
        return Some(ParsedColor {
            preview_css: format!(
                "rgba({}, {}, {}, {:.4})",
                u8::from_str_radix(&expanded[0..2], 16).ok()?,
                u8::from_str_radix(&expanded[2..4], 16).ok()?,
                u8::from_str_radix(&expanded[4..6], 16).ok()?,
                f32::from(u8::from_str_radix(&expanded[6..8], 16).ok()?) / 255.0
            ),
            picker_hex: format!("#{}", &expanded[0..6]),
        });
    }

    if hex.len() == 6 && hex.chars().all(|char| char.is_ascii_hexdigit()) {
        let picker_hex = format!("#{}", hex.to_ascii_lowercase());
        return Some(ParsedColor {
            preview_css: picker_hex.clone(),
            picker_hex,
        });
    }

    if hex.len() == 8 && hex.chars().all(|char| char.is_ascii_hexdigit()) {
        let normalized = hex.to_ascii_lowercase();
        return Some(ParsedColor {
            preview_css: format!(
                "rgba({}, {}, {}, {:.4})",
                u8::from_str_radix(&normalized[0..2], 16).ok()?,
                u8::from_str_radix(&normalized[2..4], 16).ok()?,
                u8::from_str_radix(&normalized[4..6], 16).ok()?,
                f32::from(u8::from_str_radix(&normalized[6..8], 16).ok()?) / 255.0
            ),
            picker_hex: format!("#{}", &normalized[0..6]),
        });
    }

    None
}
