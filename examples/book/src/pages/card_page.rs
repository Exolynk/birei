use birei::{Button, ButtonVariant, Card, Icon, Input, Label};
use leptos::prelude::*;
use crate::code_example::CodeExample;

#[component]
pub fn CardPage() -> impl IntoView {
    view! {
        <section class="page-header">
            <div class="page-header__eyebrow">"Component"</div>
            <h2>"Card"</h2>
            <p class="page-header__lede">
                "Surface container for grouped content, with an optional collapsible header that follows the same restrained interaction language as the rest of the library."
            </p>
        </section>

        <section class="doc-grid">
            <Card header="Static content" class="doc-card">
                <span class="doc-card__kicker">"Basics"</span>
                <div class="doc-card__preview doc-card__preview--stack">
                    <p class="doc-card__copy">
                        "Cards can hold arbitrary layout without imposing form-specific structure."
                    </p>
                    <div class="book-icon-demo-row">
                        <Icon name="sparkles" label="Sparkles"/>
                        <span>"A quiet surface for grouped UI."</span>
                    </div>
                </div>
                <CodeExample code={r#"<Card header="Static content">
    <p>"Cards can hold arbitrary layout."</p>
</Card>"#}/>
            </Card>

            <Card header="Collapsible section" class="doc-card">
                <span class="doc-card__kicker">"Collapse"</span>
                <div class="doc-card__preview doc-card__preview--stack">
                    <p class="doc-card__copy">
                        "A header enables collapsing to the title row only. The chevron is shown only for titled cards."
                    </p>
                    <div class="field">
                        <Label text="Preview field" for_id="book-card-preview-field"/>
                        <Input id="book-card-preview-field" placeholder="Collapsed sections keep their state"/>
                    </div>
                </div>
                <CodeExample code={r#"<Card header="Collapsible section">
    <Label text="Preview field" for_id="preview-field"/>
    <Input id="preview-field" placeholder="Collapsed sections keep their state"/>
</Card>"#}/>
            </Card>

            <Card header="Initially collapsed" collapsed=true class="doc-card">
                <span class="doc-card__kicker">"State"</span>
                <div class="doc-card__preview doc-card__preview--stack">
                    <p class="doc-card__copy">
                        "Use `collapsed=true` when secondary material should stay tucked away on first render."
                    </p>
                    <div class="demo-form__actions">
                        <Button>"Primary action"</Button>
                        <Button variant=ButtonVariant::Secondary>"Secondary action"</Button>
                    </div>
                </div>
                <CodeExample code={r#"<Card header="Initially collapsed" collapsed=true>
    <Button>"Primary action"</Button>
    <Button variant=ButtonVariant::Secondary>"Secondary action"</Button>
</Card>"#}/>
            </Card>

            <Card class="doc-card">
                <span class="doc-card__kicker">"Untitled"</span>
                <div class="doc-card__preview doc-card__preview--stack">
                    <p class="doc-card__copy">
                        "When no header is provided, the card is just a plain surface. No collapse affordance is rendered."
                    </p>
                </div>
                <CodeExample code={r#"<Card>
    <p>"Plain card surface"</p>
</Card>"#}/>
            </Card>
        </section>
    }
}
