use birei::{
    Card, Table, TableAlign, TableColumn, TableDensity, TableDropPosition, TableList, TableRowMeta,
    TableRowMove,
};
use js_sys::Promise;
use leptos::prelude::*;
use leptos::task::spawn_local;
use wasm_bindgen::prelude::Closure;
use wasm_bindgen::JsCast;
use wasm_bindgen_futures::JsFuture;
use crate::code_example::CodeExample;

const TABLE_BATCH_SIZE: usize = 120;
const TABLE_LIMIT: usize = 10_000;

#[derive(Clone)]
struct TeamRow {
    id: String,
    name: String,
    role: String,
    status: String,
    location: String,
}

#[component]
pub fn TablePage() -> impl IntoView {
    let reorderable_rows = RwSignal::new(vec![
        TeamRow {
            id: String::from("team-01"),
            name: String::from("Nadia Weber"),
            role: String::from("Design Lead"),
            status: String::from("Reviewing"),
            location: String::from("Zurich"),
        },
        TeamRow {
            id: String::from("team-02"),
            name: String::from("Ari Novak"),
            role: String::from("Staff Engineer"),
            status: String::from("Shipping"),
            location: String::from("Berlin"),
        },
        TeamRow {
            id: String::from("team-03"),
            name: String::from("Sofia Lange"),
            role: String::from("Product Manager"),
            status: String::from("Planning"),
            location: String::from("Vienna"),
        },
        TeamRow {
            id: String::from("team-04"),
            name: String::from("Milo Costa"),
            role: String::from("Operations"),
            status: String::from("Blocked"),
            location: String::from("Porto"),
        },
        TeamRow {
            id: String::from("team-05"),
            name: String::from("Elin Berg"),
            role: String::from("Finance"),
            status: String::from("Disabled"),
            location: String::from("Basel"),
        },
    ]);
    let reorder_selected = RwSignal::new(Some(String::from("team-02")));

    let virtual_rows = RwSignal::new(make_rows(0, 80));
    let virtual_selected = RwSignal::new(Some(String::from("row-00012")));
    let virtual_loading = RwSignal::new(false);
    let virtual_has_more = RwSignal::new(true);

    let reorder_columns = table_columns();
    let virtual_columns = table_columns();

    let handle_move = move |movement: TableRowMove| {
        reorderable_rows.update(|rows| {
            let Some(from_index) = rows.iter().position(|row| row.id == movement.from_key) else {
                return;
            };
            let Some(target_index) = rows.iter().position(|row| row.id == movement.to_key) else {
                return;
            };

            let row = rows.remove(from_index);
            let mut next_index = match movement.position {
                TableDropPosition::Before => target_index,
                TableDropPosition::After => target_index + 1,
            };
            if from_index < next_index {
                next_index = next_index.saturating_sub(1);
            }
            rows.insert(next_index.min(rows.len()), row);
        });
    };

    let load_more = move || {
        if virtual_loading.get_untracked() || !virtual_has_more.get_untracked() {
            return;
        }

        virtual_loading.set(true);
        spawn_local({
            let virtual_rows = virtual_rows;
            let virtual_loading = virtual_loading;
            let virtual_has_more = virtual_has_more;
            async move {
                sleep_ms(500).await;

                let current_len = virtual_rows.get_untracked().len();
                let next_len = (current_len + TABLE_BATCH_SIZE).min(TABLE_LIMIT);
                let next_rows = make_rows(current_len, next_len - current_len);
                virtual_rows.update(|rows| rows.extend(next_rows));
                virtual_has_more.set(next_len < TABLE_LIMIT);
                virtual_loading.set(false);
            }
        });
    };

    view! {
        <section class="page-header">
            <div class="page-header__eyebrow">"Component"</div>
            <h2>"Table"</h2>
            <p class="page-header__lede">
                "Shared table styling for reorderable data grids and fixed-height virtualized endless-scrolling tables."
            </p>
        </section>

        <section class="doc-grid">
            <Card class="doc-card">
                <span class="doc-card__kicker">"Reorderable"</span>
                <div class="doc-card__preview doc-card__preview--stack">
                    <div style="height: 320px;">
                        <Table
                            rows=reorderable_rows
                            columns=reorder_columns
                            row_key=Callback::new(|row: TeamRow| row.id)
                            selected=reorder_selected
                            density=TableDensity::Comfortable
                            on_selected_change=Callback::new(move |next| reorder_selected.set(next))
                            on_row_move=Callback::new(handle_move)
                            row_meta=Callback::new(|row: TeamRow| {
                                TableRowMeta::new(row.id)
                                    .draggable(row.status != "Blocked" && row.status != "Disabled")
                                    .disabled(row.status == "Disabled")
                            })
                        />
                    </div>
                    <p class="doc-card__copy">
                        "Selected row: "
                        <strong>{move || reorder_selected.get().unwrap_or_else(|| String::from("None"))}</strong>
                    </p>
                </div>
                <CodeExample code={r#"<Table
    rows=rows
    columns=columns
    row_key=Callback::new(|row: TeamRow| row.id)
    on_row_move=Callback::new(move |movement| reorder(movement))
/>"#}/>
            </Card>

            <Card class="doc-card">
                <span class="doc-card__kicker">"Virtualized"</span>
                <div class="doc-card__preview doc-card__preview--stack">
                    <div style="height: 360px;">
                        <TableList
                            rows=virtual_rows
                            columns=virtual_columns
                            row_key=Callback::new(|row: TeamRow| row.id)
                            selected=virtual_selected
                            density=TableDensity::Compact
                            has_more=virtual_has_more
                            is_loading=virtual_loading
                            on_selected_change=Callback::new(move |next| virtual_selected.set(next))
                            on_load_more=Callback::new(move |_| load_more())
                        />
                    </div>
                    <p class="doc-card__copy">
                        "Built for large datasets with fixed row heights and incremental loading."
                    </p>
                </div>
                <CodeExample code={r#"<TableList
    rows=rows
    columns=columns
    row_key=Callback::new(|row: TeamRow| row.id)
    has_more=has_more
    is_loading=is_loading
    on_load_more=Callback::new(move |_| load_more())
/>"#}/>
            </Card>
        </section>
    }
}

fn table_columns() -> Vec<TableColumn<TeamRow>> {
    vec![
        TableColumn::new(
            "name",
            "Name",
            Callback::new(|row: TeamRow| view! { <strong>{row.name}</strong> }.into_any()),
        )
        .min_width("14rem"),
        TableColumn::new(
            "role",
            "Role",
            Callback::new(|row: TeamRow| view! { <span>{row.role}</span> }.into_any()),
        )
        .min_width("12rem"),
        TableColumn::new(
            "status",
            "Status",
            Callback::new(|row: TeamRow| {
                view! { <span style="font-weight: 600;">{row.status}</span> }.into_any()
            }),
        )
        .width("9rem")
        .align(TableAlign::Center),
        TableColumn::new(
            "location",
            "Location",
            Callback::new(|row: TeamRow| view! { <span>{row.location}</span> }.into_any()),
        )
        .width("8rem")
        .align(TableAlign::End),
    ]
}

fn make_rows(start: usize, count: usize) -> Vec<TeamRow> {
    let roles = ["Engineer", "Designer", "Product", "Support"];
    let statuses = ["Queued", "Review", "Active", "Paused"];
    let locations = ["Zurich", "Berlin", "Vienna", "Porto"];

    (start..start + count)
        .map(|index| TeamRow {
            id: format!("row-{index:05}"),
            name: format!("Workspace Row {index:05}"),
            role: roles[index % roles.len()].to_string(),
            status: statuses[index % statuses.len()].to_string(),
            location: locations[index % locations.len()].to_string(),
        })
        .collect()
}

async fn sleep_ms(delay_ms: i32) {
    let promise = Promise::new(&mut |resolve, _reject| {
        let Some(window) = web_sys::window() else {
            let _ = resolve.call0(&wasm_bindgen::JsValue::NULL);
            return;
        };

        let callback = Closure::once_into_js(move || {
            let _ = resolve.call0(&wasm_bindgen::JsValue::NULL);
        });

        let _ = window.set_timeout_with_callback_and_timeout_and_arguments_0(
            callback.as_ref().unchecked_ref(),
            delay_ms,
        );
    });

    let _ = JsFuture::from(promise).await;
}
