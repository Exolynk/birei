use leptos::ev;
use leptos::html;
use leptos::prelude::*;
use wasm_bindgen::JsCast;
use web_sys::{HtmlElement, KeyboardEvent};

use super::{CommandExecution, CommandItem, CommandParameterOption, CommandParameterValue};
use crate::{Icon, Size, Tag};

/// Global command launcher with keyboard shortcut, search field, recent items,
/// and action execution callbacks.
#[component]
pub fn CommandPalette(
    /// Commands rendered below any recent items.
    #[prop(into)]
    items: MaybeProp<Vec<CommandItem>>,
    /// Optional recent commands rendered before regular results.
    #[prop(optional, into)]
    recent_items: MaybeProp<Vec<CommandItem>>,
    /// Controlled open state. When omitted, the component manages its own state.
    #[prop(optional, into)]
    open: MaybeProp<bool>,
    /// Controlled query. When omitted, the component manages its own query.
    #[prop(optional, into)]
    query: MaybeProp<String>,
    /// Trigger label shown next to the search icon.
    #[prop(optional, into)]
    label: MaybeProp<String>,
    /// Search input placeholder in the dialog.
    #[prop(optional, into)]
    placeholder: MaybeProp<String>,
    /// Keyboard shortcut hint shown on the trigger.
    #[prop(optional, into)]
    shortcut_label: MaybeProp<String>,
    /// Shared sizing token for the rounded trigger.
    #[prop(optional)]
    size: Size,
    /// Shows a loading row while the host resolves async results.
    #[prop(optional, into)]
    loading: MaybeProp<bool>,
    /// Disables the trigger and global shortcut handling.
    #[prop(optional)]
    disabled: bool,
    /// Enables Cmd/Ctrl+K global opening.
    #[prop(optional, default = true)]
    global_shortcut: bool,
    /// Additional CSS class names applied to the root element.
    #[prop(optional, into)]
    class: Option<String>,
    /// Fired whenever the open state should change.
    #[prop(optional)]
    on_open_change: Option<Callback<bool>>,
    /// Fired whenever the query changes.
    #[prop(optional)]
    on_query_change: Option<Callback<String>>,
) -> impl IntoView {
    let internal_open = RwSignal::new(open.get_untracked().unwrap_or(false));
    let internal_query = RwSignal::new(query.get_untracked().unwrap_or_default());
    let active_index = RwSignal::new(None::<usize>);
    let scroll_request = RwSignal::new(0_u64);
    let trigger_ref = NodeRef::<html::Div>::new();
    let input_ref = NodeRef::<html::Input>::new();
    let list_ref = NodeRef::<html::Div>::new();
    let prompted_item = RwSignal::new(None::<CommandItem>);
    let parameter_values = RwSignal::new(Vec::<CommandParameterValue>::new());
    let active_parameter_index = RwSignal::new(0_usize);
    let line_style = RwSignal::new(String::from(
        "--birei-command-line-origin: 50%; --birei-ripple-x: 50%; --birei-ripple-y: 50%; --birei-ripple-size: 0px;",
    ));
    let ripple_phase = RwSignal::new(None::<bool>);

    let current_open = move || open.get().unwrap_or_else(|| internal_open.get());
    let current_query = move || query.get().unwrap_or_else(|| internal_query.get());

    let suggestion = move || {
        let q = current_query();
        if q.is_empty() {
            return None;
        }

        if let Some(item) = prompted_item.get() {
            let parameter_index = active_parameter_index.get();
            if let Some(parameter) = item.parameters.get(parameter_index) {
                if !parameter.options.is_empty() {
                    let options = filter_parameter_options(&parameter.options, &q);
                    let index = active_index.get().unwrap_or(0);
                    if let Some(selected) = options.get(index) {
                        let label = &selected.label;
                        if label.to_lowercase().starts_with(&q.to_lowercase()) {
                            let suffix: String = label.chars().skip(q.chars().count()).collect();
                            if !suffix.is_empty() {
                                return Some(suffix);
                            }
                        }
                    }
                }
            }
        }
        None
    };
    let items_list = move || items.get().unwrap_or_default();
    let recent_list = move || recent_items.get().unwrap_or_default();
    let is_loading = move || loading.get().unwrap_or(false);

    let class_name = move || {
        let mut classes = vec!["birei-command", command_size_class_name(size)];
        if disabled {
            classes.push("birei-command--disabled");
        }
        if let Some(class) = class.as_deref() {
            classes.push(class);
        }
        classes.join(" ")
    };

    let trigger_class_name = move || {
        let mut classes = String::from("birei-command__trigger");
        if current_open() {
            classes.push_str(" birei-command__trigger--active");
        }
        if let Some(phase) = ripple_phase.get() {
            classes.push_str(if phase {
                " birei-command__trigger--ripple-a"
            } else {
                " birei-command__trigger--ripple-b"
            });
        }
        classes
    };

    let visible_items = move || {
        let query = current_query();
        if prompted_item.get().is_some() {
            return (Vec::new(), Vec::new());
        }

        let regular = filter_command_items(&items_list(), &query);
        let recent = if query.trim().is_empty() {
            recent_list()
        } else {
            Vec::new()
        };
        (recent, regular)
    };

    let flat_items = move || {
        let (recent, regular) = visible_items();
        recent.into_iter().chain(regular).collect::<Vec<_>>()
    };

    let set_open = move |next: bool| {
        internal_open.set(next);
        if !next {
            active_index.set(None);
            prompted_item.set(None);
            parameter_values.set(Vec::new());
            active_parameter_index.set(0);
        } else {
            active_index.set(None);
            scroll_request.update(|value| *value += 1);
            reset_command_scroll(&list_ref);
        }
        if let Some(on_open_change) = on_open_change.as_ref() {
            on_open_change.run(next);
        }
    };

    let set_query = move |next: String| {
        internal_query.set(next.clone());
        if prompted_item.get_untracked().is_some() {
            active_index.set(Some(0));
        } else {
            if next.trim().is_empty() {
                active_index.set(None);
            } else {
                let next_items = visible_items_for_query(&items_list(), &recent_list(), &next);
                sync_active_index(active_index, &flatten_visible_items(next_items), true);
            }
            reset_command_scroll(&list_ref);
        }
        scroll_request.update(|value| *value += 1);
        if let Some(on_query_change) = on_query_change.as_ref() {
            on_query_change.run(next);
        }
    };

    let focus_input = move || {
        if let Some(input) = input_ref.get_untracked() {
            let _ = input.focus();
            input.select();
        }
    };

    let open_palette = move || {
        if disabled {
            return;
        }
        set_open(true);
        request_animation_frame(focus_input);
    };

    let close_palette = move || {
        set_open(false);
    };

    let run_command = move |item: CommandItem, parameters: Vec<CommandParameterValue>| {
        if item.disabled {
            return;
        }

        if let Some(action) = item.action {
            action.run(CommandExecution {
                item: item.clone(),
                parameters,
            });
        }

        set_open(false);
        set_query(String::new());
        prompted_item.set(None);
        parameter_values.set(Vec::new());
        active_parameter_index.set(0);
    };

    let execute_command = move |item: CommandItem| {
        if item.disabled {
            return;
        }

        if !item.parameters.is_empty() {
            prompted_item.set(Some(item));
            parameter_values.set(Vec::new());
            active_parameter_index.set(0);
            set_query(String::new());
            active_index.set(Some(0));
            request_animation_frame(focus_input);
        } else {
            run_command(item, Vec::new());
        }
    };

    let commit_parameter_value = move |item: CommandItem, value: String| {
        let parameter_index = active_parameter_index.get_untracked();
        let Some(parameter) = item.parameters.get(parameter_index).cloned() else {
            return;
        };

        let mut next_values = parameter_values.get_untracked();
        if next_values.len() > parameter_index {
            next_values.truncate(parameter_index);
        }
        next_values.push(CommandParameterValue {
            name: parameter.name,
            value,
        });

        if parameter_index + 1 >= item.parameters.len() {
            run_command(item, next_values);
        } else {
            parameter_values.set(next_values);
            active_parameter_index.set(parameter_index + 1);
            set_query(String::new());
            active_index.set(Some(0));
            reset_command_scroll(&list_ref);
            request_animation_frame(focus_input);
        }
    };

    let execute_prompted_command = move || {
        let Some(item) = prompted_item.get() else {
            return;
        };
        let parameter_index = active_parameter_index.get();
        let Some(parameter) = item.parameters.get(parameter_index).cloned() else {
            return;
        };

        if parameter.options.is_empty() {
            let value = current_query().trim().to_owned();
            if value.is_empty() {
                return;
            }
            commit_parameter_value(item, value);
        } else {
            let options = filter_parameter_options(&parameter.options, &current_query());
            let Some(index) = active_index.get() else {
                return;
            };
            let Some(option) = options.get(index).cloned() else {
                return;
            };
            commit_parameter_value(item, option.value);
        }
    };

    let move_active = move |direction: i32| {
        if prompted_item.get_untracked().is_some() {
            let next = prompted_item
                .get_untracked()
                .and_then(|item| {
                    item.parameters
                        .get(active_parameter_index.get_untracked())
                        .filter(|parameter| !parameter.options.is_empty())
                        .map(|parameter| {
                            next_enabled_parameter_option_index(
                                &filter_parameter_options(&parameter.options, &current_query()),
                                active_index.get_untracked(),
                                direction,
                            )
                            .or(Some(0))
                        })
                })
                .flatten()
                .or(Some(0));
            active_index.set(next);
            scroll_request.update(|value| *value += 1);
            return;
        }

        let items = flat_items();
        let next = next_enabled_command_index(&items, active_index.get(), direction)
            .or_else(|| first_enabled_command_index(&items));
        active_index.set(next);
        scroll_request.update(|value| *value += 1);
    };

    let execute_active = move || {
        if prompted_item.get_untracked().is_some() {
            execute_prompted_command();
            return;
        }

        let items = flat_items();
        let Some(index) = active_index.get() else {
            return;
        };
        let Some(item) = items.get(index).cloned() else {
            return;
        };
        execute_command(item);
    };

    let handle_trigger_pointer_down = move |event: ev::PointerEvent| {
        if let Some(target) = event
            .current_target()
            .and_then(|target| target.dyn_into::<HtmlElement>().ok())
        {
            let rect = target.get_bounding_client_rect();
            let x = f64::from(event.client_x()) - rect.left();
            let y = f64::from(event.client_y()) - rect.top();
            let size = rect.width().max(rect.height()) * 1.35;
            line_style.set(format!(
                "--birei-command-line-origin: {x}px; --birei-ripple-x: {x}px; --birei-ripple-y: {y}px; --birei-ripple-size: {size}px;"
            ));
            ripple_phase.update(|phase| *phase = Some(!phase.unwrap_or(false)));
        }
    };

    Effect::new(move |_| {
        if !global_shortcut || disabled {
            return;
        }

        let keydown_handle = window_event_listener_untyped("keydown", move |event| {
            let Some(event) = event.dyn_ref::<KeyboardEvent>() else {
                return;
            };

            if event.key() == "Enter" && (event.meta_key() || event.ctrl_key()) {
                event.prevent_default();
                open_palette();
            }
        });

        on_cleanup(move || keydown_handle.remove());
    });

    Effect::new(move |_| {
        if !current_open() {
            return;
        }

        request_animation_frame(focus_input);

        let click_handle = window_event_listener_untyped("mousedown", move |_| {
            close_palette();
        });

        on_cleanup(move || click_handle.remove());
    });

    Effect::new(move |_| {
        let is_open = current_open();
        let is_prompted = prompted_item.get().is_some();
        let items = flat_items();

        if is_open && is_prompted {
            active_index.set(Some(0));
            scroll_request.update(|value| *value += 1);
        } else if is_open {
            sync_active_index(active_index, &items, !current_query().trim().is_empty());
            scroll_request.update(|value| *value += 1);
        }
    });

    Effect::new(move |_| {
        let _ = scroll_request.get();
        if !current_open() {
            return;
        }

        request_animation_frame(move || {
            let Some(index) = active_index.get_untracked() else {
                return;
            };
            let Some(list) = list_ref.get_untracked() else {
                return;
            };
            let Some(item) = find_command_item_element(&list, index) else {
                return;
            };

            sync_command_scroll(&list, &item);
        });
    });

    view! {
        <div class=class_name>
            <div
                class=trigger_class_name
                style=move || line_style.get()
                node_ref=trigger_ref
                on:pointerdown=handle_trigger_pointer_down
                on:mousedown=move |event| event.stop_propagation()
                on:click=move |_| if !current_open() { open_palette() }
            >
                <Icon name="search" size=Size::Small label="Search"/>

                {move || {
                    prompted_item.get().map(|item| {
                        view! {
                            <Tag class="birei-command__prompt-tag">
                                {item.name}
                            </Tag>
                        }
                    })
                }}
                {move || {
                    let item = prompted_item.get();
                    parameter_values
                        .get()
                        .into_iter()
                        .enumerate()
                        .map(|(i, pv)| {
                            let label = item.as_ref()
                                .and_then(|it| it.parameters.get(i))
                                .and_then(|p| {
                                    p.options.iter()
                                        .find(|o| o.value == pv.value)
                                        .map(|o| o.label.clone())
                                })
                                .unwrap_or(pv.value);
                            view! {
                                <Tag class="birei-command__prompt-tag">
                                    {label}
                                </Tag>
                            }
                        })
                        .collect_view()
                }}

                <div class="birei-command__field-wrapper">
                    <input
                        class="birei-command__field"
                        node_ref=input_ref
                        type="text"
                        autocomplete="off"
                        spellcheck="false"
                        disabled=disabled
                        placeholder=move || {
                            if !current_open() {
                                return label.get().unwrap_or_else(|| String::from("Search or run command"));
                            }
                            prompted_item
                                .get()
                                .and_then(|item| {
                                    item.parameters
                                        .get(active_parameter_index.get())
                                        .map(|parameter| parameter.placeholder.clone())
                                })
                                .or_else(|| placeholder.get())
                                .unwrap_or_else(|| String::from("Search commands..."))
                        }
                        prop:value=current_query
                        on:focus=move |_| if !current_open() { open_palette() }
                        on:input=move |event| {
                            set_query(event_target_value(&event));
                        }
                        on:keydown=move |event: KeyboardEvent| {
                            match event.key().as_str() {
                                "Tab" => {
                                    let q = current_query();
                                    let sugg = suggestion();
                                    if sugg.is_some() || !q.is_empty() {
                                        if let Some(item) = prompted_item.get_untracked() {
                                            let parameter_index = active_parameter_index.get_untracked();
                                            if let Some(parameter) = item.parameters.get(parameter_index) {
                                                if !parameter.options.is_empty() {
                                                    let options = filter_parameter_options(&parameter.options, &q);
                                                    if options.len() == 1 {
                                                        event.prevent_default();
                                                        commit_parameter_value(item, options[0].value.clone());
                                                        return;
                                                    }
                                                }
                                            }
                                        }
                                    }

                                    if let Some(suffix) = sugg {
                                        event.prevent_default();
                                        set_query(format!("{}{}", q, suffix));
                                    }
                                }
                                "ArrowDown" => {
                                    event.prevent_default();
                                    move_active(1);
                                }
                                "ArrowUp" => {
                                    event.prevent_default();
                                    move_active(-1);
                                }
                                "Enter" => {
                                    event.prevent_default();
                                    execute_active();
                                }
                                " " if prompted_item.get_untracked().is_none() => {
                                    if let Some(item) = exact_shortcut_command(&items_list(), &current_query()) {
                                        if !item.parameters.is_empty() {
                                            event.prevent_default();
                                            execute_command(item);
                                        }
                                    }
                                }
                                "Escape" => {
                                    event.prevent_default();
                                    if prompted_item.get_untracked().is_some() {
                                        prompted_item.set(None);
                                        parameter_values.set(Vec::new());
                                        active_parameter_index.set(0);
                                        set_query(String::new());
                                        sync_active_index(
                                            active_index,
                                            &flat_items(),
                                            !current_query().trim().is_empty(),
                                        );
                                    } else {
                                        close_palette();
                                    }
                                }
                                "Backspace" if current_query().is_empty() && prompted_item.get_untracked().is_some() => {
                                    event.prevent_default();
                                    let index = active_parameter_index.get_untracked();
                                    if index > 0 {
                                        let mut values = parameter_values.get_untracked();
                                        values.pop();
                                        parameter_values.set(values);
                                        active_parameter_index.set(index - 1);
                                    } else {
                                        prompted_item.set(None);
                                        parameter_values.set(Vec::new());
                                        sync_active_index(
                                            active_index,
                                            &flat_items(),
                                            !current_query().trim().is_empty(),
                                        );
                                    }
                                }
                                _ => {}
                            }
                        }
                    />
                    {move || suggestion().map(|suffix| {
                        view! {
                            <span
                                class="birei-command__suggestion"
                                aria-hidden="true"
                            >
                                <span class="birei-command__suggestion-prefix">{current_query()}</span>
                                {suffix}
                            </span>
                        }
                    })}
                </div>

                <Tag class="birei-command__shortcut">
                    {move || match shortcut_label.get() {
                        Some(label) => label.into_any(),
                        None => view! {
                            <Icon name="command" size=Size::Small label="Command"/>
                            " ↵"
                        }.into_any()
                    }}
                </Tag>
                <span class="birei-command__trigger-line" aria-hidden="true"></span>
            </div>

            {move || {
                current_open().then(|| {
                    view! {
                        <div
                            class="birei-command__dropdown"
                            on:mousedown=move |event| event.stop_propagation()
                        >
                            <div class="birei-command__list" node_ref=list_ref role="listbox">
                                        {move || {
                                            let (recent, regular) = visible_items();
                                            let loading = is_loading();
                                            let prompted = prompted_item.get();

                                            if let Some(item) = prompted {
                                                let input = current_query();
                                                let trimmed = input.trim().to_owned();
                                                let trimmed_is_empty = trimmed.is_empty();
                                                let parameter_index = active_parameter_index.get();
                                                let parameter = item.parameters.get(parameter_index).cloned();
                                                let is_last_parameter = parameter_index + 1 >= item.parameters.len();
                                                let parameter_name = parameter
                                                    .as_ref()
                                                    .map(|parameter| parameter.name.clone())
                                                    .unwrap_or_else(|| String::from("value"));

                                                if let Some(parameter) = parameter.filter(|parameter| !parameter.options.is_empty()) {
                                                    let options = filter_parameter_options(&parameter.options, &input);
                                                    view! {
                                                        <section class="birei-command__section">
                                                            <div class="birei-command__section-title">
                                                                {format!("Select {parameter_name}")}
                                                            </div>
                                                            {if options.is_empty() {
                                                                view! {
                                                                    <div class="birei-command__empty">
                                                                        "No matching options"
                                                                    </div>
                                                                }.into_any()
                                                            } else {
                                                                options
                                                                    .into_iter()
                                                                    .enumerate()
                                                                    .map(|(option_index, option)| {
                                                                        let option_for_select = option.clone();
                                                                        let item_for_select = item.clone();
                                                                        let item_icon = item.icon.clone();
                                                                        let item_name = item.name.clone();
                                                                        let item_description = item.description.clone();
                                                                        view! {
                                                                            <button
                                                                                type="button"
                                                                                data-command-index=option_index.to_string()
                                                                                class=move || command_item_class_name(active_index.get().unwrap_or(0) == option_index, false)
                                                                                role="option"
                                                                                tabindex="-1"
                                                                                aria-selected=move || if active_index.get().unwrap_or(0) == option_index { "true" } else { "false" }
                                                                                on:mousedown=move |event| event.prevent_default()
                                                                                on:mouseenter=move |_| active_index.set(Some(option_index))
                                                                                on:mousemove=move |_| active_index.set(Some(option_index))
                                                                                on:click=move |_| commit_parameter_value(item_for_select.clone(), option_for_select.value.clone())
                                                                            >
                                                                                <span class="birei-command__item-icon">
                                                                                    {item_icon.map(|icon| {
                                                                                        view! { <Icon name=icon size=Size::Small label=item_name.clone()/> }
                                                                                    })}
                                                                                </span>
                                                                                <span class="birei-command__item-body">
                                                                                    <span class="birei-command__item-name">{option.label}</span>
                                                                                    <span class="birei-command__item-description">
                                                                                        {if is_last_parameter {
                                                                                            item_description.unwrap_or_else(|| String::from("Execute command with this option"))
                                                                                        } else {
                                                                                            String::from("Continue to the next parameter")
                                                                                        }}
                                                                                    </span>
                                                                                </span>
                                                                            </button>
                                                                        }
                                                                    })
                                                                    .collect_view()
                                                                    .into_any()
                                                            }}
                                                        </section>
                                                    }.into_any()
                                                } else {
                                                    view! {
                                                        <section class="birei-command__section">
                                                            <div class="birei-command__section-title">
                                                                {format!("Parameter {}", parameter_index + 1)}
                                                            </div>
                                                            <button
                                                                type="button"
                                                                data-command-index="0"
                                                                class=move || {
                                                                    command_item_class_name(
                                                                        active_index.get().unwrap_or(0) == 0,
                                                                        trimmed_is_empty,
                                                                    )
                                                                }
                                                                role="option"
                                                                tabindex="-1"
                                                                aria-selected=move || {
                                                                    if active_index.get().unwrap_or(0) == 0 { "true" } else { "false" }
                                                                }
                                                                disabled=trimmed_is_empty
                                                                on:mousedown=move |event| event.prevent_default()
                                                                on:mouseenter=move |_| active_index.set(Some(0))
                                                                on:mousemove=move |_| active_index.set(Some(0))
                                                                on:click=move |_| execute_prompted_command()
                                                            >
                                                                <span class="birei-command__item-icon">
                                                                    {item.icon.map(|icon| {
                                                                        view! { <Icon name=icon size=Size::Small label=item.name.clone()/> }
                                                                    })}
                                                                </span>
                                                                <span class="birei-command__item-body">
                                                                    <span class="birei-command__item-name">
                                                                        {if trimmed_is_empty {
                                                                            format!("Enter {parameter_name}")
                                                                        } else if is_last_parameter {
                                                                            format!("{} \"{}\"", item.name, trimmed)
                                                                        } else {
                                                                            format!("Set {parameter_name} to \"{}\"", trimmed)
                                                                        }}
                                                                    </span>
                                                                    <span class="birei-command__item-description">
                                                                        {if is_last_parameter {
                                                                            item.description.unwrap_or_else(|| String::from("Execute command with the provided parameters"))
                                                                        } else {
                                                                            String::from("Continue to the next parameter")
                                                                        }}
                                                                    </span>
                                                                </span>
                                                            </button>
                                                        </section>
                                                    }.into_any()
                                                }
                                            } else if recent.is_empty() && regular.is_empty() && !loading {
                                                view! {
                                                    <div class="birei-command__empty">
                                                        "No matching commands"
                                                    </div>
                                                }.into_any()
                                            } else {
                                                let regular_start_index = recent.len();
                                                let recent_items = recent
                                                    .into_iter()
                                                    .enumerate()
                                                    .collect::<Vec<_>>();

                                                view! {
                                                    {(!recent_items.is_empty()).then(|| {
                                                        view! {
                                                            <CommandSection
                                                                title="Recent"
                                                                items=recent_items
                                                                active_index=active_index
                                                                on_hover=Callback::new(move |index| active_index.set(Some(index)))
                                                                on_select=Callback::new(execute_command)
                                                            />
                                                        }
                                                    })}
                                                    {(!regular.is_empty()).then(|| {
                                                        view! {
                                                            <CommandGroups
                                                                items=regular
                                                                start_index=regular_start_index
                                                                active_index=active_index
                                                                on_hover=Callback::new(move |index| active_index.set(Some(index)))
                                                                on_select=Callback::new(execute_command)
                                                            />
                                                        }
                                                    })}
                                                    {loading.then(|| {
                                                        view! {
                                                            <div class="birei-command__loading">
                                                                "Searching..."
                                                            </div>
                                                        }
                                                    })}
                                                }.into_any()
                                            }
                                        }}
                            </div>
                        </div>
                    }
                })
            }}
        </div>
    }
}

#[component]
fn CommandGroups(
    items: Vec<CommandItem>,
    start_index: usize,
    active_index: RwSignal<Option<usize>>,
    on_hover: Callback<usize>,
    on_select: Callback<CommandItem>,
) -> impl IntoView {
    let mut groups = Vec::<(String, Vec<(usize, CommandItem)>)>::new();
    for (offset, item) in items.into_iter().enumerate() {
        let absolute_index = start_index + offset;
        let group = item
            .group
            .clone()
            .unwrap_or_else(|| String::from("Commands"));
        if let Some((_, group_items)) = groups.iter_mut().find(|(name, _)| name == &group) {
            group_items.push((absolute_index, item));
        } else {
            groups.push((group, vec![(absolute_index, item)]));
        }
    }

    view! {
        <>
            {groups.into_iter().map(|(title, items)| {
                view! {
                    <CommandSection
                        title=title
                        items=items
                        active_index=active_index
                        on_hover=on_hover
                        on_select=on_select
                    />
                }
            }).collect_view()}
        </>
    }
}

#[component]
fn CommandSection(
    title: impl Into<String>,
    items: Vec<(usize, CommandItem)>,
    active_index: RwSignal<Option<usize>>,
    on_hover: Callback<usize>,
    on_select: Callback<CommandItem>,
) -> impl IntoView {
    let title = title.into();

    view! {
        <section class="birei-command__section">
            <div class="birei-command__section-title">{title}</div>
            {items.into_iter().map(|(index, item)| {
                let item_for_execute = item.clone();
                let disabled = item.disabled;
                let has_shortcut = item.shortcut.is_some();

                view! {
                    <button
                        type="button"
                        data-command-index=index.to_string()
                        class=move || command_item_class_name(active_index.get() == Some(index), disabled)
                        role="option"
                        tabindex="-1"
                        aria-selected=move || if active_index.get() == Some(index) { "true" } else { "false" }
                        disabled=disabled
                        on:mousedown=move |event| event.prevent_default()
                        on:mouseenter=move |_| {
                            if !disabled {
                                on_hover.run(index);
                            }
                        }
                        on:mousemove=move |_| {
                            if !disabled {
                                on_hover.run(index);
                            }
                        }
                        on:click=move |_| on_select.run(item_for_execute.clone())
                    >
                        <span class=if has_shortcut {
                            "birei-command__item-icon birei-command__item-icon--has-shortcut"
                        } else {
                            "birei-command__item-icon"
                        }>
                            {item.icon.map(|icon| {
                                view! {
                                    <span class="birei-command__item-icon-glyph">
                                        <Icon name=icon size=Size::Small label=item.name.clone()/>
                                    </span>
                                }
                            })}
                            {item.shortcut.clone().map(|shortcut| {
                                view! {
                                    <Tag class="birei-command__item-shortcut birei-command__item-shortcut--leading">
                                        {shortcut}
                                    </Tag>
                                }
                            })}
                        </span>
                        <span class="birei-command__item-body">
                            <span class="birei-command__item-name">{item.name}</span>
                            {item.description.map(|description| {
                                view! {
                                    <span class="birei-command__item-description">{description}</span>
                                }
                            })}
                        </span>
                        {item.shortcut.map(|shortcut| {
                            view! {
                                <Tag class="birei-command__item-shortcut">{shortcut}</Tag>
                            }
                        })}
                    </button>
                }
            }).collect_view()}
        </section>
    }
}

fn command_size_class_name(size: Size) -> &'static str {
    match size {
        Size::Small => "birei-command--small",
        Size::Medium => "birei-command--medium",
        Size::Large => "birei-command--large",
    }
}

fn command_item_class_name(active: bool, disabled: bool) -> String {
    let mut classes = String::from("birei-command__item");
    if active {
        classes.push_str(" birei-command__item--active");
    }
    if disabled {
        classes.push_str(" birei-command__item--disabled");
    }
    classes
}

fn filter_command_items(items: &[CommandItem], query: &str) -> Vec<CommandItem> {
    let needle = query.trim().to_lowercase();
    let compact_needle = compact_shortcut_query(query);
    items
        .iter()
        .filter(|item| {
            needle.is_empty()
                || item.name.to_lowercase().contains(&needle)
                || item.value.to_lowercase().contains(&needle)
                || item
                    .description
                    .as_ref()
                    .is_some_and(|description| description.to_lowercase().contains(&needle))
                || item
                    .group
                    .as_ref()
                    .is_some_and(|group| group.to_lowercase().contains(&needle))
                || item.shortcut.as_ref().is_some_and(|shortcut| {
                    shortcut.to_lowercase().contains(&needle)
                        || shortcut.to_lowercase().contains(&compact_needle)
                })
        })
        .cloned()
        .collect()
}

fn visible_items_for_query(
    items: &[CommandItem],
    recent_items: &[CommandItem],
    query: &str,
) -> (Vec<CommandItem>, Vec<CommandItem>) {
    let regular = filter_command_items(items, query);
    let recent = if query.trim().is_empty() {
        recent_items.to_vec()
    } else {
        Vec::new()
    };
    (recent, regular)
}

fn flatten_visible_items(visible_items: (Vec<CommandItem>, Vec<CommandItem>)) -> Vec<CommandItem> {
    let (recent, regular) = visible_items;
    recent.into_iter().chain(regular).collect()
}

fn exact_shortcut_command(items: &[CommandItem], query: &str) -> Option<CommandItem> {
    let needle = compact_shortcut_query(query);
    if needle.is_empty() {
        return None;
    }

    items
        .iter()
        .find(|item| {
            !item.disabled
                && item
                    .shortcut
                    .as_ref()
                    .is_some_and(|shortcut| shortcut.to_lowercase() == needle)
        })
        .cloned()
}

fn filter_parameter_options(
    options: &[CommandParameterOption],
    query: &str,
) -> Vec<CommandParameterOption> {
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

fn next_enabled_parameter_option_index(
    options: &[CommandParameterOption],
    current: Option<usize>,
    direction: i32,
) -> Option<usize> {
    if options.is_empty() {
        return None;
    }

    let len = options.len() as i32;
    let index = current
        .map(|index| index as i32)
        .unwrap_or(if direction >= 0 { -1 } else { len });
    Some((index + direction).rem_euclid(len) as usize)
}

fn compact_shortcut_query(query: &str) -> String {
    query
        .chars()
        .filter(|character| !character.is_whitespace())
        .flat_map(char::to_lowercase)
        .collect()
}

fn sync_active_index(active_index: RwSignal<Option<usize>>, items: &[CommandItem], seed: bool) {
    let next = active_index
        .get_untracked()
        .filter(|index| items.get(*index).is_some_and(|item| !item.disabled))
        .or_else(|| seed.then(|| first_enabled_command_index(items)).flatten());
    active_index.set(next);
}

fn first_enabled_command_index(items: &[CommandItem]) -> Option<usize> {
    items.iter().position(|item| !item.disabled)
}

fn next_enabled_command_index(
    items: &[CommandItem],
    current: Option<usize>,
    direction: i32,
) -> Option<usize> {
    if items.is_empty() {
        return None;
    }

    let len = items.len() as i32;
    let mut index = current
        .map(|index| index as i32)
        .unwrap_or(if direction >= 0 { -1 } else { len });

    for _ in 0..len {
        index = (index + direction).rem_euclid(len);
        let candidate = &items[index as usize];
        if !candidate.disabled {
            return Some(index as usize);
        }
    }

    None
}

fn find_command_item_element(list: &HtmlElement, item_index: usize) -> Option<HtmlElement> {
    list.query_selector(&format!(r#"[data-command-index="{item_index}"]"#))
        .ok()
        .flatten()
        .and_then(|element| element.dyn_into::<HtmlElement>().ok())
}

fn reset_command_scroll(list_ref: &NodeRef<html::Div>) {
    let list_ref = *list_ref;
    request_animation_frame(move || {
        if let Some(list) = list_ref.get_untracked() {
            list.set_scroll_top(0);
        }
    });
}

fn sync_command_scroll(list: &HtmlElement, item: &HtmlElement) {
    let item_top = item.offset_top();
    let item_bottom = item_top + item.offset_height();
    let view_top = list.scroll_top();
    let view_bottom = view_top + list.client_height();
    let padding = 8;

    if item_top - padding < view_top {
        list.set_scroll_top((item_top - padding).max(0));
    } else if item_bottom + padding > view_bottom {
        list.set_scroll_top(item_bottom + padding - list.client_height());
    }
}
