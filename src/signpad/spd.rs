use std::fmt::Write as _;
use std::sync::{Arc, Mutex};

use leptos::ev;
use leptos::html;
use leptos::prelude::*;
use wasm_bindgen::JsCast;
use web_sys::HtmlElement;

use crate::{Button, ButtonVariant, Icon, Size};

/// Imperative access to a mounted [`SignPad`] instance.
#[derive(Clone, Default)]
pub struct SignPadRef(Arc<Mutex<Option<SignPadBindings>>>);

impl SignPadRef {
    /// Creates an empty handle container that can be passed into [`SignPad`].
    pub fn new() -> Self {
        Self::default()
    }

    /// Removes all imported and locally drawn signature content.
    pub fn clear(&self) {
        if let Some(bindings) = self.bindings() {
            (bindings.clear)();
        }
    }

    /// Serializes the current signature as a standalone SVG document.
    pub fn export_svg(&self) -> Option<String> {
        self.bindings().and_then(|bindings| (bindings.export_svg)())
    }

    /// Replaces the current signature with the contents of a standalone SVG document.
    pub fn load_svg(&self, svg: &str) -> Result<(), String> {
        self.bindings()
            .ok_or_else(|| String::from("SignPad is not mounted."))
            .and_then(|bindings| (bindings.load_svg)(svg.to_owned()))
    }

    /// Returns whether the pad has any imported or locally drawn content.
    pub fn is_empty(&self) -> bool {
        self.bindings().is_none_or(|bindings| (bindings.is_empty)())
    }

    fn bindings(&self) -> Option<SignPadBindings> {
        self.0.lock().ok().and_then(|bindings| bindings.clone())
    }

    fn set(&self, bindings: Option<SignPadBindings>) {
        if let Ok(mut state) = self.0.lock() {
            *state = bindings;
        }
    }
}

#[derive(Clone)]
struct SignPadBindings {
    clear: Arc<dyn Fn() + Send + Sync>,
    export_svg: Arc<dyn Fn() -> Option<String> + Send + Sync>,
    load_svg: Arc<dyn Fn(String) -> Result<(), String> + Send + Sync>,
    is_empty: Arc<dyn Fn() -> bool + Send + Sync>,
}

#[derive(Clone, Debug, PartialEq)]
struct StrokePoint {
    x: f64,
    y: f64,
    width: f64,
    time: f64,
}

#[derive(Clone, Debug, Default, PartialEq)]
struct Stroke {
    points: Vec<StrokePoint>,
}

/// Fixed-size signature pad with SVG import/export and speed-aware stroke width.
#[component]
pub fn SignPad(
    /// Optional externally supplied SVG document to display or replace the current content.
    #[prop(optional, into)]
    value: MaybeProp<String>,
    /// Fixed drawing width in CSS pixels and SVG units.
    #[prop(optional, default = 420)]
    width: u16,
    /// Fixed drawing height in CSS pixels and SVG units.
    #[prop(optional, default = 180)]
    height: u16,
    /// Optional id applied to the root element.
    #[prop(optional, into)]
    id: Option<String>,
    /// Optional label announced by assistive technology.
    #[prop(optional, into)]
    aria_label: Option<String>,
    /// Disables all pointer interaction.
    #[prop(optional)]
    disabled: bool,
    /// Renders the signature without allowing edits.
    #[prop(optional)]
    readonly: bool,
    /// Optional placeholder shown when the pad is empty.
    #[prop(optional, into)]
    placeholder: Option<String>,
    /// Additional CSS class names applied to the root element.
    #[prop(optional, into)]
    class: Option<String>,
    /// Optional imperative ref used to access clear/import/export helpers.
    #[prop(optional)]
    signpad_ref: Option<SignPadRef>,
) -> impl IntoView {
    let root_ref = NodeRef::<html::Div>::new();
    let pad_ref = signpad_ref.unwrap_or_default();
    let strokes = RwSignal::new(Vec::<Stroke>::new());
    let active_stroke = RwSignal::new(None::<Stroke>);
    let imported_markup = RwSignal::new(None::<String>);
    let active_pointer_id = RwSignal::new(None::<i32>);
    let last_external_value = RwSignal::new(None::<String>);
    let interactive = !disabled && !readonly;
    let placeholder_text = placeholder.unwrap_or_else(|| String::from("Sign here"));

    let clear_content = move || {
        strokes.set(Vec::new());
        active_stroke.set(None);
        imported_markup.set(None);
        active_pointer_id.set(None);
    };

    let export_svg = move || {
        let mut all_strokes = strokes.get_untracked();
        if let Some(stroke) = active_stroke.get_untracked() {
            if !stroke.points.is_empty() {
                all_strokes.push(stroke);
            }
        }

        let imported = imported_markup.get_untracked();
        if imported.as_deref().is_none_or(str::is_empty) && all_strokes.is_empty() {
            return None;
        }

        Some(compose_svg_document(
            width,
            height,
            &resolve_export_color(&root_ref),
            imported.as_deref(),
            &all_strokes,
        ))
    };

    let load_svg = move |svg: String| {
        if svg.trim().is_empty() {
            clear_content();
            return Ok(());
        }

        let markup = extract_svg_inner_markup(&svg)?;
        clear_content();
        imported_markup.set(Some(markup));
        Ok(())
    };

    pad_ref.set(Some(SignPadBindings {
        clear: Arc::new(clear_content),
        export_svg: Arc::new(export_svg),
        load_svg: Arc::new(load_svg),
        is_empty: Arc::new(move || {
            imported_markup
                .get_untracked()
                .as_deref()
                .is_none_or(str::is_empty)
                && strokes.get_untracked().is_empty()
                && active_stroke
                    .get_untracked()
                    .is_none_or(|stroke| stroke.points.is_empty())
        }),
    }));

    Effect::new(move |_| {
        let next = value.get();
        if next == last_external_value.get_untracked() {
            return;
        }
        last_external_value.set(next.clone());

        match next {
            Some(svg) if !svg.trim().is_empty() => {
                if let Ok(markup) = extract_svg_inner_markup(&svg) {
                    clear_content();
                    imported_markup.set(Some(markup));
                }
            }
            _ => clear_content(),
        }
    });

    let class_name = move || {
        let mut classes = vec!["birei-sign-pad"];
        if disabled {
            classes.push("birei-sign-pad--disabled");
        }
        if readonly {
            classes.push("birei-sign-pad--readonly");
        }
        if imported_markup.get().as_deref().is_none_or(str::is_empty)
            && strokes.get().is_empty()
            && active_stroke
                .get()
                .is_none_or(|stroke| stroke.points.is_empty())
        {
            classes.push("birei-sign-pad--empty");
        }
        if let Some(class) = class.as_deref() {
            classes.push(class);
        }
        classes.join(" ")
    };
    let show_clear_button = move || {
        interactive
            && !(imported_markup.get().as_deref().is_none_or(str::is_empty)
                && strokes.get().is_empty()
                && active_stroke
                    .get()
                    .is_none_or(|stroke| stroke.points.is_empty()))
    };

    let finish_stroke = move |pointer_id: i32| {
        if active_pointer_id.get_untracked() != Some(pointer_id) {
            return;
        }

        active_pointer_id.set(None);
        if let Some(stroke) = active_stroke.get_untracked() {
            if !stroke.points.is_empty() {
                strokes.update(|all| all.push(stroke));
            }
        }
        active_stroke.set(None);
    };

    let handle_pointer_down = move |event: ev::PointerEvent| {
        if !interactive || active_pointer_id.get_untracked().is_some() {
            return;
        }

        let Some(target) = event
            .current_target()
            .and_then(|target| target.dyn_into::<HtmlElement>().ok())
        else {
            return;
        };

        event.prevent_default();
        let _ = target.set_pointer_capture(event.pointer_id());

        if let Some(point) = sample_pointer(&event, &target, None) {
            active_pointer_id.set(Some(event.pointer_id()));
            active_stroke.set(Some(Stroke {
                points: vec![point],
            }));
        }
    };

    let handle_pointer_move = move |event: ev::PointerEvent| {
        if !interactive || active_pointer_id.get_untracked() != Some(event.pointer_id()) {
            return;
        }

        let Some(target) = event
            .current_target()
            .and_then(|target| target.dyn_into::<HtmlElement>().ok())
        else {
            return;
        };

        event.prevent_default();
        active_stroke.update(|stroke| {
            let Some(stroke) = stroke.as_mut() else {
                return;
            };
            let previous = stroke.points.last().cloned();
            let Some(point) = sample_pointer(&event, &target, previous.as_ref()) else {
                return;
            };

            if previous.as_ref().is_some_and(|last| {
                distance(last.x, last.y, point.x, point.y) < 0.35
                    && (point.time - last.time).abs() < 12.0
            }) {
                return;
            }

            stroke.points.push(point);
        });
    };

    let handle_pointer_up = move |event: ev::PointerEvent| {
        if !interactive {
            return;
        }

        if let Some(target) = event
            .current_target()
            .and_then(|target| target.dyn_into::<HtmlElement>().ok())
        {
            let _ = target.release_pointer_capture(event.pointer_id());
            active_stroke.update(|stroke| {
                let Some(stroke) = stroke.as_mut() else {
                    return;
                };
                let previous = stroke.points.last().cloned();
                let Some(point) = sample_pointer(&event, &target, previous.as_ref()) else {
                    return;
                };

                if previous
                    .as_ref()
                    .is_none_or(|last| distance(last.x, last.y, point.x, point.y) >= 0.15)
                {
                    stroke.points.push(point);
                }
            });
        }

        finish_stroke(event.pointer_id());
    };

    let handle_pointer_cancel = move |event: ev::PointerEvent| {
        if let Some(target) = event
            .current_target()
            .and_then(|target| target.dyn_into::<HtmlElement>().ok())
        {
            let _ = target.release_pointer_capture(event.pointer_id());
        }

        finish_stroke(event.pointer_id());
    };

    let render_strokes = move || {
        let mut all = strokes.get();
        if let Some(active) = active_stroke.get() {
            if !active.points.is_empty() {
                all.push(active);
            }
        }

        all.into_iter()
            .enumerate()
            .map(|(index, stroke)| {
                render_stroke_group(stroke, format!("birei-sign-pad__stroke-{index}"))
            })
            .collect_view()
    };

    view! {
        <div
            node_ref=root_ref
            id=id
            class=class_name
            style=format!(
                "--birei-sign-pad-width: {width}px; --birei-sign-pad-height: {height}px;"
            )
        >
            <Show when=show_clear_button>
                <div class="birei-sign-pad__toolbar">
                    <Button
                        size=Size::Small
                        variant=ButtonVariant::Transparent
                        class="birei-sign-pad__clear"
                        on_click=Callback::new(move |_| {
                            clear_content();
                        })
                    >
                        <Icon name="eraser" label="Clear signature"/>
                        <span>"Clear"</span>
                    </Button>
                </div>
            </Show>

            <div
                class="birei-sign-pad__surface"
                role="img"
                aria-label=aria_label.unwrap_or_else(|| String::from("Signature pad"))
                on:contextmenu=move |event| {
                    event.prevent_default();
                }
                on:pointerdown=handle_pointer_down
                on:pointermove=handle_pointer_move
                on:pointerup=handle_pointer_up
                on:pointercancel=handle_pointer_cancel
            >
                <svg
                    class="birei-sign-pad__canvas"
                    width=width.to_string()
                    height=height.to_string()
                    viewBox=format!("0 0 {width} {height}")
                    xmlns="http://www.w3.org/2000/svg"
                    aria-hidden="true"
                >
                    <g
                        class="birei-sign-pad__imported"
                        inner_html=move || imported_markup.get().unwrap_or_default()
                    ></g>
                    {render_strokes}
                </svg>

                <Show
                    when=move || {
                        imported_markup
                            .get()
                            .as_deref()
                            .is_none_or(str::is_empty)
                            && strokes.get().is_empty()
                            && active_stroke.get().is_none_or(|stroke| stroke.points.is_empty())
                    }
                >
                    <div class="birei-sign-pad__placeholder">{placeholder_text.clone()}</div>
                </Show>
            </div>
        </div>
    }
}

fn sample_pointer(
    event: &ev::PointerEvent,
    target: &HtmlElement,
    previous: Option<&StrokePoint>,
) -> Option<StrokePoint> {
    let rect = target.get_bounding_client_rect();
    if rect.width() <= 0.0 || rect.height() <= 0.0 {
        return None;
    }

    let x = (f64::from(event.client_x()) - rect.left()).clamp(0.0, rect.width());
    let y = (f64::from(event.client_y()) - rect.top()).clamp(0.0, rect.height());
    let time = event.time_stamp();
    let pressure = normalized_pressure(event.pressure());

    let width = if let Some(previous) = previous {
        let elapsed = (time - previous.time).abs().max(1.0);
        let speed = distance(previous.x, previous.y, x, y) / elapsed;
        let target_width = stroke_width(speed, pressure);
        (previous.width * 0.72) + (target_width * 0.28)
    } else {
        stroke_width(0.0, pressure)
    };

    Some(StrokePoint { x, y, width, time })
}

fn normalized_pressure(pressure: f32) -> f64 {
    let pressure = f64::from(pressure);
    if pressure <= 0.0 {
        0.5
    } else {
        pressure.clamp(0.0, 1.0)
    }
}

fn stroke_width(speed: f64, pressure: f64) -> f64 {
    let min_width = 1.2_f64;
    let max_width = 4.8_f64;
    let speed_ratio = (speed / 1.6).clamp(0.0, 1.0).powf(0.65);
    let pressure_ratio = 0.7 + (pressure * 0.6);
    ((max_width - ((max_width - min_width) * speed_ratio)) * pressure_ratio).clamp(min_width, 6.2)
}

fn distance(x1: f64, y1: f64, x2: f64, y2: f64) -> f64 {
    let dx = x2 - x1;
    let dy = y2 - y1;
    (dx * dx + dy * dy).sqrt()
}

fn render_stroke_group(stroke: Stroke, class: String) -> impl IntoView {
    let start = stroke.points.first().cloned();
    let segments = stroke
        .points
        .windows(2)
        .enumerate()
        .map(|(index, points)| {
            let from = &points[0];
            let to = &points[1];
            let width = ((from.width + to.width) * 0.5).max(0.8);

            view! {
                <g class=format!("{class}__segment-{index}")>
                    <line
                        class="birei-sign-pad__segment"
                        x1=from.x.to_string()
                        y1=from.y.to_string()
                        x2=to.x.to_string()
                        y2=to.y.to_string()
                        stroke-width=width.to_string()
                        stroke-linecap="round"
                    ></line>
                    <circle
                        class="birei-sign-pad__point"
                        cx=to.x.to_string()
                        cy=to.y.to_string()
                        r=((to.width * 0.1) + 0.04).to_string()
                    ></circle>
                </g>
            }
        })
        .collect_view();

    view! {
        <g class=class>
            {start.map(|point| {
                view! {
                    <circle
                        class="birei-sign-pad__point"
                        cx=point.x.to_string()
                        cy=point.y.to_string()
                        r=((point.width * 0.1) + 0.04).to_string()
                    ></circle>
                }
            })}
            {segments}
        </g>
    }
}

fn resolve_export_color(root_ref: &NodeRef<html::Div>) -> String {
    let Some(root) = root_ref.get_untracked() else {
        return String::from("#1f2a2b");
    };
    let Some(window) = web_sys::window() else {
        return String::from("#1f2a2b");
    };
    let Ok(Some(style)) = window.get_computed_style(&root) else {
        return String::from("#1f2a2b");
    };

    let theme_color = style
        .get_property_value("--birei-sign-pad-ink")
        .ok()
        .unwrap_or_default();
    let theme_color = theme_color.trim();
    if !theme_color.is_empty() {
        return theme_color.to_owned();
    }

    let color = style.get_property_value("color").ok().unwrap_or_default();
    let color = color.trim();
    if color.is_empty() {
        String::from("#1f2a2b")
    } else {
        color.to_owned()
    }
}

fn compose_svg_document(
    width: u16,
    height: u16,
    color: &str,
    imported_markup: Option<&str>,
    strokes: &[Stroke],
) -> String {
    let mut svg = format!(
        r#"<svg xmlns="http://www.w3.org/2000/svg" width="{width}" height="{height}" viewBox="0 0 {width} {height}" fill="none">"#
    );

    if let Some(markup) = imported_markup.filter(|markup| !markup.trim().is_empty()) {
        svg.push_str(markup);
    }

    for stroke in strokes {
        push_stroke_markup(&mut svg, stroke, color);
    }

    svg.push_str("</svg>");
    svg
}

fn push_stroke_markup(buffer: &mut String, stroke: &Stroke, color: &str) {
    if stroke.points.is_empty() {
        return;
    }

    let _ = write!(buffer, r#"<g stroke="{color}" fill="{color}">"#);
    let first = &stroke.points[0];
    let _ = write!(
        buffer,
        r#"<circle cx="{:.2}" cy="{:.2}" r="{:.2}" />"#,
        first.x,
        first.y,
        (first.width * 0.1) + 0.04
    );

    for segment in stroke.points.windows(2) {
        let from = &segment[0];
        let to = &segment[1];
        let width = ((from.width + to.width) * 0.5).max(0.8);
        let _ = write!(
            buffer,
            r#"<line x1="{:.2}" y1="{:.2}" x2="{:.2}" y2="{:.2}" stroke-width="{:.2}" stroke-linecap="round" />"#,
            from.x, from.y, to.x, to.y, width
        );
        let _ = write!(
            buffer,
            r#"<circle cx="{:.2}" cy="{:.2}" r="{:.2}" />"#,
            to.x,
            to.y,
            (to.width * 0.1) + 0.04
        );
    }

    buffer.push_str("</g>");
}

fn extract_svg_inner_markup(svg: &str) -> Result<String, String> {
    let trimmed = svg.trim();
    let lower = trimmed.to_ascii_lowercase();
    if !lower.starts_with("<svg") {
        return Err(String::from("Expected an <svg> root element."));
    }

    let Some(open_end) = trimmed.find('>') else {
        return Err(String::from("Malformed SVG root element."));
    };
    let Some(close_start) = lower.rfind("</svg>") else {
        return Err(String::from("Missing closing </svg> tag."));
    };
    if close_start <= open_end {
        return Err(String::from(
            "SVG document does not contain any valid body markup.",
        ));
    }

    Ok(trimmed[open_end + 1..close_start].trim().to_owned())
}
