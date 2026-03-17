use leptos::ev;
use leptos::prelude::*;
use web_sys::HtmlInputElement;

#[component]
pub fn FontPage() -> impl IntoView {
    let font_size = RwSignal::new(48_u16);
    let weight = RwSignal::new(500_u16);
    let width = RwSignal::new(100_u16);
    let italic = RwSignal::new(0_u8);
    let letter_pairs = ('A'..='Z')
        .map(|letter| format!("{letter}{}", letter.to_ascii_lowercase()))
        .collect::<Vec<_>>();

    let update_weight = move |event: ev::Event| {
        if let Ok(value) = event_target::<HtmlInputElement>(&event)
            .value()
            .parse::<u16>()
        {
            weight.set(value);
        }
    };
    let update_font_size = move |event: ev::Event| {
        if let Ok(value) = event_target::<HtmlInputElement>(&event)
            .value()
            .parse::<u16>()
        {
            font_size.set(value);
        }
    };
    let update_width = move |event: ev::Event| {
        if let Ok(value) = event_target::<HtmlInputElement>(&event)
            .value()
            .parse::<u16>()
        {
            width.set(value);
        }
    };
    let update_italic = move |event: ev::Event| {
        if let Ok(value) = event_target::<HtmlInputElement>(&event)
            .value()
            .parse::<u8>()
        {
            italic.set(value);
        }
    };

    let specimen_style = move || {
        format!(
            "font-family: var(--birei-font-family-base); font-size: {}px; font-weight: {}; font-style: {}; font-variation-settings: \"wdth\" {};",
            font_size.get(),
            weight.get(),
            if italic.get() == 1 { "italic" } else { "normal" },
            width.get()
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
            <article class="doc-card">
                <div class="doc-card__header">
                    <span class="doc-card__kicker">"Variable axes"</span>
                    <h3>"Live specimen"</h3>
                </div>
                <div class="doc-card__preview doc-card__preview--stack">
                    <div class="book-font-controls">
                        <label class="book-font-control">
                            <span>"Size: " {move || font_size.get()} "px"</span>
                            <input type="range" min="16" max="96" step="1" value="48" on:input=update_font_size/>
                        </label>
                        <label class="book-font-control">
                            <span>"Weight: " {move || weight.get()}</span>
                            <input type="range" min="400" max="700" step="1" value="500" on:input=update_weight/>
                        </label>
                        <label class="book-font-control">
                            <span>"Width: " {move || width.get()}</span>
                            <input type="range" min="75" max="100" step="1" value="100" on:input=update_width/>
                        </label>
                        <label class="book-font-control">
                            <span>"Italic: " {move || italic.get()}</span>
                            <input type="range" min="0" max="1" step="1" value="0" on:input=update_italic/>
                        </label>
                    </div>
                    <div class="book-font-specimen" style=specimen_style>
                        "Sphinx of black quartz, judge my vow."
                    </div>
                </div>
                <pre class="doc-card__code"><code>{r#"font-family: var(--birei-font-family-base);
font-weight: 500;
font-style: normal;
font-variation-settings: "wdth" 100;"#}</code></pre>
            </article>

            <article class="doc-card">
                <div class="doc-card__header">
                    <span class="doc-card__kicker">"Glyphs"</span>
                    <h3>"Letter pair overview"</h3>
                </div>
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
            </article>
        </section>
    }
}
