use std::rc::Rc;

use leptos::html;
use leptos::prelude::*;
use wasm_bindgen::JsCast;
use web_sys::Node;

use crate::common::FloatingPopupLayout;

/// Installs the outside-click, resize, scroll, and focus behavior used by the
/// floating link editor popup.
pub(crate) fn setup_link_popup_effects(
    link_popup_open: RwSignal<bool>,
    link_button_ref: NodeRef<html::Button>,
    link_input_ref: NodeRef<html::Input>,
    link_popup_layout: RwSignal<FloatingPopupLayout>,
    close_link_popup: Rc<dyn Fn()>,
    measure_popup_layout: Rc<dyn Fn(&web_sys::DomRect) -> FloatingPopupLayout>,
) {
    Effect::new(move |_| {
        if !link_popup_open.get() {
            return;
        }

        let outside_click = window_event_listener_untyped("pointerdown", {
            let close_link_popup = Rc::clone(&close_link_popup);
            move |event| {
                let Some(target) = event
                    .target()
                    .and_then(|target| target.dyn_into::<Node>().ok())
                else {
                    return;
                };

                let clicked_inside = link_button_ref
                    .get()
                    .is_some_and(|button| button.contains(Some(&target)))
                    || link_input_ref
                        .get()
                        .and_then(|input| {
                            input.closest(".birei-markdown__link-popup").ok().flatten()
                        })
                        .is_some_and(|popup| popup.contains(Some(&target)));

                if !clicked_inside {
                    close_link_popup();
                }
            }
        });
        let resize_handle = window_event_listener_untyped("resize", {
            let measure_popup_layout = Rc::clone(&measure_popup_layout);
            move |_| {
                if let Some(button) = link_button_ref.get() {
                    link_popup_layout.set(measure_popup_layout(&button.get_bounding_client_rect()));
                }
            }
        });
        let scroll_handle = window_event_listener_untyped("scroll", {
            let measure_popup_layout = Rc::clone(&measure_popup_layout);
            move |_| {
                if let Some(button) = link_button_ref.get() {
                    link_popup_layout.set(measure_popup_layout(&button.get_bounding_client_rect()));
                }
            }
        });

        on_cleanup(move || {
            outside_click.remove();
            resize_handle.remove();
            scroll_handle.remove();
        });
    });

    Effect::new(move |_| {
        if !link_popup_open.get() {
            return;
        }

        if let Some(input) = link_input_ref.get() {
            let _ = input.focus();
        }
    });
}

/// Installs dismissal and layout-sync behavior for the heading menu popup.
pub(crate) fn setup_heading_popup_effects(
    heading_popup_open: RwSignal<bool>,
    heading_button_ref: NodeRef<html::Button>,
    heading_popup_ref: NodeRef<html::Div>,
    heading_popup_layout: RwSignal<FloatingPopupLayout>,
    close_heading_popup: Rc<dyn Fn()>,
    measure_popup_layout: Rc<dyn Fn(&web_sys::DomRect) -> FloatingPopupLayout>,
) {
    Effect::new(move |_| {
        if !heading_popup_open.get() {
            return;
        }

        let outside_click = window_event_listener_untyped("pointerdown", {
            let close_heading_popup = Rc::clone(&close_heading_popup);
            move |event| {
                let Some(target) = event
                    .target()
                    .and_then(|target| target.dyn_into::<Node>().ok())
                else {
                    return;
                };

                let clicked_inside = heading_button_ref
                    .get()
                    .is_some_and(|button| button.contains(Some(&target)))
                    || heading_popup_ref
                        .get()
                        .is_some_and(|popup| popup.contains(Some(&target)));

                if !clicked_inside {
                    close_heading_popup();
                }
            }
        });
        let resize_handle = window_event_listener_untyped("resize", {
            let measure_popup_layout = Rc::clone(&measure_popup_layout);
            move |_| {
                if let Some(button) = heading_button_ref.get() {
                    heading_popup_layout
                        .set(measure_popup_layout(&button.get_bounding_client_rect()));
                }
            }
        });
        let scroll_handle = window_event_listener_untyped("scroll", {
            let measure_popup_layout = Rc::clone(&measure_popup_layout);
            move |_| {
                if let Some(button) = heading_button_ref.get() {
                    heading_popup_layout
                        .set(measure_popup_layout(&button.get_bounding_client_rect()));
                }
            }
        });

        on_cleanup(move || {
            outside_click.remove();
            resize_handle.remove();
            scroll_handle.remove();
        });
    });
}

/// Installs dismissal and layout-sync behavior for the table action popup.
pub(crate) fn setup_table_popup_effects(
    table_popup_open: RwSignal<bool>,
    table_button_ref: NodeRef<html::Button>,
    table_popup_ref: NodeRef<html::Div>,
    table_popup_layout: RwSignal<FloatingPopupLayout>,
    close_table_popup: Rc<dyn Fn()>,
    measure_popup_layout: Rc<dyn Fn(&web_sys::DomRect) -> FloatingPopupLayout>,
) {
    Effect::new(move |_| {
        if !table_popup_open.get() {
            return;
        }

        let outside_click = window_event_listener_untyped("pointerdown", {
            let close_table_popup = Rc::clone(&close_table_popup);
            move |event| {
                let Some(target) = event
                    .target()
                    .and_then(|target| target.dyn_into::<Node>().ok())
                else {
                    return;
                };

                let clicked_popup = table_button_ref
                    .get()
                    .is_some_and(|button| button.contains(Some(&target)))
                    || table_popup_ref
                        .get()
                        .is_some_and(|popup| popup.contains(Some(&target)));

                if !clicked_popup {
                    close_table_popup();
                }
            }
        });
        let resize_handle = window_event_listener_untyped("resize", {
            let measure_popup_layout = Rc::clone(&measure_popup_layout);
            move |_| {
                if let Some(button) = table_button_ref.get() {
                    table_popup_layout
                        .set(measure_popup_layout(&button.get_bounding_client_rect()));
                }
            }
        });
        let scroll_handle = window_event_listener_untyped("scroll", {
            let measure_popup_layout = Rc::clone(&measure_popup_layout);
            move |_| {
                if let Some(button) = table_button_ref.get() {
                    table_popup_layout
                        .set(measure_popup_layout(&button.get_bounding_client_rect()));
                }
            }
        });

        on_cleanup(move || {
            outside_click.remove();
            resize_handle.remove();
            scroll_handle.remove();
        });
    });
}
