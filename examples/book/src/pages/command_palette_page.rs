use crate::code_example::CodeExample;
use birei::{
    Card, CommandExecution, CommandItem, CommandPalette, CommandParameterOption,
    NotificationManager,
};
use leptos::prelude::*;

#[component]
pub fn CommandPalettePage() -> impl IntoView {
    let query = RwSignal::new(String::new());
    let last_action = RwSignal::new(String::from("None"));
    let loading = RwSignal::new(false);
    let notify_action = move |execution: CommandExecution| {
        NotificationManager::global().success(format!("Executed action: {}", execution.item.name));
        last_action.set(execution.item.value);
    };

    let commands = Memo::new(move |_| {
        let mut items = vec![
            CommandItem::new("view-dashboard", "Open dashboard")
                .icon("layout-dashboard")
                .group("Views")
                .shortcut("GD")
                .action(Callback::new(notify_action)),
            CommandItem::new("view-records", "Open records")
                .description("Browse active model records")
                .icon("database")
                .group("Views")
                .shortcut("GR")
                .action(Callback::new(notify_action)),
            CommandItem::new("create-record", "Create record")
                .description("Start a new record in the active model")
                .icon("plus")
                .group("Commands")
                .shortcut("CR")
                .parameter_options(
                    "model",
                    "Record model",
                    vec![
                        CommandParameterOption::new("file", "File"),
                        CommandParameterOption::new("animal", "Animal"),
                        CommandParameterOption::new("user", "User Account"),
                    ],
                )
                .parameter("name", "Record name")
                .action(Callback::new(move |execution: CommandExecution| {
                    let model = execution
                        .parameters
                        .iter()
                        .find(|parameter| parameter.name == "model")
                        .map(|parameter| parameter.value.clone())
                        .unwrap_or_default();
                    let name = execution
                        .parameters
                        .iter()
                        .find(|parameter| parameter.name == "name")
                        .map(|parameter| parameter.value.clone())
                        .unwrap_or_default();
                    last_action.set(format!("create-record:{model}:{name}"));
                    NotificationManager::global()
                        .success(format!("{model} record with name {name} is created."));
                })),
            CommandItem::new("invite-user", "Invite user")
                .description("Send a workspace invitation")
                .icon("user-plus")
                .group("Commands")
                .action(Callback::new(notify_action)),
            CommandItem::new("sync-records", "Sync records")
                .description("Refresh records from the server")
                .icon("refresh-cw")
                .group("Commands")
                .action(Callback::new(notify_action)),
            CommandItem::new("archived-view", "Open archived view")
                .icon("archive")
                .group("Views")
                .action(Callback::new(notify_action))
                .disabled(true),
        ];

        let current_query = query.get();
        if current_query.trim().len() > 2 {
            items.push(
                CommandItem::new(
                    format!("record-{}", current_query.trim().to_lowercase()),
                    format!("Record matching '{}'", current_query.trim()),
                )
                .description("Async server result placeholder")
                .icon("file-text")
                .group("Records")
                .action(Callback::new(notify_action)),
            );
        }

        items
    });

    let recent = vec![
        CommandItem::new("recent-settings", "Open settings")
            .description("Recently used command")
            .icon("settings")
            .action(Callback::new(notify_action)),
        CommandItem::new("recent-profile", "Open profile")
            .description("Recently opened view")
            .icon("user")
            .action(Callback::new(notify_action)),
    ];

    view! {
        <section class="page-header">
            <div class="page-header__eyebrow">"Component"</div>
            <h2>"Command Palette"</h2>
            <p class="page-header__lede">
                "Global command launcher with a rounded trigger, shortcut label, recent items, async-ready query callbacks, and action execution."
            </p>
        </section>

        <section class="doc-grid">
            <Card header="Global command trigger" class="doc-card">
                <span class="doc-card__kicker">"Actions"</span>
                <div class="doc-card__preview doc-card__preview--stack">
                    <CommandPalette
                        items=commands
                        recent_items=recent
                        query=query
                        loading=loading
                        on_query_change=Callback::new(move |next: String| {
                            query.set(next.clone());
                            loading.set(next.trim().len() == 2);
                        })
                    />
                    <p class="doc-card__copy">
                        "Executed action: "
                        <strong>{move || last_action.get()}</strong>
                    </p>
                </div>
                <CodeExample rows=16 code={r#"let create_record = CommandItem::new("create-record", "Create record")
    .shortcut("CR")
    .parameter_options(
        "model",
        "Record model",
        vec![
            CommandParameterOption::new("file", "File"),
            CommandParameterOption::new("animal", "Animal"),
            CommandParameterOption::new("user", "User Account"),
        ],
    )
    .parameter("name", "Record name")
    .action(Callback::new(move |execution: CommandExecution| {
        let model = execution
            .parameters
            .iter()
            .find(|parameter| parameter.name == "model")
            .map(|parameter| parameter.value.clone())
            .unwrap_or_default();
        let name = execution
            .parameters
            .iter()
            .find(|parameter| parameter.name == "name")
            .map(|parameter| parameter.value.clone())
            .unwrap_or_default();
        NotificationManager::global()
            .success(format!("{model} record with name {name} is created."));
    }));

<CommandPalette
    items=commands
    recent_items=recent
    query=query
    loading=loading
    on_query_change=Callback::new(move |next| {
        query.set(next);
    })
/>"#}/>
            </Card>
        </section>
    }
}
