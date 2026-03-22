use leptos::ev;
use leptos::html;
use leptos::prelude::*;
use wasm_bindgen::JsCast;
use web_sys::HtmlElement;

use super::SliderStepLabel;
use crate::Size;

/// Native range slider with optional labeled stops and animated fill feedback.
#[component]
pub fn Slider(
    /// Current slider value.
    #[prop(optional, into)]
    value: MaybeProp<f64>,
    /// Minimum selectable value.
    #[prop(optional, default = 0.0)]
    min: f64,
    /// Maximum selectable value.
    #[prop(optional, default = 100.0)]
    max: f64,
    /// Step increment used by the native range input.
    #[prop(optional, default = 1.0)]
    step: f64,
    /// Optional input name for form submission.
    #[prop(optional, into)]
    name: Option<String>,
    /// Optional input id.
    #[prop(optional, into)]
    id: Option<String>,
    /// Shared sizing token aligned with buttons and inputs.
    #[prop(optional)]
    size: Size,
    /// Disables the slider and prevents interaction.
    #[prop(optional)]
    disabled: bool,
    /// Marks the slider as invalid for styling and accessibility.
    #[prop(optional)]
    invalid: bool,
    /// Optional labeled stops shown below the track.
    #[prop(optional, into)]
    step_labels: MaybeProp<Vec<SliderStepLabel>>,
    /// Additional CSS class names applied to the root element.
    #[prop(optional, into)]
    class: Option<String>,
    /// Value change callback for controlled usage.
    #[prop(optional)]
    on_value_change: Option<Callback<f64>>,
    /// Input event handler.
    #[prop(optional)]
    on_input: Option<Callback<ev::Event>>,
    /// Change event handler.
    #[prop(optional)]
    on_change: Option<Callback<ev::Event>>,
    /// Focus event handler.
    #[prop(optional)]
    on_focus: Option<Callback<ev::FocusEvent>>,
    /// Blur event handler.
    #[prop(optional)]
    on_blur: Option<Callback<ev::FocusEvent>>,
) -> impl IntoView {
    let mut classes = vec!["birei-slider", size.slider_class_name()];

    if disabled {
        classes.push("birei-slider--disabled");
    }
    if invalid {
        classes.push("birei-slider--invalid");
    }
    if let Some(class) = class.as_deref() {
        classes.push(class);
    }

    let class_name = classes.join(" ");
    let input_ref = NodeRef::<html::Input>::new();
    let ripple_style = RwSignal::new(String::from(
        "--birei-slider-ripple-origin: 50%; --birei-slider-ripple-size: 0px;",
    ));
    let ripple_phase = RwSignal::new(None::<bool>);

    let value_signal = value;
    let fill_style = move || {
        let ratio = slider_ratio(current_value(value_signal.get(), min), min, max);
        format!(
            "--birei-slider-fill-percent: {:.4}%; {}",
            ratio * 100.0,
            ripple_style.get()
        )
    };
    let slider_class = move || {
        let mut classes = class_name.clone();

        if let Some(phase) = ripple_phase.get() {
            classes.push(' ');
            classes.push_str(if phase {
                "birei-slider--ripple-a"
            } else {
                "birei-slider--ripple-b"
            });
        }

        classes
    };

    let trigger_ripple = move |origin_ratio: f64| {
        if let Some(input) = input_ref.get() {
            let rect = input.get_bounding_client_rect();
            let origin = rect.width() * origin_ratio.clamp(0.0, 1.0);
            let size = rect.width().max(48.0) * 0.42;

            ripple_style.set(format!(
                "--birei-slider-ripple-origin: {origin}px; --birei-slider-ripple-size: {size}px;"
            ));
            ripple_phase.update(|phase| {
                *phase = Some(!phase.unwrap_or(false));
            });
        }
    };

    let handle_pointer_down = move |event: ev::PointerEvent| {
        if let Some(target) = event
            .current_target()
            .and_then(|target| target.dyn_into::<HtmlElement>().ok())
        {
            let rect = target.get_bounding_client_rect();
            let x = (f64::from(event.client_x()) - rect.left()).clamp(0.0, rect.width());
            let size = rect.width().max(48.0) * 0.42;

            ripple_style.set(format!(
                "--birei-slider-ripple-origin: {x}px; --birei-slider-ripple-size: {size}px;"
            ));
            ripple_phase.update(|phase| {
                *phase = Some(!phase.unwrap_or(false));
            });
        }
    };

    let handle_input = move |event: ev::Event| {
        let next = event_target_value(&event)
            .parse::<f64>()
            .ok()
            .unwrap_or(min);
        trigger_ripple(slider_ratio(next, min, max));

        if let Some(on_value_change) = on_value_change.as_ref() {
            on_value_change.run(next);
        }
        if let Some(on_input) = on_input.as_ref() {
            on_input.run(event);
        }
    };

    let handle_change = move |event: ev::Event| {
        let next = event_target_value(&event)
            .parse::<f64>()
            .ok()
            .unwrap_or(min);

        if let Some(on_value_change) = on_value_change.as_ref() {
            on_value_change.run(next);
        }
        if let Some(on_change) = on_change.as_ref() {
            on_change.run(event);
        }
    };

    let handle_focus = move |event: ev::FocusEvent| {
        trigger_ripple(slider_ratio(
            current_value(value_signal.get(), min),
            min,
            max,
        ));

        if let Some(on_focus) = on_focus.as_ref() {
            on_focus.run(event);
        }
    };

    let label_buttons = move || {
        step_labels
            .get()
            .unwrap_or_default()
            .into_iter()
            .map(|entry| {
                let value = entry.value;
                let label = entry.label;
                let percent = slider_ratio(value, min, max) * 100.0;

                view! {
                    <button
                        type="button"
                        class="birei-slider__step"
                        style=format!("left: {percent:.4}%;")
                        disabled=disabled
                        aria-pressed=move || if is_current_step(current_value(value_signal.get(), min), value, step, min, max) {
                            "true"
                        } else {
                            "false"
                        }
                        on:click=move |_| {
                            trigger_ripple(slider_ratio(value, min, max));
                            if let Some(input) = input_ref.get() {
                                let _ = input.focus();
                            }
                            if let Some(on_value_change) = on_value_change.as_ref() {
                                on_value_change.run(value);
                            }
                        }
                    >
                        <span class="birei-slider__step-mark" aria-hidden="true"></span>
                        <span class="birei-slider__step-label">{label.clone()}</span>
                    </button>
                }
            })
            .collect_view()
    };

    view! {
        <div
            class=slider_class
            style=fill_style
            on:pointerdown=handle_pointer_down
        >
            <div class="birei-slider__control">
                <input
                    node_ref=input_ref
                    class="birei-slider__native"
                    type="range"
                    id=id
                    name=name
                    min=min.to_string()
                    max=max.to_string()
                    step=step.to_string()
                    prop:value=move || current_value(value_signal.get(), min).to_string()
                    disabled=disabled
                    aria-invalid=move || if invalid { "true" } else { "false" }
                    on:input=handle_input
                    on:change=handle_change
                    on:focus=handle_focus
                    on:blur=move |event| {
                        if let Some(on_blur) = on_blur.as_ref() {
                            on_blur.run(event);
                        }
                    }
                />
                <span class="birei-slider__track" aria-hidden="true">
                    <span class="birei-slider__track-base"></span>
                    <span class="birei-slider__track-fill"></span>
                    <span class="birei-slider__track-ripple"></span>
                </span>
                <span class="birei-slider__thumb" aria-hidden="true"></span>
            </div>
            {move || {
                let labels = step_labels.get().unwrap_or_default();
                (!labels.is_empty()).then(|| {
                    view! {
                        <div class="birei-slider__steps">
                            {label_buttons()}
                        </div>
                    }
                })
            }}
        </div>
    }
}

fn slider_ratio(value: f64, min: f64, max: f64) -> f64 {
    let span = max - min;
    if span <= f64::EPSILON {
        return 0.0;
    }

    ((value - min) / span).clamp(0.0, 1.0)
}

fn current_value(value: Option<f64>, min: f64) -> f64 {
    value.unwrap_or(min)
}

fn is_current_step(current: f64, candidate: f64, step: f64, min: f64, max: f64) -> bool {
    let tolerance = if step > 0.0 {
        step / 2.0
    } else {
        (max - min).abs() * 0.005
    };
    (current - candidate).abs() <= tolerance.max(0.000_1)
}
