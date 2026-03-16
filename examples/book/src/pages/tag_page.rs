use birei::Tag;
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
            <div class="page-header__actions">
                <Tag label="Featured"/>
                <Tag label="Removable" on_remove=Callback::new(|_| {})/>
            </div>
        </section>

        <section class="doc-grid">
            <article class="doc-card">
                <div class="doc-card__header">
                    <span class="doc-card__kicker">"Basics"</span>
                    <h3>"Standalone tags"</h3>
                </div>
                <div class="doc-card__preview">
                    <Tag label="Design"/>
                    <Tag label="Engineering"/>
                    <Tag label="Docs"/>
                </div>
                <pre class="doc-card__code"><code>{r#"<Tag label="Design"/>
<Tag label="Engineering"/>
<Tag label="Docs"/>"#}</code></pre>
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
                                            label=topic
                                            on_remove=Callback::new(move |_| {
                                                topics.update(|items| items.retain(|item| item != &label));
                                            })
                                        />
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
    label="UI systems"
    on_remove=Callback::new(move |_| {
        topics.update(|items| items.retain(|item| item != "UI systems"));
    })
/>"#}</code></pre>
            </article>
        </section>
    }
}
