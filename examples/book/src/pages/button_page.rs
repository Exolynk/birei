use crate::code_example::CodeExample;
use birei::{Button, ButtonGroup, ButtonType, ButtonVariant, Card, Input, Label, Size};
use leptos::ev;
use leptos::prelude::*;
use web_sys::HtmlInputElement;

#[component]
pub fn ButtonPage() -> impl IntoView {
    const DEFAULT_NAME: &str = "Aiko";

    let click_count = RwSignal::new(0);
    let name = RwSignal::new(String::from(DEFAULT_NAME));

    let on_input = move |event: ev::Event| {
        let value = event_target::<HtmlInputElement>(&event).value();
        name.set(value);
    };

    view! {
        <section class="page-header">
            <div class="page-header__eyebrow">"Component"</div>
            <h2>"Button"</h2>
            <p class="page-header__lede">
                "Typed Leptos buttons with brand-aware variants, grouped behavior, scoped token overrides, and click feedback."
            </p>
        </section>

        <section class="doc-grid">
            <Card header="Intent and emphasis" class="doc-card">
                <span class="doc-card__kicker">"Variants"</span>
                <div class="doc-card__preview doc-card__preview--stack">
                    <Button variant=ButtonVariant::Primary>"Primary"</Button>
                    <Button variant=ButtonVariant::Secondary>"Secondary"</Button>
                    <Button variant=ButtonVariant::Transparent>"Transparent"</Button>
                </div>
                <CodeExample code={r#"<Button variant=ButtonVariant::Primary>"Primary"</Button>
<Button variant=ButtonVariant::Secondary>"Secondary"</Button>
<Button variant=ButtonVariant::Transparent>"Transparent"</Button>"#}/>
            </Card>

            <Card header="Disabled" class="doc-card">
                <span class="doc-card__kicker">"State"</span>
                <div class="doc-card__preview doc-card__preview--stack">
                    <Button disabled=true>"Primary disabled"</Button>
                    <Button variant=ButtonVariant::Secondary disabled=true>
                        "Secondary disabled"
                    </Button>
                    <Button variant=ButtonVariant::Transparent disabled=true>
                        "Transparent disabled"
                    </Button>
                </div>
                <CodeExample code={r#"<Button disabled=true>"Primary disabled"</Button>
<Button variant=ButtonVariant::Secondary disabled=true>"Secondary disabled"</Button>
<Button variant=ButtonVariant::Transparent disabled=true>"Transparent disabled"</Button>"#}/>
            </Card>

            <Card header="Compact to spacious" class="doc-card">
                <span class="doc-card__kicker">"Sizes"</span>
                <div class="doc-card__preview doc-card__preview--stack">
                    <Button size=Size::Small>"Small · 1.5rem / 24px"</Button>
                    <Button size=Size::Medium>"Medium · 2rem / 32px"</Button>
                    <Button size=Size::Large>"Large · 2.5rem / 40px"</Button>
                </div>
                <CodeExample code={r#"<Button size=Size::Small>"Small · 1.5rem / 24px"</Button>
<Button size=Size::Medium>"Medium · 2rem / 32px"</Button>
<Button size=Size::Large>"Large · 2.5rem / 40px"</Button>"#}/>
            </Card>

            <Card header="Shared button context" class="doc-card">
                <span class="doc-card__kicker">"Grouping"</span>
                <div class="doc-card__preview doc-card__preview--stack">
                    <ButtonGroup size=Size::Small>
                        <Button variant=ButtonVariant::Primary>"Primary"</Button>
                        <Button variant=ButtonVariant::Secondary>"Secondary"</Button>
                        <Button disabled=true>"Disabled"</Button>
                    </ButtonGroup>
                    <ButtonGroup vertical=true size=Size::Small>
                        <Button variant=ButtonVariant::Primary>"Primary"</Button>
                        <Button variant=ButtonVariant::Secondary>"Secondary"</Button>
                        <Button disabled=true>"Disabled"</Button>
                    </ButtonGroup>
                </div>
                <CodeExample code={r#"<ButtonGroup vertical=true size=Size::Small>
    <Button variant=ButtonVariant::Primary>"Primary"</Button>
    <Button variant=ButtonVariant::Secondary>"Secondary"</Button>
    <Button disabled=true>"Disabled"</Button>
</ButtonGroup>"#}/>
            </Card>

            <Card header="Round, circle, block" class="doc-card">
                <span class="doc-card__kicker">"Shape"</span>
                <div class="doc-card__preview doc-card__preview--stack">
                    <Button round=true>"Rounded"</Button>
                    <Button circle=true>"×"</Button>
                    <Button block=true variant=ButtonVariant::Secondary>
                        "Full width action"
                    </Button>
                </div>
                <CodeExample code={r#"<Button round=true>"Rounded"</Button>
<Button circle=true>"×"</Button>
<Button block=true>"Full width action"</Button>"#}/>
            </Card>

            <Card header="Native button semantics" class="doc-card">
                <span class="doc-card__kicker">"Form type"</span>
                <div class="doc-card__preview doc-card__preview--stack">
                    <form
                        class="demo-form"
                        on:submit=move |event| {
                            event.prevent_default();
                            click_count.update(|count| *count += 1);
                        }
                        on:reset=move |_| {
                            name.set(String::from(DEFAULT_NAME));
                        }
                    >
                        <div class="field">
                            <Label text="Preview name" for_id="book-button-preview-name"/>
                            <Input
                                id="book-button-preview-name"
                                value=name
                                placeholder=DEFAULT_NAME
                                on_input=Callback::new(on_input)
                            />
                        </div>
                        <p class="demo-form__copy">
                            "Hello, "
                            <strong>{move || name.get()}</strong>
                            ". Submit increments the same counter from the page header."
                        </p>
                        <div class="demo-form__actions">
                            <Button button_type=ButtonType::Submit>"Submit"</Button>
                            <Button button_type=ButtonType::Reset variant=ButtonVariant::Secondary>
                                "Reset"
                            </Button>
                        </div>
                    </form>
                </div>
                <CodeExample code={r#"<form>
    <Label text="Preview name" for_id="preview-name" />
    <Input id="preview-name" value=name />
    <Button button_type=ButtonType::Submit>"Submit"</Button>
    <Button button_type=ButtonType::Reset>"Reset"</Button>
</form>"#}/>
            </Card>

        </section>
    }
}
