use leptos::html;
use leptos::prelude::*;
use wasm_bindgen::JsCast;
use web_sys::{HtmlElement, KeyboardEvent};

use super::{TabItem, TabLinePosition};

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
    let indicator_style = RwSignal::new(String::from(
        "--birei-tab-list-indicator-x: 0px; --birei-tab-list-indicator-width: 0px;",
    ));
    let internal_value = RwSignal::new(
        value
            .get()
            .flatten()
            .or_else(|| first_enabled_value(&tabs.get())),
    );

    let current_value = move || value.get().flatten().or_else(|| internal_value.get());
    let selected_value = Memo::new(move |_| current_value());
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

    let sync_indicator = move || {
        let tabs = tabs.get().unwrap_or_default();
        let Some(selected_index) = selected_index(&tabs, selected_value.get().as_deref()) else {
            indicator_style.set(String::from(
                "--birei-tab-list-indicator-x: 0px; --birei-tab-list-indicator-width: 0px;",
            ));
            return;
        };

        let Some(root) = root_ref.get() else {
            return;
        };

        let Ok(Some(tab)) =
            root.query_selector(&format!("[data-birei-tab-index=\"{selected_index}\"]"))
        else {
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

    Effect::new({
        move |_| {
            tabs.get();
            selected_value.get();
            sync_indicator();
        }
    });

    let select_tab = move |tab: &TabItem| {
        if tab.disabled {
            return;
        }

        internal_value.set(Some(tab.value.clone()));

        if let Some(on_value_change) = on_value_change.as_ref() {
            on_value_change.run(tab.value.clone());
        }
    };

    let handle_keydown = move |event: KeyboardEvent, index: usize| {
        let key = event.key();
        if !matches!(key.as_str(), "ArrowLeft" | "ArrowRight" | "Home" | "End") {
            return;
        }

        event.prevent_default();

        let tabs = tabs.get().unwrap_or_default();
        let next_index = match key.as_str() {
            "ArrowLeft" => adjacent_enabled_index(&tabs, index, -1),
            "ArrowRight" => adjacent_enabled_index(&tabs, index, 1),
            "Home" => first_enabled_index(&tabs),
            "End" => last_enabled_index(&tabs),
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
                each=move || tabs.get().unwrap_or_default().into_iter().enumerate()
                key=|(index, tab)| format!("{index}:{}", tab.value)
                children=move |(index, tab)| {
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
                }
            />
        </div>
    }
}

fn selected_index(tabs: &[TabItem], value: Option<&str>) -> Option<usize> {
    value
        .and_then(|value| {
            tabs.iter()
                .position(|tab| !tab.disabled && tab.value == value)
        })
        .or_else(|| first_enabled_index(tabs))
}

fn first_enabled_value(tabs: &Option<Vec<TabItem>>) -> Option<String> {
    tabs.as_ref()
        .and_then(|tabs| tabs.iter().find(|tab| !tab.disabled))
        .map(|tab| tab.value.clone())
}

fn first_enabled_index(tabs: &[TabItem]) -> Option<usize> {
    tabs.iter().position(|tab| !tab.disabled)
}

fn last_enabled_index(tabs: &[TabItem]) -> Option<usize> {
    tabs.iter().rposition(|tab| !tab.disabled)
}

fn adjacent_enabled_index(tabs: &[TabItem], start: usize, direction: i32) -> Option<usize> {
    if tabs.is_empty() {
        return None;
    }

    let len = tabs.len() as i32;
    let mut index = start as i32;

    for _ in 0..tabs.len() {
        index = (index + direction).rem_euclid(len);
        let candidate = index as usize;

        if tabs.get(candidate).is_some_and(|tab| !tab.disabled) {
            return Some(candidate);
        }
    }

    None
}
