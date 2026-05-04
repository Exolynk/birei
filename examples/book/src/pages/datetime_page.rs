use crate::code_example::CodeExample;
use birei::{Card, DateTimeInput, DateTimeInputMode, Label, Size};
use jiff::civil::{date, DateTime};
use jiff::tz::TimeZone;
use jiff::Zoned;
use js_sys::{Array, Intl::DateTimeFormat, Object, Reflect};
use leptos::prelude::*;
use wasm_bindgen::JsValue;

#[component]
pub fn DateTimePage() -> impl IntoView {
    let due_date = RwSignal::new(Some(local_zoned(date(2026, 4, 3).at(0, 0, 0, 0))));
    let meeting_time = RwSignal::new(Some(local_zoned(date(2026, 4, 3).at(9, 30, 0, 0))));
    let launch_slot = RwSignal::new(Some(local_zoned(date(2026, 4, 3).at(9, 30, 0, 0))));

    view! {
        <section class="page-header">
            <div class="page-header__eyebrow">"Component"</div>
            <h2>"DateTime Input"</h2>
            <p class="page-header__lede">
                "Shared text-input shell with a native picker trigger for date, time, or local datetime values."
            </p>
        </section>

        <section class="doc-grid">
            <Card header="Date, time, and datetime" class="doc-card">
                <span class="doc-card__kicker">"Modes"</span>
                <div class="doc-card__preview doc-card__preview--stack">
                    <div class="field">
                        <Label text="Due date" for_id="book-datetime-date"/>
                        <DateTimeInput
                            id="book-datetime-date"
                            mode=DateTimeInputMode::Date
                            value=due_date
                            placeholder="Select date"
                            on_value_change=Callback::new(move |next| due_date.set(next))
                        />
                        <p class="doc-card__copy">
                            "Value: "
                            <strong>{move || due_date.get().map(|value| value.to_string()).unwrap_or_else(|| String::from("None"))}</strong>
                        </p>
                    </div>
                    <div class="field">
                        <Label text="Meeting time" for_id="book-datetime-time"/>
                        <DateTimeInput
                            id="book-datetime-time"
                            mode=DateTimeInputMode::Time
                            value=meeting_time
                            placeholder="Select time"
                            on_value_change=Callback::new(move |next| meeting_time.set(next))
                        />
                        <p class="doc-card__copy">
                            "Value: "
                            <strong>{move || meeting_time.get().map(|value| value.to_string()).unwrap_or_else(|| String::from("None"))}</strong>
                        </p>
                    </div>
                    <div class="field">
                        <Label text="Launch slot" for_id="book-datetime-local"/>
                        <DateTimeInput
                            id="book-datetime-local"
                            mode=DateTimeInputMode::DateTime
                            value=launch_slot
                            placeholder="Select date and time"
                            on_value_change=Callback::new(move |next| launch_slot.set(next))
                        />
                        <p class="doc-card__copy">
                            "Value: "
                            <strong>{move || launch_slot.get().map(|value| value.to_string()).unwrap_or_else(|| String::from("None"))}</strong>
                        </p>
                    </div>
                </div>
                <CodeExample code={r#"<DateTimeInput
    mode=DateTimeInputMode::Date
    value=due_date
    on_value_change=Callback::new(move |next| due_date.set(next))
/>
<DateTimeInput
    mode=DateTimeInputMode::Time
    value=meeting_time
    on_value_change=Callback::new(move |next| meeting_time.set(next))
/>
<DateTimeInput
    mode=DateTimeInputMode::DateTime
    value=launch_slot
    on_value_change=Callback::new(move |next| launch_slot.set(next))
/>"#}/>
            </Card>

            <Card header="Shared input sizing and state" class="doc-card">
                <span class="doc-card__kicker">"States"</span>
                <div class="doc-card__preview doc-card__preview--stack">
                    <DateTimeInput mode=DateTimeInputMode::Date size=Size::Small value=Some(local_zoned(date(2026, 4, 3).at(0, 0, 0, 0))) />
                    <DateTimeInput mode=DateTimeInputMode::DateTime size=Size::Medium value=Some(local_zoned(date(2026, 4, 3).at(9, 30, 0, 0))) />
                    <DateTimeInput mode=DateTimeInputMode::Time size=Size::Large value=Some(local_zoned(date(2026, 4, 3).at(9, 30, 0, 0))) />
                    <DateTimeInput mode=DateTimeInputMode::Date placeholder="Disabled date" disabled=true/>
                    <DateTimeInput mode=DateTimeInputMode::Time value=Some(local_zoned(date(2026, 4, 3).at(13, 45, 0, 0))) readonly=true/>
                    <DateTimeInput mode=DateTimeInputMode::DateTime value=Some(local_zoned(date(2026, 4, 3).at(9, 30, 0, 0))) invalid=true/>
                </div>
                <CodeExample code={r#"<DateTimeInput
    mode=DateTimeInputMode::Date
    size=Size::Small
    value=Some(date(2026, 4, 3).at(0, 0, 0, 0))
/>
<DateTimeInput
    mode=DateTimeInputMode::DateTime
    size=Size::Medium
    value=Some(date(2026, 4, 3).at(9, 30, 0, 0))
/>
<DateTimeInput
    mode=DateTimeInputMode::Time
    size=Size::Large
    value=Some(date(2026, 4, 3).at(9, 30, 0, 0))
/>"#}/>
            </Card>
        </section>
    }
}

fn local_zoned(datetime: DateTime) -> Zoned {
    datetime
        .to_zoned(browser_timezone().unwrap_or(TimeZone::UTC))
        .expect("example datetime should be valid in the system timezone")
}

fn browser_timezone() -> Option<TimeZone> {
    let formatter = DateTimeFormat::new(&Array::new(), &Object::new());
    let options = formatter.resolved_options();
    let timezone = Reflect::get(options.as_ref(), &JsValue::from_str("timeZone"))
        .ok()?
        .as_string()?;

    TimeZone::get(&timezone).ok()
}
