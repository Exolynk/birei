use birei::{icon, Icon, Size};
use leptos::prelude::*;

#[component]
pub fn IconPage() -> impl IntoView {
    view! {
        <section class="page-header">
            <div class="page-header__eyebrow">"Component"</div>
            <h2>"Icon"</h2>
            <p class="page-header__lede">
                "Lucide font icons wrapped in a typed component with shared sizing tokens and accessible labels."
            </p>
            <div class="page-header__actions">
                <Icon name="sparkles" size=Size::Large label="Sparkles"/>
                <Icon name="search" size=Size::Medium label="Search"/>
                <Icon name="arrow-right" size=Size::Medium label="Arrow right"/>
                <Icon name="settings-2" size=Size::Large label="Settings"/>
            </div>
        </section>

        <section class="doc-grid">
            <article class="doc-card">
                <div class="doc-card__header">
                    <span class="doc-card__kicker">"Basics"</span>
                    <h3>"String names or generated indices"</h3>
                </div>
                <div class="doc-card__preview doc-card__preview--stack">
                    <div class="icon-demo-row">
                        <Icon name="search" label="Search"/>
                        <code>"name=\"search\""</code>
                    </div>
                    <div class="icon-demo-row">
                        <Icon name=icon::MAIL label="Mail"/>
                        <code>"name=icon::MAIL"</code>
                    </div>
                    <div class="icon-demo-row">
                        <Icon name="arrow-right" label="Arrow right"/>
                        <code>"name=\"arrow-right\""</code>
                    </div>
                    <div class="icon-demo-row">
                        <Icon name=icon::SETTINGS_2 label="Settings"/>
                        <code>"name=icon::SETTINGS_2"</code>
                    </div>
                </div>
                <pre class="doc-card__code"><code>{r#"<Icon name="search" label="Search"/>
<Icon name=icon::MAIL label="Mail"/>
<Icon name="arrow-right" label="Arrow right"/>
<Icon name=icon::SETTINGS_2 label="Settings"/>"#}</code></pre>
            </article>

            <article class="doc-card">
                <div class="doc-card__header">
                    <span class="doc-card__kicker">"Sizes"</span>
                    <h3>"Aligned with the component scale"</h3>
                </div>
                <div class="doc-card__preview">
                    <div class="icon-demo-grid">
                        <div class="icon-demo-tile">
                            <Icon name="star" size=Size::Small label="Star small"/>
                            <span>"Small"</span>
                        </div>
                        <div class="icon-demo-tile">
                            <Icon name="star" size=Size::Medium label="Star medium"/>
                            <span>"Medium"</span>
                        </div>
                        <div class="icon-demo-tile">
                            <Icon name="star" size=Size::Large label="Star large"/>
                            <span>"Large"</span>
                        </div>
                    </div>
                </div>
                <pre class="doc-card__code"><code>{r#"<Icon name="star" size=Size::Small label="Star small"/>
<Icon name="star" size=Size::Medium label="Star medium"/>
<Icon name="star" size=Size::Large label="Star large"/>"#}</code></pre>
            </article>

            <article class="doc-card">
                <div class="doc-card__header">
                    <span class="doc-card__kicker">"Custom class"</span>
                    <h3>"Per-instance styling remains available"</h3>
                </div>
                <div class="doc-card__preview">
                    <div class="icon-demo-grid">
                        <div class="icon-demo-tile">
                            <Icon name="flame" size=Size::Large class="icon-accent-warm" label="Warm accent"/>
                            <span>"Warm"</span>
                        </div>
                        <div class="icon-demo-tile">
                            <Icon name="leaf" size=Size::Large class="icon-accent-cool" label="Cool accent"/>
                            <span>"Cool"</span>
                        </div>
                    </div>
                </div>
                <pre class="doc-card__code"><code>{r#"<Icon
    name="flame"
    size=Size::Large
    class="icon-accent-warm"
    label="Warm accent"
/>

.icon-accent-warm {
    color: #A67676;
}

.icon-accent-cool {
    color: #255459;
}"#}</code></pre>
            </article>
        </section>
    }
}
