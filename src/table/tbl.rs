use leptos::ev;
use leptos::html;
use leptos::prelude::*;
use wasm_bindgen::JsCast;
use web_sys::{HtmlElement, KeyboardEvent};

use super::drag::{build_row_move, drag_target_from_y, DragState, DragTarget};
use super::types::{TableColumn, TableDensity, TableRowMeta, TableRowMove};
use super::view::{
    body_cell_class, drag_handle, drag_target_for_row, grid_template, header_cell_class,
    root_class_name, row_class_name, row_meta_or_default,
};

/// Table with sticky header, custom cell renderers, keyboard navigation, and optional row reordering.
#[component]
pub fn Table<Row>(
    #[prop(into)] rows: MaybeProp<Vec<Row>>,
    #[prop(into)] columns: MaybeProp<Vec<TableColumn<Row>>>,
    row_key: Callback<Row, String>,
    #[prop(optional, into)] selected: MaybeProp<Option<String>>,
    #[prop(optional)] density: TableDensity,
    #[prop(optional, default = true)] sticky_header: bool,
    #[prop(optional, default = true)] keyboard_navigation: bool,
    #[prop(optional, into)] class: Option<String>,
    #[prop(optional)] row_meta: Option<Callback<Row, TableRowMeta>>,
    #[prop(optional)] on_selected_change: Option<Callback<Option<String>>>,
    #[prop(optional)] on_row_activate: Option<Callback<String>>,
    #[prop(optional)] on_header_click: Option<Callback<String>>,
    #[prop(optional)] on_row_move: Option<Callback<TableRowMove>>,
) -> impl IntoView
where
    Row: Clone + Send + Sync + 'static,
{
    let rows_list = move || rows.get().unwrap_or_default();
    let columns_list = move || columns.get().unwrap_or_default();
    let selected_internal = RwSignal::new(selected.get_untracked().flatten());
    let active_index = RwSignal::new(None::<usize>);
    let keyboard_mode = RwSignal::new(false);
    let drag_state = RwSignal::new(None::<DragState>);
    let drag_target = RwSignal::new(None::<DragTarget>);
    let root_ref = NodeRef::<html::Div>::new();

    let selected_value = move || selected.get().flatten().or_else(|| selected_internal.get());
    let reorderable = move || on_row_move.is_some();
    let template = move || grid_template(&columns_list(), reorderable());
    let class_name = move || root_class_name(density, keyboard_mode.get(), class.as_deref());

    let ensure_row_visible = move |index: usize| {
        let Some(root) = root_ref.get() else {
            return;
        };

        let Some(row) = root
            .query_selector(&format!("[data-birei-table-row-index=\"{index}\"]"))
            .ok()
            .flatten()
            .and_then(|row| row.dyn_into::<HtmlElement>().ok())
        else {
            return;
        };

        let root_rect = root.get_bounding_client_rect();
        let row_rect = row.get_bounding_client_rect();

        if row_rect.top() < root_rect.top() {
            root.set_scroll_top(root.scroll_top() - (root_rect.top() - row_rect.top()) as i32 - 8);
        } else if row_rect.bottom() > root_rect.bottom() {
            root.set_scroll_top(
                root.scroll_top() + (row_rect.bottom() - root_rect.bottom()) as i32 + 8,
            );
        }
    };

    let activate_row = move |index: usize| {
        active_index.set(Some(index));
        keyboard_mode.set(true);
        ensure_row_visible(index);
    };

    let select_row = move |index: usize| {
        let rows = rows_list();
        let Some(row) = rows.get(index).cloned() else {
            return;
        };
        let key = row_key.run(row);
        let next = if selected_value().as_deref() == Some(key.as_str()) {
            None
        } else {
            Some(key)
        };

        selected_internal.set(next.clone());
        active_index.set(Some(index));
        if let Some(on_selected_change) = on_selected_change.as_ref() {
            on_selected_change.run(next);
        }
    };

    Effect::new(move |_| {
        let rows = rows_list();
        if rows.is_empty() {
            if active_index.get_untracked().is_some() {
                active_index.set(None);
            }
            return;
        }

        let next_active = active_index
            .get()
            .filter(|index| *index < rows.len())
            .or_else(|| {
                selected_value().and_then(|selected| {
                    rows.iter()
                        .position(|row| row_key.run(row.clone()) == selected)
                })
            })
            .or(Some(0));
        if active_index.get_untracked() != next_active {
            active_index.set(next_active);
        }
    });

    Effect::new(move |_| {
        let Some(state) = drag_state.get() else {
            return;
        };

        let move_handle = window_event_listener_untyped("mousemove", {
            move |event| {
                let Ok(event) = event.dyn_into::<web_sys::MouseEvent>() else {
                    return;
                };

                let Some(root) = root_ref.get_untracked() else {
                    return;
                };

                let Ok(rows) = root.query_selector_all("[data-birei-table-row-key]") else {
                    return;
                };

                let client_y = f64::from(event.client_y());
                let mut next_target = None::<DragTarget>;

                for index in 0..rows.length() {
                    let Some(node) = rows.item(index) else {
                        continue;
                    };
                    let Ok(row) = node.dyn_into::<HtmlElement>() else {
                        continue;
                    };
                    let key = row
                        .get_attribute("data-birei-table-row-key")
                        .unwrap_or_default();
                    if key == state.from_key {
                        continue;
                    }

                    let rect = row.get_bounding_client_rect();
                    if client_y >= rect.top() && client_y <= rect.bottom() {
                        next_target = Some(drag_target_from_y(
                            client_y,
                            rect.top(),
                            rect.height(),
                            &key,
                        ));
                        break;
                    }
                }

                drag_target.set(next_target);
            }
        });
        let up_handle = window_event_listener_untyped("mouseup", move |_| {
            if let (Some(state), Some(target)) = (drag_state.get(), drag_target.get()) {
                if let Some(on_row_move) = on_row_move.as_ref() {
                    if let Some(next_move) = build_row_move(&state, &target) {
                        on_row_move.run(next_move);
                    }
                }
            }

            drag_state.set(None);
            drag_target.set(None);
        });

        on_cleanup(move || {
            move_handle.remove();
            up_handle.remove();
        });
    });

    view! {
        <div
            class=class_name
            node_ref=root_ref
            tabindex="0"
            role="grid"
            aria-activedescendant=move || active_index.get().map(|index| format!("birei-table-row-{index}")).unwrap_or_default()
            on:focus=move |_| {
                if keyboard_navigation && !rows_list().is_empty() {
                    keyboard_mode.set(true);
                    let rows = rows_list();
                    let next_active = selected_value()
                        .and_then(|selected| {
                            rows.iter()
                                .position(|row| row_key.run(row.clone()) == selected)
                        })
                        .or(Some(0));
                    if active_index.get_untracked() != next_active {
                        active_index.set(next_active);
                    }
                }
            }
            on:blur=move |_| keyboard_mode.set(false)
            on:keydown=move |event: KeyboardEvent| {
                if !keyboard_navigation {
                    return;
                }

                let rows = rows_list();
                if rows.is_empty() {
                    return;
                }
                let current = active_index.get().unwrap_or(0);
                match event.key().as_str() {
                    "ArrowDown" => {
                        event.prevent_default();
                        activate_row((current + 1).min(rows.len() - 1));
                    }
                    "ArrowUp" => {
                        event.prevent_default();
                        activate_row(current.saturating_sub(1));
                    }
                    "Home" => {
                        event.prevent_default();
                        activate_row(0);
                    }
                    "End" => {
                        event.prevent_default();
                        activate_row(rows.len() - 1);
                    }
                    "Enter" | " " => {
                        event.prevent_default();
                        select_row(current);
                        if let Some(row) = rows.get(current).cloned() {
                            if let Some(on_row_activate) = on_row_activate.as_ref() {
                                on_row_activate.run(row_key.run(row));
                            }
                        }
                    }
                    _ => {}
                }
            }
        >
            <div
                class=move || {
                    let mut classes = String::from("birei-table__header");
                    if sticky_header {
                        classes.push_str(" birei-table__header--sticky");
                    }
                    classes
                }
                style=move || format!("grid-template-columns: {};", template())
                role="row"
            >
                {move || {
                    let columns = columns_list();
                    let header_click = on_header_click;
                    let mut header_cells = Vec::new();
                    if reorderable() {
                        header_cells.push(view! {
                            <div class="birei-table__cell birei-table__cell--header birei-table__cell--handle" role="columnheader"></div>
                        }.into_any());
                    }

                    header_cells.extend(columns.into_iter().map(|column| {
                        let clickable = header_click.is_some();
                        let column_key = column.key.clone();
                        let header = column.header.clone();
                        let header_view = column.header_view;
                        view! {
                            <button
                                type="button"
                                class=header_cell_class(&column, clickable)
                                role="columnheader"
                                disabled=!clickable
                                on:click=move |_| {
                                    if let Some(on_header_click) = header_click.as_ref() {
                                        on_header_click.run(column_key.clone());
                                    }
                                }
                            >
                                {header_view.map(|render| render.run(())).unwrap_or_else(|| view! { <span>{header.clone()}</span> }.into_any())}
                            </button>
                        }.into_any()
                    }));
                    header_cells
                }}
            </div>

            <div class="birei-table__body">
                {move || {
                    rows_list()
                        .into_iter()
                        .enumerate()
                        .map(|(index, row)| {
                            let meta = row_meta_or_default(
                                row_meta.as_ref().map(|callback| callback.run(row.clone())),
                                row_key.run(row.clone()),
                            );
                            let key = meta.key.clone();
                            let selected_key = key.clone();
                            let drag_target_key = key.clone();
                            let click_key = key.clone();
                            let dragstart_key = key.clone();
                            let is_active =
                                move || keyboard_mode.get() && active_index.get() == Some(index);
                            let is_selected =
                                move || selected_value().as_deref() == Some(selected_key.as_str());
                            let row_drag_target = move || {
                                drag_target_for_row(
                                    drag_state.get(),
                                    drag_target.get(),
                                    &drag_target_key,
                                )
                            };

                            view! {
                                <div
                                    id=format!("birei-table-row-{index}")
                                    class=move || {
                                        let (is_dragging, drop_position) = row_drag_target();
                                        row_class_name(is_active(), is_selected(), meta.disabled, is_dragging, drop_position)
                                    }
                                    style=move || format!("grid-template-columns: {};", template())
                                    role="row"
                                    data-birei-table-row-index=index
                                    data-birei-table-row-key=key.clone()
                                    on:mousemove=move |_| {
                                        if keyboard_navigation {
                                            keyboard_mode.set(false);
                                            active_index.set(Some(index));
                                        }
                                    }
                                    on:click=move |_| {
                                        if meta.disabled {
                                            return;
                                        }
                                        select_row(index);
                                        if let Some(on_row_activate) = on_row_activate.as_ref() {
                                            on_row_activate.run(click_key.clone());
                                        }
                                    }
                                >
                                    {if reorderable() {
                                        let handle_mouse_down =
                                            Callback::new(move |event: ev::MouseEvent| {
                                                if !meta.draggable
                                                    || meta.disabled
                                                    || on_row_move.is_none()
                                                {
                                                    return;
                                                }

                                                event.prevent_default();
                                                keyboard_mode.set(false);
                                                active_index.set(Some(index));
                                                drag_state.set(Some(DragState {
                                                    from_key: dragstart_key.clone(),
                                                }));
                                                drag_target.set(None);
                                            });
                                        view! {
                                            <div class="birei-table__cell birei-table__cell--handle" role="gridcell">
                                                {if meta.draggable && !meta.disabled {
                                                    drag_handle(handle_mouse_down)
                                                } else {
                                                    ().into_any()
                                                }}
                                            </div>
                                        }
                                            .into_any()
                                    } else {
                                        ().into_any()
                                    }}
                                    {columns_list()
                                        .into_iter()
                                        .map(|column| {
                                            let cell = column.cell;
                                            view! {
                                                <div class=body_cell_class(&column) role="gridcell">
                                                    {cell.run(row.clone())}
                                                </div>
                                            }.into_any()
                                        })
                                        .collect_view()}
                                </div>
                            }.into_any()
                        })
                        .collect_view()
                }}
            </div>
        </div>
    }
}
