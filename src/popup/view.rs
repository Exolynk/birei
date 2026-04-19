use leptos::ev;
use leptos::html;
use leptos::portal::Portal;
use leptos::prelude::*;
use wasm_bindgen::JsCast;
use web_sys::KeyboardEvent;

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
    on_open_change: Option<Callback<bool>>,
    /// Optional custom footer content, typically action buttons.
    #[prop(optional, into)]
    actions: Option<ViewFn>,
    /// Additional class names applied to the popup panel.
    #[prop(optional, into)]
    class: Option<String>,
) -> impl IntoView {
    let panel_ref = NodeRef::<html::Div>::new();

    let request_close = Callback::new(move |_| {
        if let Some(on_open_change) = on_open_change.as_ref() {
            on_open_change.run(false);
        }
    });

    Effect::new(move |_| {
        if !open.get() {
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

    Effect::new(move |_| {
        if !open.get() {
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

    Effect::new(move |_| {
        if !open.get() {
            return;
        }

        let Some(panel) = panel_ref.get() else {
            return;
        };

        let _ = panel.focus();
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

    view! {
        {move || {
            let panel_class = panel_class.clone();
            let header_text = header_text.clone();
            let aria_label = aria_label.clone();
            let actions = actions.clone();
            let children = children.clone();

            open.get().then(move || {
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
