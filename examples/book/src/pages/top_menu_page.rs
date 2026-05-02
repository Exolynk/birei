use crate::code_example::CodeExample;
use birei::{
    ActionCard, Button, ButtonVariant, Card, CommandExecution, CommandItem, CommandPalette,
    Icon, NotificationManager, Size, TopMenuShell,
};
use leptos::prelude::*;

#[component]
pub fn TopMenuPage() -> impl IntoView {
    let query = RwSignal::new(String::new());
    let last_action = RwSignal::new(String::from("None"));

    let command_items = vec![
        CommandItem::new("open-dashboard", "Open dashboard")
            .icon("layout-dashboard")
            .shortcut("GD")
            .action(Callback::new(move |execution: CommandExecution| {
                let value = execution.item.value;
                last_action.set(value.clone());
                NotificationManager::global().success(format!("Command executed: {value}"));
            })),
        CommandItem::new("open-records", "Open records")
            .icon("database")
            .shortcut("GR")
            .action(Callback::new(move |execution: CommandExecution| {
                let value = execution.item.value;
                last_action.set(value.clone());
                NotificationManager::global().success(format!("Command executed: {value}"));
            })),
    ];

    let action_cards = vec![
        ("go-notifications", "Notifications", "Review latest updates", "bell"),
        ("go-automation", "Automation", "Open workflow actions", "bot"),
        ("go-settings", "Settings", "Manage workspace config", "settings-2"),
        ("go-users", "Users", "Manage user access", "users"),
        ("go-billing", "Billing", "Open billing overview", "credit-card"),
        ("go-reports", "Reports", "Browse analytics reports", "bar-chart-3"),
        ("go-integrations", "Integrations", "Configure external apps", "plug"),
        ("go-incidents", "Incidents", "Track open incidents", "siren"),
        ("go-backups", "Backups", "Review backup jobs", "database-backup"),
    ];
    let action_cards = StoredValue::new(action_cards);

    view! {
        <section class="page-header">
            <div class="page-header__eyebrow">"Component"</div>
            <h2>"Top Menu Shell"</h2>
            <p class="page-header__lede">
                "Composable top navigation shell with user-defined command area and popup action content."
            </p>
        </section>

        <section class="doc-grid">
            <Card header="Shell with dynamic actions" class="doc-card">
                <span class="doc-card__kicker">"Layout + Slots"</span>
                <div class="doc-card__preview doc-card__preview--stack">
                    <TopMenuShell
                        logo=move || view! { <Icon name="sparkles" size=Size::Small/> }
                        title=move || view! { <span>"Birei Ops"</span> }
                        command=move || view! {
                            <CommandPalette
                                items=command_items.clone()
                                query=query
                                on_query_change=Callback::new(move |next| query.set(next))
                            />
                        }
                        actions_content=move || view! {
                            <div class="book-top-menu-actions-grid">
                                <For
                                    each=move || action_cards.get_value().clone()
                                    key=move |card| card.0
                                    children=move |card| {
                                        let card_value = String::from(card.0);
                                        let title = String::from(card.1);
                                        let subtitle = String::from(card.2);
                                        let icon = String::from(card.3);

                                        view! {
                                            <ActionCard
                                                title=title
                                                subtitle=subtitle
                                                icon=icon
                                                on_click=Callback::new(move |_| {
                                                    last_action.set(card_value.clone());
                                                    NotificationManager::global().success(format!(
                                                        "Action selected: {}",
                                                        card_value
                                                    ));
                                                })
                                            />
                                        }
                                    }
                                />
                            </div>
                        }
                    />
                    <div class="doc-card__preview doc-card__preview--stack">
                        <p class="doc-card__copy">
                            "Main page content lives below the top menu component."
                        </p>
                        <p class="doc-card__copy">
                            "Last action: "
                            <strong>{move || last_action.get()}</strong>
                        </p>
                        <Button variant=ButtonVariant::Secondary>"Example content action"</Button>
                    </div>
                </div>
                <CodeExample rows=18 code={r#"<TopMenuShell
    logo=move || view! { <Icon name="sparkles" size=Size::Small/> }
    title=move || view! { <span>"Birei Ops"</span> }
    command=move || view! { <CommandPalette items=commands query=query /> }
    actions_content=move || view! {
        <ActionCard
            title="Settings"
            subtitle="Manage workspace config"
            icon="settings-2"
            on_click=Callback::new(move |_| {
                on_action.run(String::from("settings"));
            })
        />
    }
/>"#}/>
            </Card>
        </section>
    }
}
