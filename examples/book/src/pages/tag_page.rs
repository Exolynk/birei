use birei::{Icon, Size, Tag};
use leptos::prelude::*;

#[component]
pub fn TagPage() -> impl IntoView {
    let topics = RwSignal::new(vec![
        String::from("UI systems"),
        String::from("Rust"),
        String::from("Documentation"),
    ]);

    view! {
        <section class="page-header">
            <div class="page-header__eyebrow">"Component"</div>
            <h2>"Tag"</h2>
            <p class="page-header__lede">
                "Compact inline tags for selected values, metadata chips, and removable token groups."
            </p>
        </section>

        <section class="doc-grid">
            <article class="doc-card">
                <div class="doc-card__header">
                    <span class="doc-card__kicker">"Basics"</span>
                    <h3>"Standalone tags"</h3>
                </div>
                <div class="doc-card__preview">
                    <Tag>"Design"</Tag>
                    <Tag>"Engineering"</Tag>
                    <Tag>"Docs"</Tag>
                </div>
                <pre class="doc-card__code"><code>{r#"<Tag>"Design"</Tag>
<Tag>"Engineering"</Tag>
<Tag>"Docs"</Tag>"#}</code></pre>
            </article>

            <article class="doc-card">
                <div class="doc-card__header">
                    <span class="doc-card__kicker">"Content"</span>
                    <h3>"Tags can include icons"</h3>
                </div>
                <div class="doc-card__preview">
                    <Tag>
                        <Icon name="palette" size=Size::Small label="Palette"/>
                        <span>" Design system"</span>
                    </Tag>
                    <Tag>
                        <Icon name="code" size=Size::Small label="Code"/>
                        <span>" Rust"</span>
                    </Tag>
                    <Tag>
                        <Icon name="rocket" size=Size::Small label="Rocket"/>
                        <span>" Launch"</span>
                    </Tag>
                </div>
                <pre class="doc-card__code"><code>{r#"<Tag>
    <Icon name="palette" size=Size::Small label="Palette"/>
    <span>" Design system"</span>
</Tag>"#}</code></pre>
            </article>

            <article class="doc-card">
                <div class="doc-card__header">
                    <span class="doc-card__kicker">"Removable"</span>
                    <h3>"Interactive token groups"</h3>
                </div>
                <div class="doc-card__preview doc-card__preview--stack">
                    <div class="doc-card__preview">
                        {move || {
                            topics
                                .get()
                                .into_iter()
                                .map(|topic| {
                                    let label = topic.clone();
                                    view! {
                                        <Tag
                                            on_remove=Callback::new(move |_| {
                                                topics.update(|items| items.retain(|item| item != &label));
                                            })
                                        >
                                            {topic}
                                        </Tag>
                                    }
                                })
                                .collect_view()
                        }}
                    </div>
                    <p class="doc-card__copy">
                        "The same component is used internally by the multi-select to render selected values."
                    </p>
                </div>
                <pre class="doc-card__code"><code>{r#"<Tag
    on_remove=Callback::new(move |_| {
        topics.update(|items| items.retain(|item| item != "UI systems"));
    })
>
    "UI systems"
</Tag>"#}</code></pre>
            </article>
        </section>
    }
}
