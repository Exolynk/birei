use crate::code_example::CodeExample;
use birei::{
    Card, Checkbox, DateTimeInput, DateTimeInputMode, Field, Input, InputType, Select,
    SelectOption,
};
use jiff::civil::date;
use jiff::tz::TimeZone;
use jiff::Zoned;
use leptos::ev;
use leptos::prelude::*;
use web_sys::HtmlInputElement;

#[component]
pub fn FieldPage() -> impl IntoView {
    let display_name = RwSignal::new(String::from("Aiko Tanaka"));
    let contact_email = RwSignal::new(String::from("aiko@birei.dev"));
    let due_date = RwSignal::new(Some(local_zoned(date(2026, 5, 22).at(0, 0, 0, 0))));
    let enabled = RwSignal::new(true);
    let access_label = Signal::derive(move || {
        if enabled.get() {
            String::from("Access enabled")
        } else {
            String::from("Access disabled")
        }
    });
    let role = RwSignal::new(Some(String::from("designer")));
    let role_options = vec![
        SelectOption::new("designer", "Product designer"),
        SelectOption::new("engineer", "Frontend engineer"),
        SelectOption::new("producer", "Launch producer"),
        SelectOption::new("writer", "Technical writer"),
    ];

    let update_signal = |signal: RwSignal<String>| {
        move |event: ev::Event| {
            signal.set(event_target::<HtmlInputElement>(&event).value());
        }
    };

    view! {
        <section class="page-header">
            <div class="page-header__eyebrow">"Component"</div>
            <h2>"Field"</h2>
            <p class="page-header__lede">
                "Responsive label and control layout for composing existing inputs without replacing their behavior."
            </p>
        </section>

        <section class="doc-grid">
            <Card header="Wrap existing controls" class="doc-card">
                <span class="doc-card__kicker">"Composition"</span>
                <div class="doc-card__preview doc-card__preview--stack">
                    <Field label="Display name" for_id="book-field-display-name" required=true>
                        <Input
                            id="book-field-display-name"
                            value=display_name
                            required=true
                            placeholder="Display name"
                            on_input=Callback::new(update_signal(display_name))
                        />
                    </Field>
                    <Field label="Due date" for_id="book-field-due-date">
                        <DateTimeInput
                            id="book-field-due-date"
                            mode=DateTimeInputMode::Date
                            value=due_date
                            placeholder="Select date"
                            on_value_change=Callback::new(move |next| due_date.set(next))
                        />
                    </Field>
                    <Field label="Role" for_id="book-field-role">
                        <Select
                            id="book-field-role"
                            options=role_options.clone()
                            value=role
                            placeholder="Choose role"
                            on_value_change=Callback::new(move |next| role.set(next))
                        />
                    </Field>
                    <Field label=access_label for_id="book-field-access">
                        <Checkbox
                            id="book-field-access"
                            checked=enabled
                            on_checked_change=Callback::new(move |next| enabled.set(next))
                        >
                            <span>"Allow this member to review prototype builds."</span>
                        </Checkbox>
                    </Field>
                </div>
                <CodeExample code={r#"<Field label="Display name" for_id="display-name" required=true>
    <Input id="display-name" required=true placeholder="Display name"/>
</Field>

<Field label="Due date" for_id="due-date">
    <DateTimeInput id="due-date" mode=DateTimeInputMode::Date/>
</Field>

<Field label="Role" for_id="role">
    <Select id="role" options=role_options.clone()/>
</Field>

let access_label = Signal::derive(move || {
    if enabled.get() {
        String::from("Access enabled")
    } else {
        String::from("Access disabled")
    }
});

<Field label=access_label for_id="access">
    <Checkbox id="access">
        <span>"Allow this member to review prototype builds."</span>
    </Checkbox>
</Field>"#}/>
            </Card>

            <Card header="Stacks when the field is narrow" class="doc-card">
                <span class="doc-card__kicker">"Responsive"</span>
                <div class="doc-card__preview doc-card__preview--stack">
                    <div class="book-field-narrow-demo">
                        <Field label="Contact email" for_id="book-field-contact-email">
                            <Input
                                id="book-field-contact-email"
                                value=contact_email
                                input_type=InputType::Email
                                placeholder="name@birei.dev"
                                on_input=Callback::new(update_signal(contact_email))
                            />
                        </Field>
                    </div>
                    <p class="doc-card__copy">
                        "The layout responds to the field container, so it can stack inside a narrow panel even when the page itself is wide."
                    </p>
                </div>
                <CodeExample code={r#"<div style="max-width: 22rem">
    <Field label="Contact email" for_id="contact-email">
        <Input id="contact-email" input_type=InputType::Email/>
    </Field>
</div>"#}/>
            </Card>
        </section>
    }
}

fn local_zoned(datetime: jiff::civil::DateTime) -> Zoned {
    datetime
        .to_zoned(TimeZone::system())
        .expect("book demo datetime should be valid in the local timezone")
}
