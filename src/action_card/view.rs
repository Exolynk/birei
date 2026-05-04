use std::cell::{Cell, RefCell};
use std::rc::Rc;
use std::sync::atomic::{AtomicU64, Ordering};
use std::sync::Arc;

use leptos::ev;
use leptos::prelude::*;
use wasm_bindgen::closure::Closure;
use wasm_bindgen::JsCast;
use web_sys::HtmlElement;

use crate::{IcnName, Icon, Size};

/// Compact highlight card for home screens and overview panels.
#[component]
pub fn ActionCard(
    /// Primary label shown beneath the icon or number.
    #[prop(into)]
    title: String,
    /// Secondary text shown beneath the title.
    #[prop(into)]
    subtitle: String,
    /// Optional icon shown when no numeric value is provided.
    #[prop(optional, into)]
    icon: Option<IcnName>,
    /// Optional numeric value shown instead of the icon.
    #[prop(optional, into)]
    value: MaybeProp<f64>,
    /// Decimal precision used for numeric rendering.
    #[prop(optional, default = 0)]
    precision: usize,
    /// Abbreviates large numbers using `k`, `m`, `b`, and `t`.
    #[prop(optional)]
    abbreviate: bool,
    /// Additional class names applied to the root element.
    #[prop(optional, into)]
    class: Option<String>,
    /// Optional click handler. When provided, the card renders as a button.
    #[prop(optional)]
    on_click: Option<Callback<ev::MouseEvent>>,
) -> impl IntoView {
    let displayed_value = RwSignal::new(value.get_untracked().unwrap_or_default());
    let animation_generation = Arc::new(AtomicU64::new(0));
    let ripple_style = RwSignal::new(String::from(
        "--birei-ripple-x: 50%; --birei-ripple-y: 50%; --birei-ripple-size: 0px;",
    ));
    let ripple_phase = RwSignal::new(None::<bool>);

    Effect::new(move |_| {
        let Some(target) = value.get() else {
            return;
        };

        let start = displayed_value.get_untracked();
        if (target - start).abs() < f64::EPSILON {
            displayed_value.set(target);
            return;
        }

        let duration_ms = 1800.0_f64;
        let frame = Rc::new(RefCell::new(None::<Closure<dyn FnMut(f64)>>));
        let frame_ref = Rc::clone(&frame);
        let started_at = Rc::new(Cell::new(None::<f64>));
        let started_at_ref = Rc::clone(&started_at);
        let display_signal = displayed_value;
        let generation = animation_generation.fetch_add(1, Ordering::Relaxed) + 1;
        let animation_generation_ref = Arc::clone(&animation_generation);

        *frame.borrow_mut() = Some(Closure::wrap(Box::new(move |timestamp: f64| {
            if animation_generation_ref.load(Ordering::Relaxed) != generation {
                frame_ref.borrow_mut().take();
                return;
            }

            let start_time = started_at_ref.get().unwrap_or(timestamp);
            started_at_ref.set(Some(start_time));

            let progress = ((timestamp - start_time) / duration_ms).clamp(0.0, 1.0);
            let eased = 1.0 - (1.0 - progress).powi(3);
            display_signal.set(start + ((target - start) * eased));

            if progress < 1.0 {
                if let Some(window) = web_sys::window() {
                    let _ = window.request_animation_frame(
                        frame_ref
                            .borrow()
                            .as_ref()
                            .expect("animation frame closure should exist")
                            .as_ref()
                            .unchecked_ref(),
                    );
                }
            } else {
                display_signal.set(target);
                frame_ref.borrow_mut().take();
            }
        }) as Box<dyn FnMut(f64)>));

        if let Some(window) = web_sys::window() {
            let _ = window.request_animation_frame(
                frame
                    .borrow()
                    .as_ref()
                    .expect("animation frame closure should exist")
                    .as_ref()
                    .unchecked_ref(),
            );
        }
    });

    let class_name = move || {
        let mut classes = vec!["birei-action-card"];
        if on_click.is_some() {
            classes.push("birei-action-card--interactive");
        }
        if value.get().is_some() {
            classes.push("birei-action-card--number");
        } else {
            classes.push("birei-action-card--icon");
        }
        if let Some(class) = class.as_deref() {
            classes.push(class);
        }

        let mut classes = classes.join(" ");
        if let Some(phase) = ripple_phase.get() {
            classes.push(' ');
            classes.push_str(if phase {
                "birei-action-card--ripple-a"
            } else {
                "birei-action-card--ripple-b"
            });
        }

        classes
    };

    let number_text = move || {
        let number = displayed_value.get();
        if abbreviate {
            abbreviate_value(number, precision)
        } else {
            format!("{number:.precision$}")
        }
    };
    let number_size = move || {
        let length = number_text().chars().count();
        match length {
            0..=3 => 2.2,
            4 => 1.95,
            5 => 1.72,
            6 => 1.48,
            _ => 1.26_f64.max(2.0 - (length as f64 * 0.12)),
        }
    };

    let content = move || {
        let title = title.clone();
        let subtitle = subtitle.clone();

        view! {
            <>
                <div class="birei-action-card__hero" aria-hidden="true">
                    {value
                        .get()
                        .map(|_| {
                            view! {
                                <span
                                    class="birei-action-card__number"
                                    style=move || format!("font-size: {:.2}rem;", number_size())
                                >
                                    {number_text}
                                </span>
                            }
                                .into_any()
                        })
                        .or_else(|| {
                            icon.clone().map(|icon_name| {
                                view! {
                                    <span class="birei-action-card__icon">
                                        <Icon name=icon_name size=Size::Large/>
                                    </span>
                                }
                                    .into_any()
                            })
                        })}
                </div>
                <div class="birei-action-card__copy">
                    <div class="birei-action-card__title">{title}</div>
                    <div class="birei-action-card__subtitle">{subtitle}</div>
                </div>
            </>
        }
    };

    let handle_click = move |event: ev::MouseEvent| {
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

        if let Some(on_click) = on_click.as_ref() {
            on_click.run(event);
        }
    };

    if on_click.is_some() {
        view! {
            <button
                type="button"
                class=class_name
                style=move || ripple_style.get()
                on:click=handle_click
            >
                {content}
            </button>
        }
        .into_any()
    } else {
        view! {
            <div class=class_name>
                {content}
            </div>
        }
        .into_any()
    }
}

fn abbreviate_value(value: f64, original_precision: usize) -> String {
    let precision = 10usize.pow(original_precision as u32) as f64;
    let abbreviations = ['k', 'm', 'b', 't'];
    let sign = if value.is_sign_negative() { -1.0 } else { 1.0 };
    let abs_value = value.abs();

    for mut index in (0..abbreviations.len()).rev() {
        let size = 10_u64.pow((index as u32 + 1) * 3) as f64;
        if abs_value < size {
            continue;
        }

        let mut shortened = ((abs_value * precision) / size).round() / precision;
        if shortened >= 1000.0 && index < abbreviations.len() - 1 {
            shortened = 1.0;
            index += 1;
        }

        return format!("{}{}", shortened * sign, abbreviations[index]);
    }

    format!("{value:.original_precision$}")
}
