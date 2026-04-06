use leptos::prelude::*;
use wasm_bindgen::closure::Closure;
use wasm_bindgen::JsCast;

use crate::{Icon, Size};

/// Surface container with an optional collapsible header.
#[component]
pub fn Card(
    /// Body content rendered inside the card.
    children: Children,
    /// Optional header text. When present, the card becomes collapsible.
    #[prop(optional, into)]
    header: Option<String>,
    /// Initial collapsed state when a header is present.
    #[prop(optional)]
    collapsed: bool,
    /// Additional CSS class names applied to the root element.
    #[prop(optional, into)]
    class: Option<String>,
) -> impl IntoView {
    // A header opt-in turns the card into a collapsible disclosure surface.
    let collapsible = header.is_some();
    let collapsed = RwSignal::new(collapsible && collapsed);
    let body_wrap_ref = NodeRef::<leptos::html::Div>::new();
    let body_ref = NodeRef::<leptos::html::Div>::new();
    // The body animation is driven by inline styles so the component can
    // transition between `display: none`, measured height, and `auto`.
    let body_style = RwSignal::new(if collapsed.get_untracked() {
        String::from("display: none; height: 0px; opacity: 0; overflow: hidden;")
    } else {
        String::new()
    });

    // Root classes reflect collapse capability and current open state.
    let class_name = move || {
        let mut classes = vec!["birei-card"];

        if collapsible {
            classes.push("birei-card--collapsible");
        }
        if collapsed.get() {
            classes.push("birei-card--collapsed");
        }
        if let Some(class) = class.as_deref() {
            classes.push(class);
        }

        classes.join(" ")
    };

    // The collapse animation uses measured height to bridge between fully
    // collapsed and natural `auto` height without snapping.
    let animate_body = move |open: bool| {
        let Some(body_wrap) = body_wrap_ref.get_untracked() else {
            return;
        };

        if open {
            body_style.set(String::from(
                "display: block; height: 0px; opacity: 0; overflow: hidden;",
            ));
            run_on_next_frame(move || {
                let Some(body) = body_ref.get_untracked() else {
                    return;
                };
                let expanded_height = body.scroll_height();

                body_style.set(format!(
                    "display: block; height: {expanded_height}px; opacity: 1; overflow: hidden;"
                ));
            });
            run_after(240, {
                let body_style = body_style;
                move || {
                    body_style.set(String::from(
                        "display: block; height: auto; opacity: 1; overflow: visible;",
                    ));
                }
            });
        } else {
            let current_height = body_wrap.scroll_height();
            body_style.set(format!(
                "display: block; height: {current_height}px; opacity: 1; overflow: hidden;"
            ));
            run_on_next_frame({
                let body_style = body_style;
                move || {
                    body_style.set(String::from(
                        "display: block; height: 0px; opacity: 0; overflow: hidden;",
                    ));
                }
            });
            run_after(240, {
                let body_style = body_style;
                move || {
                    body_style.set(String::from(
                        "display: none; height: 0px; opacity: 0; overflow: hidden;",
                    ));
                }
            });
        }
    };

    view! {
        <div class=class_name>
            {header.as_ref().map(|header| {
                let header = header.clone();

                view! {
                    <button
                        type="button"
                        class="birei-card__header"
                        aria-expanded=move || if collapsed.get() { "false" } else { "true" }
                        on:click=move |_| {
                            let next_open = collapsed.get();
                            collapsed.update(|value| *value = !*value);
                            animate_body(next_open);
                        }
                    >
                        <span class="birei-card__header-main">
                            <span
                                class=move || {
                                    if collapsed.get() {
                                        "birei-card__header-icon"
                                    } else {
                                        "birei-card__header-icon birei-card__header-icon--open"
                                    }
                                }
                            >
                                <Icon name="chevron-right" size=Size::Small/>
                            </span>
                            <span class="birei-card__header-title">{header.clone()}</span>
                        </span>
                    </button>
                }
            })}
            <div class="birei-card__body-wrap" node_ref=body_wrap_ref style=move || body_style.get()>
                <div class="birei-card__body" node_ref=body_ref>
                    {children()}
                </div>
            </div>
        </div>
    }
}

/// Defers a callback to the next animation frame so layout can settle before
/// the component reads or writes animated height values.
fn run_on_next_frame(callback: impl FnOnce() + 'static) {
    let Some(window) = web_sys::window() else {
        return;
    };

    let callback = Closure::once_into_js(callback);
    let _ = window.request_animation_frame(callback.unchecked_ref());
}

/// Runs a one-shot timeout used to finish the collapse/expand transition and
/// restore the final steady-state body styles.
fn run_after(timeout_ms: i32, callback: impl FnOnce() + 'static) {
    let Some(window) = web_sys::window() else {
        return;
    };

    let callback = Closure::once_into_js(callback);
    let _ = window.set_timeout_with_callback_and_timeout_and_arguments_0(
        callback.unchecked_ref(),
        timeout_ms,
    );
}
