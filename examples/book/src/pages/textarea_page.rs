use crate::code_example::CodeExample;
use birei::{Button, ButtonVariant, Card, Label, Size, Textarea};
use leptos::ev;
use leptos::prelude::*;
use web_sys::HtmlTextAreaElement;

#[component]
pub fn TextareaPage() -> impl IntoView {
    let bio = RwSignal::new(String::from(
        "Product designer working across interaction systems, tooling, and launch narratives.",
    ));
    let notes = RwSignal::new(String::from(
        "Keep the layout breathable.\nSupport keyboard users.\nDocument edge cases in the book.",
    ));
    let summary = RwSignal::new(String::new());

    let update_signal = |signal: RwSignal<String>| {
        move |event: ev::Event| {
            signal.set(event_target::<HtmlTextAreaElement>(&event).value());
        }
    };

    view! {
        <section class="page-header">
            <div class="page-header__eyebrow">"Component"</div>
            <h2>"Textarea"</h2>
            <p class="page-header__lede">
                "Multiline text fields with the same visual language as inputs, including shared sizing, required states, and animated focus treatment."
            </p>
        </section>

        <section class="doc-grid">
            <Card header="Plain multiline input" class="doc-card">
                <span class="doc-card__kicker">"Basics"</span>
                <div class="doc-card__preview doc-card__preview--stack">
                    <div class="field">
                        <Label text="Bio" required=true for_id="book-textarea-bio"/>
                        <Textarea
                            id="book-textarea-bio"
                            value=bio
                            placeholder="Tell the team about your practice"
                            required=true
                            rows=4
                            on_input=Callback::new(update_signal(bio))
                        />
                    </div>
                    <p class="doc-card__copy">
                        "Character count: "
                        <strong>{move || bio.get().chars().count()}</strong>
                    </p>
                </div>
                <CodeExample code={r#"<Label text="Bio" required=true for_id="profile-bio"/>
<Textarea
    id="profile-bio"
    value=bio
    placeholder="Tell the team about your practice"
    required=true
    rows=4
    on_input=Callback::new(update_signal(bio))
/>"#}/>
            </Card>

            <Card header="Shared sizing" class="doc-card">
                <span class="doc-card__kicker">"Sizes"</span>
                <div class="doc-card__preview doc-card__preview--stack">
                    <Textarea size=Size::Small rows=3 placeholder="Small textarea"/>
                    <Textarea size=Size::Medium rows=4 placeholder="Medium textarea"/>
                    <Textarea size=Size::Large rows=5 placeholder="Large textarea"/>
                </div>
                <CodeExample code={r#"<Textarea size=Size::Small rows=3 placeholder="Small textarea"/>
<Textarea size=Size::Medium rows=4 placeholder="Medium textarea"/>
<Textarea size=Size::Large rows=5 placeholder="Large textarea"/>"#}/>
            </Card>

            <Card header="Readonly, disabled, invalid" class="doc-card">
                <span class="doc-card__kicker">"State"</span>
                <div class="doc-card__preview doc-card__preview--stack">
                    <Textarea
                        value=notes
                        rows=4
                        readonly=true
                        on_input=Callback::new(update_signal(notes))
                    />
                    <Textarea
                        disabled=true
                        rows=4
                        placeholder="Disabled multiline field"
                    />
                    <Textarea
                        invalid=true
                        required=true
                        rows=4
                        placeholder="This field needs more detail"
                    />
                </div>
                <CodeExample code={r#"<Textarea value=notes rows=4 readonly=true/>
<Textarea disabled=true rows=4 placeholder="Disabled multiline field"/>
<Textarea invalid=true required=true rows=4 placeholder="This field needs more detail"/>"#}/>
            </Card>

            <Card header="Works inside forms" class="doc-card">
                <span class="doc-card__kicker">"Composition"</span>
                <div class="doc-card__preview doc-card__preview--stack">
                    <div class="field">
                        <Label text="Project summary" for_id="book-textarea-summary"/>
                        <Textarea
                            id="book-textarea-summary"
                            value=summary
                            rows=5
                            placeholder="Summarize the release in a few lines"
                            on_input=Callback::new(update_signal(summary))
                        />
                    </div>
                    <div class="demo-form__actions">
                        <Button>"Save draft"</Button>
                        <Button variant=ButtonVariant::Secondary>"Request review"</Button>
                    </div>
                </div>
                <CodeExample code={r#"<Label text="Project summary" for_id="summary"/>
<Textarea
    id="summary"
    value=summary
    rows=5
    placeholder="Summarize the release in a few lines"
    on_input=Callback::new(update_signal(summary))
/>
<Button>"Save draft"</Button>"#}/>
            </Card>
        </section>
    }
}
