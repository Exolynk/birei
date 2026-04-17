use crate::code_example::{CodeExample, CodeExampleLanguage};
use birei::{Button, ButtonVariant, Card, SignPad, SignPadRef};
use leptos::prelude::*;

const SAMPLE_SIGNATURE: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="520" height="200" viewBox="0 0 520 200" fill="none"><g stroke="#1f2a2b" fill="#1f2a2b"><circle cx="100.00" cy="118.00" r="0.52" /><line x1="100.00" y1="118.00" x2="114.00" y2="109.00" stroke-width="4.20" stroke-linecap="round" /><circle cx="114.00" cy="109.00" r="0.46" /><line x1="114.00" y1="109.00" x2="136.00" y2="93.00" stroke-width="3.85" stroke-linecap="round" /><circle cx="136.00" cy="93.00" r="0.43" /><line x1="136.00" y1="93.00" x2="156.00" y2="83.00" stroke-width="3.30" stroke-linecap="round" /><circle cx="156.00" cy="83.00" r="0.37" /><line x1="156.00" y1="83.00" x2="178.00" y2="80.00" stroke-width="2.95" stroke-linecap="round" /><circle cx="178.00" cy="80.00" r="0.34" /><line x1="178.00" y1="80.00" x2="201.00" y2="88.00" stroke-width="2.70" stroke-linecap="round" /><circle cx="201.00" cy="88.00" r="0.31" /><line x1="201.00" y1="88.00" x2="226.00" y2="109.00" stroke-width="2.62" stroke-linecap="round" /><circle cx="226.00" cy="109.00" r="0.30" /><line x1="226.00" y1="109.00" x2="250.00" y2="125.00" stroke-width="2.48" stroke-linecap="round" /><circle cx="250.00" cy="125.00" r="0.29" /><line x1="250.00" y1="125.00" x2="281.00" y2="117.00" stroke-width="2.41" stroke-linecap="round" /><circle cx="281.00" cy="117.00" r="0.28" /><line x1="281.00" y1="117.00" x2="311.00" y2="87.00" stroke-width="2.36" stroke-linecap="round" /><circle cx="311.00" cy="87.00" r="0.28" /><line x1="311.00" y1="87.00" x2="338.00" y2="67.00" stroke-width="2.64" stroke-linecap="round" /><circle cx="338.00" cy="67.00" r="0.30" /><line x1="338.00" y1="67.00" x2="366.00" y2="71.00" stroke-width="2.78" stroke-linecap="round" /><circle cx="366.00" cy="71.00" r="0.32" /><line x1="366.00" y1="71.00" x2="399.00" y2="103.00" stroke-width="2.91" stroke-linecap="round" /><circle cx="399.00" cy="103.00" r="0.33" /></g></svg>"##;

#[component]
pub fn SignPadPage() -> impl IntoView {
    let signpad_ref = SignPadRef::new();
    let exported_svg = RwSignal::new(String::new());
    let readonly_svg = RwSignal::new(String::from(SAMPLE_SIGNATURE));

    let export_signature = {
        let signpad_ref = signpad_ref.clone();
        move |_| {
            let svg = signpad_ref.export_svg().unwrap_or_default();
            exported_svg.set(svg.clone());
            if !svg.is_empty() {
                readonly_svg.set(svg);
            }
        }
    };

    let load_sample = {
        let signpad_ref = signpad_ref.clone();
        move |_| {
            let _ = signpad_ref.load_svg(SAMPLE_SIGNATURE);
            exported_svg.set(String::from(SAMPLE_SIGNATURE));
            readonly_svg.set(String::from(SAMPLE_SIGNATURE));
        }
    };

    view! {
        <section class="page-header">
            <div class="page-header__eyebrow">"Component"</div>
            <h2>"Sign Pad"</h2>
            <p class="page-header__lede">
                "Fixed-size signature capture with speed-sensitive stroke width, stylus pressure support, and SVG import/export through an imperative handle."
            </p>
        </section>

        <section class="doc-grid">
            <Card header="Interactive capture" class="doc-card">
                <span class="doc-card__kicker">"Live pad"</span>
                <div class="doc-card__preview doc-card__preview--stack">
                    <SignPad
                        signpad_ref=signpad_ref.clone()
                        width=520
                        height=200
                        aria_label="Draw a signature"
                    />
                    <div class="book-sign-pad-actions">
                        <Button on_click=Callback::new(export_signature)>"Export SVG"</Button>
                        <Button variant=ButtonVariant::Secondary on_click=Callback::new(load_sample)>
                            "Load sample"
                        </Button>
                    </div>
                </div>
                <CodeExample
                    code={r#"let signpad_ref = SignPadRef::new();

<SignPad
    signpad_ref=signpad_ref.clone()
    width=520
    height=200
/>

<Button on_click=Callback::new(move |_| {
    let svg = signpad_ref.export_svg().unwrap_or_default();
    logging::log!("{svg}");
})>
    "Export SVG"
</Button>"#}
                />
            </Card>

            <Card header="Readonly and disabled" class="doc-card">
                <span class="doc-card__kicker">"Display states"</span>
                <div class="book-sign-pad-gallery">
                    <div class="book-sign-pad-gallery__item">
                        <span class="field__label">"Readonly"</span>
                        <SignPad value=readonly_svg readonly=true width=520 height=200 aria_label="Readonly signature preview"/>
                    </div>
                    <div class="book-sign-pad-gallery__item">
                        <span class="field__label">"Disabled"</span>
                        <SignPad value=readonly_svg disabled=true width=520 height=200 aria_label="Disabled signature preview"/>
                    </div>
                </div>
            </Card>

            <Card header="SVG export" class="doc-card">
                <span class="doc-card__kicker">"Serialized output"</span>
                <CodeExample
                    code_signal=Signal::derive(move || {
                        let current = exported_svg.get();
                        if current.is_empty() {
                            String::from("Export the current signature to inspect the generated SVG.")
                        } else {
                            current
                        }
                    })
                    language=CodeExampleLanguage::PlainText
                    title="SVG XML"
                    rows=12
                />
            </Card>
        </section>
    }
}
