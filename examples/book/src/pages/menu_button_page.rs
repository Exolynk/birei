use birei::{Card, MenuButton, MenuButtonItem, Size};
use leptos::prelude::*;

#[component]
pub fn MenuButtonPage() -> impl IntoView {
    let last_action = RwSignal::new(String::from("None yet"));

    let items = vec![
        MenuButtonItem::new("share", "Share link").icon("link"),
        MenuButtonItem::new("duplicate", "Duplicate").icon("copy"),
        MenuButtonItem::new("archive", "Archive").icon("archive"),
        MenuButtonItem::new("delete", "Delete").icon("trash-2"),
    ];
    let long_items = (1..=40)
        .map(|index| {
            MenuButtonItem::new(
                format!("action-{index:02}"),
                format!("Long action item {index:02}"),
            )
        })
        .collect::<Vec<_>>();

    view! {
        <section class="page-header">
            <div class="page-header__eyebrow">"Component"</div>
            <h2>"Menu Button"</h2>
            <p class="page-header__lede">
                "Button-triggered action menus that share the same floating popup behavior as the select without inheriting its combobox semantics."
            </p>
        </section>

        <section class="doc-grid">
            <Card header="Action menu" class="doc-card">
                <span class="doc-card__kicker">"Basics"</span>
                <div class="doc-card__preview">
                    <MenuButton
                        label="Project actions"
                        icon="settings-2"
                        items=items
                        on_select=Callback::new(move |value| last_action.set(value))
                    />
                    <p class="doc-card__copy">
                        "Last action: "
                        <strong>{move || last_action.get()}</strong>
                    </p>
                </div>
                <pre class="doc-card__code"><code>{r#"<MenuButton
    label="Project actions"
    icon=Some("settings-2")
    items=items
    on_select=Callback::new(move |value| last_action.set(value))
/>"#}</code></pre>
            </Card>

            <Card header="Shared button sizing" class="doc-card">
                <span class="doc-card__kicker">"Sizes"</span>
                <div class="doc-card__preview">
                    <MenuButton label="Small menu" items=long_items.clone() size=Size::Small/>
                    <MenuButton label="Medium menu" items=long_items.clone() size=Size::Medium/>
                    <MenuButton label="Large menu" items=long_items size=Size::Large/>
                </div>
                <pre class="doc-card__code"><code>{r#"<MenuButton label="Small menu" items=items.clone() size=Size::Small/>
<MenuButton label="Medium menu" items=items.clone() size=Size::Medium/>
<MenuButton label="Large menu" items=items size=Size::Large/>"#}</code></pre>
            </Card>
        </section>
    }
}
