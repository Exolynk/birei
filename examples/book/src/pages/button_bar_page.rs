use birei::{ButtonBar, ButtonBarItem, ButtonVariant, Card, Size};
use leptos::prelude::*;

#[component]
pub fn ButtonBarPage() -> impl IntoView {
    let last_action = RwSignal::new(String::from("refresh"));

    let compact_items = vec![
        ButtonBarItem::new("refresh", "Refresh").icon("refresh-cw"),
        ButtonBarItem::new("filter", "Filter").icon("sliders-horizontal"),
        ButtonBarItem::new("sort", "Sort").icon("arrow-up-down"),
        ButtonBarItem::new("share", "Share").icon("share-2"),
    ];
    let many_items = vec![
        ButtonBarItem::new("refresh", "Refresh").icon("refresh-cw"),
        ButtonBarItem::new("filter", "Filter by status").icon("sliders-horizontal"),
        ButtonBarItem::new("sort", "Sort newest first").icon("arrow-up-down"),
        ButtonBarItem::new("group", "Group by owner").icon("folders"),
        ButtonBarItem::new("export", "Export CSV").icon("download"),
        ButtonBarItem::new("share", "Share view").icon("share-2"),
        ButtonBarItem::new("duplicate", "Duplicate preset").icon("copy"),
        ButtonBarItem::new("archive", "Archive selection").icon("archive"),
    ];
    let stateful_items = vec![
        ButtonBarItem::new("play", "Start sync").icon("play"),
        ButtonBarItem::new("pause", "Pause queue").icon("pause"),
        ButtonBarItem::new("stop", "Stop jobs").icon("square"),
        ButtonBarItem::new("delete", "Delete run")
            .icon("trash-2")
            .disabled(true),
    ];

    view! {
        <section class="page-header">
            <div class="page-header__eyebrow">"Component"</div>
            <h2>"Button Bar"</h2>
            <p class="page-header__lede">
                "Horizontal action bars that keep visible buttons inline until space runs out, then move the remainder into a dropdown."
            </p>
        </section>

        <section class="doc-grid">
            <Card header="Inline actions with icons" class="doc-card">
                <span class="doc-card__kicker">"Basics"</span>
                <div class="doc-card__preview doc-card__preview--stack">
                    <ButtonBar
                        items=compact_items.clone()
                        on_select=Callback::new(move |next| last_action.set(next))
                    />
                    <p class="doc-card__copy">
                        "Last action: " <strong>{move || last_action.get()}</strong>
                    </p>
                </div>
                <pre class="doc-card__code"><code>{r#"<ButtonBar
    items=vec![
        ButtonBarItem::new("refresh", "Refresh").icon("refresh-cw"),
        ButtonBarItem::new("filter", "Filter").icon("sliders-horizontal"),
        ButtonBarItem::new("sort", "Sort").icon("arrow-up-down"),
        ButtonBarItem::new("share", "Share").icon("share-2"),
    ]
    on_select=Callback::new(move |next| last_action.set(next))
/>"#}</code></pre>
            </Card>

            <Card header="Overflow dropdown for narrow widths" class="doc-card">
                <span class="doc-card__kicker">"Overflow"</span>
                <div class="doc-card__preview doc-card__preview--stack">
                    <div style="max-width: 34rem; width: 100%;">
                        <ButtonBar
                            items=many_items.clone()
                            variant=ButtonVariant::Secondary
                            on_select=Callback::new(move |next| last_action.set(next))
                        />
                    </div>
                    <p class="doc-card__copy">
                        "Resize the viewport or use the narrow card width to push trailing actions into the menu."
                    </p>
                </div>
                <pre class="doc-card__code"><code>{r#"<ButtonBar
    items=actions
    variant=ButtonVariant::Secondary
    on_select=Callback::new(move |next| last_action.set(next))
/>"#}</code></pre>
            </Card>

            <Card header="Sizes and disabled actions" class="doc-card">
                <span class="doc-card__kicker">"State"</span>
                <div class="doc-card__preview doc-card__preview--stack">
                    <ButtonBar
                        items=stateful_items.clone()
                        size=Size::Small
                        variant=ButtonVariant::Transparent
                        on_select=Callback::new(move |next| last_action.set(next))
                    />
                    <p class="doc-card__copy">
                        "Disabled actions stay visible when they fit and remain disabled in the overflow menu."
                    </p>
                </div>
                <pre class="doc-card__code"><code>{r#"<ButtonBar
    items=vec![
        ButtonBarItem::new("play", "Start sync").icon("play"),
        ButtonBarItem::new("pause", "Pause queue").icon("pause"),
        ButtonBarItem::new("stop", "Stop jobs").icon("square"),
        ButtonBarItem::new("delete", "Delete run").icon("trash-2").disabled(true),
    ]
    size=Size::Small
    variant=ButtonVariant::Transparent
/>"#}</code></pre>
            </Card>
        </section>
    }
}
