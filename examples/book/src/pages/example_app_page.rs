use birei::{
    ButtonBar, ButtonBarItem, ButtonVariant, CommandExecution, CommandItem, CommandPalette,
    CommandParameterOption, FlexibleColumn, FlexibleColumns, Input, Label, List, ListDensity,
    ListEntry, MarkdownEditor, NotificationManager, TabItem, TabList, Timeline, TimelineItem,
    TopMenuShell,
};
use leptos::ev;
use leptos::prelude::*;

#[derive(Clone, Copy)]
struct Equipment {
    id: &'static str,
    name: &'static str,
    category: &'static str,
    model: &'static str,
    owner: &'static str,
    os: &'static str,
    status: &'static str,
    serial: &'static str,
    last_seen: &'static str,
    location: &'static str,
}

const EQUIPMENT: [Equipment; 9] = [
    Equipment {
        id: "eq-001",
        name: "Dell Latitude 7440",
        category: "Laptop",
        model: "Latitude 7440",
        owner: "Mina Keller",
        os: "Windows 11 Pro 24H2",
        status: "Managed",
        serial: "DL74-18Q2",
        last_seen: "Today, 10:12",
        location: "Zurich HQ",
    },
    Equipment {
        id: "eq-002",
        name: "MacBook Pro 14",
        category: "Laptop",
        model: "MacBook Pro 14 M3",
        owner: "Noah Ito",
        os: "macOS 15.4",
        status: "Managed",
        serial: "MBP3-22AB",
        last_seen: "Today, 09:41",
        location: "Basel Office",
    },
    Equipment {
        id: "eq-003",
        name: "ThinkPad X1 Carbon",
        category: "Laptop",
        model: "ThinkPad X1 Gen 11",
        owner: "Lena Gashi",
        os: "Windows 11 Pro 24H2",
        status: "Attention",
        serial: "TPX1-2G9L",
        last_seen: "Today, 07:55",
        location: "Remote",
    },
    Equipment {
        id: "eq-004",
        name: "Surface Pro 10",
        category: "Tablet",
        model: "Surface Pro 10",
        owner: "Kai Huber",
        os: "Windows 11 Pro 24H2",
        status: "Managed",
        serial: "SF10-BB20",
        last_seen: "Yesterday, 18:20",
        location: "Zurich HQ",
    },
    Equipment {
        id: "eq-005",
        name: "iPad Pro 13",
        category: "Tablet",
        model: "iPad Pro 13 M4",
        owner: "Sora Kim",
        os: "iPadOS 19.1",
        status: "Managed",
        serial: "IPD4-4R8N",
        last_seen: "Yesterday, 16:48",
        location: "Bern Office",
    },
    Equipment {
        id: "eq-006",
        name: "Galaxy Tab S10",
        category: "Tablet",
        model: "Galaxy Tab S10",
        owner: "Mats Weber",
        os: "Android 16",
        status: "Offline",
        serial: "GTS0-Z19A",
        last_seen: "2 days ago",
        location: "Warehouse",
    },
    Equipment {
        id: "eq-007",
        name: "HP ZBook Studio",
        category: "PC",
        model: "ZBook Studio G11",
        owner: "Anja Pfeiffer",
        os: "Windows 11 Pro 24H2",
        status: "Managed",
        serial: "HPZB-92DK",
        last_seen: "Today, 08:13",
        location: "Zurich HQ",
    },
    Equipment {
        id: "eq-008",
        name: "Intel NUC 14 Pro",
        category: "PC",
        model: "NUC 14 Pro",
        owner: "Lab Pool",
        os: "Ubuntu 24.04 LTS",
        status: "Managed",
        serial: "NUC1-J2LQ",
        last_seen: "Today, 06:21",
        location: "Testing Lab",
    },
    Equipment {
        id: "eq-009",
        name: "Mac Mini M4",
        category: "PC",
        model: "Mac Mini M4",
        owner: "Ops Shared",
        os: "macOS 15.4",
        status: "Pending setup",
        serial: "MCM4-U8WE",
        last_seen: "Never",
        location: "IT Depot",
    },
];

#[component]
pub fn ExampleAppPage() -> impl IntoView {
    let selected_equipment = RwSignal::new(Some(String::from(EQUIPMENT[0].id)));
    let detail_tab = RwSignal::new(String::from("overview"));
    let comment_markdown = RwSignal::new(String::new());
    let command_query = RwSignal::new(String::new());
    let inventory_search = RwSignal::new(String::new());
    let focused = RwSignal::new(FlexibleColumn::Middle);
    let ratios = RwSignal::new([22.0_f32, 58.0_f32, 20.0_f32]);

    let inventory_entries = Signal::derive(move || {
        let query = inventory_search.get().trim().to_lowercase();
        EQUIPMENT
            .iter()
            .filter(|item| {
                query.is_empty()
                    || item.name.to_lowercase().contains(&query)
                    || item.category.to_lowercase().contains(&query)
                    || item.owner.to_lowercase().contains(&query)
            })
            .map(|item| {
                ListEntry::new(item.id, item.name)
                    .description(format!("{} · {}", item.category, item.owner))
                    .meta(item.status)
            })
            .collect::<Vec<_>>()
    });

    let selected = Signal::derive(move || {
        let selected_id = selected_equipment
            .get()
            .unwrap_or_else(|| String::from(EQUIPMENT[0].id));
        EQUIPMENT
            .iter()
            .find(|item| item.id == selected_id)
            .copied()
            .unwrap_or(EQUIPMENT[0])
    });

    let detail_tabs = vec![
        TabItem::new("overview", "Overview"),
        TabItem::new("history", "History"),
        TabItem::new("comment", "Comment"),
    ];
    let list_tabs = vec![TabItem::new("inventory", "Inventory")];
    let action_items = vec![
        ButtonBarItem::new("reboot", "Reboot").icon("power"),
        ButtonBarItem::new("lock", "Lock").icon("lock"),
        ButtonBarItem::new("reinstall", "Reinstall OS").icon("wrench"),
        ButtonBarItem::new("rotate", "Rotate key").icon("key-round"),
        ButtonBarItem::new("retire", "Retire").icon("archive"),
    ];
    let notify_selected_command = move |execution: CommandExecution| {
        let item = selected.get();
        NotificationManager::global()
            .success(format!("{} for {}.", execution.item.name, item.name));
    };

    let command_items = vec![
        CommandItem::new("register-device", "Register device")
            .description("Create a new inventory entry")
            .icon("plus")
            .group("Device")
            .shortcut("RD")
            .parameter_options(
                "category",
                "Device category",
                vec![
                    CommandParameterOption::new("Laptop", "Laptop"),
                    CommandParameterOption::new("Tablet", "Tablet"),
                    CommandParameterOption::new("PC", "PC"),
                    CommandParameterOption::new("Peripheral", "Peripheral"),
                ],
            )
            .parameter("name", "Device name")
            .action(Callback::new(move |execution: CommandExecution| {
                let category = command_parameter(&execution, "category");
                let name = command_parameter(&execution, "name");
                NotificationManager::global()
                    .success(format!("Created {category} device \"{name}\"."));
            })),
        CommandItem::new("sync-fleet", "Run sync job")
            .description("Refresh agent state from all managed devices")
            .icon("refresh-cw")
            .group("Operations")
            .shortcut("SY")
            .action(Callback::new(notify_selected_command)),
        CommandItem::new("reboot-device", "Reboot selected device")
            .description("Queue a restart for the active endpoint")
            .icon("power")
            .group("Operations")
            .shortcut("RB")
            .action(Callback::new(notify_selected_command)),
        CommandItem::new("lock-device", "Lock selected device")
            .description("Force a remote lock on the current endpoint")
            .icon("lock")
            .group("Operations")
            .shortcut("LK")
            .action(Callback::new(notify_selected_command)),
        CommandItem::new("rotate-key", "Rotate recovery key")
            .description("Issue a new recovery credential")
            .icon("key-round")
            .group("Security")
            .shortcut("RK")
            .action(Callback::new(notify_selected_command)),
        CommandItem::new("compliance-report", "Open compliance report")
            .description("Review policy posture for the fleet")
            .icon("shield-check")
            .group("Reports")
            .shortcut("CP")
            .action(Callback::new(move |_| {
                NotificationManager::global().info("Compliance report opened.");
            })),
        CommandItem::new("export-status", "Export current status")
            .description("Download a CSV for the visible inventory")
            .icon("download")
            .group("Reports")
            .shortcut("EX")
            .action(Callback::new(move |_| {
                NotificationManager::global().success("Current fleet status export started.");
            })),
    ];

    view! {
        <section class="book-example-app-page">
            <TopMenuShell
                class="book-example-app-menu"
                logo=move || view! { <birei::Icon name="monitor-cog" size=birei::Size::Small/> }
                title=move || view! { <span>"Device Fleet Console"</span> }
                popup_width=530.0
                popup_height=260.0
                command=move || view! {
                    <CommandPalette
                        items=command_items.clone()
                        query=command_query
                        on_query_change=Callback::new(move |next| command_query.set(next))
                    />
                }
                actions_content=move || view! {
                    <div class="book-top-menu-actions-grid">
                        <birei::ActionCard
                            title="Add Device"
                            subtitle="Enroll new endpoint"
                            icon="plus"
                            on_click=Callback::new(move |_| {
                                NotificationManager::global().success("Add Device action opened.");
                            })
                        />
                        <birei::ActionCard
                            title="Bulk Update"
                            subtitle="Push config profile"
                            icon="folder-sync"
                            on_click=Callback::new(move |_| {
                                NotificationManager::global().success("Bulk Update action opened.");
                            })
                        />
                        <birei::ActionCard
                            title="Alerts"
                            subtitle="Open incident queue"
                            icon="triangle-alert"
                            on_click=Callback::new(move |_| {
                                NotificationManager::global().warning("Alerts action opened.");
                            })
                        />
                        <birei::ActionCard
                            title="Reports"
                            subtitle="Export current status"
                            icon="file-chart-column"
                            on_click=Callback::new(move |_| {
                                NotificationManager::global().info("Reports action opened.");
                            })
                        />
                    </div>
                }
            />

            <FlexibleColumns
                class="book-example-app-columns"
                focused=focused
                initial_ratios=ratios
                on_focus_change=Callback::new(move |next| focused.set(next))
                on_ratios_change=Callback::new(move |next| ratios.set(next))
                start=move || {
                        view! {
                            <div class="book-example-pane">
                                <div class="book-example-pane__head">
                                    <TabList
                                        tabs=list_tabs.clone()
                                    />
                                    <Input
                                        placeholder="Search devices"
                                        value=inventory_search
                                        on_input=Callback::new(move |event: ev::Event| {
                                            inventory_search.set(event_target_value(&event));
                                        })
                                    />
                                </div>
                                <div class="book-example-pane__body book-example-pane__body--list">
                                    <List
                                        items=inventory_entries
                                        density=ListDensity::Detailed
                                        selected=selected_equipment
                                        on_selected_change=Callback::new(move |next: Option<String>| {
                                            selected_equipment.set(next);
                                        })
                                    />
                                </div>
                            </div>
                        }
                    }
                    middle=move || {
                        view! {
                            <div class="book-example-pane">
                                <div class="book-example-pane__head">
                                    <TabList
                                        tabs=detail_tabs.clone()
                                        value=Signal::derive(move || Some(detail_tab.get()))
                                        on_value_change=Callback::new(move |next: String| {
                                            detail_tab.set(next);
                                        })
                                    />
                                    <ButtonBar
                                        items=action_items.clone()
                                        variant=ButtonVariant::Transparent
                                        on_select=Callback::new(move |action: String| {
                                            let item = selected.get();
                                            NotificationManager::global().success(format!(
                                                "{} queued for {}.",
                                                action_label(&action),
                                                item.name
                                            ));
                                        })
                                    />
                                </div>
                                <div class="book-example-pane__body book-example-pane__body--scroll">
                                    {move || {
                                        let item = selected.get();
                                        match detail_tab.get().as_str() {
                                            "overview" => view! {
                                                <div class="book-example-overview">
                                                    <h3>{item.name}</h3>
                                                    <div class="book-example-overview-form">
                                                        <div class="field">
                                                            <Label text="Category" for_id="eq-category"/>
                                                            <Input id="eq-category" value=item.category/>
                                                        </div>
                                                        <div class="field">
                                                            <Label text="Model" for_id="eq-model"/>
                                                            <Input id="eq-model" value=item.model/>
                                                        </div>
                                                        <div class="field">
                                                            <Label text="Owner" for_id="eq-owner"/>
                                                            <Input id="eq-owner" value=item.owner/>
                                                        </div>
                                                        <div class="field">
                                                            <Label text="Operating System" for_id="eq-os"/>
                                                            <Input id="eq-os" value=item.os/>
                                                        </div>
                                                        <div class="field">
                                                            <Label text="Status" for_id="eq-status"/>
                                                            <Input id="eq-status" value=item.status/>
                                                        </div>
                                                        <div class="field">
                                                            <Label text="Serial" for_id="eq-serial"/>
                                                            <Input id="eq-serial" value=item.serial/>
                                                        </div>
                                                        <div class="field">
                                                            <Label text="Last Seen" for_id="eq-last-seen"/>
                                                            <Input id="eq-last-seen" value=item.last_seen/>
                                                        </div>
                                                        <div class="field">
                                                            <Label text="Location" for_id="eq-location"/>
                                                            <Input id="eq-location" value=item.location/>
                                                        </div>
                                                    </div>
                                                </div>
                                            }.into_any(),
                                            "history" => view! {
                                                <Timeline>
                                                    <TimelineItem
                                                        icon="check-circle-2"
                                                        name="Agent"
                                                        title="Compliance check completed"
                                                        subtitle="Today · 09:14"
                                                    >
                                                        <p>{format!("{} passed the last policy sweep.", item.name)}</p>
                                                    </TimelineItem>
                                                    <TimelineItem
                                                        icon="refresh-cw"
                                                        name="System"
                                                        title="Patch baseline updated"
                                                        subtitle="Yesterday · 21:03"
                                                    >
                                                        <p>{format!("{} received cumulative updates.", item.name)}</p>
                                                    </TimelineItem>
                                                    <TimelineItem
                                                        icon="user-round-check"
                                                        name="IT Ops"
                                                        title="Owner assignment confirmed"
                                                        subtitle="Yesterday · 10:36"
                                                    >
                                                        <p>{format!("Ownership mapped to {}.", item.owner)}</p>
                                                    </TimelineItem>
                                                </Timeline>
                                            }.into_any(),
                                            "comment" => view! {
                                                <div class="book-example-comment">
                                                    <MarkdownEditor
                                                        value=comment_markdown
                                                        placeholder="Add maintenance notes for this device"
                                                        on_change=Callback::new(move |next| {
                                                            comment_markdown.set(next);
                                                        })
                                                    />
                                                </div>
                                            }.into_any(),
                                            _ => ().into_any(),
                                        }
                                    }}
                                </div>
                            </div>
                        }
                    }
                    end=move || {
                        view! {
                            <div class="book-example-pane">
                                <div class="book-example-pane__head">
                                    <h3 class="book-example-pane__title">"Quick Info"</h3>
                                </div>
                                <div class="book-example-pane__body book-example-pane__body--scroll">
                                    {move || {
                                        let item = selected.get();
                                        view! {
                                            <div class="book-example-side">
                                                <p><strong>"Selected"</strong></p>
                                                <p>{item.name}</p>
                                                <p><strong>"Current Owner"</strong></p>
                                                <p>{item.owner}</p>
                                                <p><strong>"Status"</strong></p>
                                                <p>{item.status}</p>
                                                <p><strong>"Last Seen"</strong></p>
                                                <p>{item.last_seen}</p>
                                            </div>
                                        }
                                    }}
                                </div>
                            </div>
                        }
                    }
                />
        </section>
    }
}

fn command_parameter(execution: &CommandExecution, name: &str) -> String {
    execution
        .parameters
        .iter()
        .find(|parameter| parameter.name == name)
        .map(|parameter| parameter.value.clone())
        .unwrap_or_default()
}

fn action_label(action: &str) -> &'static str {
    match action {
        "reboot" => "Reboot",
        "lock" => "Lock",
        "reinstall" => "OS reinstall",
        "rotate" => "Key rotation",
        "retire" => "Retirement",
        _ => "Action",
    }
}
