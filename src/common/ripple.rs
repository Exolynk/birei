use leptos::{ev, prelude::*};
use wasm_bindgen::JsCast;
use web_sys::HtmlElement;

/// Initial ripple variables before the first click.
const RESTING_STYLE: &str =
    "--birei-ripple-x: 50%; --birei-ripple-y: 50%; --birei-ripple-size: 0px;";

/// Click ripple of a control: CSS variables for origin and size plus an alternating phase.
///
/// The phase alternates between two animation classes so repeated clicks restart the animation.
#[derive(Clone, Copy)]
pub(crate) struct Ripple {
    style: RwSignal<String>,
    phase: RwSignal<Option<bool>>,
}

impl Ripple {
    /// Creates a resting ripple owned by the current reactive scope.
    pub(crate) fn new() -> Self {
        Self {
            style: RwSignal::new(String::from(RESTING_STYLE)),
            phase: RwSignal::new(None),
        }
    }

    /// Returns the inline style carrying the ripple variables.
    pub(crate) fn style(self) -> String {
        self.style.try_get().unwrap_or_default()
    }

    /// Returns the animation class of the current phase, `None` before the first click.
    pub(crate) fn class(
        self,
        phase_a: &'static str,
        phase_b: &'static str,
    ) -> Option<&'static str> {
        self.phase
            .try_get()
            .flatten()
            .map(|phase| if phase { phase_a } else { phase_b })
    }

    /// Starts the ripple at the click position, or centered for keyboard activation.
    pub(crate) fn start(self, event: &ev::MouseEvent) {
        let Some(target) = event
            .current_target()
            .and_then(|target| target.dyn_into::<HtmlElement>().ok())
        else {
            return;
        };
        let rect = target.get_bounding_client_rect();
        let (x, y) = if event.detail() == 0 {
            (rect.width() / 2.0, rect.height() / 2.0)
        } else {
            (
                f64::from(event.client_x()) - rect.left(),
                f64::from(event.client_y()) - rect.top(),
            )
        };
        let size = rect.width().max(rect.height()) * 1.35;

        self.style.set(format!(
            "--birei-ripple-x: {x}px; --birei-ripple-y: {y}px; --birei-ripple-size: {size}px;"
        ));
        self.phase
            .update(|phase| *phase = Some(!phase.unwrap_or(false)));
    }
}
