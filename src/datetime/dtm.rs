use jiff::civil::{Date, DateTime, Time};
use js_sys::Date as JsDate;
use leptos::ev;
use leptos::html;
use leptos::prelude::*;
use web_sys::HtmlInputElement;

use super::DateTimeInputMode;
use crate::{Icon, Input, Size};

/// Text input with a native date, time, or datetime picker trigger.
#[component]
pub fn DateTimeInput(
    /// Current civil datetime value.
    #[prop(optional, into)]
    value: MaybeProp<Option<DateTime>>,
    /// Placeholder text shown when the value is empty.
    #[prop(optional, into)]
    placeholder: MaybeProp<String>,
    /// Optional input name for form submission.
    #[prop(optional, into)]
    name: Option<String>,
    /// Optional input id for the visible text field.
    #[prop(optional, into)]
    id: Option<String>,
    /// Picker mode.
    #[prop(optional)]
    mode: DateTimeInputMode,
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
    on_value_change: Option<Callback<Option<DateTime>>>,
    /// Input event handler for the native picker.
    #[prop(optional)]
    on_input: Option<Callback<ev::Event>>,
    /// Change event handler for the native picker.
    #[prop(optional)]
    on_change: Option<Callback<ev::Event>>,
) -> impl IntoView {
    let picker_ref = NodeRef::<html::Input>::new();
    let current_value = move || value.get().flatten();
    let picker_value = Memo::new(move |_| {
        current_value()
            .map(|value| format_picker_value(value, mode))
            .unwrap_or_default()
    });
    let mut classes = vec!["birei-datetime-input", datetime_size_class_name(size)];
    if let Some(class) = class.as_deref() {
        classes.push(class);
    }
    let class_name = classes.join(" ");

    let open_picker = move || {
        if disabled || readonly {
            return;
        }

        if let Some(input) = picker_ref.get_untracked() {
            let _ = input.show_picker();
            let _ = input.focus();
        }
    };

    let handle_picker_input = move |event: ev::Event| {
        let next = picker_value_to_datetime(
            &event_target::<HtmlInputElement>(&event).value(),
            current_value(),
            mode,
        );

        if let Some(on_value_change) = on_value_change.as_ref() {
            on_value_change.run(next);
        }
        if let Some(on_input) = on_input.as_ref() {
            on_input.run(event);
        }
    };

    let handle_picker_change = move |event: ev::Event| {
        let next = picker_value_to_datetime(
            &event_target::<HtmlInputElement>(&event).value(),
            current_value(),
            mode,
        );

        if let Some(on_value_change) = on_value_change.as_ref() {
            on_value_change.run(next);
        }
        if let Some(on_change) = on_change.as_ref() {
            on_change.run(event);
        }
    };

    view! {
        <div class=class_name>
            <input
                node_ref=picker_ref
                class="birei-datetime-input__native"
                id=id
                type=mode.native_input_type()
                name=name
                tabindex=if disabled || readonly { "-1" } else { "0" }
                disabled=disabled || readonly
                prop:value=picker_value
                on:input=handle_picker_input
                on:change=handle_picker_change
            />
            <Input
                value=String::new()
                placeholder=placeholder
                size=size
                disabled=disabled
                readonly=true
                invalid=invalid
                required=required
                tabindex=-1
                class="birei-datetime-input__field"
                suffix=move || {
                    view! {
                        <span
                            class="birei-datetime-input__trigger"
                            aria-hidden="true"
                            on:mousedown=move |event| {
                                event.prevent_default();
                                open_picker();
                            }
                        >
                            <Icon name=mode.icon_name() label=picker_label(mode)/>
                        </span>
                    }
                }
            />
        </div>
    }
}

fn picker_value_to_datetime(
    value: &str,
    current: Option<DateTime>,
    mode: DateTimeInputMode,
) -> Option<DateTime> {
    let trimmed = value.trim();
    if trimmed.is_empty() {
        return None;
    }

    match mode {
        DateTimeInputMode::Date => {
            let date = trimmed.parse::<Date>().ok()?;
            let time = current.map(DateTime::time).unwrap_or_else(Time::midnight);
            Some(date.to_datetime(time))
        }
        DateTimeInputMode::Time => {
            let time = trimmed.parse::<Time>().ok()?;
            let date = current.map(DateTime::date).unwrap_or_else(today_local_date);
            Some(date.to_datetime(time))
        }
        DateTimeInputMode::DateTime => parse_datetime_local(trimmed),
    }
}

fn parse_datetime_local(value: &str) -> Option<DateTime> {
    value
        .parse::<DateTime>()
        .ok()
        .or_else(|| format!("{value}:00").parse::<DateTime>().ok())
}

fn format_picker_value(value: DateTime, mode: DateTimeInputMode) -> String {
    match mode {
        DateTimeInputMode::Date => value.date().strftime("%Y-%m-%d").to_string(),
        DateTimeInputMode::Time => format_picker_time(value.time()),
        DateTimeInputMode::DateTime => {
            let date = value.date().strftime("%Y-%m-%d").to_string();
            let time = format_picker_time(value.time());
            format!("{date}T{time}")
        }
    }
}

fn format_picker_time(value: Time) -> String {
    if value.second() == 0 && value.subsec_nanosecond() == 0 {
        value.strftime("%H:%M").to_string()
    } else {
        value.strftime("%H:%M:%S").to_string()
    }
}

fn today_local_date() -> Date {
    let now = JsDate::new_0();
    Date::new(
        now.get_full_year() as i16,
        (now.get_month() + 1) as i8,
        now.get_date() as i8,
    )
    .unwrap_or_else(|_| Date::new(1970, 1, 1).expect("valid fallback date"))
}

fn picker_label(mode: DateTimeInputMode) -> &'static str {
    match mode {
        DateTimeInputMode::Date => "Open date picker",
        DateTimeInputMode::Time => "Open time picker",
        DateTimeInputMode::DateTime => "Open date and time picker",
    }
}

fn datetime_size_class_name(size: Size) -> &'static str {
    match size {
        Size::Small => "birei-datetime-input--small",
        Size::Medium => "birei-datetime-input--medium",
        Size::Large => "birei-datetime-input--large",
    }
}
