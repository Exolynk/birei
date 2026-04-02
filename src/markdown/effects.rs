use std::cell::RefCell;
use std::rc::Rc;

use leptos::html;
use leptos::prelude::*;
use wasm_bindgen::JsCast;
use web_sys::{Node, Range};

use crate::common::{measure_floating_popup_layout, FloatingPopupLayout};

pub(crate) fn setup_link_popup_effects(
    link_popup_open: RwSignal<bool>,
    link_input_ref: NodeRef<html::Input>,
    link_popup_layout: RwSignal<FloatingPopupLayout>,
    saved_range: Rc<RefCell<Option<Range>>>,
    close_link_popup: Rc<dyn Fn()>,
) {
    let saved_range_for_link_resize = Rc::clone(&saved_range);
    let saved_range_for_link_scroll = Rc::clone(&saved_range);
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

                let clicked_popup = link_input_ref
                    .get()
                    .and_then(|input| input.closest(".birei-markdown__link-popup").ok().flatten())
                    .is_some_and(|popup| popup.contains(Some(&target)));

                if !clicked_popup {
                    close_link_popup();
                }
            }
        });
        let resize_handle = window_event_listener_untyped("resize", {
            let saved_range = Rc::clone(&saved_range_for_link_resize);
            move |_| {
                if let Some(range) = saved_range.borrow().clone() {
                    link_popup_layout.set(measure_floating_popup_layout(
                        &range.get_bounding_client_rect(),
                    ));
                }
            }
        });
        let scroll_handle = window_event_listener_untyped("scroll", {
            let saved_range = Rc::clone(&saved_range_for_link_scroll);
            move |_| {
                if let Some(range) = saved_range.borrow().clone() {
                    link_popup_layout.set(measure_floating_popup_layout(
                        &range.get_bounding_client_rect(),
                    ));
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

pub(crate) fn setup_heading_popup_effects(
    heading_popup_open: RwSignal<bool>,
    heading_button_ref: NodeRef<html::Button>,
    heading_popup_ref: NodeRef<html::Div>,
    heading_popup_layout: RwSignal<FloatingPopupLayout>,
    close_heading_popup: Rc<dyn Fn()>,
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
        let resize_handle = window_event_listener_untyped("resize", move |_| {
            if let Some(button) = heading_button_ref.get() {
                heading_popup_layout.set(measure_floating_popup_layout(
                    &button.get_bounding_client_rect(),
                ));
            }
        });
        let scroll_handle = window_event_listener_untyped("scroll", move |_| {
            if let Some(button) = heading_button_ref.get() {
                heading_popup_layout.set(measure_floating_popup_layout(
                    &button.get_bounding_client_rect(),
                ));
            }
        });

        on_cleanup(move || {
            outside_click.remove();
            resize_handle.remove();
            scroll_handle.remove();
        });
    });
}

pub(crate) fn setup_table_popup_effects(
    table_popup_open: RwSignal<bool>,
    table_button_ref: NodeRef<html::Button>,
    table_popup_ref: NodeRef<html::Div>,
    table_popup_layout: RwSignal<FloatingPopupLayout>,
    close_table_popup: Rc<dyn Fn()>,
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
        let resize_handle = window_event_listener_untyped("resize", move |_| {
            if let Some(button) = table_button_ref.get() {
                table_popup_layout.set(measure_floating_popup_layout(
                    &button.get_bounding_client_rect(),
                ));
            }
        });
        let scroll_handle = window_event_listener_untyped("scroll", move |_| {
            if let Some(button) = table_button_ref.get() {
                table_popup_layout.set(measure_floating_popup_layout(
                    &button.get_bounding_client_rect(),
                ));
            }
        });

        on_cleanup(move || {
            outside_click.remove();
            resize_handle.remove();
            scroll_handle.remove();
        });
    });
}
