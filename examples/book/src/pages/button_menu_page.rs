use birei::{ButtonMenu, ButtonMenuItem, ButtonVariant, Card, Size};
use leptos::prelude::*;
use crate::code_example::CodeExample;

#[component]
pub fn ButtonMenuPage() -> impl IntoView {
    let last_action = RwSignal::new(String::from("None yet"));

    let items = vec![
        ButtonMenuItem::new("share", "Share link").icon("link"),
        ButtonMenuItem::new("duplicate", "Duplicate").icon("copy"),
        ButtonMenuItem::new("archive", "Archive").icon("archive"),
        ButtonMenuItem::new("delete", "Delete").icon("trash-2"),
    ];
    let basic_items = items.clone();
    let variant_items = items.clone();
    let long_items = (1..=40)
        .map(|index| {
            ButtonMenuItem::new(
                format!("action-{index:02}"),
                format!("Long action item {index:02}"),
            )
        })
        .collect::<Vec<_>>();

    view! {
        <section class="page-header">
            <div class="page-header__eyebrow">"Component"</div>
            <h2>"Button Menu"</h2>
            <p class="page-header__lede">
                "Button-triggered action menus that share the same floating popup behavior as the select without inheriting its combobox semantics."
            </p>
        </section>

        <section class="doc-grid">
            <Card header="Action menu" class="doc-card">
                <span class="doc-card__kicker">"Basics"</span>
                <div class="doc-card__preview">
                    <ButtonMenu
                        label="Project actions"
                        icon="settings-2"
                        items=basic_items
                        on_select=Callback::new(move |value| last_action.set(value))
                    />
                    <p class="doc-card__copy">
                        "Last action: "
                        <strong>{move || last_action.get()}</strong>
                    </p>
                </div>
                <CodeExample code={r#"<ButtonMenu
    label="Project actions"
    icon=Some("settings-2")
    items=items
    on_select=Callback::new(move |value| last_action.set(value))
/>"#}/>
            </Card>

            <Card header="Shared button sizing" class="doc-card">
                <span class="doc-card__kicker">"Sizes"</span>
                <div class="doc-card__preview">
                    <ButtonMenu label="Small menu" items=long_items.clone() size=Size::Small/>
                    <ButtonMenu label="Medium menu" items=long_items.clone() size=Size::Medium/>
                    <ButtonMenu label="Large menu" items=long_items size=Size::Large/>
                </div>
                <CodeExample code={r#"<ButtonMenu label="Small menu" items=items.clone() size=Size::Small/>
<ButtonMenu label="Medium menu" items=items.clone() size=Size::Medium/>
<ButtonMenu label="Large menu" items=items size=Size::Large/>"#}/>
            </Card>

            <Card header="Shared button variants" class="doc-card">
                <span class="doc-card__kicker">"Variants"</span>
                <div class="doc-card__preview">
                    <ButtonMenu
                        label="Primary menu"
                        items=variant_items.clone()
                        variant=ButtonVariant::Primary
                    />
                    <ButtonMenu
                        label="Secondary menu"
                        items=variant_items.clone()
                        variant=ButtonVariant::Secondary
                    />
                    <ButtonMenu
                        label="Transparent menu"
                        items=variant_items
                        variant=ButtonVariant::Transparent
                    />
                </div>
                <CodeExample code={r#"<ButtonMenu label="Primary menu" items=items.clone() variant=ButtonVariant::Primary/>
<ButtonMenu label="Secondary menu" items=items.clone() variant=ButtonVariant::Secondary/>
<ButtonMenu label="Transparent menu" items=items.clone() variant=ButtonVariant::Transparent/>"#}/>
            </Card>
        </section>
    }
}
