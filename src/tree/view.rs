use crate::ArcOneCallback;
use leptos::ev;
use leptos::html;
use leptos::prelude::*;
use wasm_bindgen::closure::Closure;
use wasm_bindgen::JsCast;
use web_sys::{Element, HtmlElement, KeyboardEvent, ResizeObserver};

use super::model::{is_selected, toggle_expanded, visible_nodes, visible_range, ROW_HEIGHT};
use super::row::TreeRow;
use super::TreeNode;

/// Full-height virtualized tree with optional controlled selection and expansion.
#[component]
pub fn TreeView(
    /// Nested items with values unique across the whole tree.
    #[prop(into)]
    nodes: MaybeProp<Vec<TreeNode>>,
    /// Expanded branch values. Supply `on_expanded_change` when controlling this prop.
    #[prop(optional, into)]
    expanded: MaybeProp<Vec<String>>,
    /// Selected item values for controlled usage.
    #[prop(optional, into)]
    selected: MaybeProp<Vec<String>>,
    /// Number of extra rows mounted above and below the viewport.
    #[prop(optional, default = 6)]
    overscan: usize,
    /// Number of rows before the visible end that triggers `on_load_more`.
    #[prop(optional, default = 6)]
    load_more_threshold: usize,
    /// Whether more top-level nodes can be loaded.
    #[prop(optional, into)]
    has_more: MaybeProp<bool>,
    /// Whether a top-level load request is in progress.
    #[prop(optional, into)]
    is_loading: MaybeProp<bool>,
    /// Additional class names applied to the scroll container.
    #[prop(optional, into)]
    class: Option<String>,
    /// Accessible name for the tree.
    #[prop(optional, into)]
    label: Option<String>,
    /// Fired with the complete set of expanded branch values after a toggle.
    #[prop(optional, into)]
    on_expanded_change: Option<ArcOneCallback<Vec<String>>>,
    /// Fired when selection changes. `None` clears the selection.
    #[prop(optional, into)]
    on_selected_change: Option<ArcOneCallback<Option<String>>>,
    /// Fired when the scroll window approaches the end of the loaded nodes.
    #[prop(optional, into)]
    on_load_more: Option<ArcOneCallback<()>>,
) -> impl IntoView {
    let nodes_list = move || nodes.try_get().flatten().unwrap_or_default();
    let expanded_internal =
        RwSignal::new(expanded.try_get_untracked().flatten().unwrap_or_default());
    let selected_internal =
        RwSignal::new(selected.try_get_untracked().flatten().unwrap_or_default());
    let active_value = RwSignal::new(None::<String>);
    let keyboard_mode = RwSignal::new(false);
    let scroll_top = RwSignal::new(0.0_f64);
    let viewport_height = RwSignal::new(0.0_f64);
    let last_load_request_len = RwSignal::new(None::<usize>);
    let root_ref = NodeRef::<html::Div>::new();
    let resize_observer_attached = RwSignal::new(false);
    let resize_observer = StoredValue::new_local(None::<ResizeObserver>);
    let resize_callback =
        StoredValue::new_local(None::<Closure<dyn FnMut(js_sys::Array, ResizeObserver)>>);

    let expanded_value = move || {
        expanded
            .try_get()
            .flatten()
            .unwrap_or_else(|| expanded_internal.try_get().unwrap_or_default())
    };
    let selected_values = move || {
        selected
            .try_get()
            .flatten()
            .unwrap_or_else(|| selected_internal.try_get().unwrap_or_default())
    };
    let visible_rows = Memo::new(move |_| {
        let roots = nodes_list();
        visible_nodes(&roots, &expanded_value())
    });
    let root_count = Memo::new(move |_| {
        nodes
            .try_get()
            .flatten()
            .map(|nodes| nodes.len())
            .unwrap_or_default()
    });

    // Keep the fallback state current if a caller-owned signal is disposed first.
    Effect::new(move |_| {
        if let Some(value) = expanded.try_get().flatten() {
            let _ = expanded_internal.try_set(value);
        }
        if let Some(value) = selected.try_get().flatten() {
            let _ = selected_internal.try_set(value);
        }
    });

    let ensure_visible = move |index: usize| {
        let Some(root) = root_ref.try_get_untracked().flatten() else {
            return;
        };
        let top = index as f64 * ROW_HEIGHT;
        let bottom = top + ROW_HEIGHT;
        let scroll = f64::from(root.scroll_top());
        let height = f64::from(root.client_height());
        let next = if top < scroll {
            Some(top)
        } else if bottom > scroll + height {
            Some((bottom - height).max(0.0))
        } else {
            None
        };
        if let Some(next) = next {
            root.set_scroll_top(next as i32);
            let _ = scroll_top.try_set(next);
        }
    };

    let activate = move |index: usize, value: String| {
        let _ = active_value.try_set(Some(value));
        let _ = keyboard_mode.try_set(true);
        ensure_visible(index);
    };

    let toggle = move |value: String| {
        let next = toggle_expanded(&expanded_value(), &value);
        let _ = expanded_internal.try_set(next.clone());
        if let Some(callback) = on_expanded_change.as_ref() {
            callback.run(next);
        }
    };

    let select = move |value: String| {
        let next = if is_selected(&selected_values(), &value) {
            None
        } else {
            Some(value.clone())
        };
        let _ = active_value.try_set(Some(value));
        let _ = selected_internal.try_set(next.iter().cloned().collect());
        if let Some(callback) = on_selected_change.as_ref() {
            callback.run(next);
        }
    };

    // Expansion and data changes can remove the active row from the visible set.
    Effect::new(move |_| {
        let _ = visible_rows.try_with(|visible| {
            let current = active_value.try_get().flatten();
            if current
                .as_ref()
                .is_some_and(|value| visible.iter().any(|item| item.value == *value))
            {
                return;
            }
            let next = selected_values()
                .into_iter()
                .find(|value| visible.iter().any(|item| item.value == *value))
                .or_else(|| visible.first().map(|item| item.value.clone()));
            if current != next {
                let _ = active_value.try_set(next);
            }
        });
    });

    // Request the next root page once per loaded root count near the virtual tail.
    Effect::new(move |_| {
        let count = root_count.try_get().unwrap_or_default();
        if count == 0 {
            let _ = last_load_request_len.try_set(None);
            return;
        }
        if !has_more.try_get().flatten().unwrap_or(false)
            || is_loading.try_get().flatten().unwrap_or(false)
        {
            return;
        }
        let Some(callback) = on_load_more.as_ref() else {
            return;
        };
        let _ = visible_rows.try_with(|visible| {
            let (_, end) = visible_range(
                visible.len(),
                overscan,
                scroll_top.try_get().unwrap_or_default(),
                viewport_height.try_get().unwrap_or_default(),
            );
            if end >= visible.len().saturating_sub(load_more_threshold)
                && last_load_request_len.try_get().flatten() != Some(count)
            {
                let _ = last_load_request_len.try_set(Some(count));
                callback.run(());
            }
        });
    });

    // Keep the window in range when closing a branch shortens the scrollable content.
    Effect::new(move |_| {
        let _ = visible_rows.try_with(|visible| {
            let height = viewport_height.try_get().unwrap_or_default();
            let maximum = (visible.len() as f64 * ROW_HEIGHT - height).max(0.0);
            if scroll_top.try_get().unwrap_or_default() > maximum {
                if let Some(root) = root_ref.try_get_untracked().flatten() {
                    root.set_scroll_top(maximum as i32);
                }
                let _ = scroll_top.try_set(maximum);
            }
        });
    });

    // Observe the parent-sized viewport so the virtual window follows layout changes.
    Effect::new(move |_| {
        let Some(root) = root_ref.try_get_untracked().flatten() else {
            return;
        };
        if resize_observer_attached
            .try_get_untracked()
            .unwrap_or_default()
        {
            return;
        }
        let _ = viewport_height.try_set(f64::from(root.client_height()));
        let callback = Closure::wrap(Box::new(
            move |_entries: js_sys::Array, _observer: ResizeObserver| {
                if let Some(root) = root_ref.try_get_untracked().flatten() {
                    let _ = viewport_height.try_set(f64::from(root.client_height()));
                    let _ = scroll_top.try_set(f64::from(root.scroll_top()));
                }
            },
        ) as Box<dyn FnMut(js_sys::Array, ResizeObserver)>);

        if let Ok(observer) = ResizeObserver::new(callback.as_ref().unchecked_ref()) {
            observer.observe(root.as_ref());
            let _ = resize_observer_attached.try_set(true);
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
            let _ = resize_observer_attached.try_set(false);
        });
    });

    view! {
        <div
            class=move || {
                let mut classes = String::from("birei-tree");
                if keyboard_mode.try_get().unwrap_or_default() {
                    classes.push_str(" birei-tree--keyboard");
                }
                if let Some(class) = class.as_deref() {
                    classes.push(' ');
                    classes.push_str(class);
                }
                classes
            }
            node_ref=root_ref
            tabindex="0"
            role="tree"
            aria-label=label
            aria-activedescendant=move || {
                let active = active_value.try_get().flatten();
                visible_rows.try_with(|visible| {
                    visible.iter()
                        .position(|item| Some(item.value.as_str()) == active.as_deref())
                        .map(|index| format!("birei-tree-row-{index}"))
                })
                    .flatten()
                    .unwrap_or_default()
            }
            on:scroll=move |event: ev::Event| {
                if let Some(root) = event.current_target().and_then(|target| target.dyn_into::<HtmlElement>().ok()) {
                    let _ = scroll_top.try_set(f64::from(root.scroll_top()));
                    let _ = viewport_height.try_set(f64::from(root.client_height()));
                }
            }
            on:focus=move |_| {
                let _ = visible_rows.try_with(|visible| {
                    if !visible.is_empty() {
                        let _ = keyboard_mode.try_set(true);
                        if active_value.try_get_untracked().flatten().is_none() {
                            let next = selected_values()
                                .into_iter()
                                .find(|value| visible.iter().any(|item| item.value == *value))
                                .or_else(|| visible.first().map(|item| item.value.clone()));
                            let _ = active_value.try_set(next);
                        }
                    }
                });
            }
            on:blur=move |_| {
                let _ = keyboard_mode.try_set(false);
            }
            on:keydown=move |event: KeyboardEvent| {
                if event_targets_control(&event) {
                    return;
                }
                let _ = visible_rows.try_with(|visible| {
                    if visible.is_empty() {
                        return;
                    }
                    let current = active_value.try_get().flatten()
                        .and_then(|value| visible.iter().position(|item| item.value == value))
                        .unwrap_or(0);
                    let item = &visible[current];
                    match event.key().as_str() {
                        "ArrowDown" => {
                            event.prevent_default();
                            let next = (current + 1).min(visible.len() - 1);
                            activate(next, visible[next].value.clone());
                        }
                        "ArrowUp" => {
                            event.prevent_default();
                            let next = current.saturating_sub(1);
                            activate(next, visible[next].value.clone());
                        }
                        "Home" => {
                            event.prevent_default();
                            activate(0, visible[0].value.clone());
                        }
                        "End" => {
                            event.prevent_default();
                            let next = visible.len() - 1;
                            activate(next, visible[next].value.clone());
                        }
                        "ArrowRight" if item.has_children => {
                            event.prevent_default();
                            if item.expanded {
                                if visible.get(current + 1).is_some_and(|next| next.parent == Some(current)) {
                                    activate(current + 1, visible[current + 1].value.clone());
                                }
                            } else {
                                toggle(item.value.clone());
                            }
                        }
                        "ArrowLeft" if item.has_children && item.expanded => {
                            event.prevent_default();
                            toggle(item.value.clone());
                        }
                        "ArrowLeft" if item.parent.is_some() => {
                            event.prevent_default();
                            if let Some(parent) = item.parent {
                                activate(parent, visible[parent].value.clone());
                            }
                        }
                        "Enter" | " " => {
                            event.prevent_default();
                            select(item.value.clone());
                        }
                        _ => {}
                    }
                });
            }
        >
            {move || {
                visible_rows.try_with(|visible| {
                    let (start, end) = visible_range(
                        visible.len(),
                        overscan,
                        scroll_top.try_get().unwrap_or_default(),
                        viewport_height.try_get().unwrap_or_default(),
                    );
                    let active = active_value.try_get().flatten();
                    let selected = selected_values();
                    let keyboard = keyboard_mode.try_get().unwrap_or_default();
                    let top = start as f64 * ROW_HEIGHT;
                    let bottom = visible.len().saturating_sub(end) as f64 * ROW_HEIGHT;
                    view! {
                        <div class="birei-tree__spacer" style=format!("height: {top}px;")></div>
                            <div class="birei-tree__rows">
                            {visible[start..end].iter().enumerate().map(|(offset, item)| {
                                let value = item.value.clone();
                                let select_value = value.clone();
                                let toggle_value = value.clone();
                                let hover_value = value.clone();
                                let row_index = start + offset;
                                view! {
                                    <TreeRow
                                        index=row_index
                                        title=item.title.clone()
                                        icon=item.icon.clone()
                                        meta=item.meta.clone()
                                        highlight=item.highlight.clone()
                                        depth=item.depth
                                        position=item.position
                                        sibling_count=item.sibling_count
                                        has_children=item.has_children
                                        expanded=item.expanded
                                        selected=is_selected(&selected, &value)
                                        active=keyboard && active.as_deref() == Some(value.as_str())
                                        on_toggle=ArcOneCallback::new(move |()| {
                                            let _ = active_value.try_set(Some(toggle_value.clone()));
                                            toggle(toggle_value.clone());
                                            if let Some(root) = root_ref.try_get_untracked().flatten() {
                                                let _ = root.focus();
                                            }
                                        })
                                        on_select=ArcOneCallback::new(move |()| {
                                            select(select_value.clone());
                                            if let Some(root) = root_ref.try_get_untracked().flatten() {
                                                let _ = root.focus();
                                            }
                                            let _ = keyboard_mode.try_set(false);
                                        })
                                        on_hover=ArcOneCallback::new(move |()| {
                                            let _ = active_value.try_set(Some(hover_value.clone()));
                                            let _ = keyboard_mode.try_set(false);
                                        })
                                    />
                                }
                            }).collect_view()}
                        </div>
                        <div class="birei-tree__spacer" style=format!("height: {bottom}px;")></div>
                    }.into_any()
                }).unwrap_or_else(|| ().into_any())
            }}
            {move || is_loading.try_get().flatten().unwrap_or(false).then(|| {
                view! { <div class="birei-tree__status">"Loading more records…"</div> }
            })}
        </div>
    }
}

/// Leaves keyboard events inside embedded controls to their own handlers.
fn event_targets_control(event: &KeyboardEvent) -> bool {
    event
        .target()
        .and_then(|target| target.dyn_into::<Element>().ok())
        .and_then(|target| {
            target
                .closest("button, input, textarea, select, [contenteditable=true]")
                .ok()
                .flatten()
        })
        .is_some()
}
