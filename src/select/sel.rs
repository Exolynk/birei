use leptos::ev;
use leptos::html;
use leptos::prelude::*;
use std::sync::atomic::{AtomicUsize, Ordering};
use wasm_bindgen::JsCast;
use web_sys::{HtmlElement, KeyboardEvent};

use super::SelectOption;
use crate::{Icon, Label, Size, Tag};

static NEXT_SELECT_ID: AtomicUsize = AtomicUsize::new(1);

#[derive(Clone)]
struct SelectedTagData {
    value: String,
    label: String,
}

/// Searchable select with single and multi-select modes.
#[component]
pub fn Select(
    /// Available options rendered into the dropdown list.
    #[prop(into)]
    options: MaybeProp<Vec<SelectOption>>,
    /// Current value for single-select mode.
    #[prop(optional, into)]
    value: MaybeProp<Option<String>>,
    /// Current values for multi-select mode.
    #[prop(optional, into)]
    values: MaybeProp<Vec<String>>,
    /// Placeholder text shown when nothing is selected.
    #[prop(optional, into)]
    placeholder: MaybeProp<String>,
    /// Optional field name for form submission.
    #[prop(optional, into)]
    name: Option<String>,
    /// Optional input id.
    #[prop(optional, into)]
    id: Option<String>,
    /// Optional label shown above the field.
    #[prop(optional, into)]
    label: Option<String>,
    /// Shared sizing token aligned with buttons and inputs.
    #[prop(optional)]
    size: Size,
    /// Enables multi-select behavior.
    #[prop(optional)]
    multiple: bool,
    /// Allows returning to an empty selection.
    #[prop(optional)]
    nullable: bool,
    /// Disables the select and prevents user interaction.
    #[prop(optional)]
    disabled: bool,
    /// Marks the select as invalid for styling and accessibility.
    #[prop(optional)]
    invalid: bool,
    /// Marks the field as required.
    #[prop(optional)]
    required: bool,
    /// Additional CSS class names applied to the root element.
    #[prop(optional, into)]
    class: Option<String>,
    /// Input event handler for the search field.
    #[prop(optional)]
    on_input: Option<Callback<ev::Event>>,
    /// Change event handler for the search field.
    #[prop(optional)]
    on_change: Option<Callback<ev::Event>>,
    /// Selection callback for single-select mode.
    #[prop(optional)]
    on_value_change: Option<Callback<Option<String>>>,
    /// Selection callback for multi-select mode.
    #[prop(optional)]
    on_values_change: Option<Callback<Vec<String>>>,
) -> impl IntoView {
    let class_name = build_select_class_name(size, multiple, disabled, invalid, nullable, class);
    let select_id = id.unwrap_or_else(|| {
        format!(
            "birei-select-{}",
            NEXT_SELECT_ID.fetch_add(1, Ordering::Relaxed)
        )
    });
    let line_style = RwSignal::new(String::from("--birei-select-line-origin: 50%;"));
    let is_open = RwSignal::new(false);
    let query = RwSignal::new(String::new());
    let internal_value = RwSignal::new(value.get().flatten());
    let internal_values = RwSignal::new(values.get().unwrap_or_default());
    let active_index = RwSignal::new(None::<usize>);
    let input_ref = NodeRef::<html::Input>::new();
    let menu_ref = NodeRef::<html::Div>::new();
    let scroll_request = RwSignal::new(0_u64);

    // Resolve the currently visible selection from controlled props first, then local interaction state.
    let selected_value = move || value.get().flatten().or_else(|| internal_value.get());
    let selected_values = move || values.get().unwrap_or_else(|| internal_values.get());
    let placeholder = move || placeholder.get().unwrap_or_default();
    let options_list = move || options.get().unwrap_or_default();

    let selected_label = move || find_option_label(&options_list(), selected_value().as_deref());

    let has_selection = move || {
        if multiple {
            !selected_values().is_empty()
        } else {
            selected_value().is_some()
        }
    };

    let display_value = move || {
        if multiple {
            query.get()
        } else {
            let current_query = query.get();
            if current_query.is_empty() && !is_open.get() {
                selected_label()
            } else {
                current_query
            }
        }
    };

    let filtered_options = move || filter_options_by_query(&options_list(), &query.get());

    // Keep the active option aligned with the filtered list and skip disabled entries.
    let sync_active_index = move || {
        let filtered = filtered_options();
        let current_active = active_index.get();
        let next_active = current_active
            .filter(|index| filtered.get(*index).is_some_and(|option| !option.disabled))
            .or_else(|| first_enabled_index(&filtered));
        active_index.set(next_active);
    };

    let focus_input = move || {
        if let Some(input) = input_ref.get() {
            let _ = input.focus();
        }
    };

    // These commit helpers centralize state updates so pointer and keyboard selection stay consistent.
    let commit_single = move |next: Option<String>| {
        internal_value.set(next.clone());
        query.set(String::new());
        is_open.set(false);
        active_index.set(None);

        if let Some(on_value_change) = on_value_change.as_ref() {
            on_value_change.run(next);
        }
    };

    let commit_multiple = move |next: Vec<String>| {
        internal_values.set(next.clone());
        query.set(String::new());
        is_open.set(true);
        sync_active_index();

        if let Some(on_values_change) = on_values_change.as_ref() {
            on_values_change.run(next);
        }
    };

    // Clearing routes through the same commit logic used by option selection.
    let clear_selection = move |event: ev::MouseEvent| {
        event.prevent_default();
        event.stop_propagation();

        if !nullable || disabled {
            return;
        }

        if multiple {
            commit_multiple(Vec::new());
        } else {
            commit_single(None);
        }

        focus_input();
    };

    // Capture the pointer position to keep the underline animation origin aligned with the interaction point.
    let handle_pointer_down = move |event: ev::PointerEvent| {
        if let Some(target) = event
            .current_target()
            .and_then(|target| target.dyn_into::<HtmlElement>().ok())
        {
            let rect = target.get_bounding_client_rect();
            let x = f64::from(event.client_x()) - rect.left();
            line_style.set(format!("--birei-select-line-origin: {x}px;"));
        }
    };

    // Opening the menu also refreshes the active option and requests popup-local scrolling if needed.
    let open_menu = move || {
        if disabled {
            return;
        }

        is_open.set(true);
        sync_active_index();
        scroll_request.update(|value| *value += 1);
    };

    // Arrow-key navigation wraps through enabled options only.
    let move_active = move |direction: i32| {
        let filtered = filtered_options();
        if filtered.is_empty() {
            active_index.set(None);
            return;
        }

        let next_index = next_enabled_index(&filtered, active_index.get(), direction)
            .or_else(|| first_enabled_index(&filtered));
        active_index.set(next_index);
        scroll_request.update(|value| *value += 1);
    };

    // Enter selects the active option for both single and multi-select variants.
    let select_active_option = move || {
        let filtered = filtered_options();
        let Some(index) = active_index.get() else {
            return;
        };
        let Some(option) = filtered.get(index).cloned() else {
            return;
        };
        if option.disabled {
            return;
        }

        if multiple {
            let mut next = selected_values();
            if let Some(existing_index) = next.iter().position(|value| value == &option.value) {
                next.remove(existing_index);
            } else {
                next.push(option.value);
            }
            commit_multiple(next);
        } else {
            commit_single(Some(option.value));
        }
    };

    // Keep keyboard-driven active rows visible without changing the outer page scroll position.
    Effect::new({
        let select_id = select_id.clone();

        move |_| {
            let _ = scroll_request.get();

            if !is_open.get() {
                return;
            }

            let Some(index) = active_index.get() else {
                return;
            };

            let Some(menu) = menu_ref.get() else {
                return;
            };

            let Some(document) = web_sys::window().and_then(|window| window.document()) else {
                return;
            };

            let Some(option) = document
                .get_element_by_id(&format!("{select_id}-option-{index}"))
                .and_then(|element| element.dyn_into::<HtmlElement>().ok())
            else {
                return;
            };

            sync_menu_scroll(&menu, &option);
        }
    });

    view! {
        <div class="birei-select-root">
            {label.as_ref().map(|label| {
                view! { <Label text=label.clone() for_id=select_id.clone() required=required/> }
            })}
            <div
                class=class_name
                style=move || line_style.get()
                on:pointerdown=handle_pointer_down
            >
                {move || render_hidden_inputs(name.clone(), multiple, selected_values(), selected_value())}
                <div
                    class="birei-select__surface"
                    aria-expanded=move || if is_open.get() { "true" } else { "false" }
                    aria-controls=format!("{select_id}-listbox")
                    on:click=move |_| {
                        open_menu();
                        focus_input();
                    }
                    >
                        <span class="birei-select__control">
                        {move || {
                            if multiple {
                                let tags = collect_selected_tags(&options_list(), &selected_values());

                                tags
                                    .into_iter()
                                    .map(|tag| {
                                        let value = tag.value;
                                        let label = tag.label;

                                        view! {
                                            <Tag
                                                on_remove=Callback::new(move |event: ev::MouseEvent| {
                                                    event.prevent_default();
                                                    event.stop_propagation();

                                                    let next = selected_values()
                                                        .into_iter()
                                                        .filter(|current| current != &value)
                                                        .collect::<Vec<_>>();
                                                    commit_multiple(next);
                                                    focus_input();
                                                })
                                            >
                                                {label}
                                            </Tag>
                                        }
                                    })
                                    .collect_view()
                                    .into_any()
                            } else {
                                ().into_any()
                            }
                        }}
                        <input
                            class="birei-select__field"
                            id=select_id.clone()
                            node_ref=input_ref
                            type="text"
                            autocomplete="off"
                            spellcheck="false"
                            disabled=disabled
                            required=required && !nullable && !multiple
                            aria-invalid=move || if invalid { "true" } else { "false" }
                            aria-controls=format!("{select_id}-listbox")
                            aria-expanded=move || if is_open.get() { "true" } else { "false" }
                            placeholder=move || {
                                if multiple && has_selection() {
                                    String::from("Filter selections")
                                } else {
                                    placeholder()
                                }
                            }
                            prop:value=display_value
                            on:focus=move |_| {
                                if multiple {
                                    open_menu();
                                }
                            }
                            on:blur=move |_| {
                                is_open.set(false);
                                query.set(String::new());
                            }
                            on:input=move |event| {
                                query.set(event_target_value(&event));
                                open_menu();
                                sync_active_index();

                                if let Some(on_input) = on_input.as_ref() {
                                    on_input.run(event);
                                }
                            }
                            on:keydown=move |event: KeyboardEvent| {
                                match event.key().as_str() {
                                    "ArrowDown" => {
                                        event.prevent_default();
                                        if is_open.get() {
                                            move_active(1);
                                        } else {
                                            open_menu();
                                        }
                                    }
                                    "ArrowUp" => {
                                        event.prevent_default();
                                        if is_open.get() {
                                            move_active(-1);
                                        } else {
                                            open_menu();
                                        }
                                    }
                                    "Enter" => {
                                        if is_open.get() {
                                            event.prevent_default();
                                            select_active_option();
                                        }
                                    }
                                    "Escape" => {
                                        if is_open.get() {
                                            event.prevent_default();
                                            is_open.set(false);
                                            active_index.set(None);
                                            query.set(String::new());
                                        }
                                    }
                                    _ => {}
                                }
                            }
                            on:change=move |event| {
                                if let Some(on_change) = on_change.as_ref() {
                                    on_change.run(event);
                                }
                            }
                        />
                    </span>
                    <span class="birei-select__actions">
                        {move || {
                            (nullable && has_selection() && !disabled).then(|| {
                                view! {
                                    <button
                                        type="button"
                                        class="birei-select__clear"
                                        aria-label="Clear selection"
                                        tabindex="-1"
                                        on:mousedown=move |event| {
                                            event.prevent_default();
                                            event.stop_propagation();
                                        }
                                        on:click=clear_selection
                                    >
                                        "x"
                                    </button>
                                }
                            })
                        }}
                        <button
                            type="button"
                            class="birei-select__toggle"
                            aria-label="Open select options"
                            tabindex="-1"
                            disabled=disabled
                            on:mousedown=move |event| {
                                event.prevent_default();
                            }
                            on:click=move |_| {
                                if is_open.get() {
                                    is_open.set(false);
                                    active_index.set(None);
                                } else {
                                    open_menu();
                                    focus_input();
                                }
                            }
                        >
                            <span class="birei-select__indicator" aria-hidden="true"></span>
                        </button>
                    </span>
                </div>
                {move || {
                    is_open.get().then(|| {
                        let selected_single = selected_value();
                        let selected_multi = selected_values();
                        let options = filtered_options();
                        let current_active = active_index.get();

                        view! {
                            <div
                                class="birei-select__menu"
                                id=format!("{select_id}-listbox")
                                node_ref=menu_ref
                                role="listbox"
                                aria-multiselectable=move || if multiple { "true" } else { "false" }
                            >
                                {if options.is_empty() {
                                    view! {
                                        <div class="birei-select__empty">
                                            "No matching values"
                                        </div>
                                    }
                                        .into_any()
                                } else {
                                    options
                                        .into_iter()
                                        .enumerate()
                                        .map(|(option_index, option)| {
                                            let option_value = option.value;
                                            let option_label = option.label;
                                            let option_icon = option.icon;
                                            let option_disabled = option.disabled;
                                            let is_selected = if multiple {
                                                selected_multi.iter().any(|value| value == &option_value)
                                            } else {
                                                selected_single.as_ref() == Some(&option_value)
                                            };
                                            let is_active = current_active == Some(option_index);

                                            view! {
                                                <SelectMenuOption
                                                    option_id=format!("{select_id}-option-{option_index}")
                                                    label=option_label
                                                    icon=option_icon
                                                    disabled=option_disabled
                                                    selected=is_selected
                                                    active=is_active
                                                    on_hover=Callback::new(move |_| {
                                                        if !option_disabled {
                                                            active_index.set(Some(option_index));
                                                        }
                                                    })
                                                    on_select=Callback::new(move |_| {
                                                        if option_disabled {
                                                            return;
                                                        }

                                                        if multiple {
                                                            let mut next = selected_values();
                                                            if let Some(index) = next.iter().position(|value| value == &option_value) {
                                                                next.remove(index);
                                                            } else {
                                                                next.push(option_value.clone());
                                                            }
                                                            commit_multiple(next);
                                                        } else {
                                                            commit_single(Some(option_value.clone()));
                                                        }

                                                        focus_input();
                                                    })
                                                />
                                            }
                                        })
                                        .collect_view()
                                        .into_any()
                                }}
                            </div>
                        }
                    })
                }}
            </div>
        </div>
    }
}

/// Render a popup row including optional icon, selection mark, and active styling.
#[component]
fn SelectMenuOption(
    option_id: String,
    label: String,
    icon: Option<crate::IcnName>,
    disabled: bool,
    selected: bool,
    active: bool,
    on_hover: Callback<()>,
    on_select: Callback<()>,
) -> impl IntoView {
    view! {
        <button
            type="button"
            id=option_id
            class=option_class_name(selected, active, disabled)
            disabled=disabled
            on:mousedown=move |event| {
                event.prevent_default();
            }
            on:mouseenter=move |_| on_hover.run(())
            on:click=move |_| on_select.run(())
        >
            <span class="birei-select__option-content">
                {icon.map(|icon| {
                    view! {
                        <Icon
                            name=icon
                            size=Size::Small
                            label=label.clone()
                        />
                    }
                })}
                <span>{label.clone()}</span>
            </span>
            <span class="birei-select__option-mark" aria-hidden="true">
                {if selected { "✓" } else { "" }}
            </span>
        </button>
    }
}

/// Find the first selectable option in the current filtered set.
fn first_enabled_index(options: &[SelectOption]) -> Option<usize> {
    options.iter().position(|option| !option.disabled)
}

/// Build the root class list from the shared size token and state flags.
fn build_select_class_name(
    size: Size,
    multiple: bool,
    disabled: bool,
    invalid: bool,
    nullable: bool,
    class: Option<String>,
) -> String {
    let mut classes = vec!["birei-select", size.select_class_name()];

    if multiple {
        classes.push("birei-select--multiple");
    }
    if disabled {
        classes.push("birei-select--disabled");
    }
    if invalid {
        classes.push("birei-select--invalid");
    }
    if nullable {
        classes.push("birei-select--nullable");
    }
    if let Some(class) = class.as_deref() {
        classes.push(class);
    }

    classes.join(" ")
}

/// Resolve the display label for a selected single value.
fn find_option_label(options: &[SelectOption], value: Option<&str>) -> String {
    options
        .iter()
        .find(|option| Some(option.value.as_str()) == value)
        .map(|option| option.label.clone())
        .unwrap_or_default()
}

/// Filter options by label or value against the free-text query.
fn filter_options_by_query(options: &[SelectOption], query: &str) -> Vec<SelectOption> {
    let needle = query.trim().to_lowercase();

    options
        .iter()
        .filter(|option| {
            needle.is_empty()
                || option.label.to_lowercase().contains(&needle)
                || option.value.to_lowercase().contains(&needle)
        })
        .cloned()
        .collect()
}

/// Map selected values back to their labels for multi-select token rendering.
fn collect_selected_tags(
    options: &[SelectOption],
    selected_values: &[String],
) -> Vec<SelectedTagData> {
    selected_values
        .iter()
        .filter_map(|selected_value| {
            options
                .iter()
                .find(|option| option.value == *selected_value)
                .map(|option| SelectedTagData {
                    value: option.value.clone(),
                    label: option.label.clone(),
                })
        })
        .collect()
}

/// Emit hidden inputs so the custom control still participates in native form submission.
fn render_hidden_inputs(
    name: Option<String>,
    multiple: bool,
    selected_values: Vec<String>,
    selected_value: Option<String>,
) -> AnyView {
    match name {
        Some(name) if multiple => selected_values
            .into_iter()
            .map(|value| view! { <input type="hidden" name=name.clone() value=value/> })
            .collect_view()
            .into_any(),
        Some(name) => view! {
            <input
                type="hidden"
                name=name
                value=selected_value.unwrap_or_default()
            />
        }
        .into_any(),
        None => ().into_any(),
    }
}

/// Build the popup option class string from active, selected, and disabled state.
fn option_class_name(selected: bool, active: bool, disabled: bool) -> String {
    let mut classes = String::from("birei-select__option");

    if selected {
        classes.push_str(" birei-select__option--selected");
    }
    if active {
        classes.push_str(" birei-select__option--active");
    }
    if disabled {
        classes.push_str(" birei-select__option--disabled");
    }

    classes
}

/// Scroll only the popup container enough to keep the active option visible.
fn sync_menu_scroll(menu: &HtmlElement, option: &HtmlElement) {
    let option_top = option.offset_top();
    let option_bottom = option_top + option.offset_height();
    let view_top = menu.scroll_top();
    let view_bottom = view_top + menu.client_height();

    if option_top < view_top {
        menu.set_scroll_top(option_top);
    } else if option_bottom > view_bottom {
        menu.set_scroll_top(option_bottom - menu.client_height());
    }
}

/// Move to the next enabled option in the requested direction, wrapping around the list edges.
fn next_enabled_index(
    options: &[SelectOption],
    current: Option<usize>,
    direction: i32,
) -> Option<usize> {
    if options.is_empty() {
        return None;
    }

    let len = options.len() as i32;
    let mut index = current
        .map(|index| index as i32)
        .unwrap_or(if direction >= 0 { -1 } else { len });

    for _ in 0..len {
        index = (index + direction).rem_euclid(len);
        let candidate = &options[index as usize];
        if !candidate.disabled {
            return Some(index as usize);
        }
    }

    None
}
