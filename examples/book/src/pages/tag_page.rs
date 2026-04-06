use crate::code_example::CodeExample;
use birei::{Card, Icon, Size, Tag};
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
            <Card header="Standalone tags" class="doc-card">
                <span class="doc-card__kicker">"Basics"</span>
                <div class="doc-card__preview">
                    <Tag>"Design"</Tag>
                    <Tag>"Engineering"</Tag>
                    <Tag>"Docs"</Tag>
                </div>
                <CodeExample code={r#"<Tag>"Design"</Tag>
<Tag>"Engineering"</Tag>
<Tag>"Docs"</Tag>"#}/>
            </Card>

            <Card header="Tags can include icons" class="doc-card">
                <span class="doc-card__kicker">"Content"</span>
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
                <CodeExample code={r#"<Tag>
    <Icon name="palette" size=Size::Small label="Palette"/>
    <span>" Design system"</span>
</Tag>"#}/>
            </Card>

            <Card header="Interactive token groups" class="doc-card">
                <span class="doc-card__kicker">"Removable"</span>
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
                <CodeExample code={r#"<Tag
    on_remove=Callback::new(move |_| {
        topics.update(|items| items.retain(|item| item != "UI systems"));
    })
>
    "UI systems"
</Tag>"#}/>
            </Card>
        </section>
    }
}
