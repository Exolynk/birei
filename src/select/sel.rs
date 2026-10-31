use crate::ArcOneCallback;
use leptos::ev;
use leptos::html;
use leptos::portal::Portal;
use leptos::prelude::*;
use wasm_bindgen::JsCast;
use web_sys::{HtmlElement, KeyboardEvent};

use super::layout::{
    fit_selected_tags, keep_menu_in_viewport, sync_menu_scroll_to_index, update_menu_layout,
};
use super::menu::SelectMenu;
use super::options::{
    collect_selected_tags, filter_options_by_query, find_option_label, first_enabled_index,
    first_selected_index, next_enabled_index,
};
use super::SelectOption;
use crate::common::run_on_next_frame;
use crate::common::use_resize_observer;
use crate::common::FloatingPopupLayout;
use crate::{Size, Tag};

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
    /// Marks the select as read-only while keeping the current value visible.
    #[prop(optional)]
    readonly: bool,
    /// Marks the select as invalid for styling and accessibility.
    #[prop(optional)]
    invalid: bool,
    /// Marks the field as required.
    #[prop(optional)]
    required: bool,
    /// Prefix content rendered before the field.
    #[prop(optional, into)]
    prefix: Option<ViewFn>,
    /// Suffix content rendered after the field.
    #[prop(optional, into)]
    suffix: Option<ViewFn>,
    /// Additional CSS class names applied to the root element.
    #[prop(optional, into)]
    class: Option<String>,
    /// Input event handler for the search field.
    #[prop(optional, into)]
    on_input: Option<ArcOneCallback<ev::Event>>,
    /// Change event handler for the search field.
    #[prop(optional, into)]
    on_change: Option<ArcOneCallback<ev::Event>>,
    /// Selection callback for single-select mode.
    #[prop(optional, into)]
    on_value_change: Option<ArcOneCallback<Option<String>>>,
    /// Selection callback for multi-select mode.
    #[prop(optional, into)]
    on_values_change: Option<ArcOneCallback<Vec<String>>>,
) -> impl IntoView {
    // The select keeps its own query, active option, popup layout, and local
    // fallback value state so it can support both controlled and uncontrolled usage.
    let has_prefix = prefix.is_some();
    let has_suffix = suffix.is_some();
    let class_name = build_select_class_name(
        size, multiple, disabled, readonly, invalid, nullable, has_prefix, has_suffix, class,
    );
    let line_style = RwSignal::new(String::from("--birei-select-line-origin: 50%;"));
    let is_open = RwSignal::new(false);
    let query = RwSignal::new(String::new());
    let internal_value = RwSignal::new(value.try_get_untracked().flatten().flatten());
    let internal_values = RwSignal::new(values.try_get_untracked().flatten().unwrap_or_default());
    let active_index = RwSignal::new(None::<usize>);
    let input_ref = NodeRef::<html::Input>::new();
    let surface_ref = NodeRef::<html::Div>::new();
    let menu_ref = NodeRef::<html::Div>::new();
    let control_ref = NodeRef::<html::Span>::new();
    // Labels of selected tags that do not fit on a single-line control, shown as a `+N` chip.
    let hidden_tag_labels = RwSignal::new(Vec::<String>::new());
    let scroll_request = RwSignal::new(0_u64);
    let menu_scroll_top = RwSignal::new(0.0_f64);
    let menu_viewport_height = RwSignal::new(0.0_f64);
    let menu_layout = RwSignal::new(FloatingPopupLayout::default());

    // Resolve the currently visible selection from controlled props first, then local interaction state.
    let selected_value = move || {
        value
            .try_get()
            .flatten()
            .flatten()
            .or_else(|| internal_value.try_get().flatten())
    };
    let selected_values = move || {
        values
            .try_get()
            .flatten()
            .unwrap_or_else(|| internal_values.try_get().unwrap_or_default())
    };
    let placeholder = move || placeholder.try_get().flatten().unwrap_or_default();
    let options_list = Memo::new(move |_| options.try_get().flatten().unwrap_or_default());

    let selected_label = move || {
        let options = options_list.try_get().unwrap_or_default();
        find_option_label(&options, selected_value().as_deref())
    };

    // Selection state drives placeholder, clear-button visibility, and tag rendering.
    let has_selection = move || {
        if multiple {
            !selected_values().is_empty()
        } else {
            selected_value().is_some()
        }
    };

    // In single-select mode the field shows the selected label while closed,
    // but switches back to the raw query while the popup is filtering.
    let display_value = move || {
        if multiple {
            query.try_get().unwrap_or_default()
        } else {
            let current_query = query.try_get().unwrap_or_default();
            if current_query.is_empty() && !is_open.try_get().unwrap_or_default() {
                selected_label()
            } else {
                current_query
            }
        }
    };

    // Filter the option set against the current query on both label and value.
    let filtered_options = Memo::new(move |_| {
        let options = options_list.try_get().unwrap_or_default();
        filter_options_by_query(&options, &query.try_get().unwrap_or_default())
    });

    // Keep the active option aligned with the filtered list and skip disabled entries.
    let sync_active_index = move || {
        let current_active = active_index.try_get_untracked().flatten();
        let filtered = filtered_options.try_get_untracked().unwrap_or_default();
        let next_active = current_active
            .filter(|index| filtered.get(*index).is_some_and(|option| !option.disabled))
            .or_else(|| first_enabled_index(&filtered));
        active_index.set(next_active);
    };

    // When the menu opens, try to align the active option with the current selection.
    let sync_active_to_selection = move || {
        let filtered = filtered_options.try_get_untracked().unwrap_or_default();
        let next_active = first_selected_index(
            &filtered,
            untrack(selected_value).as_deref(),
            &untrack(selected_values),
            multiple,
        )
        .or_else(|| first_enabled_index(&filtered));
        active_index.set(next_active);
    };

    // Programmatic focus returns to the text field after actions such as clear
    // or menu selection.
    let focus_input = move || {
        if let Some(input) = input_ref.try_get_untracked().flatten() {
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

    // Multi-select stays open after selection changes so users can pick several options quickly.
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

        if !nullable || disabled || readonly {
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
        if disabled || readonly {
            return;
        }

        let was_open = is_open.try_get_untracked().unwrap_or(false);
        is_open.set(true);
        if !was_open {
            sync_active_to_selection();
            scroll_request.update(|value| *value += 1);
            menu_scroll_top.set(0.0);
        }
        update_menu_layout(&surface_ref, menu_layout);
    };

    // Arrow-key navigation wraps through enabled options only.
    let move_active = move |direction: i32| {
        let filtered = filtered_options.try_get_untracked().unwrap_or_default();
        let next_index = if filtered.is_empty() {
            None
        } else {
            next_enabled_index(
                &filtered,
                active_index.try_get_untracked().flatten(),
                direction,
            )
            .or_else(|| first_enabled_index(&filtered))
        };

        if next_index.is_none() {
            active_index.set(None);
            return;
        }

        active_index.set(next_index);
        scroll_request.update(|value| *value += 1);
    };

    // Enter selects the active option for both single and multi-select variants.
    let select_active_option = move || {
        let Some(index) = active_index.try_get_untracked().flatten() else {
            return;
        };
        let Some(option) = filtered_options
            .try_get_untracked()
            .and_then(|filtered| filtered.get(index).cloned())
        else {
            return;
        };
        if option.disabled {
            return;
        }

        if multiple {
            let mut next = untrack(selected_values);
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

    // The values the menu marks as selected, for both selection modes.
    let selected_list = Signal::derive(move || {
        if multiple {
            selected_values()
        } else {
            selected_value().into_iter().collect()
        }
    });
    // Choosing an option toggles it in multi-select mode and replaces the value otherwise.
    let select_option = ArcOneCallback::new(move |option_value: String| {
        if multiple {
            let mut next = untrack(selected_values);
            if let Some(index) = next.iter().position(|value| value == &option_value) {
                next.remove(index);
            } else {
                next.push(option_value);
            }
            commit_multiple(next);
        } else {
            commit_single(Some(option_value));
        }
        focus_input();
    });

    // Keep keyboard-driven active rows visible without changing the outer page scroll position.
    Effect::new(move |_| {
        let _ = scroll_request.try_get().unwrap_or_default();

        if !is_open.try_get_untracked().unwrap_or(false) {
            return;
        }

        let Some(index) = active_index.try_get_untracked().flatten() else {
            return;
        };

        let Some(menu) = menu_ref.try_get_untracked().flatten() else {
            return;
        };

        sync_menu_scroll_to_index(&menu, index);
        menu_scroll_top.set(f64::from(menu.scroll_top()));
        menu_viewport_height.set(f64::from(menu.client_height()));
    });

    // The portaled menu needs one frame before its max-height and scrollable
    // viewport are measurable. Seed the virtualization window immediately
    // after mount so the first paint fills the popup instead of only overscan.
    Effect::new(move |_| {
        if !is_open.try_get().unwrap_or_default() {
            return;
        }

        run_on_next_frame(move || {
            let Some(menu) = menu_ref.try_get_untracked().flatten() else {
                return;
            };

            if let Some(index) = active_index.try_get_untracked().flatten() {
                sync_menu_scroll_to_index(&menu, index);
            }

            keep_menu_in_viewport(menu_ref, menu_layout);
            let next_scroll_top = f64::from(menu.scroll_top());
            let next_viewport_height = f64::from(menu.client_height());

            if menu_scroll_top.try_get_untracked() != Some(next_scroll_top) {
                let _ = menu_scroll_top.try_set(next_scroll_top);
            }
            if menu_viewport_height.try_get_untracked() != Some(next_viewport_height) {
                let _ = menu_viewport_height.try_set(next_viewport_height);
            }
        });
    });

    // A single-line control, as in table cells, collapses the tags that do not fit into a
    // `+N` chip; refit after the selection, the open state, or the control's size changes.
    if multiple {
        Effect::new(move |_| {
            let _ = selected_values();
            let _ = options_list.try_get();
            let _ = is_open.try_get();
            run_on_next_frame(move || fit_selected_tags(control_ref, hidden_tag_labels));
        });

        use_resize_observer(control_ref, move |_| {
            fit_selected_tags(control_ref, hidden_tag_labels);
        });
    }

    // While the menu is open, keep its floating layout synced to viewport
    // resize and scroll events.
    Effect::new(move |_| {
        if !is_open.try_get().unwrap_or_default() {
            return;
        }

        update_menu_layout(&surface_ref, menu_layout);

        let resize_handle = window_event_listener_untyped("resize", {
            move |_| {
                update_menu_layout(&surface_ref, menu_layout);
                run_on_next_frame(move || keep_menu_in_viewport(menu_ref, menu_layout));
            }
        });
        let scroll_handle = window_event_listener_untyped("scroll", {
            move |_| {
                update_menu_layout(&surface_ref, menu_layout);
                run_on_next_frame(move || keep_menu_in_viewport(menu_ref, menu_layout));
            }
        });

        on_cleanup(move || {
            resize_handle.remove();
            scroll_handle.remove();
        });
    });

    view! {
        <div
            class=class_name
            style=move || line_style.try_get().unwrap_or_default()
            on:pointerdown=handle_pointer_down
        >
                {move || render_hidden_inputs(name.clone(), multiple, selected_values(), selected_value())}
                <div
                    class="birei-select__surface"
                    node_ref=surface_ref
                    aria-expanded=move || if is_open.try_get().unwrap_or_default() { "true" } else { "false" }
                    on:click=move |_| {
                        open_menu();
                        focus_input();
                    }
                    >
                        {prefix.as_ref().map(|prefix| {
                            view! {
                                <span
                                    class="birei-select__affix birei-select__affix--prefix"
                                    on:click=move |event| event.stop_propagation()
                                >
                                    {prefix.run()}
                                </span>
                            }
                        })}
                        <span class="birei-select__control" node_ref=control_ref>
                        {move || {
                            if multiple {
                                let options = options_list.try_get().unwrap_or_default();
                                let tags = collect_selected_tags(&options, &selected_values());

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

                                                    let next = untrack(selected_values)
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
                        {move || {
                            let hidden = hidden_tag_labels.try_get().unwrap_or_default();
                            (!hidden.is_empty()).then(|| {
                                view! {
                                    <span class="birei-select__more" title=hidden.join(", ")>
                                        {format!("+{}", hidden.len())}
                                    </span>
                                }
                            })
                        }}
                        <input
                            class="birei-select__field"
                            id=id.clone()
                            node_ref=input_ref
                            type="text"
                            autocomplete="off"
                            spellcheck="false"
                            disabled=disabled
                            readonly=readonly
                            required=required && !nullable && !multiple
                            aria-invalid=move || if invalid { "true" } else { "false" }
                            placeholder=move || {
                                if multiple && has_selection() {
                                    String::from("Filter selections")
                                } else {
                                    placeholder()
                                }
                            }
                            prop:value=display_value
                            on:focus=move |_| {
                                if !readonly && !disabled {
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
                                        if is_open.try_get_untracked().unwrap_or_default() {
                                            move_active(1);
                                        } else {
                                            open_menu();
                                        }
                                    }
                                    "ArrowUp" => {
                                        event.prevent_default();
                                        if is_open.try_get_untracked().unwrap_or_default() {
                                            move_active(-1);
                                        } else {
                                            open_menu();
                                        }
                                    }
                                    "Enter" if is_open.try_get_untracked().unwrap_or_default() => {
                                        event.prevent_default();
                                        select_active_option();
                                    }
                                    "Escape" if is_open.try_get_untracked().unwrap_or_default() => {
                                        event.prevent_default();
                                        is_open.set(false);
                                        active_index.set(None);
                                        query.set(String::new());
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
                    {move || {
                        let show_clear = nullable && has_selection() && !disabled && !readonly;
                        let show_toggle = !disabled && !readonly;

                        (show_clear || show_toggle).then(|| {
                            view! {
                                <span class="birei-select__actions">
                                    {show_clear.then(|| {
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
                                    })}
                                    {show_toggle.then(|| {
                                        view! {
                                            <button
                                                type="button"
                                                class="birei-select__toggle"
                                                aria-label="Open select options"
                                                tabindex="-1"
                                                on:mousedown=move |event| {
                                                    event.prevent_default();
                                                }
                                                on:click=move |_| {
                                                    if is_open.try_get_untracked().unwrap_or_default() {
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
                                        }
                                    })}
                                </span>
                            }
                        })
                    }}
                    {suffix.as_ref().map(|suffix| {
                        view! {
                            <span
                                class="birei-select__affix birei-select__affix--suffix"
                                on:click=move |event| event.stop_propagation()
                            >
                                {suffix.run()}
                            </span>
                        }
                    })}
            </div>
            {move || {
                is_open.try_get().unwrap_or_default().then(|| {
                    view! {
                        <Portal>
                            <SelectMenu
                                options=filtered_options
                                selected=selected_list
                                active_index=active_index
                                layout=menu_layout
                                scroll_top=menu_scroll_top
                                viewport_height=menu_viewport_height
                                menu_ref=menu_ref
                                multiple=multiple
                                on_select=select_option
                            />
                        </Portal>
                    }
                })
            }}
        </div>
    }
}

/// Build the root class list from the shared size token and state flags.
#[allow(clippy::too_many_arguments)]
fn build_select_class_name(
    size: Size,
    multiple: bool,
    disabled: bool,
    readonly: bool,
    invalid: bool,
    nullable: bool,
    has_prefix: bool,
    has_suffix: bool,
    class: Option<String>,
) -> String {
    let mut classes = vec!["birei-select", size.class_name()];

    if has_prefix {
        classes.push("birei-select--with-prefix");
    }
    if has_suffix {
        classes.push("birei-select--with-suffix");
    }
    if multiple {
        classes.push("birei-select--multiple");
    }
    if disabled {
        classes.push("birei-select--disabled");
    }
    if readonly {
        classes.push("birei-select--readonly");
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
