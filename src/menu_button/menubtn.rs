use leptos::ev;
use leptos::html;
use leptos::portal::Portal;
use leptos::prelude::*;
use wasm_bindgen::JsCast;
use web_sys::{HtmlElement, KeyboardEvent};

use super::MenuButtonItem;
use crate::button::ButtonVariant;
use crate::common::{
    dropdown_menu_theme_style, measure_floating_popup_layout, FloatingPopupLayout,
    FLOATING_POPUP_EDGE_PADDING,
};
use crate::{IcnName, Icon, Size};

#[derive(Clone, Default)]
struct DropdownMenuTheme {
    style: String,
}

/// Button-triggered popup action menu.
#[component]
pub fn MenuButton(
    /// Visible button label.
    #[prop(into)]
    label: String,
    /// Menu items rendered in the popup.
    #[prop(into)]
    items: MaybeProp<Vec<MenuButtonItem>>,
    /// Leading icon shown before the label.
    #[prop(optional, into)]
    icon: Option<IcnName>,
    /// Shared button variant.
    #[prop(optional)]
    variant: ButtonVariant,
    /// Shared size token.
    #[prop(optional)]
    size: Size,
    /// Disables the trigger and prevents interaction.
    #[prop(optional)]
    disabled: bool,
    /// Additional class names applied to the wrapper.
    #[prop(optional, into)]
    class: Option<String>,
    /// Callback fired with the selected item value.
    #[prop(optional)]
    on_select: Option<Callback<String>>,
) -> impl IntoView {
    let trigger_ref = NodeRef::<html::Button>::new();
    let menu_ref = NodeRef::<html::Div>::new();
    let is_open = RwSignal::new(false);
    let active_index = RwSignal::new(None::<usize>);
    let menu_layout = RwSignal::new(FloatingPopupLayout::default());
    let menu_theme = RwSignal::new(DropdownMenuTheme::default());
    let scroll_request = RwSignal::new(0_u64);
    let items_list = move || items.get().unwrap_or_default();

    let class_name = move || {
        let mut classes = vec!["birei-dropdown-button"];
        if let Some(class) = class.as_deref() {
            classes.push(class);
        }
        classes.join(" ")
    };

    let sync_active_index = move || {
        let next_active = active_index
            .get()
            .filter(|index| items_list().get(*index).is_some_and(|item| !item.disabled))
            .or_else(|| first_enabled_item_index(&items_list()));
        active_index.set(next_active);
    };

    let focus_trigger = move || {
        if let Some(button) = trigger_ref.get() {
            let _ = button.focus();
        }
    };

    let open_menu = move || {
        if disabled {
            return;
        }

        is_open.set(true);
        sync_active_index();
        scroll_request.update(|value| *value += 1);
        update_dropdown_menu_state(&trigger_ref, menu_layout, menu_theme);
    };

    let close_menu = move || {
        is_open.set(false);
        active_index.set(None);
    };

    let select_item = move |item: &MenuButtonItem| {
        if item.disabled {
            return;
        }

        close_menu();
        if let Some(on_select) = on_select.as_ref() {
            on_select.run(item.value.clone());
        }
        focus_trigger();
    };

    let move_active = move |direction: i32| {
        let items = items_list();
        if items.is_empty() {
            active_index.set(None);
            return;
        }

        let next_index = next_enabled_dropdown_index(&items, active_index.get(), direction)
            .or_else(|| first_enabled_item_index(&items));
        active_index.set(next_index);
        scroll_request.update(|value| *value += 1);
    };

    let select_active_item = move || {
        let items = items_list();
        let Some(index) = active_index.get() else {
            return;
        };
        let Some(item) = items.get(index) else {
            return;
        };

        select_item(item);
    };

    Effect::new(move |_| {
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
        let Some(option) = find_dropdown_item_element(&menu, index) else {
            return;
        };

        sync_dropdown_menu_scroll(&menu, &option);
    });

    Effect::new(move |_| {
        if !is_open.get() {
            return;
        }

        update_dropdown_menu_state(&trigger_ref, menu_layout, menu_theme);

        let resize_handle = window_event_listener_untyped("resize", {
            move |_| update_dropdown_menu_state(&trigger_ref, menu_layout, menu_theme)
        });
        let scroll_handle = window_event_listener_untyped("scroll", {
            move |_| update_dropdown_menu_state(&trigger_ref, menu_layout, menu_theme)
        });
        let pointer_handle = window_event_listener_untyped("pointerdown", {
            move |event| {
                let Some(target) = event
                    .target()
                    .and_then(|target| target.dyn_into::<web_sys::Node>().ok())
                else {
                    return;
                };

                let clicked_trigger = trigger_ref
                    .get()
                    .is_some_and(|trigger| trigger.contains(Some(&target)));
                let clicked_menu = menu_ref
                    .get()
                    .is_some_and(|menu| menu.contains(Some(&target)));

                if !clicked_trigger && !clicked_menu {
                    close_menu();
                }
            }
        });

        on_cleanup(move || {
            resize_handle.remove();
            scroll_handle.remove();
            pointer_handle.remove();
        });
    });

    view! {
        <div class=class_name>
            <button
                node_ref=trigger_ref
                type="button"
                class=dropdown_trigger_class_name(variant, size, disabled)
                aria-expanded=move || if is_open.get() { "true" } else { "false" }
                aria-haspopup="menu"
                disabled=disabled
                on:click=move |_| {
                    if is_open.get() {
                        close_menu();
                    } else {
                        open_menu();
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
                        "Enter" | " " => {
                            event.prevent_default();
                            if is_open.get() {
                                select_active_item();
                            } else {
                                open_menu();
                            }
                        }
                        "Escape" => {
                            if is_open.get() {
                                event.prevent_default();
                                close_menu();
                            }
                        }
                        _ => {}
                    }
                }
            >
                {icon.map(|icon_name| view! { <Icon name=icon_name size=size label=label.clone()/> })}
                <span>{label.clone()}</span>
                <span class="birei-dropdown-button__divider" aria-hidden="true"></span>
                <span class="birei-dropdown-button__caret" aria-hidden="true">
                    <Icon name="chevron-down" size=Size::Small/>
                </span>
            </button>

            {move || {
                is_open.get().then(|| {
                    view! {
                        <Portal>
                            {move || {
                                let items = items_list();
                                let current_active = active_index.get();

                                view! {
                                    <div
                                        class=move || {
                                            let layout = menu_layout.get();
                                            if layout.open_upward {
                                                "birei-dropdown-button__menu birei-dropdown-button__menu--upward"
                                            } else {
                                                "birei-dropdown-button__menu"
                                            }
                                        }
                                        style=move || {
                                            let layout = menu_layout.get();
                                            let theme = menu_theme.get();
                                            format!(
                                                "left: {}px; top: {}px; width: {}px; max-height: {}px; {}",
                                                layout.left, layout.top, layout.width, layout.max_height, theme.style
                                            )
                                        }
                                        node_ref=menu_ref
                                        role="menu"
                                    >
                                        {items
                                            .into_iter()
                                            .enumerate()
                                                .map(|(item_index, item)| {
                                                    let item_for_select = item.clone();
                                                    let item_label = item.label;
                                                    let item_icon = item.icon;
                                                    let item_disabled = item.disabled;
                                                    let is_active = current_active == Some(item_index);

                                                view! {
                                                    <DropdownMenuItem
                                                        item_index=item_index
                                                        label=item_label
                                                        icon=item_icon
                                                        disabled=item_disabled
                                                        active=is_active
                                                        on_hover=Callback::new(move |_| {
                                                            if !item_disabled {
                                                                active_index.set(Some(item_index));
                                                            }
                                                        })
                                                            on_select=Callback::new(move |_| {
                                                                select_item(&item_for_select);
                                                            })
                                                        />
                                                    }
                                            })
                                            .collect_view()}
                                    </div>
                                }
                            }}
                        </Portal>
                    }
                })
            }}
        </div>
    }
}

fn dropdown_trigger_class_name(variant: ButtonVariant, size: Size, disabled: bool) -> String {
    let mut classes = vec![
        "birei-button",
        variant.class_name(),
        size.button_class_name(),
        "birei-dropdown-button__trigger",
    ];
    if disabled {
        classes.push("birei-button--disabled");
    }
    classes.join(" ")
}

#[component]
fn DropdownMenuItem(
    item_index: usize,
    label: String,
    icon: Option<IcnName>,
    disabled: bool,
    active: bool,
    on_hover: Callback<()>,
    on_select: Callback<()>,
) -> impl IntoView {
    view! {
        <button
            type="button"
            data-dropdown-index=item_index.to_string()
            class=dropdown_item_class_name(active, disabled)
            role="menuitem"
            disabled=disabled
            on:mousedown=move |event: ev::MouseEvent| {
                event.prevent_default();
            }
            on:mouseenter=move |_| on_hover.run(())
            on:click=move |_| on_select.run(())
        >
            <span class="birei-dropdown-button__item-content">
                {icon.map(|icon_name| view! { <Icon name=icon_name size=Size::Small label=label.clone()/> })}
                <span>{label}</span>
            </span>
        </button>
    }
}

fn dropdown_item_class_name(active: bool, disabled: bool) -> String {
    let mut classes = String::from("birei-dropdown-button__item");
    if active {
        classes.push_str(" birei-dropdown-button__item--active");
    }
    if disabled {
        classes.push_str(" birei-dropdown-button__item--disabled");
    }
    classes
}

fn update_dropdown_menu_state(
    trigger_ref: &NodeRef<html::Button>,
    menu_layout: RwSignal<FloatingPopupLayout>,
    menu_theme: RwSignal<DropdownMenuTheme>,
) {
    let Some(trigger) = trigger_ref.get() else {
        return;
    };
    let rect = trigger.get_bounding_client_rect();
    menu_layout.set(measure_floating_popup_layout(&rect));

    if let Some(window) = web_sys::window() {
        if let Ok(Some(computed_style)) = window.get_computed_style(&trigger) {
            menu_theme.set(DropdownMenuTheme {
                style: dropdown_menu_theme_style(&computed_style),
            });
        }
    }
}

fn find_dropdown_item_element(menu: &HtmlElement, option_index: usize) -> Option<HtmlElement> {
    menu.query_selector(&format!(r#"[data-dropdown-index="{option_index}"]"#))
        .ok()
        .flatten()
        .and_then(|element| element.dyn_into::<HtmlElement>().ok())
}

fn sync_dropdown_menu_scroll(menu: &HtmlElement, option: &HtmlElement) {
    let option_top = option.offset_top();
    let option_bottom = option_top + option.offset_height();
    let view_top = menu.scroll_top();
    let view_bottom = view_top + menu.client_height();

    if option_top - FLOATING_POPUP_EDGE_PADDING < view_top {
        menu.set_scroll_top((option_top - FLOATING_POPUP_EDGE_PADDING).max(0));
    } else if option_bottom + FLOATING_POPUP_EDGE_PADDING > view_bottom {
        menu.set_scroll_top(option_bottom + FLOATING_POPUP_EDGE_PADDING - menu.client_height());
    }
}

fn first_enabled_item_index(items: &[MenuButtonItem]) -> Option<usize> {
    items.iter().position(|item| !item.disabled)
}

fn next_enabled_dropdown_index(
    items: &[MenuButtonItem],
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
