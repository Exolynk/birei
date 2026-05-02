use leptos::ev;
use leptos::html;
use leptos::portal::Portal;
use leptos::prelude::*;
use wasm_bindgen::JsCast;
use web_sys::KeyboardEvent;

use crate::common::FloatingPopupLayout;
use crate::{Icon, Popup, Size};

const DESKTOP_POPUP_WIDTH: f64 = 560.0;
const DESKTOP_POPUP_HEIGHT: f64 = 520.0;
const DESKTOP_POPUP_EDGE_PADDING: f64 = 20.0;

/// Composable top navigation menu with left branding, centered command area,
/// and right-side popup actions.
#[component]
pub fn TopMenuShell(
    /// Left logo slot. Hidden on mobile.
    #[prop(optional, into)]
    logo: Option<ViewFn>,
    /// Left text/title slot shown next to the logo. Hidden on mobile.
    #[prop(optional, into)]
    title: Option<ViewFn>,
    /// Center slot, typically a command palette trigger or input.
    #[prop(optional, into)]
    command: Option<ViewFn>,
    /// Popup body slot, usually action cards with user-defined callbacks.
    #[prop(optional, into)]
    actions_content: Option<ViewFn>,
    /// Optional custom trigger replacing the default menu icon button.
    #[prop(optional, into)]
    trigger: Option<ViewFn>,
    /// Desktop popup width in pixels.
    #[prop(optional, default = DESKTOP_POPUP_WIDTH)]
    popup_width: f64,
    /// Desktop popup height in pixels, capped by available viewport space.
    #[prop(optional, default = DESKTOP_POPUP_HEIGHT)]
    popup_height: f64,
    /// Keeps the menu visible at the top of the viewport.
    #[prop(optional, default = true)]
    sticky: bool,
    /// Optional extra classes for the menu root.
    #[prop(optional, into)]
    class: Option<String>,
) -> impl IntoView {
    let open = RwSignal::new(false);
    let is_mobile = RwSignal::new(false);
    let desktop_popup_layout = RwSignal::new(FloatingPopupLayout::default());
    let trigger_ref = NodeRef::<html::Div>::new();
    let popup_ref = NodeRef::<html::Div>::new();
    let actions_content = StoredValue::new(actions_content);
    let trigger = StoredValue::new(trigger);

    let menu_class = move || {
        let mut classes = vec!["birei-top-menu-shell"];
        if sticky {
            classes.push("birei-top-menu-shell--sticky");
        }
        if let Some(custom) = class.as_deref() {
            classes.push(custom);
        }
        classes.join(" ")
    };

    let sync_mobile_state = move || {
        let mobile = web_sys::window()
            .and_then(|window| window.inner_width().ok())
            .and_then(|value| value.as_f64())
            .is_some_and(|width| width <= 640.0);
        is_mobile.set(mobile);
    };

    let update_popup_layout = move || {
        let Some(trigger) = trigger_ref.get() else {
            return;
        };
        let rect = trigger.get_bounding_client_rect();
        let viewport_width = web_sys::window()
            .and_then(|window| window.inner_width().ok())
            .and_then(|value| value.as_f64())
            .unwrap_or(rect.left() + popup_width + DESKTOP_POPUP_EDGE_PADDING);
        let viewport_height = web_sys::window()
            .and_then(|window| window.inner_height().ok())
            .and_then(|value| value.as_f64())
            .unwrap_or(rect.bottom() + popup_height + DESKTOP_POPUP_EDGE_PADDING);
        let gap = 7.2_f64;
        let edge_padding = DESKTOP_POPUP_EDGE_PADDING;
        let available_below = (viewport_height - rect.bottom() - gap - edge_padding).max(0.0);
        let available_above = (rect.top() - gap - edge_padding).max(0.0);
        let open_upward = available_below < popup_height && available_above > available_below;
        let max_height = if open_upward {
            available_above
        } else {
            available_below
        }
        .max(96.0);
        let rendered_height = popup_height.min(max_height);
        let left = rect.left().clamp(
            DESKTOP_POPUP_EDGE_PADDING,
            (viewport_width - popup_width - DESKTOP_POPUP_EDGE_PADDING)
                .max(DESKTOP_POPUP_EDGE_PADDING),
        );
        desktop_popup_layout.set(FloatingPopupLayout {
            top: if open_upward {
                rect.top() - gap - rendered_height
            } else {
                rect.bottom() + gap
            },
            left,
            width: popup_width,
            max_height,
            open_upward,
        });
    };

    Effect::new(move |_| {
        sync_mobile_state();
        let resize_handle = window_event_listener_untyped("resize", move |_| sync_mobile_state());
        on_cleanup(move || resize_handle.remove());
    });

    Effect::new(move |_| {
        if !open.get() || is_mobile.get() {
            return;
        }

        update_popup_layout();
        let resize_handle = window_event_listener_untyped("resize", move |_| update_popup_layout());
        let scroll_handle = window_event_listener_untyped("scroll", move |_| update_popup_layout());

        let pointer_handle = window_event_listener_untyped("pointerdown", move |event| {
            let Some(target) = event
                .target()
                .and_then(|target| target.dyn_into::<web_sys::Node>().ok())
            else {
                return;
            };

            let clicked_trigger = trigger_ref
                .get()
                .is_some_and(|trigger| trigger.contains(Some(&target)));
            let clicked_popup = popup_ref
                .get()
                .is_some_and(|popup| popup.contains(Some(&target)));
            if !clicked_trigger && !clicked_popup {
                open.set(false);
            }
        });

        let keydown_handle = window_event_listener_untyped("keydown", move |event| {
            let Some(event) = event.dyn_into::<KeyboardEvent>().ok() else {
                return;
            };
            if event.key() == "Escape" {
                event.prevent_default();
                open.set(false);
            }
        });

        on_cleanup(move || {
            resize_handle.remove();
            scroll_handle.remove();
            pointer_handle.remove();
            keydown_handle.remove();
        });
    });

    view! {
        <header class=menu_class>
            <div class="birei-top-menu">
                <div class="birei-top-menu__left">
                    {logo.as_ref().map(|logo| {
                        view! { <div class="birei-top-menu__logo">{logo.run()}</div> }
                    })}
                    {title.as_ref().map(|title| {
                        view! { <div class="birei-top-menu__title">{title.run()}</div> }
                    })}
                </div>

                <div class="birei-top-menu__center">
                    {command.as_ref().map(|command| command.run())}
                </div>

                <div class="birei-top-menu__right" node_ref=trigger_ref>
                    <button
                        class="birei-top-menu__trigger"
                        type="button"
                        aria-label="Open actions"
                        aria-haspopup="menu"
                        aria-expanded=move || if open.get() { "true" } else { "false" }
                        on:click=move |_| open.update(|value| *value = !*value)
                    >
                        {move || {
                            trigger
                                .get_value()
                                .as_ref()
                                .map(|trigger| trigger.run())
                                .unwrap_or_else(|| {
                                    view! { <Icon name="menu" size=Size::Large/> }.into_any()
                                })
                        }}
                    </button>
                </div>
            </div>

            {move || {
                (open.get() && is_mobile.get()).then(|| {
                    view! {
                        <Popup
                            open=open
                            on_open_change=Callback::new(move |next_open| open.set(next_open))
                            class="birei-top-menu__popup"
                        >
                            {move || {
                                if let Some(content) = actions_content.get_value().as_ref() {
                                    content.run()
                                } else {
                                    view! { <div class="birei-top-menu__empty-actions"></div> }.into_any()
                                }
                            }}
                        </Popup>
                    }
                })
            }}

            {move || {
                (open.get() && !is_mobile.get()).then(|| {
                    view! {
                        <Portal>
                            <div
                                node_ref=popup_ref
                                class=move || {
                                    let mut classes = String::from("birei-top-menu__desktop-popup");
                                    if desktop_popup_layout.get().open_upward {
                                        classes.push_str(" birei-top-menu__desktop-popup--upward");
                                    }
                                    classes
                                }
                                style=move || {
                                    let layout = desktop_popup_layout.get();
                                    format!(
                                        "left: {}px; top: {}px; width: {}px; height: min({}px, {}px); max-height: min({}px, {}px);",
                                        layout.left,
                                        layout.top,
                                        popup_width,
                                        popup_height,
                                        layout.max_height,
                                        popup_height,
                                        layout.max_height
                                    )
                                }
                                role="menu"
                                on:mousedown=move |event: ev::MouseEvent| event.stop_propagation()
                            >
                                {move || {
                                    actions_content
                                        .get_value()
                                        .as_ref()
                                        .map(|content| content.run())
                                        .unwrap_or_else(|| {
                                            view! { <div class="birei-top-menu__empty-actions"></div> }
                                                .into_any()
                                        })
                                }}
                            </div>
                        </Portal>
                    }
                })
            }}
        </header>
    }
}
