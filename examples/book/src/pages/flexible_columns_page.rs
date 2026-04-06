use birei::{
    Button, ButtonVariant, Card, FlexibleColumn, FlexibleColumns, Input, List, ListDensity,
    ListEntry,
};
use leptos::prelude::*;
use crate::code_example::CodeExample;

#[derive(Clone, Copy, PartialEq, Eq)]
struct DemoEntry {
    id: &'static str,
    title: &'static str,
    subtitle: &'static str,
    owner: &'static str,
    status: &'static str,
    note: &'static str,
}

const DEMO_ENTRIES: [DemoEntry; 3] = [
    DemoEntry {
        id: "roadmap",
        title: "Roadmap review",
        subtitle: "Q2 planning package",
        owner: "Akari",
        status: "Draft",
        note: "Outline the roadmap decisions and capture any blockers before review.",
    },
    DemoEntry {
        id: "design",
        title: "Design sync",
        subtitle: "Shared decisions and next steps",
        owner: "Mina",
        status: "In review",
        note: "The middle panel shows the selected item details and can reveal deeper metadata.",
    },
    DemoEntry {
        id: "handoff",
        title: "Support handoff",
        subtitle: "Escalation notes and owners",
        owner: "Noah",
        status: "Ready",
        note:
            "Keep the handoff summary crisp so on-call engineers can act without context switching.",
    },
];

#[component]
pub fn FlexibleColumnsPage() -> impl IntoView {
    let focused = RwSignal::new(FlexibleColumn::Start);
    let current_ratios = RwSignal::new([100.0_f32, 0.0_f32, 0.0_f32]);
    let selected_entry = RwSignal::new(None::<DemoEntry>);
    let selected_value = RwSignal::new(None::<String>);
    let detail_note = RwSignal::new(String::new());
    let show_right = RwSignal::new(false);
    let available_columns = Signal::derive(move || {
        [
            true,
            selected_entry.get().is_some(),
            selected_entry.get().is_some() && show_right.get(),
        ]
    });

    let list_entries = DEMO_ENTRIES
        .into_iter()
        .map(|entry| {
            ListEntry::new(entry.id, entry.title)
                .description(entry.subtitle)
                .meta(entry.status)
        })
        .collect::<Vec<_>>();

    let open_middle = move |entry: DemoEntry| {
        selected_entry.set(Some(entry));
        selected_value.set(Some(entry.id.to_string()));
        detail_note.set(entry.note.to_string());
        show_right.set(false);
        current_ratios.set([20.0, 80.0, 0.0]);
        focused.set(FlexibleColumn::Middle);
    };

    let close_middle = move || {
        selected_entry.set(None);
        selected_value.set(None);
        detail_note.set(String::new());
        show_right.set(false);
        current_ratios.set([100.0, 0.0, 0.0]);
        focused.set(FlexibleColumn::Start);
    };

    let open_right = move || {
        if selected_entry.get_untracked().is_none() {
            return;
        }
        show_right.set(true);
        current_ratios.set([20.0, 60.0, 20.0]);
        focused.set(FlexibleColumn::End);
    };

    let close_right = move || {
        show_right.set(false);
        current_ratios.set([20.0, 80.0, 0.0]);
        focused.set(FlexibleColumn::Middle);
    };

    view! {
        <section class="page-header">
            <div class="page-header__eyebrow">"Component"</div>
            <h2>"Flexible Columns"</h2>
            <p class="page-header__lede">
                "A low-level three-panel layout for list-detail-detail flows with responsive collapse, draggable dividers, and focus-aware emphasis."
            </p>
        </section>

        <section class="doc-grid">
            <Card header="Interactive layout" class="doc-card">
                <span class="doc-card__kicker">"Responsive"</span>
                <div class="doc-card__preview doc-card__preview--stack">
                    <p class="doc-card__copy">
                        "The example behaves like a simple list-detail-detail flow: pick an entry on the left, edit it in the middle, then open the supporting detail info on the right."
                    </p>
                    <div class="book-flex-demo">
                        <FlexibleColumns
                            focused=focused
                            initial_ratios=current_ratios
                            available_columns=available_columns
                            on_focus_change=Callback::new(move |next| focused.set(next))
                            on_ratios_change=Callback::new(move |next| current_ratios.set(next))
                            start=move || {
                                view! {
                                    <div class="book-flex-pane book-flex-pane--flush-list">
                                        <div class="book-flex-pane__stack book-flex-pane__stack--inset">
                                            <div class="book-flex-pane__eyebrow">"Start"</div>
                                            <h3>"Entries"</h3>
                                            <p>"Select an entry to show its details in the middle column."</p>
                                        </div>
                                        <div class="book-list-demo book-list-demo--detailed">
                                            <List
                                                items=list_entries.clone()
                                                density=ListDensity::Detailed
                                                selected=selected_value
                                                on_selected_change=Callback::new(move |next: Option<String>| {
                                                    match next.as_deref() {
                                                        Some(value) => {
                                                            if let Some(entry) = DEMO_ENTRIES
                                                                .into_iter()
                                                                .find(|entry| entry.id == value)
                                                            {
                                                                open_middle(entry);
                                                            }
                                                        }
                                                        None => close_middle(),
                                                    }
                                                })
                                            />
                                        </div>
                                    </div>
                                }
                            }
                            middle=move || {
                                view! {
                                    <div class="book-flex-pane">
                                        {move || {
                                            selected_entry.get().map(|entry| {
                                                view! {
                                                    <div class="book-flex-pane__stack">
                                                        <div class="book-flex-pane__eyebrow">"Middle"</div>
                                                        <h3>{entry.title}</h3>
                                                        <p>{entry.subtitle}</p>
                                                        <Input
                                                            id="book-flex-title"
                                                            value=entry.title
                                                        />
                                                        <textarea
                                                            class="field__input book-flex-note"
                                                            rows="6"
                                                            prop:value=move || detail_note.get()
                                                            on:input=move |event| {
                                                                detail_note.set(event_target_value(&event));
                                                            }
                                                        ></textarea>
                                                        <div class="page-header__actions">
                                                            <Button>"Save"</Button>
                                                            <Button
                                                                variant=ButtonVariant::Secondary
                                                                on_click=Callback::new(move |_| close_middle())
                                                            >
                                                                "Close details"
                                                            </Button>
                                                            <Button
                                                                variant=ButtonVariant::Secondary
                                                                on_click=Callback::new(move |_| open_right())
                                                            >
                                                                "Show detail info"
                                                            </Button>
                                                        </div>
                                                    </div>
                                                }
                                                .into_any()
                                            }).unwrap_or_else(|| ().into_any())
                                        }}
                                    </div>
                                }
                            }
                            end=move || {
                                view! {
                                    <div class="book-flex-pane">
                                        {move || {
                                            if !show_right.get() {
                                                return ().into_any();
                                            }

                                            selected_entry
                                                .get()
                                                .map(|entry| {
                                                    view! {
                                                        <div class="book-flex-pane__stack">
                                                            <div class="book-flex-pane__eyebrow">"End"</div>
                                                            <h3>"Detail info"</h3>
                                                            <p class="book-flex-copy">
                                                                {format!(
                                                                    "{} is currently owned by {} and marked as {}.",
                                                                    entry.title, entry.owner, entry.status
                                                                )}
                                                            </p>
                                                            <p class="book-flex-copy">
                                                                "This side panel is intended for supporting information only, not another nested list."
                                                            </p>
                                                            <p class="book-flex-copy">
                                                                {format!("Current note: {}", detail_note.get())}
                                                            </p>
                                                            <div class="page-header__actions">
                                                                <Button
                                                                    variant=ButtonVariant::Secondary
                                                                    on_click=Callback::new(move |_| close_right())
                                                                >
                                                                    "Close detail info"
                                                                </Button>
                                                            </div>
                                                        </div>
                                                    }
                                                    .into_any()
                                                })
                                                .unwrap_or_else(|| ().into_any())
                                        }}
                                    </div>
                                }
                            }
                        />
                    </div>
                    <p class="doc-card__copy">
                        "Focused column: "
                        <strong>{move || format!("{:?}", focused.get())}</strong>
                        " | Ratios: "
                        <strong>{move || {
                            let ratios = current_ratios.get();
                            format!("{:.0} / {:.0} / {:.0}", ratios[0], ratios[1], ratios[2])
                        }}</strong>
                    </p>
                </div>
                <CodeExample code={r#"<FlexibleColumns
    focused=move || focused.get()
    initial_ratios=move || ratios.get()
    available_columns=move || [true, has_middle.get(), has_right.get()]
    on_focus_change=Callback::new(move |next| focused.set(next))
    on_ratios_change=Callback::new(move |next| ratios.set(next))
    start=move || view! { ... }
    middle=move || view! { ... }
    end=move || view! { ... }
/>"#}/>
            </Card>
        </section>
    }
}
