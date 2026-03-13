use leptos::prelude::*;

#[component]
pub fn InputPage() -> impl IntoView {
    view! {
        <section class="page-header">
            <div class="page-header__eyebrow">"Component"</div>
            <h2>"Input"</h2>
            <p class="page-header__lede">
                "Input documentation will live here once the component API is implemented."
            </p>
        </section>

        <section class="doc-grid">
            <article class="doc-card">
                <div class="doc-card__header">
                    <span class="doc-card__kicker">"Status"</span>
                    <h3>"Planned"</h3>
                </div>
                <div class="doc-card__preview doc-card__preview--stack">
                    <p class="doc-card__copy">
                        "The router and sidebar are ready. Add the input component page content in this file when the public input API is available."
                    </p>
                </div>
                <pre class="doc-card__code"><code>{r#"// examples/book/src/pages/input_page.rs
    // Add Input examples here once the component exists."#}</code></pre>
            </article>
        </section>
    }
}
