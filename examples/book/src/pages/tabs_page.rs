use birei::{Card, TabItem, TabLinePosition, TabList};
use leptos::prelude::*;

#[component]
pub fn TabsPage() -> impl IntoView {
    let current_section = RwSignal::new(Some(String::from("overview")));
    let current_stage = RwSignal::new(Some(String::from("draft")));
    let current_top = RwSignal::new(Some(String::from("activity")));

    let section_tabs = vec![
        TabItem::new("overview", "Overview"),
        TabItem::new("activity", "Activity"),
        TabItem::new("members", "Members"),
        TabItem::new("settings", "Settings"),
    ];
    let stage_tabs = vec![
        TabItem::new("draft", "Draft"),
        TabItem::new("review", "In review"),
        TabItem::new("approved", "Approved"),
        TabItem::new("archived", "Archived").disabled(true),
    ];
    let top_tabs = vec![
        TabItem::new("overview", "Overview"),
        TabItem::new("activity", "Activity"),
        TabItem::new("members", "Members"),
        TabItem::new("settings", "Settings"),
    ];
    view! {
        <section class="page-header">
            <div class="page-header__eyebrow">"Component"</div>
            <h2>"Tabs"</h2>
            <p class="page-header__lede">
                "Horizontal tab triggers with a traveling selection line, bold active labels, and a lighter hover line for previewing adjacent sections."
            </p>
        </section>

        <section class="doc-grid">
            <Card header="Traveling underline" class="doc-card">
                <span class="doc-card__kicker">"Basics"</span>
                <div class="doc-card__preview doc-card__preview--stack">
                    <TabList
                        tabs=section_tabs.clone()
                        value=current_section
                        on_value_change=Callback::new(move |next| current_section.set(Some(next)))
                    />
                    <p class="doc-card__copy">
                        "Current section: "
                        <strong>{move || current_section.get().unwrap_or_else(|| String::from("None"))}</strong>
                    </p>
                </div>
                <pre class="doc-card__code"><code>{r#"<TabList
    tabs=vec![
        TabItem::new("overview", "Overview"),
        TabItem::new("activity", "Activity"),
        TabItem::new("members", "Members"),
        TabItem::new("settings", "Settings"),
    ]
    value=current_section
    on_value_change=Callback::new(move |next| current_section.set(Some(next)))
/>"#}</code></pre>
            </Card>

            <Card header="Disabled tab remains visible" class="doc-card">
                <span class="doc-card__kicker">"State"</span>
                <div class="doc-card__preview doc-card__preview--stack">
                    <TabList
                        tabs=stage_tabs.clone()
                        value=current_stage
                        on_value_change=Callback::new(move |next| current_stage.set(Some(next)))
                    />
                    <p class="doc-card__copy">
                        "Workflow stage: "
                        <strong>{move || current_stage.get().unwrap_or_else(|| String::from("None"))}</strong>
                    </p>
                </div>
                <pre class="doc-card__code"><code>{r#"<TabList
    tabs=vec![
        TabItem::new("draft", "Draft"),
        TabItem::new("review", "In review"),
        TabItem::new("approved", "Approved"),
        TabItem::new("archived", "Archived").disabled(true),
    ]
    value=current_stage
    on_value_change=Callback::new(move |next| current_stage.set(Some(next)))
/>"#}</code></pre>
            </Card>

            <Card header="Line above the tabs" class="doc-card">
                <span class="doc-card__kicker">"Position"</span>
                <div class="doc-card__preview doc-card__preview--stack">
                    <TabList
                        tabs=top_tabs.clone()
                        value=current_top
                        line_position=TabLinePosition::Above
                        on_value_change=Callback::new(move |next| current_top.set(Some(next)))
                    />
                    <p class="doc-card__copy">
                        "Top-aligned section: "
                        <strong>{move || current_top.get().unwrap_or_else(|| String::from("None"))}</strong>
                    </p>
                </div>
                <pre class="doc-card__code"><code>{r#"<TabList
    tabs=top_tabs.clone()
    value=current_top
    line_position=TabLinePosition::Above
    on_value_change=Callback::new(move |next| current_top.set(Some(next)))
/>"#}</code></pre>
            </Card>

        </section>
    }
}
