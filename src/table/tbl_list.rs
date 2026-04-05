use leptos::ev;
use leptos::html;
use leptos::prelude::*;
use wasm_bindgen::closure::Closure;
use wasm_bindgen::JsCast;
use web_sys::{HtmlElement, KeyboardEvent, ResizeObserver};

use super::types::{TableColumn, TableDensity, TableRowMeta};
use super::view::{
    body_cell_class, grid_template, header_cell_class, root_class_name, row_class_name,
    row_meta_or_default,
};
use super::virtualize::{should_load_more, visible_range};

/// Fixed-height virtualized table with sticky header and endless scrolling.
#[component]
pub fn TableList<Row>(
    #[prop(into)] rows: MaybeProp<Vec<Row>>,
    #[prop(into)] columns: MaybeProp<Vec<TableColumn<Row>>>,
    row_key: Callback<Row, String>,
    #[prop(optional, into)] selected: MaybeProp<Option<String>>,
    #[prop(optional)] density: TableDensity,
    #[prop(optional, default = 6)] overscan: usize,
    #[prop(optional, default = 6)] load_more_threshold: usize,
    #[prop(optional, into)] has_more: MaybeProp<bool>,
    #[prop(optional, into)] is_loading: MaybeProp<bool>,
    #[prop(optional, default = true)] sticky_header: bool,
    #[prop(optional, default = true)] keyboard_navigation: bool,
    #[prop(optional, into)] class: Option<String>,
    #[prop(optional)] row_meta: Option<Callback<Row, TableRowMeta>>,
    #[prop(optional)] on_selected_change: Option<Callback<Option<String>>>,
    #[prop(optional)] on_row_activate: Option<Callback<String>>,
    #[prop(optional)] on_header_click: Option<Callback<String>>,
    #[prop(optional)] on_load_more: Option<Callback<()>>,
) -> impl IntoView
where
    Row: Clone + Send + Sync + 'static,
{
    let row_height = density.row_height();
    let rows_list = move || rows.get().unwrap_or_default();
    let columns_list = move || columns.get().unwrap_or_default();
    let selected_internal = RwSignal::new(selected.get_untracked().flatten());
    let active_index = RwSignal::new(None::<usize>);
    let keyboard_mode = RwSignal::new(false);
    let scroll_top = RwSignal::new(0.0_f64);
    let viewport_height = RwSignal::new(0.0_f64);
    let last_load_request_len = RwSignal::new(None::<usize>);
    let root_ref = NodeRef::<html::Div>::new();
    let resize_observer_attached = RwSignal::new(false);
    let resize_observer = StoredValue::new_local(None::<ResizeObserver>);
    let resize_callback =
        StoredValue::new_local(None::<Closure<dyn FnMut(js_sys::Array, ResizeObserver)>>);

    let selected_value = move || selected.get().flatten().or_else(|| selected_internal.get());
    let template = move || grid_template(&columns_list(), false);
    let class_name = move || root_class_name(density, keyboard_mode.get(), class.as_deref());

    let maybe_request_load_more = move |row_count: usize, visible_end: usize| {
        if on_load_more.is_none() {
            return;
        }
        if !should_load_more(
            row_count,
            visible_end,
            load_more_threshold,
            has_more.get().unwrap_or(false),
            is_loading.get().unwrap_or(false),
        ) {
            last_load_request_len.set(None);
            return;
        }
        if last_load_request_len.get() == Some(row_count) {
            return;
        }

        last_load_request_len.set(Some(row_count));
        if let Some(on_load_more) = on_load_more.as_ref() {
            on_load_more.run(());
        }
    };

    let ensure_row_visible = move |index: usize| {
        let Some(root) = root_ref.get() else {
            return;
        };

        let row_top = index as f64 * row_height;
        let row_bottom = row_top + row_height;
        let view_top = f64::from(root.scroll_top());
        let view_bottom = view_top + f64::from(root.client_height());

        if row_top < view_top {
            root.set_scroll_top(row_top as i32);
            scroll_top.set(row_top);
        } else if row_bottom > view_bottom {
            let next = (row_bottom - f64::from(root.client_height())).max(0.0);
            root.set_scroll_top(next as i32);
            scroll_top.set(next);
        }
    };

    let activate_row = move |index: usize| {
        active_index.set(Some(index));
        keyboard_mode.set(true);
        ensure_row_visible(index);
        maybe_request_load_more(rows_list().len(), index.saturating_add(1));
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
        let rows = rows_list();
        let (_start, end) = visible_range(
            rows.len(),
            row_height,
            overscan,
            scroll_top.get(),
            viewport_height.get(),
        );
        maybe_request_load_more(rows.len(), end);
    });

    Effect::new(move |_| {
        let Some(root) = root_ref.get_untracked() else {
            return;
        };
        if resize_observer_attached.get_untracked() {
            return;
        }

        viewport_height.set(f64::from(root.client_height()));

        let callback = Closure::wrap(Box::new(
            move |_entries: js_sys::Array, _observer: ResizeObserver| {
                if let Some(root) = root_ref.get_untracked() {
                    viewport_height.set(f64::from(root.client_height()));
                    scroll_top.set(f64::from(root.scroll_top()));
                }
            },
        ) as Box<dyn FnMut(js_sys::Array, ResizeObserver)>);

        if let Ok(observer) = ResizeObserver::new(callback.as_ref().unchecked_ref()) {
            observer.observe(root.as_ref());
            resize_observer_attached.set(true);
            resize_callback.update_value(|stored| *stored = Some(callback));
            resize_observer.update_value(|stored| *stored = Some(observer));
        }

        on_cleanup(move || {
            resize_observer.update_value(|stored| {
                if let Some(observer) = stored.take() {
                    observer.disconnect();
                }
            });
            resize_callback.update_value(|stored| {
                stored.take();
            });
            resize_observer_attached.set(false);
        });
    });

    view! {
        <div
            class=class_name
            node_ref=root_ref
            tabindex="0"
            role="grid"
            aria-activedescendant=move || active_index.get().map(|index| format!("birei-table-row-{index}")).unwrap_or_default()
            on:scroll=move |event: ev::Event| {
                if let Some(target) = event.current_target().and_then(|target| target.dyn_into::<HtmlElement>().ok()) {
                    scroll_top.set(f64::from(target.scroll_top()));
                    viewport_height.set(f64::from(target.client_height()));
                }
            }
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
                    columns_list().into_iter().map(|column| {
                        let clickable = on_header_click.is_some();
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
                                    if let Some(on_header_click) = on_header_click.as_ref() {
                                        on_header_click.run(column_key.clone());
                                    }
                                }
                            >
                                {header_view.map(|render| render.run(())).unwrap_or_else(|| view! { <span>{header.clone()}</span> }.into_any())}
                            </button>
                        }.into_any()
                    }).collect_view()
                }}
            </div>

            {move || {
                let rows = rows_list();
                let (start, end) = visible_range(
                    rows.len(),
                    row_height,
                    overscan,
                    scroll_top.get(),
                    viewport_height.get(),
                );
                let top_spacer = start as f64 * row_height;
                let bottom_spacer = rows.len().saturating_sub(end) as f64 * row_height;
                let current_selected = selected_value();
                let current_active = active_index.get();

                view! {
                    <div class="birei-table__spacer" style=format!("height: {top_spacer}px;")></div>
                    <div class="birei-table__rows">
                        {rows[start..end]
                            .iter()
                            .cloned()
                            .enumerate()
                            .map(|(offset, row)| {
                                let index = start + offset;
                                let meta = row_meta_or_default(
                                    row_meta.as_ref().map(|callback| callback.run(row.clone())),
                                    row_key.run(row.clone()),
                                );
                                let key = meta.key.clone();
                                view! {
                                    <div
                                        id=format!("birei-table-row-{index}")
                                        class=row_class_name(
                                            keyboard_mode.get() && current_active == Some(index),
                                            current_selected.as_deref() == Some(key.as_str()),
                                            meta.disabled,
                                            false,
                                            None,
                                        )
                                        style=format!("grid-template-columns: {};", template())
                                        role="row"
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
                                                on_row_activate.run(key.clone());
                                            }
                                        }
                                    >
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
                            .collect_view()}
                    </div>
                    <div class="birei-table__spacer" style=format!("height: {bottom_spacer}px;")></div>
                    {if is_loading.get().unwrap_or(false) {
                        view! { <div class="birei-table__status">"Loading more rows…"</div> }.into_any()
                    } else if rows.is_empty() {
                        view! { <div class="birei-table__status">"No rows yet"</div> }.into_any()
                    } else if !has_more.get().unwrap_or(false) {
                        view! { <div class="birei-table__status">"End of table"</div> }.into_any()
                    } else {
                        ().into_any()
                    }}
                }
            }}
        </div>
    }
}
