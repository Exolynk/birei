use leptos::ev;
use leptos::html;
use leptos::prelude::*;
use wasm_bindgen::closure::Closure;
use wasm_bindgen::JsCast;
use web_sys::{window, HtmlElement, KeyboardEvent, ResizeObserver};

use super::ButtonBarItem;
use crate::{ButtonVariant, Icon, MenuButton, MenuButtonItem, Size};

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
    /// Callback fired when a button is activated directly or through the overflow menu.
    #[prop(optional)]
    on_select: Option<Callback<String>>,
) -> impl IntoView {
    let root_ref = NodeRef::<html::Div>::new();
    let resize_observer_attached = RwSignal::new(false);
    let container_width = RwSignal::new(0.0_f64);
    let measured_button_widths = RwSignal::new(Vec::<f64>::new());
    let overflow_trigger_width = RwSignal::new(0.0_f64);
    let button_gap = RwSignal::new(0.0_f64);
    let resize_observer = StoredValue::new_local(None::<ResizeObserver>);
    let resize_callback =
        StoredValue::new_local(None::<Closure<dyn FnMut(js_sys::Array, ResizeObserver)>>);

    let overflow_layout = Memo::new(move |_| {
        compute_overflow_layout(
            &items.get().unwrap_or_default(),
            &measured_button_widths.get(),
            overflow_trigger_width.get(),
            button_gap.get(),
            container_width.get(),
        )
    });
    let class_name = move || {
        let mut classes = vec!["birei-button-bar"];
        if let Some(class) = class.as_deref() {
            classes.push(class);
        }
        classes.join(" ")
    };

    let measure_button_widths = move || {
        let items = items.get().unwrap_or_default();
        let Some(root) = root_ref.get_untracked() else {
            return;
        };

        let widths = items
            .iter()
            .enumerate()
            .map(|(index, _)| {
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
        measured_button_widths.set(widths);

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
        overflow_trigger_width.set(menu_width);

        let gap = window()
            .and_then(|window| window.get_computed_style(&root).ok().flatten())
            .and_then(|style| style.get_property_value("column-gap").ok())
            .and_then(|value| value.trim_end_matches("px").parse::<f64>().ok())
            .unwrap_or(0.0);
        button_gap.set(gap);
    };

    Effect::new(move |_| {
        items.get();
        measure_button_widths();
    });

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
                if let Some(root) = root_ref.get_untracked() {
                    container_width.set(f64::from(root.client_width()));
                    measure_button_widths();
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

    let select_item = move |item: &ButtonBarItem| {
        if item.disabled {
            return;
        }

        if let Some(on_select) = on_select.as_ref() {
            on_select.run(item.value.clone());
        }
    };

    let focus_visible_button = move |index: usize| {
        if let Some(root) = root_ref.get() {
            if let Ok(Some(button)) =
                root.query_selector(&format!("[data-birei-button-bar-index=\"{index}\"]"))
            {
                let _ = button.unchecked_into::<HtmlElement>().focus();
            }
        }
    };

    let handle_keydown = move |event: KeyboardEvent, index: usize| {
        let key = event.key();
        if !matches!(key.as_str(), "ArrowLeft" | "ArrowRight" | "Home" | "End") {
            return;
        }

        event.prevent_default();

        let items = items.get().unwrap_or_default();
        let visible_indices = overflow_layout.get().visible_indices;
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
                each=move || overflow_layout.get().visible_indices
                key=move |index| {
                    items.get()
                        .unwrap_or_default()
                        .get(*index)
                        .map(|item| format!("{index}:{}", item.value))
                        .unwrap_or_else(|| index.to_string())
                }
                children=move |index| {
                    let Some(item) = items.get().unwrap_or_default().get(index).cloned() else {
                        return ().into_any();
                    };
                    let item_label = item.label.clone();
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
                            ripple_phase.get(),
                        )
                    };

                    view! {
                        <button
                            type="button"
                            class=class_name
                            style=move || ripple_style.get()
                            data-birei-button-bar-index=index
                            disabled=item.disabled
                            on:click={
                                let item = item.clone();
                                move |event: ev::MouseEvent| {
                                    update_button_ripple(&event, ripple_style, ripple_phase);
                                    select_item(&item);
                                }
                            }
                            on:keydown=move |event| handle_keydown(event, index)
                        >
                            {item_icon.map(|icon| {
                                view! {
                                    <Icon
                                        name=icon
                                        size=size
                                        label=format!("{} icon", item_label)
                                        class="birei-button-bar__icon"
                                    />
                                }
                            })}
                            <span>{item_label}</span>
                        </button>
                    }
                    .into_any()
                }
            />
            {move || {
                let layout = overflow_layout.get();
                let items = items.get().unwrap_or_default();

                (!layout.overflow_indices.is_empty()).then(|| {
                    let menu_items = layout
                        .overflow_indices
                        .iter()
                        .filter_map(|index| items.get(*index))
                        .map(|item| {
                            let mut menu_item =
                                MenuButtonItem::new(item.value.clone(), item.label.clone())
                                    .disabled(item.disabled);
                            if let Some(icon) = item.icon.clone() {
                                menu_item = menu_item.icon(icon);
                            }
                            menu_item
                        })
                        .collect::<Vec<_>>();

                    view! {
                        <MenuButton
                            label="More"
                            items=menu_items
                            class="birei-button-bar__overflow"
                            variant=variant
                            size=size
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
                    each=move || items.get().unwrap_or_default().into_iter().enumerate()
                    key=|(index, item)| format!("measure-{index}:{}", item.value)
                    children=move |(index, item)| {
                        view! {
                            <button
                                type="button"
                                class=button_bar_button_class_name(variant, size, item.disabled, None)
                                tabindex="-1"
                                data-birei-button-bar-measure-index=index
                            >
                                {item.icon.map(|icon| {
                                    view! {
                                        <Icon
                                            name=icon
                                            size=size
                                            label=format!("{} icon", item.label)
                                            class="birei-button-bar__icon"
                                        />
                                    }
                                })}
                                <span>{item.label}</span>
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

fn first_enabled_visible_index(
    items: &[ButtonBarItem],
    visible_indices: &[usize],
) -> Option<usize> {
    visible_indices
        .iter()
        .copied()
        .find(|index| items.get(*index).is_some_and(|item| !item.disabled))
}

fn last_enabled_visible_index(items: &[ButtonBarItem], visible_indices: &[usize]) -> Option<usize> {
    visible_indices
        .iter()
        .rev()
        .copied()
        .find(|index| items.get(*index).is_some_and(|item| !item.disabled))
}

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
        let x = f64::from(event.client_x()) - rect.left();
        let y = f64::from(event.client_y()) - rect.top();
        let size = rect.width().max(rect.height()) * 1.35;

        ripple_style.set(format!(
            "--birei-ripple-x: {x}px; --birei-ripple-y: {y}px; --birei-ripple-size: {size}px;"
        ));
        ripple_phase.update(|phase| {
            *phase = Some(!phase.unwrap_or(false));
        });
    }
}
