use leptos::ev;
use leptos::html;
use leptos::portal::Portal;
use leptos::prelude::*;
use wasm_bindgen::JsCast;
use web_sys::{HtmlElement, KeyboardEvent};

use super::ButtonMenuItem;
use crate::common::{
    measure_floating_popup_layout, FloatingPopupLayout, FLOATING_POPUP_EDGE_PADDING,
};
use crate::{ButtonVariant, IcnName, Icon, Size};

/// Button-triggered popup action menu.
#[component]
pub fn ButtonMenu(
    /// Visible button label.
    #[prop(into)]
    label: String,
    /// Menu items rendered in the popup.
    #[prop(into)]
    items: MaybeProp<Vec<ButtonMenuItem>>,
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
    /// Matches the popup width to the trigger width.
    #[prop(optional, default = true)]
    match_trigger_width: bool,
    /// Callback fired with the selected item value.
    #[prop(optional)]
    on_select: Option<Callback<String>>,
) -> impl IntoView {
    // DOM refs drive popup placement, outside-click detection, and keyboard
    // scrolling of the active menu item.
    let trigger_ref = NodeRef::<html::Button>::new();
    let menu_ref = NodeRef::<html::Div>::new();
    let is_open = RwSignal::new(false);
    let active_index = RwSignal::new(None::<usize>);
    let menu_layout = RwSignal::new(FloatingPopupLayout::default());
    let scroll_request = RwSignal::new(0_u64);
    let items_list = move || items.get().unwrap_or_default();
    let ripple_style = RwSignal::new(String::from(
        "--birei-ripple-x: 50%; --birei-ripple-y: 50%; --birei-ripple-size: 0px;",
    ));
    let ripple_phase = RwSignal::new(None::<bool>);

    // Wrapper classes only expose the optional caller hook class; trigger and
    // menu styling are handled by dedicated helpers below.
    let class_name = move || {
        let mut classes = vec!["birei-dropdown-button"];
        if let Some(class) = class.as_deref() {
            classes.push(class);
        }
        classes.join(" ")
    };
    // The trigger reuses button ripple animation classes so repeated opens
    // still animate correctly.
    let trigger_class_name = move || {
        let mut classes = dropdown_trigger_class_name(variant, size, disabled);

        if let Some(phase) = ripple_phase.get() {
            classes.push(' ');
            classes.push_str(if phase {
                "birei-button--ripple-a"
            } else {
                "birei-button--ripple-b"
            });
        }

        classes
    };

    // Opening the menu should always land on a valid enabled item so keyboard
    // selection works immediately.
    let sync_active_index = move || {
        let next_active = active_index
            .get()
            .filter(|index| items_list().get(*index).is_some_and(|item| !item.disabled))
            .or_else(|| first_enabled_item_index(&items_list()));
        active_index.set(next_active);
    };

    // After menu interactions finish, focus returns to the trigger for good
    // keyboard continuity.
    let focus_trigger = move || {
        if let Some(button) = trigger_ref.get() {
            let _ = button.focus();
        }
    };

    // Opening performs all state initialization in one place: guard disabled
    // state, open the popup, choose an active item, and sync placement.
    let open_menu = move || {
        if disabled {
            return;
        }

        is_open.set(true);
        sync_active_index();
        scroll_request.update(|value| *value += 1);
        update_dropdown_menu_state(&trigger_ref, menu_layout);
    };

    // Closing always clears transient menu state.
    let close_menu = move || {
        is_open.set(false);
        active_index.set(None);
    };

    // Selection is centralized so direct clicks and keyboard activation share
    // disabled handling, callback emission, and focus restoration.
    let select_item = move |item: &ButtonMenuItem| {
        if item.disabled {
            return;
        }

        close_menu();
        if let Some(on_select) = on_select.as_ref() {
            on_select.run(item.value.clone());
        }
        focus_trigger();
    };

    // Arrow-key navigation moves between enabled items only and requests the
    // popup scroll effect to reveal the new active option.
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

    // Enter/space activation resolves the currently active item.
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

    // When the active index changes, keep the corresponding menu option in
    // view inside the scrollable popup.
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

    // While open, the menu tracks viewport changes and outside pointer events
    // so its floating position and dismissal behavior stay correct.
    Effect::new(move |_| {
        if !is_open.get() {
            return;
        }

        update_dropdown_menu_state(&trigger_ref, menu_layout);

        let resize_handle = window_event_listener_untyped("resize", {
            move |_| update_dropdown_menu_state(&trigger_ref, menu_layout)
        });
        let scroll_handle = window_event_listener_untyped("scroll", {
            move |_| update_dropdown_menu_state(&trigger_ref, menu_layout)
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
                class=trigger_class_name
                style=move || ripple_style.get()
                aria-expanded=move || if is_open.get() { "true" } else { "false" }
                aria-haspopup="menu"
                disabled=disabled
                on:click=move |event: ev::MouseEvent| {
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
                        "Escape" if is_open.get() => {
                            event.prevent_default();
                            close_menu();
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
                                            let mut classes = String::from("birei-dropdown-button__menu");
                                            if layout.open_upward {
                                                classes.push_str(" birei-dropdown-button__menu--upward");
                                            }
                                            if !match_trigger_width {
                                                classes.push_str(" birei-dropdown-button__menu--content-width");
                                            }
                                            classes
                                        }
                                        style=move || {
                                            let layout = menu_layout.get();
                                            if match_trigger_width {
                                                format!(
                                                    "left: {}px; top: {}px; width: {}px; max-height: {}px;",
                                                    layout.left, layout.top, layout.width, layout.max_height
                                                )
                                            } else {
                                                format!(
                                                    "left: {}px; top: {}px; max-height: {}px;",
                                                    layout.left, layout.top, layout.max_height
                                                )
                                            }
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

/// Builds the trigger classes shared by the visible trigger and other
/// dropdown-style buttons elsewhere in the library.
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

/// Internal menu item view with hover-to-activate and click-to-select wiring.
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

/// Builds classes for one menu item based on active and disabled state.
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

/// Recomputes the floating popup layout from the trigger's current viewport
/// position.
fn update_dropdown_menu_state(
    trigger_ref: &NodeRef<html::Button>,
    menu_layout: RwSignal<FloatingPopupLayout>,
) {
    let Some(trigger) = trigger_ref.get() else {
        return;
    };
    let rect = trigger.get_bounding_client_rect();
    menu_layout.set(measure_floating_popup_layout(&rect));
}

/// Locates a rendered menu item element by its logical item index.
fn find_dropdown_item_element(menu: &HtmlElement, option_index: usize) -> Option<HtmlElement> {
    menu.query_selector(&format!(r#"[data-dropdown-index="{option_index}"]"#))
        .ok()
        .flatten()
        .and_then(|element| element.dyn_into::<HtmlElement>().ok())
}

/// Keeps the active menu item inside the scroll viewport with the same edge
/// padding used by other floating menus in the library.
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

/// Finds the first enabled menu item, used when opening the menu.
fn first_enabled_item_index(items: &[ButtonMenuItem]) -> Option<usize> {
    items.iter().position(|item| !item.disabled)
}

/// Moves to the next enabled menu item in the requested direction, wrapping
/// around the item list when needed.
fn next_enabled_dropdown_index(
    items: &[ButtonMenuItem],
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
