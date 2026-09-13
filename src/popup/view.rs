use crate::ArcOneCallback;
use leptos::ev;
use leptos::html;
use leptos::portal::Portal;
use leptos::prelude::*;
use wasm_bindgen::JsCast;
use web_sys::{Element, HtmlElement, KeyboardEvent};

use crate::{Button, ButtonVariant, Icon, Size};

/// Controlled modal popup with header, scrollable content, and custom footer actions.
#[component]
pub fn Popup(
    /// Body content rendered inside the popup.
    children: ChildrenFn,
    /// Whether the popup is currently open.
    #[prop(into)]
    open: Signal<bool>,
    /// Optional header text shown in the sticky popup header.
    #[prop(optional, into)]
    header: Option<String>,
    /// Optional callback fired when the popup requests an open-state change.
    #[prop(optional, into)]
    on_open_change: Option<ArcOneCallback<bool>>,
    /// Optional custom footer content, typically action buttons.
    #[prop(optional, into)]
    actions: Option<ViewFn>,
    /// Additional class names applied to the popup panel.
    #[prop(optional, into)]
    class: Option<String>,
) -> impl IntoView {
    let panel_ref = NodeRef::<html::Div>::new();
    let popup_open = ArcRwSignal::new(open.get_untracked());
    let previous_focus = RwSignal::new(None::<HtmlElement>);

    let synchronized_open = popup_open.clone();
    Effect::new(move |_| synchronized_open.set(open.get()));

    let cleanup_open = popup_open.clone();
    on_cleanup(move || cleanup_open.set(false));

    let request_close = Callback::new(move |_| {
        if let Some(on_open_change) = on_open_change.as_ref() {
            on_open_change.run(false);
        }
    });

    let keyboard_open = popup_open.clone();
    Effect::new(move |_| {
        if !keyboard_open.get() {
            return;
        }

        let request_close = request_close;
        let keydown_handle = window_event_listener_untyped("keydown", move |event| {
            let Some(event) = event.dyn_into::<KeyboardEvent>().ok() else {
                return;
            };

            if event.key() == "Escape" {
                event.prevent_default();
                request_close.run(());
            }
        });

        on_cleanup(move || {
            keydown_handle.remove();
        });
    });

    let trap_panel_ref = panel_ref;
    let trap_focus = StoredValue::new(move |event: KeyboardEvent| {
        if event.key() != "Tab" {
            return;
        }

        let Some(panel) = trap_panel_ref.get() else {
            return;
        };
        trap_tab_focus(&panel.unchecked_into::<Element>(), &event);
    });

    let scroll_open = popup_open.clone();
    Effect::new(move |_| {
        if !scroll_open.get() {
            return;
        }

        let Some(document) = web_sys::window().and_then(|window| window.document()) else {
            return;
        };
        let Some(body) = document.body() else {
            return;
        };

        let previous_overflow = body
            .style()
            .get_property_value("overflow")
            .unwrap_or_default();
        let _ = body.style().set_property("overflow", "hidden");

        on_cleanup(move || {
            let _ = body.style().set_property("overflow", &previous_overflow);
        });
    });

    let focus_open = popup_open.clone();
    let focus_panel_ref = panel_ref;
    Effect::new(move |_| {
        if !focus_open.get() {
            if let Some(previous) = previous_focus.get_untracked() {
                if previous.is_connected() {
                    let _ = previous.focus();
                }
            }
            previous_focus.set(None);
            return;
        }

        previous_focus.set(active_html_element());
        request_animation_frame(move || {
            if let Some(panel) = focus_panel_ref.get() {
                focus_initial_element(&panel.unchecked_into::<Element>());
            }
        });
    });

    let panel_class = {
        let mut classes = vec!["birei-popup"];
        if let Some(class) = class.as_deref() {
            classes.push(class);
        }
        classes.join(" ")
    };
    let header_text = header;
    let aria_label = header_text.clone().unwrap_or_else(|| String::from("Popup"));
    let rendered_open = popup_open;

    view! {
        {move || {
            let panel_class = panel_class.clone();
            let header_text = header_text.clone();
            let aria_label = aria_label.clone();
            let actions = actions.clone();
            let children = children.clone();

            rendered_open.get().then(move || {
                let request_close_backdrop = request_close;
                let request_close_button = request_close;

                view! {
                    <Portal>
                        <div
                            class="birei-popup-backdrop"
                            role="presentation"
                            on:pointerdown=move |_| request_close_backdrop.run(())
                        >
                            <div
                                node_ref=panel_ref
                                class=panel_class.clone()
                                role="dialog"
                                aria-modal="true"
                                aria-label=aria_label.clone()
                                tabindex="-1"
                                on:pointerdown=move |event: ev::PointerEvent| event.stop_propagation()
                                on:keydown=move |event| {
                                    trap_focus.with_value(|trap_focus| trap_focus(event))
                                }
                            >
                                <div class="birei-popup__header">
                                    <div class="birei-popup__header-copy">
                                        {header_text.as_ref().map(|header| {
                                            view! { <h3 class="birei-popup__title">{header.clone()}</h3> }
                                        })}
                                    </div>
                                    <Button
                                        class="birei-popup__close"
                                        variant=ButtonVariant::Transparent
                                        size=Size::Small
                                        circle=true
                                        on_click=Callback::new(move |_| request_close_button.run(()))
                                    >
                                        <Icon name="x" size=Size::Small/>
                                    </Button>
                                </div>

                                <div class="birei-popup__body">{children()}</div>

                                {actions.as_ref().map(|actions| {
                                    view! {
                                        <div class="birei-popup__actions">
                                            {actions.run()}
                                        </div>
                                    }
                                })}
                            </div>
                        </div>
                    </Portal>
                }
            })
        }}
    }
}

/// Moves focus to the first enabled form control in the popup, if present.
fn focus_initial_element(panel: &Element) {
    if focus_matching(panel, FORM_CONTROL_SELECTOR) || focus_matching(panel, FOCUSABLE_SELECTOR) {
        return;
    }

    if let Ok(panel) = panel.clone().dyn_into::<HtmlElement>() {
        let _ = panel.focus();
    }
}

/// Cycles keyboard focus through the popup instead of allowing it to reach the page behind it.
fn trap_tab_focus(panel: &Element, event: &KeyboardEvent) {
    let focusable = focusable_elements(panel);
    if focusable.is_empty() {
        event.prevent_default();
        event.stop_propagation();
        focus_initial_element(panel);
        return;
    }

    let active = web_sys::window()
        .and_then(|window| window.document())
        .and_then(|document| document.active_element());
    let active_index = active.as_ref().and_then(|active| {
        focusable
            .iter()
            .position(|element| element.is_same_node(Some(active)))
    });
    let next_index = match (event.shift_key(), active_index) {
        (true, Some(0)) | (true, None) => focusable.len() - 1,
        (true, Some(index)) => index - 1,
        (false, Some(index)) if index + 1 < focusable.len() => index + 1,
        (false, _) => 0,
    };

    event.prevent_default();
    event.stop_propagation();
    let _ = focusable[next_index].focus();
}

/// Returns visible, enabled elements that can receive keyboard focus inside the popup.
fn focusable_elements(panel: &Element) -> Vec<HtmlElement> {
    let Ok(nodes) = panel.query_selector_all(FOCUSABLE_SELECTOR) else {
        return Vec::new();
    };

    (0..nodes.length())
        .filter_map(|index| nodes.item(index))
        .filter_map(|node| node.dyn_into::<HtmlElement>().ok())
        .filter(|element| element.offset_width() > 0 || element.offset_height() > 0)
        .collect()
}

/// Focuses the first visible element matching the supplied selector.
fn focus_matching(panel: &Element, selector: &str) -> bool {
    let Ok(nodes) = panel.query_selector_all(selector) else {
        return false;
    };

    for index in 0..nodes.length() {
        let Some(node) = nodes.item(index) else {
            continue;
        };
        let Ok(element) = node.dyn_into::<HtmlElement>() else {
            continue;
        };
        if element.offset_width() > 0 || element.offset_height() > 0 {
            let _ = element.focus();
            return true;
        }
    }

    false
}

/// Returns the currently focused HTML element, when one exists.
fn active_html_element() -> Option<HtmlElement> {
    web_sys::window()
        .and_then(|window| window.document())
        .and_then(|document| document.active_element())
        .and_then(|element| element.dyn_into::<HtmlElement>().ok())
}

const FORM_CONTROL_SELECTOR: &str = concat!(
    "input:not([type=hidden]):not([disabled]), ",
    "select:not([disabled]), ",
    "textarea:not([disabled]), ",
    "[contenteditable=true]"
);
const FOCUSABLE_SELECTOR: &str = concat!(
    "button:not([disabled]), ",
    "[href], ",
    "input:not([type=hidden]):not([disabled]), ",
    "select:not([disabled]), ",
    "textarea:not([disabled]), ",
    "[contenteditable=true], ",
    "[tabindex]:not([tabindex='-1'])"
);
