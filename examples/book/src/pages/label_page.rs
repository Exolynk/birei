use birei::{Input, Label, Select, SelectOption, Size};
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
            <article class="doc-card">
                <div class="doc-card__header">
                    <span class="doc-card__kicker">"Basics"</span>
                    <h3>"Standalone label"</h3>
                </div>
                <div class="doc-card__preview doc-card__preview--stack">
                    <Label text="Display name"/>
                    <Label text="Display name" required=true/>
                </div>
                <pre class="doc-card__code"><code>{r#"<Label text="Display name"/>
    <Label text="Display name" required=true/>"#}</code></pre>
            </article>

            <article class="doc-card">
                <div class="doc-card__header">
                    <span class="doc-card__kicker">"Association"</span>
                    <h3>"Targets a form control"</h3>
                </div>
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
                <pre class="doc-card__code"><code>{r#"<Label text="Email address" for_id="profile-email" required=true/>
<Input id="profile-email" placeholder="name@birei.dev"/>

<Label text="Location" for_id="profile-location"/>
<Select id="profile-location" options=location_options placeholder="Choose a studio"/>"#}</code></pre>
            </article>
        </section>
    }
}
