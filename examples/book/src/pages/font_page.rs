use crate::code_example::{CodeExample, CodeExampleLanguage};
use birei::{Card, Slider, SliderStepLabel};
use leptos::prelude::*;

#[component]
pub fn FontPage() -> impl IntoView {
    let font_size = RwSignal::new(48.0_f64);
    let weight = RwSignal::new(500.0_f64);
    let width = RwSignal::new(100.0_f64);
    let italic = RwSignal::new(0.0_f64);
    let letter_pairs = ('A'..='Z')
        .map(|letter| format!("{letter}{}", letter.to_ascii_lowercase()))
        .collect::<Vec<_>>();
    let italic_steps = vec![
        SliderStepLabel::new(0.0, "Off"),
        SliderStepLabel::new(1.0, "On"),
    ];

    let specimen_style = move || {
        format!(
            "font-family: var(--birei-font-family-base); font-size: {}px; font-weight: {}; font-style: {}; font-variation-settings: \"wdth\" {};",
            font_size.get().round() as u16,
            weight.get().round() as u16,
            if italic.get() >= 0.5 { "italic" } else { "normal" },
            width.get().round() as u16
        )
    };

    view! {
        <section class="page-header">
            <div class="page-header__eyebrow">"Foundation"</div>
            <h2>"Font"</h2>
            <p class="page-header__lede">
                "Instrument Sans is bundled into the framework and exposed through the shared font-family CSS variable."
            </p>
        </section>

        <section class="doc-grid">
            <Card header="Live specimen" class="doc-card">
                <span class="doc-card__kicker">"Variable axes"</span>
                <div class="doc-card__preview doc-card__preview--stack">
                    <div class="book-font-controls">
                        <div class="book-font-control">
                            <div class="field__label">{move || format!("Size: {:.0}px", font_size.get())}</div>
                            <Slider
                                min=16.0
                                max=96.0
                                step=1.0
                                value=font_size
                                on_value_change=Callback::new(move |next| font_size.set(next))
                            />
                        </div>
                        <div class="book-font-control">
                            <div class="field__label">{move || format!("Weight: {:.0}", weight.get())}</div>
                            <Slider
                                min=400.0
                                max=700.0
                                step=1.0
                                value=weight
                                step_labels=vec![
                                    SliderStepLabel::new(400.0, "400"),
                                    SliderStepLabel::new(500.0, "500"),
                                    SliderStepLabel::new(600.0, "600"),
                                    SliderStepLabel::new(700.0, "700"),
                                ]
                                on_value_change=Callback::new(move |next| weight.set(next))
                            />
                        </div>
                        <div class="book-font-control">
                            <div class="field__label">{move || format!("Width: {:.0}", width.get())}</div>
                            <Slider
                                min=75.0
                                max=100.0
                                step=1.0
                                value=width
                                step_labels=vec![
                                    SliderStepLabel::new(75.0, "75"),
                                    SliderStepLabel::new(85.0, "85"),
                                    SliderStepLabel::new(100.0, "100"),
                                ]
                                on_value_change=Callback::new(move |next| width.set(next))
                            />
                        </div>
                        <div class="book-font-control">
                            <div class="field__label">
                                {move || format!("Italic: {}", if italic.get() >= 0.5 { "On" } else { "Off" })}
                            </div>
                            <Slider
                                min=0.0
                                max=1.0
                                step=1.0
                                value=italic
                                step_labels=italic_steps.clone()
                                on_value_change=Callback::new(move |next| italic.set(next))
                            />
                        </div>
                    </div>
                    <div class="book-font-specimen" style=specimen_style>
                        "Sphinx of black quartz, judge my vow."
                    </div>
                </div>
                <CodeExample
                    code={r#"font-family: var(--birei-font-family-base);
font-weight: 500;
font-style: normal;
font-variation-settings: "wdth" 100;"#}
                    language=CodeExampleLanguage::PlainText
                    title="Component CSS Variables"
                />
            </Card>

            <Card header="Letter pair overview" class="doc-card">
                <span class="doc-card__kicker">"Glyphs"</span>
                <div class="book-font-pairs" style=specimen_style>
                    {letter_pairs
                        .into_iter()
                        .map(|pair| {
                            view! {
                                <div class="book-font-pair">
                                    {pair}
                                </div>
                            }
                        })
                        .collect_view()}
                </div>
            </Card>
        </section>
    }
}
