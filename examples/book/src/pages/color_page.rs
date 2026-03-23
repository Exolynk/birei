use birei::{Card, ColorInput, Label, Size};
use leptos::prelude::*;

#[component]
pub fn ColorPage() -> impl IntoView {
    let accent = RwSignal::new(String::from("#255459"));
    let highlight = RwSignal::new(String::from("#a67676"));
    let compact = RwSignal::new(String::from("#728a8c"));
    let default_size = RwSignal::new(String::from("#255459"));
    let large = RwSignal::new(String::from("#b8c4c580"));
    let invalid = RwSignal::new(String::from("not-a-color"));

    view! {
        <section class="page-header">
            <div class="page-header__eyebrow">"Component"</div>
            <h2>"Color Input"</h2>
            <p class="page-header__lede">
                "Hex color text field composed from the shared input shell, with a live swatch on the left and a native color picker trigger on the right."
            </p>
        </section>

        <section class="doc-grid">
            <Card header="Preview, text input, and picker" class="doc-card">
                <span class="doc-card__kicker">"Basics"</span>
                <div class="doc-card__preview doc-card__preview--stack">
                    <div class="field">
                        <Label text="Accent color" for_id="book-color-accent"/>
                        <ColorInput
                            id="book-color-accent"
                            value=accent
                            on_value_change=Callback::new(move |next| accent.set(next))
                        />
                    </div>
                    <p class="doc-card__copy">
                        "Current value: "
                        <strong>{move || accent.get()}</strong>
                    </p>
                </div>
                <pre class="doc-card__code"><code>{r#"<ColorInput
    id="accent-color"
    value=accent
    on_value_change=Callback::new(move |next| accent.set(next))
/>"#}</code></pre>
            </Card>

            <Card header="Clickable swatch and action button" class="doc-card">
                <span class="doc-card__kicker">"Affixes"</span>
                <div class="doc-card__preview doc-card__preview--stack">
                    <ColorInput
                        value=highlight
                        on_value_change=Callback::new(move |next| highlight.set(next))
                    />
                    <p class="doc-card__copy">
                        "Both the left swatch and the right palette button open the native color picker."
                    </p>
                </div>
                <pre class="doc-card__code"><code>{r#"<ColorInput value=highlight/>"#}</code></pre>
            </Card>

            <Card header="Shared control sizing" class="doc-card">
                <span class="doc-card__kicker">"Sizes"</span>
                <div class="doc-card__preview doc-card__preview--stack">
                    <ColorInput
                        size=Size::Small
                        value=compact
                        on_value_change=Callback::new(move |next| compact.set(next))
                    />
                    <ColorInput
                        size=Size::Medium
                        value=default_size
                        on_value_change=Callback::new(move |next| default_size.set(next))
                    />
                    <ColorInput
                        size=Size::Large
                        value=large
                        on_value_change=Callback::new(move |next| large.set(next))
                    />
                </div>
                <pre class="doc-card__code"><code>{r##"<ColorInput size=Size::Small value="#728a8c"/>
<ColorInput size=Size::Medium value="#255459"/>
<ColorInput size=Size::Large value="#b8c4c580"/>"##}</code></pre>
            </Card>

            <Card header="Invalid, readonly, disabled" class="doc-card">
                <span class="doc-card__kicker">"State"</span>
                <div class="doc-card__preview doc-card__preview--stack">
                    <ColorInput
                        value=invalid
                        on_value_change=Callback::new(move |next| invalid.set(next))
                    />
                    <ColorInput value="#255459" readonly=true/>
                    <ColorInput value="#728a8c" disabled=true/>
                </div>
                <pre class="doc-card__code"><code>{r##"<ColorInput value=invalid/>
<ColorInput value="#255459" readonly=true/>
<ColorInput value="#728a8c" disabled=true/>"##}</code></pre>
            </Card>

            <Card header="Alpha-aware hex values" class="doc-card">
                <span class="doc-card__kicker">"Alpha"</span>
                <div class="doc-card__preview doc-card__preview--stack">
                    <ColorInput
                        value=large
                        on_value_change=Callback::new(move |next| large.set(next))
                    />
                    <p class="doc-card__copy">
                        "The text field accepts "
                        <strong>"#RGBA"</strong>
                        " and "
                        <strong>"#RRGGBBAA"</strong>
                        ". The browser palette still edits the RGB portion only."
                    </p>
                </div>
                <pre class="doc-card__code"><code>{r##"<ColorInput value="#b8c4c580"/>"##}</code></pre>
            </Card>
        </section>
    }
}
