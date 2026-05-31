use crate::ArcOneCallback;
use leptos::ev;
use leptos::html;
use leptos::prelude::*;
use wasm_bindgen::closure::Closure;
use wasm_bindgen::JsCast;
use web_sys::{window, HtmlElement, KeyboardEvent, ResizeObserver};

use super::ButtonBarItem;
use crate::command_palette::cmd_collections::{
    notify_command_collection_registry, register_button_bar, unregister_button_bar,
};
use crate::{ButtonMenu, ButtonMenuItem, ButtonVariant, Icon, Size};

/// Horizontal action bar that moves overflowing buttons into a dropdown.
#[component]
pub fn ButtonBar(
    /// Available buttons rendered in order.
    #[prop(into)]
    items: MaybeProp<Vec<ButtonBarItem>>,
    /// Optional id applied to the toolbar root.
    #[prop(optional, into)]
    id: Option<String>,
    /// Additional CSS class names applied to the root element.
    #[prop(optional, into)]
    class: Option<String>,
    /// Shared button variant applied to visible buttons and the overflow trigger.
    #[prop(optional)]
    variant: ButtonVariant,
    /// Shared size applied to visible buttons and the overflow trigger.
    #[prop(optional)]
    size: Size,
    /// Registers this button bar as a source for command palette action commands.
    ///
    /// When `true`, a [`CommandPalette`](crate::CommandPalette) with
    /// `button_bar_commands` configured can generate commands for activating
    /// these items. The visible buttons and overflow trigger are removed from
    /// tab-key navigation while command-palette control is enabled.
    #[prop(optional, default = false)]
    command_palette: bool,
    /// Callback fired when a button is activated directly or through the overflow menu.
    #[prop(optional, into)]
    on_select: Option<ArcOneCallback<String>>,
) -> impl IntoView {
    // DOM measurement is required because the component decides overflow from
    // actual rendered button widths rather than estimated string lengths.
    let root_ref = NodeRef::<html::Div>::new();
    let resize_observer_attached = RwSignal::new(false);
    let container_width = RwSignal::new(0.0_f64);
    let measured_button_widths = RwSignal::new(Vec::<f64>::new());
    let overflow_trigger_width = RwSignal::new(0.0_f64);
    let button_gap = RwSignal::new(0.0_f64);
    let resize_observer = StoredValue::new_local(None::<ResizeObserver>);
    let resize_callback =
        StoredValue::new_local(None::<Closure<dyn FnMut(js_sys::Array, ResizeObserver)>>);
    let current_items = RwSignal::new(snapshot_button_items_untracked(
        items.get_untracked().unwrap_or_default(),
    ));

    Effect::new(move |_| {
        let next_items = items.get().unwrap_or_default();
        current_items.set(snapshot_button_items(next_items));
    });

    Effect::new(move |_| {
        if command_palette {
            current_items.get();
            notify_command_collection_registry();
        }
    });

    // The overflow layout is derived reactively from the latest measurements
    // and container width.
    let overflow_layout = Memo::new(move |_| {
        compute_overflow_layout(
            &current_items.get(),
            &measured_button_widths.try_get().unwrap_or_default(),
            overflow_trigger_width.try_get().unwrap_or_default(),
            button_gap.try_get().unwrap_or_default(),
            container_width.try_get().unwrap_or_default(),
        )
    });
    // Root classes only carry the optional external hook class for this
    // component; button appearance is handled per-trigger below.
    let class_name = move || {
        let mut classes = vec!["birei-button-bar"];
        if let Some(class) = class.as_deref() {
            classes.push(class);
        }
        classes.join(" ")
    };

    // Hidden measurement buttons mirror the visible styles so overflow
    // decisions are based on the exact rendered footprint.
    let measure_button_widths = move |item_count: usize| {
        let Some(root) = root_ref.try_get_untracked().flatten() else {
            return;
        };

        let widths = (0..item_count)
            .map(|index| {
                root.query_selector(&format!(
                    "[data-birei-button-bar-measure-index=\"{index}\"]"
                ))
                .ok()
                .flatten()
                .map(|button| {
                    button
                        .unchecked_into::<HtmlElement>()
                        .get_bounding_client_rect()
                        .width()
                })
                .unwrap_or(0.0)
            })
            .collect::<Vec<_>>();
        let _ = measured_button_widths.try_set(widths);

        let menu_width = root
            .query_selector("[data-birei-button-bar-measure-overflow]")
            .ok()
            .flatten()
            .map(|button| {
                button
                    .unchecked_into::<HtmlElement>()
                    .get_bounding_client_rect()
                    .width()
            })
            .unwrap_or(0.0);
        let _ = overflow_trigger_width.try_set(menu_width);

        let gap = window()
            .and_then(|window| window.get_computed_style(&root).ok().flatten())
            .and_then(|style| style.get_property_value("column-gap").ok())
            .and_then(|value| value.trim_end_matches("px").parse::<f64>().ok())
            .unwrap_or(0.0);
        let _ = button_gap.try_set(gap);
    };

    // Item changes can change labels, icons, and count, so widths are
    // remeasured whenever the item list changes.
    Effect::new(move |_| {
        let item_count = current_items.get().len();

        let Some(window) = window() else {
            measure_button_widths(item_count);
            return;
        };

        let callback = Closure::once_into_js(move || {
            measure_button_widths(item_count);
        });
        let _ = window.request_animation_frame(callback.unchecked_ref());
    });

    // A resize observer keeps the toolbar responsive as its own width changes.
    Effect::new(move |_| {
        let Some(root) = root_ref.try_get().flatten() else {
            return;
        };
        if resize_observer_attached
            .try_get_untracked()
            .unwrap_or_default()
        {
            return;
        }

        let _ = container_width.try_set(f64::from(root.client_width()));

        let callback = Closure::wrap(Box::new(
            move |_entries: js_sys::Array, _observer: ResizeObserver| {
                if let Some(root) = root_ref.try_get_untracked().flatten() {
                    let _ = container_width.try_set(f64::from(root.client_width()));
                    let item_count = current_items.get_untracked().len();
                    measure_button_widths(item_count);
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

    // Activation is centralized so direct buttons and overflow menu items
    // share the same disabled handling and external callback path.
    let select_item = move |item: &ButtonBarItem, event: ev::MouseEvent| {
        if item.disabled {
            return;
        }

        if let Some(on_click) = item.on_click.as_ref() {
            on_click.run(event);
        }
        if let Some(on_select) = on_select.as_ref() {
            on_select.run(item.value.clone());
        }
    };

    let select_item_by_value = move |item_value: &str, event: ev::MouseEvent| {
        let Some(item) = current_items
            .get_untracked()
            .into_iter()
            .find(|item| item.value == item_value)
        else {
            return;
        };

        select_item(&item, event);
    };

    Effect::new(move |_| {
        if !command_palette {
            return;
        }

        let command_select = ArcOneCallback::new(move |item_value: String| {
            let Some(item) = current_items
                .get_untracked()
                .into_iter()
                .find(|item| item.value == item_value)
            else {
                return;
            };

            if item.disabled {
                return;
            }

            if let Some(on_click) = item.on_click.as_ref() {
                if let Ok(event) = web_sys::MouseEvent::new("click") {
                    on_click.run(event);
                }
            }
            if let Some(on_select) = on_select.as_ref() {
                on_select.run(item.value);
            }
        });

        let registration_id = register_button_bar(current_items, command_select);
        on_cleanup(move || unregister_button_bar(registration_id));
    });

    // Keyboard roving focus targets only visible toolbar buttons; overflow
    // items are handled by the menu component itself.
    let focus_visible_button = move |index: usize| {
        if let Some(root) = root_ref.try_get().flatten() {
            if let Ok(Some(button)) =
                root.query_selector(&format!("[data-birei-button-bar-index=\"{index}\"]"))
            {
                let _ = button.unchecked_into::<HtmlElement>().focus();
            }
        }
    };

    // Toolbar arrow-key behavior follows the currently visible button order.
    let handle_keydown = move |event: KeyboardEvent, index: usize| {
        let key = event.key();
        if matches!(key.as_str(), "Enter" | " " | "Spacebar") {
            event.prevent_default();
            if let Some(target) = event
                .current_target()
                .and_then(|target| target.dyn_into::<HtmlElement>().ok())
            {
                target.click();
            }
            return;
        }

        if !matches!(key.as_str(), "ArrowLeft" | "ArrowRight" | "Home" | "End") {
            return;
        }

        event.prevent_default();

        let items = current_items.get_untracked();
        let visible_indices = overflow_layout
            .try_get()
            .map(|layout| layout.visible_indices)
            .unwrap_or_default();
        let next_index = match key.as_str() {
            "ArrowLeft" => adjacent_enabled_visible_index(&items, &visible_indices, index, -1),
            "ArrowRight" => adjacent_enabled_visible_index(&items, &visible_indices, index, 1),
            "Home" => first_enabled_visible_index(&items, &visible_indices),
            "End" => last_enabled_visible_index(&items, &visible_indices),
            _ => None,
        };

        if let Some(next_index) = next_index {
            focus_visible_button(next_index);
        }
    };

    view! {
        <div id=id class=class_name node_ref=root_ref role="toolbar">
            <For
                each=move || {
                    overflow_layout
                        .try_get()
                        .map(|layout| layout.visible_indices)
                        .unwrap_or_default()
                }
                key=move |index| {
                    current_items
                        .get_untracked()
                        .get(*index)
                        .map(|item| button_item_key(*index, item))
                        .unwrap_or_else(|| index.to_string())
                }
                children=move |index| {
                    let Some(item) = current_items.get().get(index).cloned()
                    else {
                        return ().into_any();
                    };
                    let item_label = item.label;
                    let item_icon = item.icon.clone();
                    let ripple_style = RwSignal::new(String::from(
                        "--birei-ripple-x: 50%; --birei-ripple-y: 50%; --birei-ripple-size: 0px;",
                    ));
                    let ripple_phase = RwSignal::new(None::<bool>);
                    let class_name = move || {
                        button_bar_button_class_name(
                            variant,
                            size,
                            item.disabled,
                            ripple_phase.try_get().flatten(),
                        )
                    };

                    view! {
                        <button
                            type="button"
                            class=class_name
                            style=move || ripple_style.try_get().unwrap_or_default()
                            data-birei-button-bar-index=index
                            tabindex=if command_palette || item.disabled { "-1" } else { "0" }
                            disabled=item.disabled
                            on:click={
                                let item_value = item.value.clone();
                                move |event: ev::MouseEvent| {
                                    update_button_ripple(&event, ripple_style, ripple_phase);
                                    select_item_by_value(&item_value, event);
                                }
                            }
                            on:keydown=move |event| {
                                if !command_palette {
                                    handle_keydown(event, index);
                                }
                            }
                        >
                            {item_icon.map(|icon| {
                                view! {
                                    <Icon
                                        name=icon
                                        size=size
                                        label=format!(
                                            "{} icon",
                                            item_label
                                                .try_get_untracked()
                                                .flatten()
                                                .unwrap_or_default(),
                                        )
                                        class="birei-button-bar__icon"
                                    />
                                }
                            })}
                            <span>{move || item_label.try_get().flatten().unwrap_or_default()}</span>
                        </button>
                    }
                    .into_any()
                }
            />
            {move || {
                let layout = overflow_layout.try_get().unwrap_or_default();
                let items = current_items.get();

                (!layout.overflow_indices.is_empty()).then(|| {
                    let menu_items = layout
                        .overflow_indices
                        .iter()
                        .filter_map(|index| items.get(*index))
                        .map(|item| {
                            let mut menu_item =
                                ButtonMenuItem::new(
                                    item.value.clone(),
                                    item.label.try_get().flatten().unwrap_or_default(),
                                )
                                    .disabled(item.disabled);
                            if let Some(icon) = item.icon.clone() {
                                menu_item = menu_item.icon(icon);
                            }
                            if let Some(on_click) = item.on_click {
                                menu_item = menu_item.on_click(on_click);
                            }
                            menu_item
                        })
                        .collect::<Vec<_>>();
                    view! {
                        <ButtonMenu
                            label="More"
                            items=menu_items
                            class="birei-button-bar__overflow"
                            variant=variant
                            size=size
                            tabindex=if command_palette { -1 } else { 0 }
                            match_trigger_width=false
                            on_select=Callback::new(move |next: String| {
                                if let Some(on_select) = on_select.as_ref() {
                                    on_select.run(next);
                                }
                            })
                        />
                    }
                })
            }}
            <div class="birei-button-bar__measure" aria-hidden="true">
                <For
                    each=move || {
                        current_items
                            .get()
                            .into_iter()
                            .enumerate()
                    }
                    key=|(index, item)| format!("measure-{}", button_item_key(*index, item))
                    children=move |(index, item)| {
                        view! {
                            <button
                                type="button"
                                class=button_bar_button_class_name(variant, size, item.disabled, None)
                                tabindex="-1"
                                data-birei-button-bar-measure-index=index
                            >
                                {item.icon.map(|icon| {
                                    let label = item.label;
                                    view! {
                                        <Icon
                                            name=icon
                                            size=size
                                            label=format!(
                                                "{} icon",
                                                label
                                                    .try_get_untracked()
                                                    .flatten()
                                                    .unwrap_or_default(),
                                            )
                                            class="birei-button-bar__icon"
                                        />
                                    }
                                })}
                                <span>{move || item.label.try_get().flatten().unwrap_or_default()}</span>
                            </button>
                        }
                    }
                />
                <button
                    type="button"
                    class=dropdown_trigger_class_name(variant, size, false)
                    tabindex="-1"
                    data-birei-button-bar-measure-overflow="true"
                >
                    <span>"More"</span>
                    <span class="birei-dropdown-button__divider" aria-hidden="true"></span>
                    <span class="birei-dropdown-button__caret" aria-hidden="true">
                        <Icon name="chevron-down" size=Size::Small/>
                    </span>
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

/// Splits the item list into visible and overflow sets based on measured
/// button widths and the current container width.
fn compute_overflow_layout(
    items: &[ButtonBarItem],
    item_widths: &[f64],
    overflow_width: f64,
    item_gap: f64,
    container_width: f64,
) -> OverflowLayout {
    if items.is_empty() {
        return OverflowLayout::default();
    }

    if item_widths.len() != items.len() || container_width <= 0.0 {
        return OverflowLayout {
            visible_indices: (0..items.len()).collect(),
            overflow_indices: Vec::new(),
        };
    }

    let total_width =
        item_widths.iter().sum::<f64>() + item_gap * items.len().saturating_sub(1) as f64;
    if total_width <= container_width {
        return OverflowLayout {
            visible_indices: (0..items.len()).collect(),
            overflow_indices: Vec::new(),
        };
    }

    let available = (container_width - overflow_width - item_gap).max(0.0);
    let mut visible_indices = Vec::new();
    let mut used_width = 0.0_f64;

    for (index, width) in item_widths.iter().enumerate() {
        let next_width = if visible_indices.is_empty() {
            *width
        } else {
            used_width + item_gap + width
        };
        if next_width > available {
            break;
        }

        visible_indices.push(index);
        used_width = next_width;
    }

    let overflow_indices = (0..items.len())
        .filter(|index| !visible_indices.contains(index))
        .collect::<Vec<_>>();

    OverflowLayout {
        visible_indices,
        overflow_indices,
    }
}

/// Finds the first enabled visible button for `Home` key navigation.
fn first_enabled_visible_index(
    items: &[ButtonBarItem],
    visible_indices: &[usize],
) -> Option<usize> {
    visible_indices
        .iter()
        .copied()
        .find(|index| items.get(*index).is_some_and(|item| !item.disabled))
}

/// Finds the last enabled visible button for `End` key navigation.
fn last_enabled_visible_index(items: &[ButtonBarItem], visible_indices: &[usize]) -> Option<usize> {
    visible_indices
        .iter()
        .rev()
        .copied()
        .find(|index| items.get(*index).is_some_and(|item| !item.disabled))
}

/// Walks left or right through the visible button indices, skipping disabled
/// items and wrapping around when necessary.
fn adjacent_enabled_visible_index(
    items: &[ButtonBarItem],
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

        if items.get(candidate).is_some_and(|item| !item.disabled) {
            return Some(candidate);
        }
    }

    None
}

/// Builds the shared class list used by both visible toolbar buttons and
/// hidden measurement buttons.
fn button_bar_button_class_name(
    variant: ButtonVariant,
    size: Size,
    disabled: bool,
    ripple_phase: Option<bool>,
) -> String {
    let mut classes = vec![
        "birei-button",
        variant.class_name(),
        size.button_class_name(),
        "birei-button-bar__button",
    ];

    if disabled {
        classes.push("birei-button--disabled");
    }
    if let Some(phase) = ripple_phase {
        classes.push(if phase {
            "birei-button--ripple-a"
        } else {
            "birei-button--ripple-b"
        });
    }

    classes.join(" ")
}

/// Matches the dropdown trigger styling used by the overflow menu button and
/// its hidden measurement counterpart.
fn dropdown_trigger_class_name(variant: ButtonVariant, size: Size, disabled: bool) -> String {
    let mut classes = vec![
        "birei-dropdown-button__trigger",
        "birei-button",
        variant.class_name(),
        size.button_class_name(),
    ];

    if disabled {
        classes.push("birei-button--disabled");
    }

    classes.join(" ")
}

fn button_item_key(index: usize, item: &ButtonBarItem) -> String {
    let label = item.label.try_get_untracked().flatten().unwrap_or_default();
    let icon = item
        .icon
        .as_ref()
        .map(|icon| icon.as_str())
        .unwrap_or_default();

    format!("{index}:{}:{label}:{icon}:{}", item.value, item.disabled)
}

fn snapshot_button_items(items: Vec<ButtonBarItem>) -> Vec<ButtonBarItem> {
    items.into_iter().map(snapshot_button_item).collect()
}

fn snapshot_button_items_untracked(items: Vec<ButtonBarItem>) -> Vec<ButtonBarItem> {
    items
        .into_iter()
        .map(snapshot_button_item_untracked)
        .collect()
}

fn snapshot_button_item(item: ButtonBarItem) -> ButtonBarItem {
    let label = item.label.get().unwrap_or_default();
    snapshot_button_item_with_label(item, label)
}

fn snapshot_button_item_untracked(item: ButtonBarItem) -> ButtonBarItem {
    let label = item.label.get_untracked().unwrap_or_default();
    snapshot_button_item_with_label(item, label)
}

fn snapshot_button_item_with_label(item: ButtonBarItem, label: String) -> ButtonBarItem {
    ButtonBarItem {
        value: item.value,
        label: label.into(),
        icon: item.icon,
        disabled: item.disabled,
        on_click: item.on_click,
    }
}

/// Reuses the same ripple animation contract as the main button component.
fn update_button_ripple(
    event: &ev::MouseEvent,
    ripple_style: RwSignal<String>,
    ripple_phase: RwSignal<Option<bool>>,
) {
    if let Some(target) = event
        .current_target()
        .and_then(|target| target.dyn_into::<HtmlElement>().ok())
    {
        let rect = target.get_bounding_client_rect();
        let (x, y) = if event.detail() == 0 {
            (rect.width() / 2.0, rect.height() / 2.0)
        } else {
            (
                f64::from(event.client_x()) - rect.left(),
                f64::from(event.client_y()) - rect.top(),
            )
        };
        let size = rect.width().max(rect.height()) * 1.35;

        ripple_style.set(format!(
            "--birei-ripple-x: {x}px; --birei-ripple-y: {y}px; --birei-ripple-size: {size}px;"
        ));
        ripple_phase.update(|phase| {
            *phase = Some(!phase.unwrap_or(false));
        });
    }
}
