use std::sync::atomic::{AtomicUsize, Ordering};

use leptos::html;
use leptos::portal::Portal;
use leptos::prelude::*;
use wasm_bindgen::JsCast;
use web_sys::KeyboardEvent;

use crate::common::{measure_tooltip_layout, FloatingTooltipLayout, TooltipPlacement};

static TOOLTIP_ID_COUNTER: AtomicUsize = AtomicUsize::new(1);

#[derive(Clone)]
struct TooltipState {
    layout: RwSignal<FloatingTooltipLayout>,
    open_timeout: RwSignal<Option<i32>>,
}

/// Delayed hover/focus tooltip with plain-text content.
#[component]
pub fn Tooltip(
    /// Element that owns the tooltip interaction.
    children: Children,
    /// Plain-text tooltip content.
    #[prop(into)]
    content: String,
    /// Preferred tooltip placement.
    #[prop(optional)]
    placement: TooltipPlacement,
    /// Delay before opening on hover in milliseconds.
    #[prop(optional, default = 1000)]
    delay_ms: i32,
    /// Additional class names applied to the trigger wrapper.
    #[prop(optional, into)]
    class: Option<String>,
) -> impl IntoView {
    let trigger_ref = NodeRef::<html::Span>::new();
    let tooltip_ref = NodeRef::<html::Div>::new();
    let is_open = RwSignal::new(false);
    let tooltip_id = format!(
        "birei-tooltip-{}",
        TOOLTIP_ID_COUNTER.fetch_add(1, Ordering::Relaxed)
    );
    let tooltip_id_for_aria = tooltip_id.clone();
    let tooltip_id_for_popup = tooltip_id.clone();
    let state = TooltipState {
        layout: RwSignal::new(FloatingTooltipLayout::default()),
        open_timeout: RwSignal::new(None),
    };

    let class_name = move || {
        let mut classes = vec!["birei-tooltip"];
        if let Some(class) = class.as_deref() {
            classes.push(class);
        }
        classes.join(" ")
    };

    let clear_pending_open = {
        let state = state.clone();
        move || {
            let Some(timeout_id) = state.open_timeout.get_untracked() else {
                return;
            };
            state.open_timeout.set(None);
            if let Some(window) = web_sys::window() {
                window.clear_timeout_with_handle(timeout_id);
            }
        }
    };

    let close_tooltip = {
        move || {
            clear_pending_open();
            is_open.set(false);
        }
    };

    let update_tooltip_layout = {
        let state = state.clone();
        move || {
            let Some(trigger) = trigger_ref.get() else {
                return;
            };
            let Some(tooltip) = tooltip_ref.get() else {
                return;
            };

            let trigger_rect = trigger.get_bounding_client_rect();
            let tooltip_width = f64::from(tooltip.offset_width());
            let tooltip_height = f64::from(tooltip.offset_height());
            state.layout.set(measure_tooltip_layout(
                &trigger_rect,
                tooltip_width,
                tooltip_height,
                placement,
            ));
        }
    };

    let open_tooltip = {
        move || {
            clear_pending_open();
            is_open.set(true);
        }
    };

    let schedule_open = {
        let state = state.clone();
        move || {
            clear_pending_open();

            let Some(window) = web_sys::window() else {
                is_open.set(true);
                return;
            };

            let callback = wasm_bindgen::closure::Closure::once_into_js({
                let is_open = is_open;
                move || {
                    is_open.set(true);
                }
            });

            if let Ok(timeout_id) = window.set_timeout_with_callback_and_timeout_and_arguments_0(
                callback.unchecked_ref(),
                delay_ms,
            ) {
                state.open_timeout.set(Some(timeout_id));
            }
        }
    };

    on_cleanup(clear_pending_open);

    Effect::new(move |_| {
        if !is_open.get() {
            return;
        }

        let Some(_tooltip) = tooltip_ref.get() else {
            return;
        };

        update_tooltip_layout();

        let resize_handle =
            window_event_listener_untyped("resize", move |_| update_tooltip_layout());
        let scroll_handle =
            window_event_listener_untyped("scroll", move |_| update_tooltip_layout());

        on_cleanup(move || {
            resize_handle.remove();
            scroll_handle.remove();
        });
    });

    view! {
        <span
            class=class_name
            node_ref=trigger_ref
            tabindex="0"
            aria-describedby=move || if is_open.get() { tooltip_id_for_aria.clone() } else { String::new() }
            on:pointerenter=move |_| schedule_open()
            on:pointerleave=move |_| close_tooltip()
            on:focusin=move |_| open_tooltip()
            on:focusout=move |_| close_tooltip()
            on:keydown=move |event: KeyboardEvent| {
                if event.key() == "Escape" && is_open.get() {
                    event.prevent_default();
                    close_tooltip();
                }
            }
        >
            {children()}
            {move || {
                let tooltip_content = content.clone();
                let tooltip_id = tooltip_id_for_popup.clone();
                is_open.get().then(|| {
                    view! {
                        <Portal>
                            <div
                                id=tooltip_id.clone()
                                class=move || {
                                    match state.layout.get().placement {
                                        TooltipPlacement::Top => "birei-tooltip__popup birei-tooltip__popup--top",
                                        TooltipPlacement::Bottom => "birei-tooltip__popup birei-tooltip__popup--bottom",
                                        TooltipPlacement::Left => "birei-tooltip__popup birei-tooltip__popup--left",
                                        TooltipPlacement::Right => "birei-tooltip__popup birei-tooltip__popup--right",
                                    }
                                }
                                style=move || {
                                    let current = state.layout.get();
                                    format!("left: {}px; top: {}px;", current.left, current.top)
                                }
                                node_ref=tooltip_ref
                                role="tooltip"
                            >
                                {tooltip_content.clone()}
                            </div>
                        </Portal>
                    }
                })
            }}
        </span>
    }
}
