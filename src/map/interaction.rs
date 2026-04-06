use leptos::ev;
use wasm_bindgen::JsCast;
use web_sys::{Element, EventTarget};

use super::geo::MapCoordinate;

const TRACKPAD_ZOOM_THRESHOLD: f64 = 100.0;
const MOUSE_WHEEL_ZOOM_THRESHOLD: f64 = 60.0;

/// Active map-pan pointer data captured on pointer down.
#[derive(Clone, Copy, Debug)]
pub(crate) struct DragState {
    pub(crate) pointer_id: i32,
    pub(crate) client_x: f64,
    pub(crate) client_y: f64,
    pub(crate) center: MapCoordinate,
}

/// Tracks which pointer currently owns marker dragging.
#[derive(Clone, Copy, Debug)]
pub(crate) struct MarkerDragState {
    pub(crate) pointer_id: i32,
}

/// Normalizes browser wheel delta modes into pixel-like values.
pub(crate) fn normalized_wheel_delta(event: &ev::WheelEvent) -> f64 {
    match event.delta_mode() {
        1 => event.delta_y() * 40.0,
        2 => event.delta_y() * 240.0,
        _ => event.delta_y(),
    }
}

/// Chooses a wheel accumulation threshold based on whether the input source
/// looks like a trackpad or a traditional mouse wheel.
pub(crate) fn wheel_zoom_threshold(event: &ev::WheelEvent) -> f64 {
    match event.delta_mode() {
        0 => TRACKPAD_ZOOM_THRESHOLD,
        _ => MOUSE_WHEEL_ZOOM_THRESHOLD,
    }
}

/// Prevents map panning from starting when the initial pointer target was the
/// draggable marker itself.
pub(crate) fn is_marker_interaction_target(target: Option<EventTarget>) -> bool {
    target
        .and_then(|target| target.dyn_into::<Element>().ok())
        .and_then(|element| element.closest(".birei-map-picker__marker").ok().flatten())
        .is_some()
}
