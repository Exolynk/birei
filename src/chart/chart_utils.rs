use wasm_bindgen::closure::Closure;
use wasm_bindgen::JsCast;

pub(crate) const CHART_VIEWBOX_WIDTH: f64 = 1000.0;
pub(crate) const CHART_VIEWBOX_HEIGHT: f64 = 360.0;

pub(crate) fn polar_to_cartesian(cx: f64, cy: f64, radius: f64, angle: f64) -> (f64, f64) {
    let radians = angle.to_radians();
    (cx + (radius * radians.cos()), cy + (radius * radians.sin()))
}

pub(crate) fn describe_arc(
    cx: f64,
    cy: f64,
    radius: f64,
    start_angle: f64,
    end_angle: f64,
) -> String {
    let (start_x, start_y) = polar_to_cartesian(cx, cy, radius, start_angle);
    let (end_x, end_y) = polar_to_cartesian(cx, cy, radius, end_angle);
    let large_arc = if end_angle - start_angle > 180.0 {
        1
    } else {
        0
    };

    format!(
        "M {start_x:.3} {start_y:.3} A {radius:.3} {radius:.3} 0 {large_arc} 1 {end_x:.3} {end_y:.3}"
    )
}

pub(crate) fn describe_ring_segment(
    cx: f64,
    cy: f64,
    outer_radius: f64,
    inner_radius: f64,
    start_angle: f64,
    end_angle: f64,
) -> String {
    let (outer_start_x, outer_start_y) = polar_to_cartesian(cx, cy, outer_radius, start_angle);
    let (outer_end_x, outer_end_y) = polar_to_cartesian(cx, cy, outer_radius, end_angle);
    let (inner_end_x, inner_end_y) = polar_to_cartesian(cx, cy, inner_radius, end_angle);
    let (inner_start_x, inner_start_y) = polar_to_cartesian(cx, cy, inner_radius, start_angle);
    let large_arc = if end_angle - start_angle > 180.0 {
        1
    } else {
        0
    };

    format!(
        "M {outer_start_x:.3} {outer_start_y:.3} \
         A {outer_radius:.3} {outer_radius:.3} 0 {large_arc} 1 {outer_end_x:.3} {outer_end_y:.3} \
         L {inner_end_x:.3} {inner_end_y:.3} \
         A {inner_radius:.3} {inner_radius:.3} 0 {large_arc} 0 {inner_start_x:.3} {inner_start_y:.3} Z"
    )
}

pub(crate) fn describe_full_circle(cx: f64, cy: f64, radius: f64) -> String {
    format!(
        "M {:.3} {:.3} \
         a {:.3} {:.3} 0 1 0 {:.3} 0 \
         a {:.3} {:.3} 0 1 0 -{:.3} 0 Z",
        cx - radius,
        cy,
        radius,
        radius,
        radius * 2.0,
        radius,
        radius,
        radius * 2.0
    )
}

pub(crate) fn describe_full_ring(cx: f64, cy: f64, outer_radius: f64, inner_radius: f64) -> String {
    format!(
        "M {:.3} {:.3} \
         a {:.3} {:.3} 0 1 0 {:.3} 0 \
         a {:.3} {:.3} 0 1 0 -{:.3} 0 \
         M {:.3} {:.3} \
         a {:.3} {:.3} 0 1 1 {:.3} 0 \
         a {:.3} {:.3} 0 1 1 -{:.3} 0 Z",
        cx - outer_radius,
        cy,
        outer_radius,
        outer_radius,
        outer_radius * 2.0,
        outer_radius,
        outer_radius,
        outer_radius * 2.0,
        cx - inner_radius,
        cy,
        inner_radius,
        inner_radius,
        inner_radius * 2.0,
        inner_radius,
        inner_radius,
        inner_radius * 2.0
    )
}

pub(crate) fn request_animation_frame_once(callback: impl FnOnce() + 'static) {
    let Some(window) = web_sys::window() else {
        return;
    };

    let callback = Closure::once_into_js(callback);
    let _ = window.request_animation_frame(callback.unchecked_ref());
}
