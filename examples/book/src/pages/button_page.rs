use birei::{Button, ButtonGroup, ButtonSize, ButtonType, ButtonVariant};
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
            <div class="page-header__actions">
                <Button
                    on_click=Callback::new(move |_| {
                        click_count.update(|count| *count += 1);
                    })
                >
                    "Clicked "
                    {move || click_count.get()}
                    " times"
                </Button>
                <Button variant=ButtonVariant::Secondary round=true>
                    "Secondary action"
                </Button>
                <Button variant=ButtonVariant::Transparent>
                    "Transparent action"
                </Button>
            </div>
        </section>

        <section class="doc-grid">
            <article class="doc-card">
                <div class="doc-card__header">
                    <span class="doc-card__kicker">"Variants"</span>
                    <h3>"Intent and emphasis"</h3>
                </div>
                <div class="doc-card__preview doc-card__preview--stack">
                    <Button variant=ButtonVariant::Primary>"Primary"</Button>
                    <Button variant=ButtonVariant::Secondary>"Secondary"</Button>
                    <Button variant=ButtonVariant::Transparent>"Transparent"</Button>
                </div>
                <pre class="doc-card__code"><code>{r#"<Button variant=ButtonVariant::Primary>"Primary"</Button>
<Button variant=ButtonVariant::Secondary>"Secondary"</Button>
<Button variant=ButtonVariant::Transparent>"Transparent"</Button>"#}</code></pre>
            </article>

            <article class="doc-card">
                <div class="doc-card__header">
                    <span class="doc-card__kicker">"State"</span>
                    <h3>"Disabled"</h3>
                </div>
                <div class="doc-card__preview doc-card__preview--stack">
                    <Button disabled=true>"Primary disabled"</Button>
                    <Button variant=ButtonVariant::Secondary disabled=true>
                        "Secondary disabled"
                    </Button>
                    <Button variant=ButtonVariant::Transparent disabled=true>
                        "Transparent disabled"
                    </Button>
                </div>
                <pre class="doc-card__code"><code>{r#"<Button disabled=true>"Primary disabled"</Button>
<Button variant=ButtonVariant::Secondary disabled=true>"Secondary disabled"</Button>
<Button variant=ButtonVariant::Transparent disabled=true>"Transparent disabled"</Button>"#}</code></pre>
            </article>

            <article class="doc-card">
                <div class="doc-card__header">
                    <span class="doc-card__kicker">"Sizes"</span>
                    <h3>"Compact to spacious"</h3>
                </div>
                <div class="doc-card__preview doc-card__preview--stack">
                    <Button size=ButtonSize::Small>"Small"</Button>
                    <Button size=ButtonSize::Medium>"Medium"</Button>
                    <Button size=ButtonSize::Large>"Large"</Button>
                </div>
                <pre class="doc-card__code"><code>{r#"<Button size=ButtonSize::Small>"Small"</Button>
<Button size=ButtonSize::Medium>"Medium"</Button>
<Button size=ButtonSize::Large>"Large"</Button>"#}</code></pre>
            </article>

            <article class="doc-card">
                <div class="doc-card__header">
                    <span class="doc-card__kicker">"Grouping"</span>
                    <h3>"Shared button context"</h3>
                </div>
                <div class="doc-card__preview doc-card__preview--stack">
                    <ButtonGroup variant=ButtonVariant::Secondary size=ButtonSize::Small>
                        <Button>"Back"</Button>
                        <Button>"Save"</Button>
                        <Button disabled=true>"Disabled"</Button>
                    </ButtonGroup>
                    <ButtonGroup vertical=true variant=ButtonVariant::Primary>
                        <Button>"North"</Button>
                        <Button>"Center"</Button>
                        <Button>"South"</Button>
                    </ButtonGroup>
                </div>
                <pre class="doc-card__code"><code>{r#"<ButtonGroup variant=ButtonVariant::Secondary size=ButtonSize::Small>
    <Button>"Back"</Button>
    <Button>"Save"</Button>
</ButtonGroup>"#}</code></pre>
            </article>

            <article class="doc-card">
                <div class="doc-card__header">
                    <span class="doc-card__kicker">"Shape"</span>
                    <h3>"Round, circle, block"</h3>
                </div>
                <div class="doc-card__preview doc-card__preview--stack">
                    <Button round=true>"Rounded"</Button>
                    <Button circle=true>"×"</Button>
                    <Button block=true variant=ButtonVariant::Secondary>
                        "Full width action"
                    </Button>
                </div>
                <pre class="doc-card__code"><code>{r#"<Button round=true>"Rounded"</Button>
<Button circle=true>"×"</Button>
<Button block=true>"Full width action"</Button>"#}</code></pre>
            </article>

            <article class="doc-card">
                <div class="doc-card__header">
                    <span class="doc-card__kicker">"Form type"</span>
                    <h3>"Native button semantics"</h3>
                </div>
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
                        <label class="field">
                            <span class="field__label">"Preview name"</span>
                            <input
                                class="field__input"
                                value=DEFAULT_NAME
                                prop:value=move || name.get()
                                on:input=on_input
                            />
                        </label>
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
                <pre class="doc-card__code"><code>{r#"<form>
    <input value="Aiko" />
    <Button button_type=ButtonType::Submit>"Submit"</Button>
    <Button button_type=ButtonType::Reset>"Reset"</Button>
</form>"#}</code></pre>
            </article>

            <article class="doc-card">
                <div class="doc-card__header">
                    <span class="doc-card__kicker">"Scoped theme"</span>
                    <h3>"Per-instance token overrides"</h3>
                </div>
                <div class="doc-card__preview">
                    <Button class="custom-accent">"Accent override"</Button>
                </div>
                <pre class="doc-card__code"><code>{r#"<Button class="custom-accent">"Accent override"</Button>

.custom-accent {
    --birei-button-bg: #A67676;
    --birei-button-bg-hover: #B08282;
    --birei-button-bg-active: #946767;
    --birei-button-border: #A67676;
    --birei-button-color: #FFFFFF;
    --birei-button-ripple-color: rgba(255, 255, 255, 0.24);
}"#}</code></pre>
            </article>
        </section>
    }
}
