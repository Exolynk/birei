use crate::code_example::CodeExample;
use birei::{Card, Input, Label, Select, SelectOption, Size};
use leptos::prelude::*;

#[component]
pub fn LabelPage() -> impl IntoView {
    let long_options = vec![
        SelectOption::new("tokyo", "Tokyo studio"),
        SelectOption::new("zurich", "Zurich studio"),
        SelectOption::new("new-york", "New York studio"),
    ];

    view! {
        <section class="page-header">
            <div class="page-header__eyebrow">"Component"</div>
            <h2>"Label"</h2>
            <p class="page-header__lede">
                "Shared field labels used by inputs and selects, including native label targeting and required markers."
            </p>
        </section>

        <section class="doc-grid">
            <Card header="Standalone label" class="doc-card">
                <span class="doc-card__kicker">"Basics"</span>
                <div class="doc-card__preview doc-card__preview--stack">
                    <Label text="Display name"/>
                    <Label text="Display name" required=true/>
                </div>
                <CodeExample code={r#"<Label text="Display name"/>
    <Label text="Display name" required=true/>"#}/>
            </Card>

            <Card header="Targets a form control" class="doc-card">
                <span class="doc-card__kicker">"Association"</span>
                <div class="doc-card__preview doc-card__preview--stack">
                    <div class="field">
                        <Label text="Email address" for_id="book-label-email" required=true/>
                        <Input id="book-label-email" placeholder="name@birei.dev"/>
                    </div>
                    <div class="field">
                        <Label text="Location" for_id="book-label-location"/>
                        <Select
                            id="book-label-location"
                            options=long_options
                            placeholder="Choose a studio"
                            size=Size::Medium
                        />
                    </div>
                </div>
                <CodeExample code={r#"<Label text="Email address" for_id="profile-email" required=true/>
<Input id="profile-email" placeholder="name@birei.dev"/>

<Label text="Location" for_id="profile-location"/>
<Select id="profile-location" options=location_options placeholder="Choose a studio"/>"#}/>
            </Card>
        </section>
    }
}
