use birei::{Card, Checkbox, Label, Size};
use leptos::prelude::*;
use crate::code_example::CodeExample;

#[component]
pub fn CheckboxPage() -> impl IntoView {
    let accepts_terms = RwSignal::new(true);
    let email_updates = RwSignal::new(false);
    let compact = RwSignal::new(true);
    let default_size = RwSignal::new(false);
    let large = RwSignal::new(true);
    let invalid = RwSignal::new(false);

    view! {
        <section class="page-header">
            <div class="page-header__eyebrow">"Component"</div>
            <h2>"Checkbox"</h2>
            <p class="page-header__lede">
                "Native checkbox input wrapped in Birei sizing tokens, animated selection feedback, and room for longer descriptive copy to the right."
            </p>
        </section>

        <section class="doc-grid">
            <Card header="Inline label content" class="doc-card">
                <span class="doc-card__kicker">"Basics"</span>
                <div class="doc-card__preview doc-card__preview--stack">
                    <div class="field">
                        <Label text="Access" for_id="book-checkbox-terms"/>
                        <Checkbox
                            id="book-checkbox-terms"
                            checked=accepts_terms
                            on_checked_change=Callback::new(move |next| accepts_terms.set(next))
                        >
                            <span>
                                "I agree to the "
                                <strong>"atelier access terms"</strong>
                                " for prototype reviews."
                            </span>
                        </Checkbox>
                    </div>
                    <p class="doc-card__copy">
                        "Checked: "
                        <strong>{move || if accepts_terms.get() { "true" } else { "false" }}</strong>
                    </p>
                </div>
                <CodeExample code={r#"<Label text="Access" for_id="terms"/>
<Checkbox
    id="terms"
    checked=accepts_terms
    on_checked_change=Callback::new(move |next| accepts_terms.set(next))
>
    <span>
        "I agree to the "
        <strong>"atelier access terms"</strong>
        " for prototype reviews."
    </span>
</Checkbox>"#}/>
            </Card>

            <Card header="Additional field label" class="doc-card">
                <span class="doc-card__kicker">"Association"</span>
                <div class="doc-card__preview doc-card__preview--stack">
                    <div class="field">
                        <Label text="Notifications" for_id="book-checkbox-notifications"/>
                        <Checkbox
                            id="book-checkbox-notifications"
                            checked=email_updates
                            on_checked_change=Callback::new(move |next| email_updates.set(next))
                        >
                            <div class="book-checkbox-copy">
                                <strong>"Weekly product digest"</strong>
                                <p>
                                    "Receive release notes, migration reminders, and curated examples from the component book."
                                </p>
                            </div>
                        </Checkbox>
                    </div>
                </div>
                <CodeExample code={r#"<Label text="Notifications" for_id="weekly-digest"/>
<Checkbox id="weekly-digest">
    <div>
        <strong>"Weekly product digest"</strong>
        <p>"Receive release notes, migration reminders, and curated examples."</p>
    </div>
</Checkbox>"#}/>
            </Card>

            <Card header="Supports longer descriptions" class="doc-card">
                <span class="doc-card__kicker">"Content"</span>
                <div class="doc-card__preview doc-card__preview--stack">
                    <Checkbox
                        checked=email_updates
                        aria_label="Weekly product digest"
                        on_checked_change=Callback::new(move |next| email_updates.set(next))
                    >
                        <div class="book-checkbox-copy">
                            <strong>"Weekly product digest"</strong>
                            <p>
                                "Receive release notes, migration reminders, and curated examples from the component book."
                            </p>
                        </div>
                    </Checkbox>
                    <Checkbox checked=true disabled=true aria_label="Enabled in account defaults">
                        <div class="book-checkbox-copy">
                            <strong>"Enabled in account defaults"</strong>
                            <p>"This preference is currently locked by your workspace policy."</p>
                        </div>
                    </Checkbox>
                </div>
                <CodeExample code={r#"<Checkbox aria_label="Weekly product digest">
    <div>
        <strong>"Weekly product digest"</strong>
        <p>"Receive release notes, migration reminders, and curated examples."</p>
    </div>
</Checkbox>"#}/>
            </Card>

            <Card header="Shared control sizing" class="doc-card">
                <span class="doc-card__kicker">"Sizes"</span>
                <div class="doc-card__preview doc-card__preview--stack">
                    <Checkbox
                        size=Size::Small
                        checked=compact
                        aria_label="Small checkbox for compact preference lists"
                        on_checked_change=Callback::new(move |next| compact.set(next))
                    >
                        <span>"Small checkbox for compact preference lists."</span>
                    </Checkbox>
                    <Checkbox
                        size=Size::Medium
                        checked=default_size
                        aria_label="Medium checkbox aligned with default input sizing"
                        on_checked_change=Callback::new(move |next| default_size.set(next))
                    >
                        <span>"Medium checkbox aligned with default input sizing."</span>
                    </Checkbox>
                    <Checkbox
                        size=Size::Large
                        checked=large
                        aria_label="Large checkbox for prominent onboarding actions"
                        on_checked_change=Callback::new(move |next| large.set(next))
                    >
                        <span>"Large checkbox for prominent onboarding actions."</span>
                    </Checkbox>
                </div>
                <CodeExample code={r#"<Checkbox size=Size::Small>"Small checkbox"</Checkbox>
<Checkbox size=Size::Medium>"Medium checkbox"</Checkbox>
<Checkbox size=Size::Large>"Large checkbox"</Checkbox>"#}/>
            </Card>

            <Card header="State variants" class="doc-card">
                <span class="doc-card__kicker">"State"</span>
                <div class="doc-card__preview doc-card__preview--stack">
                    <Checkbox
                        checked=invalid
                        invalid=true
                        aria_label="Mark an unresolved policy acknowledgement as invalid"
                        on_checked_change=Callback::new(move |next| invalid.set(next))
                    >
                        <span>"Mark an unresolved policy acknowledgement as invalid."</span>
                    </Checkbox>
                    <Checkbox disabled=true aria_label="Disabled and unchecked">
                        <span>"Disabled and unchecked."</span>
                    </Checkbox>
                    <Checkbox checked=true disabled=true aria_label="Disabled and checked">
                        <span>"Disabled and checked."</span>
                    </Checkbox>
                </div>
                <CodeExample code={r#"<Checkbox checked=invalid invalid=true/>
<Checkbox disabled=true/>
<Checkbox checked=true disabled=true/>"#}/>
            </Card>
        </section>
    }
}
