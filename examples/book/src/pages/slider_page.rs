use birei::{Label, Size, Slider, SliderStepLabel};
use leptos::prelude::*;

#[component]
pub fn SliderPage() -> impl IntoView {
    let single = RwSignal::new(42.0);
    let stepped = RwSignal::new(2.0);
    let compact = RwSignal::new(30.0);
    let spacious = RwSignal::new(72.0);
    let invalid = RwSignal::new(1.0);

    let weight_steps = vec![
        SliderStepLabel::new(0.0, "Start"),
        SliderStepLabel::new(25.0, "Calm"),
        SliderStepLabel::new(50.0, "Balanced"),
        SliderStepLabel::new(75.0, "Bright"),
        SliderStepLabel::new(100.0, "Max"),
    ];
    let font_steps = vec![
        SliderStepLabel::new(0.0, "400"),
        SliderStepLabel::new(1.0, "500"),
        SliderStepLabel::new(2.0, "600"),
        SliderStepLabel::new(3.0, "700"),
    ];

    view! {
        <section class="page-header">
            <div class="page-header__eyebrow">"Component"</div>
            <h2>"Slider"</h2>
            <p class="page-header__lede">
                "Native range input wrapped with Birei sizing tokens, labeled steps, and an animated fill ripple that echoes the input underline treatment."
            </p>
        </section>

        <section class="doc-grid">
            <article class="doc-card">
                <div class="doc-card__header">
                    <span class="doc-card__kicker">"Basics"</span>
                    <h3>"Continuous range"</h3>
                </div>
                <div class="doc-card__preview doc-card__preview--stack">
                    <div class="field">
                        <Label text="Ambient intensity" for_id="book-slider-ambient"/>
                        <Slider
                            id="book-slider-ambient"
                            min=0.0
                            max=100.0
                            step=1.0
                            value=single
                            on_value_change=Callback::new(move |next| single.set(next))
                        />
                    </div>
                    <p class="doc-card__copy">
                        "Value: "
                        <strong>{move || format!("{:.0}", single.get())}</strong>
                    </p>
                </div>
                <pre class="doc-card__code"><code>{r#"<Slider
    min=0.0
    max=100.0
    step=1.0
    value=single
    on_value_change=Callback::new(move |next| single.set(next))
/>"#}</code></pre>
            </article>

            <article class="doc-card">
                <div class="doc-card__header">
                    <span class="doc-card__kicker">"Steps"</span>
                    <h3>"Discrete stops with labels"</h3>
                </div>
                <div class="doc-card__preview doc-card__preview--stack">
                    <div class="field">
                        <Label text="Weight preset" for_id="book-slider-weight-preset"/>
                        <Slider
                            id="book-slider-weight-preset"
                            min=0.0
                            max=3.0
                            step=1.0
                            value=stepped
                            step_labels=font_steps.clone()
                            on_value_change=Callback::new(move |next| stepped.set(next))
                        />
                    </div>
                    <p class="doc-card__copy">
                        "Selected weight: "
                        <strong>{move || match stepped.get().round() as i32 {
                            0 => "400",
                            1 => "500",
                            2 => "600",
                            _ => "700",
                        }}</strong>
                    </p>
                </div>
                <pre class="doc-card__code"><code>{r#"<Slider
    min=0.0
    max=3.0
    step=1.0
    value=stepped
    step_labels=vec![
        SliderStepLabel::new(0.0, "400"),
        SliderStepLabel::new(1.0, "500"),
        SliderStepLabel::new(2.0, "600"),
        SliderStepLabel::new(3.0, "700"),
    ]
    on_value_change=Callback::new(move |next| stepped.set(next))
/>"#}</code></pre>
            </article>

            <article class="doc-card">
                <div class="doc-card__header">
                    <span class="doc-card__kicker">"Sizes"</span>
                    <h3>"Shared with other controls"</h3>
                </div>
                <div class="doc-card__preview doc-card__preview--stack">
                    <Slider
                        size=Size::Small
                        min=0.0
                        max=100.0
                        value=compact
                        step_labels=weight_steps.clone()
                        on_value_change=Callback::new(move |next| compact.set(next))
                    />
                    <Slider
                        size=Size::Medium
                        min=0.0
                        max=100.0
                        value=single
                        on_value_change=Callback::new(move |next| single.set(next))
                    />
                    <Slider
                        size=Size::Large
                        min=0.0
                        max=100.0
                        value=spacious
                        step_labels=weight_steps
                        on_value_change=Callback::new(move |next| spacious.set(next))
                    />
                </div>
                <pre class="doc-card__code"><code>{r#"<Slider size=Size::Small step_labels=marks.clone()/>
<Slider size=Size::Medium/>
<Slider size=Size::Large step_labels=marks/>"#}</code></pre>
            </article>

            <article class="doc-card">
                <div class="doc-card__header">
                    <span class="doc-card__kicker">"State"</span>
                    <h3>"Disabled and invalid"</h3>
                </div>
                <div class="doc-card__preview doc-card__preview--stack">
                    <Slider min=0.0 max=100.0 value=55.0 disabled=true/>
                    <Slider
                        min=0.0
                        max=3.0
                        step=1.0
                        value=invalid
                        invalid=true
                        step_labels=font_steps
                        on_value_change=Callback::new(move |next| invalid.set(next))
                    />
                </div>
                <pre class="doc-card__code"><code>{r#"<Slider value=55.0 disabled=true/>
    <Slider value=invalid invalid=true step_labels=font_steps/>"#}</code></pre>
            </article>
        </section>
    }
}
