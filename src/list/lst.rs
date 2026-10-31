use crate::ArcOneCallback;
use leptos::ev;
use leptos::html;
use leptos::prelude::*;
use wasm_bindgen::JsCast;
use web_sys::{HtmlElement, KeyboardEvent};

use super::{ListDensity, ListEntry};
use crate::common::use_resize_observer;
use crate::{Icon, Size};

/// Fixed-height virtualized list with single selection and incremental loading.
#[component]
pub fn List(
    /// Rows rendered by the list.
    #[prop(into)]
    items: MaybeProp<Vec<ListEntry>>,
    /// Selected row value for controlled usage.
    #[prop(optional, into)]
    selected: MaybeProp<Option<String>>,
    /// Density also controls the fixed row height used by virtualization.
    #[prop(optional)]
    density: ListDensity,
    /// Number of rows rendered above and below the viewport.
    #[prop(optional, default = 6)]
    overscan: usize,
    /// Number of rows before the end that triggers `on_load_more`.
    #[prop(optional, default = 6)]
    load_more_threshold: usize,
    /// Whether more rows can be loaded.
    #[prop(optional, into)]
    has_more: MaybeProp<bool>,
    /// Whether a load request is currently in progress.
    #[prop(optional, into)]
    is_loading: MaybeProp<bool>,
    /// Optional terminal status text shown when no more rows can be loaded.
    #[prop(optional, into)]
    end_status: Option<String>,
    /// Additional class names applied to the scroll container.
    #[prop(optional, into)]
    class: Option<String>,
    /// Fired when the selected row changes. Passing `None` clears selection.
    #[prop(optional, into)]
    on_selected_change: Option<ArcOneCallback<Option<String>>>,
    /// Fired when the user activates a row with click or keyboard.
    #[prop(optional, into)]
    on_row_click: Option<ArcOneCallback<String>>,
    /// Fired when the list nears the end and needs more rows.
    #[prop(optional, into)]
    on_load_more: Option<ArcOneCallback<()>>,
) -> impl IntoView {
    // Virtualization relies on a fixed row height derived from density so the
    // component can compute visible windows from scroll position alone.
    let row_height = density.row_height();
    let items_list = move || items.try_get().flatten().unwrap_or_default();
    let selected_internal = RwSignal::new(selected.try_get_untracked().flatten().flatten());
    let active_index = RwSignal::new(None::<usize>);
    let keyboard_navigation = RwSignal::new(false);
    let pointer_focus_pending = RwSignal::new(false);
    let scroll_top = RwSignal::new(0.0_f64);
    let viewport_height = RwSignal::new(0.0_f64);
    let last_load_request_len = RwSignal::new(None::<usize>);
    let root_ref = NodeRef::<html::Div>::new();
    let previous_item_values = StoredValue::new_local(None::<Vec<String>>);

    // Controlled selection falls back to the internal selection state when the
    // consumer does not drive the component externally.
    let selected_value = move || {
        selected
            .try_get()
            .flatten()
            .flatten()
            .or_else(|| selected_internal.try_get().flatten())
    };

    // Root classes reflect density and whether keyboard navigation styling
    // should be visible.
    let class_name = move || {
        let mut classes = vec!["birei-list", density.class_name()];
        if keyboard_navigation.try_get().unwrap_or_default() {
            classes.push("birei-list--keyboard");
        }
        if let Some(class) = class.as_deref() {
            classes.push(class);
        }
        classes.join(" ")
    };

    // Computes the overscanned item window that should be rendered for the
    // current scroll offset and viewport height.
    let items_len = move || {
        items
            .try_with(|items| items.as_ref().map(Vec::len).unwrap_or_default())
            .unwrap_or_default()
    };
    let visible_range = move || {
        let items_len = items_len();
        if items_len == 0 {
            return (0_usize, 0_usize);
        }

        let scroll = scroll_top.try_get().unwrap_or_default();
        let height = viewport_height.try_get().unwrap_or_default();
        let start = ((scroll / row_height).floor() as isize - overscan as isize).max(0) as usize;
        let end = (((scroll + height) / row_height).ceil() as usize + overscan).min(items_len);
        (start, end)
    };

    // Load-more requests are throttled by item count so the same window does
    // not repeatedly trigger duplicate fetches.
    let maybe_request_load_more = move |items_len: usize, visible_end: usize| {
        if !has_more.try_get().flatten().unwrap_or(false)
            || is_loading.try_get().flatten().unwrap_or(false)
            || on_load_more.is_none()
            || items_len == 0
        {
            return;
        }

        let trigger_index = items_len.saturating_sub(load_more_threshold);
        if visible_end < trigger_index {
            let _ = last_load_request_len.try_set(None);
            return;
        }

        if last_load_request_len.try_get() == Some(Some(items_len)) {
            return;
        }

        let _ = last_load_request_len.try_set(Some(items_len));
        if let Some(on_load_more) = on_load_more.as_ref() {
            on_load_more.run(());
        }
    };

    // Keeps the keyboard-active row inside the scroll viewport with a small
    // top/bottom padding.
    let ensure_row_visible = move |index: usize| {
        let Some(root) = root_ref.try_get().flatten() else {
            return;
        };

        let row_top = index as f64 * row_height;
        let row_bottom = row_top + row_height;
        let view_top = f64::from(root.scroll_top());
        let client_height = f64::from(root.client_height());
        let view_bottom = view_top + client_height;
        let padding = 8.0_f64;
        let mut next_scroll_top = None;

        if row_top - padding < view_top {
            next_scroll_top = Some((row_top - padding).max(0.0));
        } else if row_bottom + padding > view_bottom {
            next_scroll_top = Some((row_bottom + padding - client_height).max(0.0));
        }

        if let Some(next_scroll_top) = next_scroll_top {
            root.set_scroll_top(next_scroll_top as i32);
            let _ = scroll_top.try_set(next_scroll_top);
            let _ = viewport_height.try_set(client_height);
        }
    };

    // Keyboard navigation updates the active row without changing selection.
    let activate_row = move |index: usize| {
        let items_len = untrack(|| items_list().len());
        let _ = active_index.try_set(Some(index));
        let _ = keyboard_navigation.try_set(true);
        untrack(|| ensure_row_visible(index));
        untrack(|| maybe_request_load_more(items_len, index.saturating_add(1)));
    };

    // Row activation toggles selection and fans out to both controlled and
    // click callbacks.
    let commit_selection = move |index: usize| {
        let items = untrack(items_list);
        let Some(item) = items.get(index) else {
            return;
        };

        let next_selected = if untrack(selected_value).as_deref() == Some(item.value.as_str()) {
            None
        } else {
            Some(item.value.clone())
        };

        let _ = selected_internal.try_set(next_selected.clone());
        let _ = active_index.try_set(Some(index));

        if let Some(on_selected_change) = on_selected_change.as_ref() {
            on_selected_change.run(next_selected.clone());
        }
        if let Some(on_row_click) = on_row_click.as_ref() {
            on_row_click.run(item.value.clone());
        }
    };

    let set_active_index = move |next: Option<usize>| {
        if active_index.try_get_untracked() != Some(next) {
            let _ = active_index.try_set(next);
        }
    };

    let reset_scroll_top = move || {
        if let Some(root) = root_ref.try_get_untracked().flatten() {
            if root.scroll_top() != 0 {
                root.set_scroll_top(0);
            }
            let _ = viewport_height.try_set(f64::from(root.client_height()));
        }
        if scroll_top.try_get_untracked().unwrap_or_default() != 0.0 {
            let _ = scroll_top.try_set(0.0);
        }
    };

    // Replaced or filtered item sets should start at the top. Appended item
    // sets keep their current scroll position for incremental loading.
    Effect::new(move |_| {
        let next_values = items_list()
            .into_iter()
            .map(|item| item.value)
            .collect::<Vec<_>>();

        previous_item_values.update_value(|previous| {
            if let Some(previous_values) = previous.as_ref() {
                if !next_values.starts_with(previous_values) {
                    reset_scroll_top();
                    let _ = last_load_request_len.try_set(None);
                }
            }
            *previous = Some(next_values);
        });
    });

    // Active index is kept valid as the item set changes and seeded from the
    // selected value when possible.
    Effect::new(move |_| {
        let items = items_list();
        let active = active_index.try_get_untracked().flatten();
        let Some(active) = active else {
            let next_active = selected_value()
                .as_ref()
                .and_then(|selected| items.iter().position(|item| item.value == *selected));
            set_active_index(next_active.or_else(|| (!items.is_empty()).then_some(0)));
            return;
        };

        if items.is_empty() {
            set_active_index(None);
        } else if active >= items.len() {
            set_active_index(Some(items.len() - 1));
        }
    });

    // Whenever the rendered window changes, reevaluate whether more data
    // should be requested.
    Effect::new(move |_| {
        let items = items_list();
        let (_start, end) = visible_range();
        maybe_request_load_more(items.len(), end);
    });

    // Keyboard mode owns active-row visibility, so moving the active index via
    // keys automatically scrolls it into view.
    Effect::new(move |_| {
        let items = items_list();
        let active = active_index.try_get().flatten();
        let keyboard_mode = keyboard_navigation.try_get().unwrap_or_default();

        if keyboard_mode && !items.is_empty() {
            if let Some(active) = active {
                if active < items.len() {
                    ensure_row_visible(active);
                }
            }
        }
    });

    // A resize observer keeps virtualization math aligned with the actual
    // scroll viewport height.
    use_resize_observer(root_ref, move |_| {
        if let Some(root) = root_ref.try_get_untracked().flatten() {
            let _ = viewport_height.try_set(f64::from(root.client_height()));
            let _ = scroll_top.try_set(f64::from(root.scroll_top()));
        }
    });

    let spacer_style = move |top: bool| {
        let (start, end) = visible_range();
        let len = items_len();
        let rows = if top { start } else { len.saturating_sub(end) };
        format!("height: {}px;", rows as f64 * row_height)
    };

    view! {
        <div
            class=class_name
            node_ref=root_ref
            tabindex="0"
            on:scroll=move |event: ev::Event| {
                if let Some(target) = event
                    .current_target()
                    .and_then(|target| target.dyn_into::<HtmlElement>().ok())
                {
                    let _ = scroll_top.try_set(f64::from(target.scroll_top()));
                    let _ = viewport_height.try_set(f64::from(target.client_height()));
                }
            }
            on:pointerdown=move |_| {
                // Pointer focus happens before the row click. Do not let the
                // focus handler replace the virtualized window in between.
                let _ = pointer_focus_pending.try_set(true);
            }
            on:focus=move |_| {
                if pointer_focus_pending.try_get_untracked().unwrap_or_default() {
                    let _ = pointer_focus_pending.try_set(false);
                    return;
                }

                let items = untrack(items_list);
                if items.is_empty() {
                    return;
                }

                let _ = keyboard_navigation.try_set(true);
                let next_active = untrack(selected_value)
                    .as_ref()
                    .and_then(|selected| items.iter().position(|item| item.value == *selected))
                    .or(Some(0));
                if active_index.try_get_untracked() != Some(next_active) {
                    let _ = active_index.try_set(next_active);
                }
            }
            on:blur=move |_| {
                let _ = keyboard_navigation.try_set(false);
                let _ = pointer_focus_pending.try_set(false);
            }
            on:keydown=move |event: KeyboardEvent| {
                let items = untrack(items_list);
                if items.is_empty() {
                    return;
                }

                let current = active_index.try_get_untracked().flatten().unwrap_or(0);
                match event.key().as_str() {
                    "ArrowDown" => {
                        event.prevent_default();
                        let next = (current + 1).min(items.len() - 1);
                        activate_row(next);
                    }
                    "ArrowUp" => {
                        event.prevent_default();
                        let next = current.saturating_sub(1);
                        activate_row(next);
                    }
                    "Home" => {
                        event.prevent_default();
                        activate_row(0);
                    }
                    "End" => {
                        event.prevent_default();
                        activate_row(items.len() - 1);
                    }
                    "Enter" | " " => {
                        event.prevent_default();
                        let _ = keyboard_navigation.try_set(true);
                        commit_selection(current);
                    }
                    _ => {}
                }
            }
            role="listbox"
            aria-activedescendant=move || {
                active_index
                    .try_get()
                    .flatten()
                    .map(|index| format!("birei-list-row-{index}"))
                    .unwrap_or_default()
            }
        >
            <div class="birei-list__spacer" style=move || spacer_style(true)></div>
            <div class="birei-list__rows">
                // Rows are keyed by position and entry: hovering, selecting and scrolling within
                // the window keep their DOM and only update classes.
                <For
                    each=move || {
                        let (start, end) = visible_range();
                        items
                            .try_with(|items| {
                                items.as_deref().unwrap_or_default()[start.min(end)..end]
                                    .iter()
                                    .cloned()
                                    .enumerate()
                                    .map(|(offset, item)| (start + offset, item))
                                    .collect::<Vec<_>>()
                            })
                            .unwrap_or_default()
                    }
                    key=|entry| entry.clone()
                    children=move |(absolute_index, item)| {
                        let item_value = item.value.clone();
                        let active = Signal::derive(move || {
                            keyboard_navigation.try_get().unwrap_or_default()
                                && active_index.try_get().flatten() == Some(absolute_index)
                        });
                        let selected = Signal::derive(move || {
                            selected_value().as_deref() == Some(item_value.as_str())
                        });
                        view! {
                            <ListRow
                                index=absolute_index
                                entry=item
                                density=density
                                active=active
                                selected=selected
                                on_hover=Callback::new(move |_| {
                                    let _ = keyboard_navigation.try_set(false);
                                    if active_index.try_get_untracked().flatten() != Some(absolute_index) {
                                        let _ = active_index.try_set(Some(absolute_index));
                                    }
                                })
                                on_select=Callback::new(move |_| commit_selection(absolute_index))
                            />
                        }
                    }
                />
            </div>
            <div class="birei-list__spacer" style=move || spacer_style(false)></div>
            {move || {
                let items_len = items_len();
                if is_loading.try_get().flatten().unwrap_or(false) {
                    view! { <div class="birei-list__status">"Loading more entries…"</div> }.into_any()
                } else if !has_more.try_get().flatten().unwrap_or(false) && items_len > 0 {
                    end_status
                        .as_ref()
                        .map(|text| view! { <div class="birei-list__status">{text.clone()}</div> }.into_any())
                        .unwrap_or_else(|| ().into_any())
                } else if items_len == 0 {
                    view! { <div class="birei-list__status">"No entries yet"</div> }.into_any()
                } else {
                    ().into_any()
                }
            }}
        </div>
    }
}

/// One visible row inside the virtualized window.
#[component]
fn ListRow(
    index: usize,
    entry: ListEntry,
    density: ListDensity,
    active: Signal<bool>,
    selected: Signal<bool>,
    #[prop(into)] on_hover: ArcOneCallback<()>,
    #[prop(into)] on_select: ArcOneCallback<()>,
) -> impl IntoView {
    view! {
        <div
            id=format!("birei-list-row-{index}")
            role="option"
            aria-selected=move || if selected.try_get().unwrap_or_default() { "true" } else { "false" }
            class=move || {
                list_row_class_name(
                    active.try_get().unwrap_or_default(),
                    selected.try_get().unwrap_or_default(),
                )
            }
            style=list_row_style(entry.highlight.as_deref())
            on:mousemove=move |_| on_hover.run(())
            on:click=move |_| on_select.run(())
        >
            <span class="birei-list__icon" aria-hidden="true">
                {entry.icon.map(|icon| view! { <Icon name=icon size=Size::Small/> })}
            </span>
            <span class="birei-list__body">
                <span class="birei-list__title">{entry.title.clone()}</span>
                {if density == ListDensity::Detailed {
                    entry.description
                        .as_ref()
                        .map(|description| {
                            view! { <span class="birei-list__description">{description.clone()}</span> }
                        })
                        .into_any()
                } else {
                    ().into_any()
                }}
            </span>
            <span class="birei-list__meta">
                {entry.meta.unwrap_or_default()}
            </span>
        </div>
    }
}

/// Builds the row classes for active and selected visual states.
fn list_row_class_name(active: bool, selected: bool) -> String {
    let mut classes = String::from("birei-list__row");
    if active {
        classes.push_str(" birei-list__row--active");
    }
    if selected {
        classes.push_str(" birei-list__row--selected");
    }
    classes
}

fn list_row_style(highlight: Option<&str>) -> String {
    highlight
        .filter(|highlight| !highlight.trim().is_empty())
        // highlight colors are data; dark mode tones them down so text stays readable
        .map(|highlight| {
            format!(
                "--birei-list-row-highlight: color-mix(in srgb, {highlight} var(--birei-highlight-strength), transparent);"
            )
        })
        .unwrap_or_default()
}
