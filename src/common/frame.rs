use wasm_bindgen::{closure::Closure, JsCast};

/// Runs `callback` once on the next animation frame, after layout has settled.
pub(crate) fn run_on_next_frame(callback: impl FnOnce() + 'static) {
    let Some(window) = web_sys::window() else {
        return;
    };

    let callback = Closure::once_into_js(callback);
    let _ = window.request_animation_frame(callback.unchecked_ref());
}
