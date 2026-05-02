use leptos::html;
use leptos::prelude::*;
use wasm_bindgen::closure::Closure;
use wasm_bindgen::JsCast;
use web_sys::{window, HtmlElement, KeyboardEvent, ResizeObserver};

use super::{TabItem, TabLinePosition};
use crate::{ButtonMenu, ButtonMenuItem, ButtonVariant};

/// Horizontal tab trigger list with animated selection underline.
#[component]
pub fn TabList(
    /// Available tabs rendered in order.
    #[prop(into)]
    tabs: MaybeProp<Vec<TabItem>>,
    /// Currently selected tab value for controlled usage.
    #[prop(optional, into)]
    value: MaybeProp<Option<String>>,
    /// Optional id applied to the tablist root.
    #[prop(optional, into)]
    id: Option<String>,
    /// Additional CSS class names applied to the root element.
    #[prop(optional, into)]
    class: Option<String>,
    /// Places the indicator line above or below the tab labels.
    #[prop(optional)]
    line_position: TabLinePosition,
    /// Selection callback fired when the active tab changes.
    #[prop(optional)]
    on_value_change: Option<Callback<String>>,
) -> impl IntoView {
    let root_ref = NodeRef::<html::Div>::new();
    // These measurements let the component switch between inline tabs and an overflow menu
    // without hard-coding widths in Rust or CSS.
    let resize_observer_attached = RwSignal::new(false);
    let container_width = RwSignal::new(0.0_f64);
    let measured_tab_widths = RwSignal::new(Vec::<f64>::new());
    let overflow_trigger_width = RwSignal::new(0.0_f64);
    let tab_gap = RwSignal::new(0.0_f64);
    let indicator_style = RwSignal::new(String::from(
        "--birei-tab-list-indicator-x: 0px; --birei-tab-list-indicator-width: 0px;",
    ));
    let resize_observer = StoredValue::new_local(None::<ResizeObserver>);
    let resize_callback =
        StoredValue::new_local(None::<Closure<dyn FnMut(js_sys::Array, ResizeObserver)>>);
    let internal_value = RwSignal::new(
        value
            .get_untracked()
            .flatten()
            .or_else(|| first_enabled_value(&tabs.get_untracked())),
    );

    let current_value = move || value.get().flatten().or_else(|| internal_value.get());
    let selected_value = Memo::new(move |_| current_value());
    // Keep the selected index memoized because it is reused by indicator positioning and overflow
    // layout decisions.
    let selected_tab_index = Memo::new(move |_| {
        selected_index(
            &tabs.get().unwrap_or_default(),
            selected_value.get().as_deref(),
        )
    });
    // Overflow layout is derived from measured widths instead of hand-maintained breakpoints.
    let overflow_layout = Memo::new(move |_| {
        compute_overflow_layout(
            &tabs.get().unwrap_or_default(),
            &measured_tab_widths.get(),
            overflow_trigger_width.get(),
            tab_gap.get(),
            container_width.get(),
            selected_tab_index.get(),
        )
    });
    let class_name = move || {
        let mut classes = vec!["birei-tab-list"];

        classes.push(match line_position {
            TabLinePosition::Below => "birei-tab-list--line-below",
            TabLinePosition::Above => "birei-tab-list--line-above",
        });
        if let Some(class) = class.as_deref() {
            classes.push(class);
        }

        classes.join(" ")
    };

    // Re-measure the active trigger and move the indicator underline to match the selected tab.
    let sync_indicator = move || {
        let tabs = tabs
            .try_get_untracked()
            .flatten()
            .unwrap_or_default();
        let Some(selected_value) = selected_value.try_get_untracked().flatten() else {
            return;
        };
        let Some(selected_index) = selected_index(&tabs, Some(selected_value.as_str()))
        else {
            indicator_style.set(String::from(
                "--birei-tab-list-indicator-x: 0px; --birei-tab-list-indicator-width: 0px;",
            ));
            return;
        };

        let Some(root) = root_ref.try_get_untracked().flatten() else {
            return;
        };

        let Ok(Some(tab)) =
            root.query_selector(&format!("[data-birei-tab-index=\"{selected_index}\"]"))
        else {
            indicator_style.set(String::from(
                "--birei-tab-list-indicator-x: 0px; --birei-tab-list-indicator-width: 0px;",
            ));
            return;
        };

        let root_rect = root.get_bounding_client_rect();
        let tab = tab.unchecked_into::<HtmlElement>();
        let tab_rect = tab.get_bounding_client_rect();
        let offset = tab_rect.left() - root_rect.left();

        indicator_style.set(format!(
            "--birei-tab-list-indicator-x: {offset}px; --birei-tab-list-indicator-width: {}px;",
            tab_rect.width()
        ));
    };

    // Render a hidden measurement row so overflow decisions are based on the actual trigger styles.
    let measure_tab_widths = move || {
        let tabs = tabs.get().unwrap_or_default();
        let Some(root) = root_ref.try_get_untracked().flatten() else {
            return;
        };

        let widths = tabs
            .iter()
            .enumerate()
            .map(|(index, _)| {
                root.query_selector(&format!("[data-birei-tab-measure-index=\"{index}\"]"))
                    .ok()
                    .flatten()
                    .map(|tab| {
                        tab.unchecked_into::<HtmlElement>()
                            .get_bounding_client_rect()
                            .width()
                    })
                    .unwrap_or(0.0)
            })
            .collect::<Vec<_>>();
        measured_tab_widths.set(widths);

        let menu_width = root
            .query_selector("[data-birei-tab-measure-overflow]")
            .ok()
            .flatten()
            .map(|button| {
                button
                    .unchecked_into::<HtmlElement>()
                    .get_bounding_client_rect()
                    .width()
            })
            .unwrap_or(0.0);
        overflow_trigger_width.set(menu_width);

        let gap = window()
            .and_then(|window| window.get_computed_style(&root).ok().flatten())
            .and_then(|style| style.get_property_value("column-gap").ok())
            .and_then(|value| value.trim_end_matches("px").parse::<f64>().ok())
            .unwrap_or(0.0);
        tab_gap.set(gap);
    };

    // Defer indicator measurement to the next animation frame so DOM updates from selection and
    // overflow recalculation have settled before querying bounds.
    Effect::new({
        move |_| {
            tabs.get();
            selected_value.get();
            overflow_layout.get();

            let Some(window) = window() else {
                sync_indicator();
                return;
            };

            let callback = Closure::once_into_js(move || {
                sync_indicator();
            });
            let _ = window.request_animation_frame(callback.unchecked_ref());
        }
    });

    // Tab width measurement reruns whenever the available items change.
    Effect::new(move |_| {
        tabs.get();
        measure_tab_widths();
    });

    // Attach one resize observer to the root so container width and measured tab widths stay in
    // sync with responsive layout changes.
    Effect::new(move |_| {
        let Some(root) = root_ref.get() else {
            return;
        };
        if resize_observer_attached.get_untracked() {
            return;
        }

        container_width.set(f64::from(root.client_width()));

        let callback = Closure::wrap(Box::new(
            move |_entries: js_sys::Array, _observer: ResizeObserver| {
                if let Some(root) = root_ref.try_get_untracked().flatten() {
                    container_width.set(f64::from(root.client_width()));
                    measure_tab_widths();
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

    // Centralize tab selection so click, keyboard, and overflow-menu paths share the same guardrails.
    let select_tab = move |tab: &TabItem| {
        if tab.disabled {
            return;
        }

        internal_value.set(Some(tab.value.clone()));

        if let Some(on_value_change) = on_value_change.as_ref() {
            on_value_change.run(tab.value.clone());
        }
    };

    // Overflow menu items select by value, so resolve them back to the full tab definition here.
    let select_tab_by_value = move |next_value: &str| {
        let tabs = tabs.get().unwrap_or_default();
        let Some(tab) = tabs.iter().find(|tab| tab.value == next_value) else {
            return;
        };

        select_tab(tab);
    };

    // Roving focus stays limited to visible tabs; hidden entries are reachable through the overflow menu.
    let handle_keydown = move |event: KeyboardEvent, index: usize| {
        let key = event.key();
        if !matches!(key.as_str(), "ArrowLeft" | "ArrowRight" | "Home" | "End") {
            return;
        }

        event.prevent_default();

        let tabs = tabs.get().unwrap_or_default();
        let visible_indices = overflow_layout.get().visible_indices;
        let next_index = match key.as_str() {
            "ArrowLeft" => adjacent_enabled_visible_index(&tabs, &visible_indices, index, -1),
            "ArrowRight" => adjacent_enabled_visible_index(&tabs, &visible_indices, index, 1),
            "Home" => first_enabled_visible_index(&tabs, &visible_indices),
            "End" => last_enabled_visible_index(&tabs, &visible_indices),
            _ => None,
        };

        let Some(next_index) = next_index else {
            return;
        };

        let Some(next_tab) = tabs.get(next_index) else {
            return;
        };

        select_tab(next_tab);

        if let Some(root) = root_ref.get() {
            if let Ok(Some(button)) =
                root.query_selector(&format!("[data-birei-tab-index=\"{next_index}\"]"))
            {
                let _ = button.unchecked_into::<HtmlElement>().focus();
            }
        }
    };

    view! {
        <div
            id=id
            class=class_name
            style=move || indicator_style.get()
            node_ref=root_ref
            role="tablist"
        >
            <span class="birei-tab-list__indicator" aria-hidden="true"></span>
            <For
                each=move || overflow_layout.get().visible_indices
                key=move |index| {
                    tabs.get()
                        .unwrap_or_default()
                        .get(*index)
                        .map(|tab| format!("{index}:{}", tab.value))
                        .unwrap_or_else(|| index.to_string())
                }
                children=move |index| {
                    let Some(tab) = tabs.get().unwrap_or_default().get(index).cloned() else {
                        return ().into_any();
                    };
                    let tab_value = tab.value.clone();
                    let tab_label = tab.label.clone();
                    let is_disabled = tab.disabled;

                    view! {
                        <button
                            type="button"
                            class="birei-tab-list__tab"
                            data-birei-tab-index=index
                            role="tab"
                            disabled=is_disabled
                            aria-selected={
                                let tab_value = tab_value.clone();
                                move || {
                                    if selected_value.get().as_deref() == Some(tab_value.as_str()) {
                                        "true"
                                    } else {
                                        "false"
                                    }
                                }
                            }
                            tabindex={
                                let tab_value = tab_value.clone();
                                move || {
                                    if selected_value.get().as_deref() == Some(tab_value.as_str()) {
                                        "0"
                                    } else {
                                        "-1"
                                    }
                                }
                            }
                            on:click={
                                let tab = tab.clone();
                                move |_| select_tab(&tab)
                            }
                            on:keydown=move |event| handle_keydown(event, index)
                        >
                            {tab_label}
                        </button>
                    }
                    .into_any()
                }
            />
            {move || {
                let layout = overflow_layout.get();
                let tabs = tabs.get().unwrap_or_default();

                (!layout.overflow_indices.is_empty()).then(|| {
                    let items = layout
                        .overflow_indices
                        .iter()
                        .filter_map(|index| tabs.get(*index))
                        .map(|tab| ButtonMenuItem::new(tab.value.clone(), tab.label.clone()).disabled(tab.disabled))
                        .collect::<Vec<_>>();

                    view! {
                        <ButtonMenu
                            label="More"
                            items=items
                            class="birei-tab-list__overflow"
                            variant=ButtonVariant::Transparent
                            match_trigger_width=false
                            on_select=Callback::new(move |next: String| select_tab_by_value(&next))
                        />
                    }
                })
            }}
            <div class="birei-tab-list__measure" aria-hidden="true">
                <For
                    each=move || tabs.get().unwrap_or_default().into_iter().enumerate()
                    key=|(index, tab)| format!("measure-{index}:{}", tab.value)
                    children=move |(index, tab)| {
                        view! {
                            <button
                                type="button"
                                class="birei-tab-list__tab"
                                tabindex="-1"
                                data-birei-tab-measure-index=index
                            >
                                {tab.label}
                            </button>
                        }
                    }
                />
                <button
                    type="button"
                    class="birei-tab-list__tab birei-tab-list__tab--overflow"
                    tabindex="-1"
                    data-birei-tab-measure-overflow="true"
                >
                    <span>"More"</span>
                    <span class="birei-tab-list__overflow-caret" aria-hidden="true">"▾"</span>
                </button>
            </div>
        </div>
    }
}

#[derive(Clone, Default, PartialEq)]
struct OverflowLayout {
    visible_indices: Vec<usize>,
    overflow_indices: Vec<usize>,
}

/// Decide which tabs stay inline and which move into the overflow trigger for the current width.
fn compute_overflow_layout(
    tabs: &[TabItem],
    tab_widths: &[f64],
    overflow_width: f64,
    tab_gap: f64,
    container_width: f64,
    selected_index: Option<usize>,
) -> OverflowLayout {
    if tabs.is_empty() {
        return OverflowLayout::default();
    }

    if tab_widths.len() != tabs.len() || container_width <= 0.0 {
        return OverflowLayout {
            visible_indices: (0..tabs.len()).collect(),
            overflow_indices: Vec::new(),
        };
    }

    let total_width =
        tab_widths.iter().sum::<f64>() + tab_gap * tabs.len().saturating_sub(1) as f64;
    if total_width <= container_width {
        return OverflowLayout {
            visible_indices: (0..tabs.len()).collect(),
            overflow_indices: Vec::new(),
        };
    }

    let Some(selected_index) = selected_index else {
        return OverflowLayout {
            visible_indices: (0..tabs.len()).collect(),
            overflow_indices: Vec::new(),
        };
    };

    let available = (container_width - overflow_width - tab_gap).max(0.0);
    let mut visible_indices = Vec::new();
    let mut used_width = 0.0_f64;

    for (index, width) in tab_widths.iter().enumerate() {
        let next_width = if visible_indices.is_empty() {
            *width
        } else {
            used_width + tab_gap + width
        };
        if next_width > available {
            break;
        }

        visible_indices.push(index);
        used_width = next_width;
    }

    // Always keep the selected tab visible so the indicator and roving tabindex remain coherent.
    if !visible_indices.contains(&selected_index) {
        let selected_width = tab_widths[selected_index];
        while !visible_indices.is_empty() {
            let next_width = used_width + tab_gap + selected_width;
            if next_width <= available {
                break;
            }
            if let Some(removed) = visible_indices.pop() {
                used_width -= tab_widths[removed];
                if !visible_indices.is_empty() {
                    used_width -= tab_gap;
                }
            }
        }

        visible_indices.retain(|index| *index != selected_index);
        visible_indices.push(selected_index);
    }

    if visible_indices.is_empty() {
        visible_indices.push(selected_index);
    }

    let overflow_indices = (0..tabs.len())
        .filter(|index| !visible_indices.contains(index))
        .collect::<Vec<_>>();

    OverflowLayout {
        visible_indices,
        overflow_indices,
    }
}

/// Resolve the current selected value back to its enabled tab index.
fn selected_index(tabs: &[TabItem], value: Option<&str>) -> Option<usize> {
    value
        .and_then(|value| {
            tabs.iter()
                .position(|tab| !tab.disabled && tab.value == value)
        })
        .or_else(|| first_enabled_index(tabs))
}

/// Pick a sensible uncontrolled default by selecting the first enabled tab.
fn first_enabled_value(tabs: &Option<Vec<TabItem>>) -> Option<String> {
    tabs.as_ref()
        .and_then(|tabs| tabs.iter().find(|tab| !tab.disabled))
        .map(|tab| tab.value.clone())
}

/// Shared helpers for roving-focus navigation across enabled tabs only.
fn first_enabled_index(tabs: &[TabItem]) -> Option<usize> {
    tabs.iter().position(|tab| !tab.disabled)
}

fn first_enabled_visible_index(tabs: &[TabItem], visible_indices: &[usize]) -> Option<usize> {
    visible_indices
        .iter()
        .copied()
        .find(|index| tabs.get(*index).is_some_and(|tab| !tab.disabled))
}

fn last_enabled_visible_index(tabs: &[TabItem], visible_indices: &[usize]) -> Option<usize> {
    visible_indices
        .iter()
        .rev()
        .copied()
        .find(|index| tabs.get(*index).is_some_and(|tab| !tab.disabled))
}

/// Move left or right within the visible tab strip while skipping disabled entries.
fn adjacent_enabled_visible_index(
    tabs: &[TabItem],
    visible_indices: &[usize],
    start: usize,
    direction: i32,
) -> Option<usize> {
    if visible_indices.is_empty() {
        return None;
    }

    let current_position = visible_indices.iter().position(|index| *index == start)?;
    let len = visible_indices.len() as i32;
    let mut position = current_position as i32;

    for _ in 0..visible_indices.len() {
        position = (position + direction).rem_euclid(len);
        let candidate = visible_indices[position as usize];

        if tabs.get(candidate).is_some_and(|tab| !tab.disabled) {
            return Some(candidate);
        }
    }

    None
}
