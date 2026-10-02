use crate::ArcOneCallback;
use jiff::civil::{Date, DateTime, Time};
use jiff::tz::TimeZone;
use jiff::Zoned;
use js_sys::{Array, Intl::DateTimeFormat, Object, Reflect};
use leptos::ev;
use leptos::html;
use leptos::prelude::*;
use wasm_bindgen::JsValue;
use web_sys::{HtmlInputElement, KeyboardEvent};

use super::DateTimeInputMode;
use crate::{Icon, Input, Size};

/// Text input with a native date, time, or datetime picker trigger.
#[component]
pub fn DateTimeInput(
    /// Current timezone-aware datetime value.
    #[prop(optional, into)]
    value: MaybeProp<Option<Zoned>>,
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
    /// Publishes changed, valid values when focus leaves the picker or Enter is pressed.
    #[prop(optional, into)]
    on_value_change: Option<ArcOneCallback<Option<Zoned>>>,
    /// Input event handler for the native picker.
    #[prop(optional, into)]
    on_input: Option<ArcOneCallback<ev::Event>>,
    /// Change event handler for the native picker.
    #[prop(optional, into)]
    on_change: Option<ArcOneCallback<ev::Event>>,
) -> impl IntoView {
    // The visible shell is a readonly shared input; the real date/time value
    // lives in a native picker input that browsers can enhance.
    let picker_ref = NodeRef::<html::Input>::new();
    let active = RwSignal::new(true);
    // Native pickers can emit a final change event while their old DOM node is removed.
    on_cleanup(move || {
        let _ = active.try_set(false);
    });
    let current_value = move || value.try_get().unwrap_or_default().flatten();
    // Picker formatting is centralized so all display modes map through one
    // serialization path.
    let picker_value = Memo::new(move |_| {
        current_value()
            .map(|value| format_picker_value(value, mode))
            .unwrap_or_default()
    });
    // Wrapper classes add mode-specific sizing without reimplementing the base
    // shared input shell styling.
    let mut classes = vec!["birei-datetime-input", datetime_size_class_name(size)];
    if let Some(class) = class.as_deref() {
        classes.push(class);
    }
    let class_name = classes.join(" ");

    // The trigger asks the browser to open its native picker UI when allowed.
    let open_picker = move || {
        if disabled || readonly {
            return;
        }

        if let Some(input) = picker_ref.try_get_untracked().unwrap_or_default() {
            let _ = input.show_picker();
            let _ = input.focus();
        }
    };

    // Keep native edits local until the user finishes editing the picker.
    let handle_picker_input = move |event: ev::Event| {
        run_picker_callback(active, on_input.as_ref(), event);
    };

    let handle_picker_change = move |event: ev::Event| {
        run_picker_callback(active, on_change.as_ref(), event);
    };

    let commit_picker_value = move |input: HtmlInputElement| {
        if active.try_get_untracked() != Some(true) {
            return;
        }
        let next = picker_value_update(
            &input.value(),
            input.validity().bad_input(),
            untrack(current_value),
            mode,
        );

        if let Some(next) = next {
            run_picker_callback(active, on_value_change.as_ref(), next);
        }
    };

    let handle_picker_blur = move |event: ev::FocusEvent| {
        commit_picker_value(event_target::<HtmlInputElement>(&event));
    };

    let handle_picker_keydown = move |event: KeyboardEvent| {
        if event.key() == "Enter" && !event.is_composing() {
            event.prevent_default();
            commit_picker_value(event_target::<HtmlInputElement>(&event));
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
                on:blur=handle_picker_blur
                on:keydown=handle_picker_keydown
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

/// Forwards a picker callback only while its component's reactive owner is active.
fn run_picker_callback<T: 'static>(
    active: RwSignal<bool>,
    callback: Option<&ArcOneCallback<T>>,
    value: T,
) {
    if active.try_get_untracked() != Some(true) {
        return;
    }
    if let Some(callback) = callback {
        callback.run(value);
    }
}

/// Returns only valid, changed values; Some(None) represents a deliberate clear.
fn picker_value_update(
    value: &str,
    bad_input: bool,
    current: Option<Zoned>,
    mode: DateTimeInputMode,
) -> Option<Option<Zoned>> {
    if bad_input {
        return None;
    }
    let next = if value.trim().is_empty() {
        None
    } else {
        Some(picker_value_to_zoned(value, &current, mode)?)
    };
    (next != current).then_some(next)
}

/// Parses a native picker string back into a zoned datetime while preserving
/// the missing date, time, or timezone portion from the current value.
fn picker_value_to_zoned(
    value: &str,
    current: &Option<Zoned>,
    mode: DateTimeInputMode,
) -> Option<Zoned> {
    let trimmed = value.trim();
    if trimmed.is_empty() {
        return None;
    }

    let timezone = current_timezone(current);
    match mode {
        DateTimeInputMode::Date => {
            let date = trimmed.parse::<Date>().ok()?;
            let time = current
                .as_ref()
                .map(Zoned::time)
                .unwrap_or_else(Time::midnight);
            date.to_datetime(time).to_zoned(timezone).ok()
        }
        DateTimeInputMode::Time => {
            let time = trimmed.parse::<Time>().ok()?;
            let date = current
                .as_ref()
                .map(Zoned::date)
                .unwrap_or_else(today_local_date);
            date.to_datetime(time).to_zoned(timezone).ok()
        }
        DateTimeInputMode::DateTime => parse_datetime_local(trimmed)?.to_zoned(timezone).ok(),
    }
}

/// Accepts browser datetime-local values with or without explicit seconds.
fn parse_datetime_local(value: &str) -> Option<DateTime> {
    value
        .parse::<DateTime>()
        .ok()
        .or_else(|| format!("{value}:00").parse::<DateTime>().ok())
}

/// Formats the current value into the exact string shape expected by the
/// selected native picker mode.
fn format_picker_value(value: Zoned, mode: DateTimeInputMode) -> String {
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

/// Uses the existing value's timezone when available, otherwise the user's
/// current system timezone.
fn current_timezone(current: &Option<Zoned>) -> TimeZone {
    current
        .as_ref()
        .map(Zoned::time_zone)
        .filter(|timezone| !timezone.is_unknown())
        .cloned()
        .or_else(browser_timezone)
        .unwrap_or(TimeZone::UTC)
}

/// Browser WASM cannot reliably discover the host timezone through Jiff's
/// system timezone lookup, so use the standard Intl API first.
fn browser_timezone() -> Option<TimeZone> {
    let formatter = DateTimeFormat::new(&Array::new(), &Object::new());
    let options = formatter.resolved_options();
    let timezone = Reflect::get(options.as_ref(), &JsValue::from_str("timeZone"))
        .ok()?
        .as_string()?;

    TimeZone::get(&timezone).ok()
}

/// Omits seconds when they are zero so the native time UI stays compact.
fn format_picker_time(value: Time) -> String {
    if value.second() == 0 && value.subsec_nanosecond() == 0 {
        value.strftime("%H:%M").to_string()
    } else {
        value.strftime("%H:%M:%S").to_string()
    }
}

/// Falls back to the local calendar date when a time-only picker needs a date
/// to build a full civil datetime value.
fn today_local_date() -> Date {
    let timezone = browser_timezone().unwrap_or(TimeZone::UTC);
    Zoned::now().with_time_zone(timezone).date()
}

/// Accessible label used by the readonly suffix trigger icon.
fn picker_label(mode: DateTimeInputMode) -> &'static str {
    match mode {
        DateTimeInputMode::Date => "Open date picker",
        DateTimeInputMode::Time => "Open time picker",
        DateTimeInputMode::DateTime => "Open date and time picker",
    }
}

/// Datetime input sizes map to dedicated classes instead of reusing the text
/// input classes directly because the trigger spacing differs slightly.
fn datetime_size_class_name(size: Size) -> &'static str {
    match size {
        Size::Small => "birei-datetime-input--small",
        Size::Medium => "birei-datetime-input--medium",
        Size::Large => "birei-datetime-input--large",
    }
}

#[cfg(test)]
mod tests {
    use super::{picker_value_update, run_picker_callback};
    use crate::{ArcOneCallback, DateTimeInputMode};
    use jiff::Zoned;
    use leptos::prelude::*;
    use std::sync::{
        atomic::{AtomicUsize, Ordering},
        Arc,
    };

    /// An incomplete native edit must not be published as clearing the stored value.
    #[test]
    fn picker_value_update_ignores_bad_input() {
        for mode in [
            DateTimeInputMode::Date,
            DateTimeInputMode::Time,
            DateTimeInputMode::DateTime,
        ] {
            assert!(picker_value_update("", true, None, mode).is_none());
        }
    }

    /// Unparseable nonempty entries must not be published as clearing the stored value.
    #[test]
    fn picker_value_update_ignores_unparseable_input() {
        let current = "2026-09-01T09:30:00+00:00[UTC]".parse::<Zoned>().unwrap();
        for (mode, value) in [
            (DateTimeInputMode::Date, "2026-10"),
            (DateTimeInputMode::Time, "12:"),
            (DateTimeInputMode::DateTime, "2026-10-02T"),
        ] {
            assert!(picker_value_update(value, false, Some(current.clone()), mode).is_none());
        }
    }

    /// An empty picker without bad input still deliberately clears the stored value.
    #[test]
    fn picker_value_update_allows_clearing() {
        let current = "2026-09-01T09:30:00+00:00[UTC]".parse::<Zoned>().unwrap();
        for mode in [
            DateTimeInputMode::Date,
            DateTimeInputMode::Time,
            DateTimeInputMode::DateTime,
        ] {
            let next = picker_value_update("", false, Some(current.clone()), mode);
            assert!(matches!(next, Some(None)));
        }
    }

    /// Complete entries are published while date-only and time-only edits retain other components.
    #[test]
    fn picker_value_update_allows_complete_input() {
        let current = "2026-09-01T09:30:00+00:00[UTC]".parse::<Zoned>().unwrap();
        for (mode, value, date, time) in [
            (
                DateTimeInputMode::Date,
                "2026-10-02",
                "2026-10-02",
                "09:30:00",
            ),
            (DateTimeInputMode::Time, "12:45", "2026-09-01", "12:45:00"),
            (
                DateTimeInputMode::DateTime,
                "2026-10-02T12:45",
                "2026-10-02",
                "12:45:00",
            ),
        ] {
            let next = picker_value_update(value, false, Some(current.clone()), mode)
                .flatten()
                .unwrap();
            assert_eq!(next.date().to_string(), date);
            assert_eq!(next.time().to_string(), time);
            assert_eq!(next.time_zone(), current.time_zone());
        }
    }

    /// Leaving an untouched picker must not trigger a model update or popup rebuild.
    #[test]
    fn picker_value_update_ignores_unchanged_input() {
        let current = "2026-09-01T09:30:00+00:00[UTC]".parse::<Zoned>().unwrap();
        for (mode, value) in [
            (DateTimeInputMode::Date, "2026-09-01"),
            (DateTimeInputMode::Time, "09:30"),
            (DateTimeInputMode::DateTime, "2026-09-01T09:30"),
        ] {
            assert!(picker_value_update(value, false, Some(current.clone()), mode).is_none());
        }
        assert!(picker_value_update("", false, None, DateTimeInputMode::DateTime).is_none());
    }

    /// A blur following an Enter commit must not publish the same value again.
    #[test]
    fn picker_value_update_ignores_repeated_commit() {
        let current = "2026-09-01T09:30:00+00:00[UTC]".parse::<Zoned>().unwrap();
        let committed = picker_value_update(
            "2026-10-02T12:45",
            false,
            Some(current),
            DateTimeInputMode::DateTime,
        )
        .unwrap();

        assert!(picker_value_update(
            "2026-10-02T12:45",
            false,
            committed,
            DateTimeInputMode::DateTime,
        )
        .is_none());
    }

    /// A retained event handler stops forwarding after its reactive owner is cleaned up.
    #[test]
    fn picker_callback_ignores_disposed_owner() {
        let owner = Owner::new();
        let calls = Arc::new(AtomicUsize::new(0));
        let callback_calls = calls.clone();
        let (active, callback) = owner.with(|| {
            let active = RwSignal::new(true);
            let callback = ArcOneCallback::new(move |()| {
                callback_calls.fetch_add(1, Ordering::Relaxed);
            });
            (active, callback)
        });

        run_picker_callback(active, Some(&callback), ());
        assert_eq!(calls.load(Ordering::Relaxed), 1);
        owner.cleanup();
        run_picker_callback(active, Some(&callback), ());
        assert_eq!(calls.load(Ordering::Relaxed), 1);
    }

    /// Cleanup triggered by the value callback prevents forwarding the subsequent raw event.
    #[test]
    fn picker_callback_rechecks_owner_after_value_update() {
        let owner = Owner::new();
        let calls = Arc::new(AtomicUsize::new(0));
        let callback_calls = calls.clone();
        let cleanup_owner = owner.clone();
        let (active, value_callback, event_callback) = owner.with(|| {
            let active = RwSignal::new(true);
            let value_callback = ArcOneCallback::new(move |()| cleanup_owner.cleanup());
            let event_callback = ArcOneCallback::new(move |()| {
                callback_calls.fetch_add(1, Ordering::Relaxed);
            });
            (active, value_callback, event_callback)
        });

        run_picker_callback(active, Some(&value_callback), ());
        run_picker_callback(active, Some(&event_callback), ());
        assert_eq!(calls.load(Ordering::Relaxed), 0);
    }
}
