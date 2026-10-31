use leptos::{html::ElementType, prelude::*};
use wasm_bindgen::{closure::Closure, JsCast};
use web_sys::{Element, ResizeObserver};

/// JS callback type expected by `ResizeObserver`.
type ResizeCallback = Closure<dyn FnMut(js_sys::Array, ResizeObserver)>;

/// Active observation of one element; the callback must live as long as the observer.
struct Observation {
    observer: ResizeObserver,
    _callback: ResizeCallback,
    element: Element,
}

/// Runs `on_resize` with the element behind `node_ref` once it mounts and on every resize.
///
/// The observation follows remounts of the referenced element and disconnects together with
/// the calling component.
pub(crate) fn use_resize_observer<E, F>(node_ref: NodeRef<E>, on_resize: F)
where
    E: ElementType,
    E::Output: JsCast + Clone + 'static,
    F: Fn(&Element) + Clone + 'static,
{
    let observation = StoredValue::new_local(None::<Observation>);

    // Tracking the node ref attaches the observer also when the element mounts late.
    Effect::new(move |_| {
        let Some(element) = node_ref.try_get().flatten() else {
            return;
        };
        let element: Element = element.unchecked_into();
        let observed = observation
            .try_with_value(|current| {
                current
                    .as_ref()
                    .is_some_and(|current| current.element == element)
            })
            .unwrap_or(true);
        if observed {
            return;
        }
        disconnect(observation);

        on_resize(&element);
        let target = element.clone();
        let handler = on_resize.clone();
        let callback: ResizeCallback =
            Closure::wrap(Box::new(move |_: js_sys::Array, _: ResizeObserver| {
                handler(&target)
            }));
        let Ok(observer) = ResizeObserver::new(callback.as_ref().unchecked_ref()) else {
            return;
        };
        observer.observe(&element);
        observation.set_value(Some(Observation {
            observer,
            _callback: callback,
            element,
        }));
    });

    on_cleanup(move || disconnect(observation));
}

/// Stops the current observation, if any.
fn disconnect(observation: StoredValue<Option<Observation>, LocalStorage>) {
    observation.update_value(|current| {
        if let Some(current) = current.take() {
            current.observer.disconnect();
        }
    });
}
