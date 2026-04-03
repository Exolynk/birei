use leptos::ev;
use leptos::html;
use leptos::prelude::*;
use wasm_bindgen::closure::Closure;
use wasm_bindgen::JsCast;
use web_sys::{HtmlElement, ResizeObserver};

use super::effects::sync_center_prop;
use super::geo::{
    compute_visible_tiles, marker_style, project, unproject, MapCoordinate, WorldPoint,
    DEFAULT_CENTER,
};
use super::interaction::{
    is_marker_interaction_target, normalized_wheel_delta, wheel_zoom_threshold, DragState,
    MarkerDragState,
};
use super::view::{
    render_attribution, render_hidden_inputs, render_marker, render_tiles, render_zoom_controls,
    MarkerViewProps,
};

/// Interactive OpenStreetMap viewer with draggable marker, pan, and zoom controls.
#[component]
pub fn MapViewer(
    /// Current marker position. `None` renders no marker.
    #[prop(optional, into)]
    value: MaybeProp<Option<MapCoordinate>>,
    /// Initial or externally controlled viewport center.
    #[prop(optional, into)]
    center: MaybeProp<MapCoordinate>,
    /// Initial zoom level.
    #[prop(optional, default = 13)]
    zoom: u8,
    /// Minimum zoom level.
    #[prop(optional, default = 1)]
    min_zoom: u8,
    /// Maximum zoom level.
    #[prop(optional, default = 19)]
    max_zoom: u8,
    /// Fixed map height in pixels.
    #[prop(optional, default = 360)]
    height: u16,
    /// Optional id applied to the root element.
    #[prop(optional, into)]
    id: Option<String>,
    /// Optional `name` prefix used to render hidden latitude/longitude inputs.
    #[prop(optional, into)]
    name: Option<String>,
    /// Disables all interaction.
    #[prop(optional)]
    disabled: bool,
    /// Prevents marker changes while keeping viewport navigation available.
    #[prop(optional)]
    readonly: bool,
    /// Additional CSS class names applied to the root element.
    #[prop(optional, into)]
    class: Option<String>,
    /// Marker change callback for controlled usage.
    #[prop(optional)]
    on_value_change: Option<Callback<Option<MapCoordinate>>>,
) -> impl IntoView {
    let root_ref = NodeRef::<html::Div>::new();
    let resize_observer_attached = RwSignal::new(false);
    let resize_observer = StoredValue::new_local(None::<ResizeObserver>);
    let resize_callback =
        StoredValue::new_local(None::<Closure<dyn FnMut(js_sys::Array, ResizeObserver)>>);
    let map_size = RwSignal::new((0.0_f64, f64::from(height)));
    let initial_center = center
        .get_untracked()
        .or_else(|| value.get_untracked().flatten())
        .unwrap_or(DEFAULT_CENTER);
    let viewport_center = RwSignal::new(initial_center);
    let viewport_zoom = RwSignal::new(zoom.clamp(min_zoom, max_zoom));
    let pan_state = RwSignal::new(None::<DragState>);
    let marker_drag_state = RwSignal::new(None::<MarkerDragState>);
    let wheel_delta = RwSignal::new(0.0_f64);

    let class_name = move || {
        let mut classes = vec!["birei-map-picker"];
        if disabled {
            classes.push("birei-map-picker--disabled");
        }
        if readonly {
            classes.push("birei-map-picker--readonly");
        }
        if let Some(class) = class.as_deref() {
            classes.push(class);
        }
        classes.join(" ")
    };

    sync_center_prop(center, viewport_center);

    let update_map_size = move || {
        if let Some(root) = root_ref.get_untracked() {
            map_size.set((
                root.get_bounding_client_rect().width(),
                root.get_bounding_client_rect().height(),
            ));
        }
    };

    Effect::new(move |_| {
        let Some(root) = root_ref.get() else {
            return;
        };
        if resize_observer_attached.get_untracked() {
            return;
        }

        update_map_size();

        let callback = Closure::wrap(Box::new(
            move |_entries: js_sys::Array, _observer: ResizeObserver| {
                update_map_size();
            },
        ) as Box<dyn FnMut(js_sys::Array, ResizeObserver)>);

        if let Ok(observer) = ResizeObserver::new(callback.as_ref().unchecked_ref()) {
            observer.observe(root.as_ref());
            resize_observer_attached.set(true);
            resize_callback.update_value(|stored| *stored = Some(callback));
            resize_observer.update_value(|stored| *stored = Some(observer));
        }

        on_cleanup(move || {
            resize_observer.update_value(|stored| {
                if let Some(observer) = stored.take() {
                    observer.disconnect();
                }
            });
            resize_callback.update_value(|stored| {
                stored.take();
            });
            resize_observer_attached.set(false);
        });
    });

    let tiles = Memo::new(move |_| {
        compute_visible_tiles(viewport_center.get(), viewport_zoom.get(), map_size.get())
    });
    let marker_style = Signal::derive(move || {
        value.get().flatten().map(|position| {
            marker_style(
                position,
                viewport_center.get(),
                viewport_zoom.get(),
                map_size.get(),
            )
        })
    });

    let emit_value = Callback::new(move |next: Option<MapCoordinate>| {
        if let Some(on_value_change) = on_value_change.as_ref() {
            on_value_change.run(next);
        }
    });

    let handle_pointer_down = move |event: ev::PointerEvent| {
        if disabled {
            return;
        }
        if marker_drag_state.get_untracked().is_some()
            || is_marker_interaction_target(event.target())
        {
            return;
        }

        let Some(target) = event
            .current_target()
            .and_then(|target| target.dyn_into::<HtmlElement>().ok())
        else {
            return;
        };

        let _ = target.set_pointer_capture(event.pointer_id());
        pan_state.set(Some(DragState {
            pointer_id: event.pointer_id(),
            client_x: f64::from(event.client_x()),
            client_y: f64::from(event.client_y()),
            center: viewport_center.get_untracked(),
        }));
    };

    let handle_pointer_move = move |event: ev::PointerEvent| {
        if marker_drag_state.get_untracked().is_some() {
            return;
        }

        let Some(active_drag) = pan_state.get() else {
            return;
        };
        if active_drag.pointer_id != event.pointer_id() {
            return;
        }

        let delta_x = f64::from(event.client_x()) - active_drag.client_x;
        let delta_y = f64::from(event.client_y()) - active_drag.client_y;
        let zoom = viewport_zoom.get_untracked();
        let anchor = project(active_drag.center, zoom);
        viewport_center.set(unproject(
            WorldPoint {
                x: anchor.x - delta_x,
                y: anchor.y - delta_y,
            },
            zoom,
        ));
    };

    let handle_pointer_up = move |event: ev::PointerEvent| {
        if marker_drag_state.get_untracked().is_some() {
            return;
        }

        if let Some(active_drag) = pan_state.get() {
            if active_drag.pointer_id == event.pointer_id() {
                if let Some(target) = event
                    .current_target()
                    .and_then(|target| target.dyn_into::<HtmlElement>().ok())
                {
                    let _ = target.release_pointer_capture(event.pointer_id());
                }
                pan_state.set(None);
            }
        }
    };

    let apply_zoom = move |next_zoom: u8, anchor: Option<(f64, f64)>| {
        let current_zoom = viewport_zoom.get_untracked();
        if next_zoom == current_zoom {
            return;
        }

        let (width, height) = map_size.get_untracked();
        let current_center = viewport_center.get_untracked();
        let next_center = if let Some((anchor_x, anchor_y)) = anchor {
            let current_center_world = project(current_center, current_zoom);
            let world_under_pointer = WorldPoint {
                x: current_center_world.x - (width / 2.0) + anchor_x,
                y: current_center_world.y - (height / 2.0) + anchor_y,
            };
            let pinned_geo = unproject(world_under_pointer, current_zoom);
            let pinned_world = project(pinned_geo, next_zoom);

            unproject(
                WorldPoint {
                    x: pinned_world.x - anchor_x + (width / 2.0),
                    y: pinned_world.y - anchor_y + (height / 2.0),
                },
                next_zoom,
            )
        } else {
            current_center
        };

        viewport_zoom.set(next_zoom);
        viewport_center.set(next_center);
    };

    let handle_wheel = move |event: ev::WheelEvent| {
        if disabled {
            return;
        }
        event.prevent_default();

        let Some(target) = event
            .current_target()
            .and_then(|target| target.dyn_into::<HtmlElement>().ok())
        else {
            return;
        };
        let rect = target.get_bounding_client_rect();
        let anchor = (
            f64::from(event.client_x()) - rect.left(),
            f64::from(event.client_y()) - rect.top(),
        );
        let delta = normalized_wheel_delta(&event);
        let threshold = wheel_zoom_threshold(&event);

        wheel_delta.update(|accumulated| {
            if accumulated.signum() != 0.0 && accumulated.signum() != delta.signum() {
                *accumulated = 0.0;
            }

            *accumulated += delta;
            if accumulated.abs() < threshold {
                return;
            }

            let current_zoom = viewport_zoom.get_untracked();
            let next_zoom = if *accumulated < 0.0 {
                current_zoom.saturating_add(1).min(max_zoom)
            } else {
                current_zoom.saturating_sub(1).max(min_zoom)
            };

            if next_zoom != current_zoom {
                apply_zoom(next_zoom, Some(anchor));
            }

            *accumulated = 0.0;
        });
    };

    let zoom_in = Callback::new(move |_| {
        let current_zoom = viewport_zoom.get_untracked();
        let next_zoom = current_zoom.saturating_add(1).min(max_zoom);
        if next_zoom != current_zoom {
            apply_zoom(next_zoom, None);
        }
    });
    let zoom_out = Callback::new(move |_| {
        let current_zoom = viewport_zoom.get_untracked();
        let next_zoom = current_zoom.saturating_sub(1).max(min_zoom);
        if next_zoom != current_zoom {
            apply_zoom(next_zoom, None);
        }
    });

    let start_marker_drag = Callback::new(move |event: ev::PointerEvent| {
        if disabled || readonly || value.get_untracked().flatten().is_none() {
            return;
        }

        event.stop_propagation();

        if let Some(target) = event
            .current_target()
            .and_then(|target| target.dyn_into::<HtmlElement>().ok())
        {
            let _ = target.set_pointer_capture(event.pointer_id());
        }

        marker_drag_state.set(Some(MarkerDragState {
            pointer_id: event.pointer_id(),
        }));
    });
    let move_marker = Callback::new(move |event: ev::PointerEvent| {
        let Some(active_drag) = marker_drag_state.get() else {
            return;
        };
        if active_drag.pointer_id != event.pointer_id() {
            return;
        }

        event.stop_propagation();

        let Some(root) = root_ref.get_untracked() else {
            return;
        };
        let rect = root.get_bounding_client_rect();
        let x = f64::from(event.client_x()) - rect.left();
        let y = f64::from(event.client_y()) - rect.top();
        let (width, height) = map_size.get_untracked();
        let center = project(
            viewport_center.get_untracked(),
            viewport_zoom.get_untracked(),
        );

        emit_value.run(Some(unproject(
            WorldPoint {
                x: center.x - (width / 2.0) + x,
                y: center.y - (height / 2.0) + y,
            },
            viewport_zoom.get_untracked(),
        )));
    });
    let stop_marker_drag = Callback::new(move |event: ev::PointerEvent| {
        let Some(active_drag) = marker_drag_state.get() else {
            return;
        };
        if active_drag.pointer_id != event.pointer_id() {
            return;
        }

        event.stop_propagation();

        if let Some(target) = event
            .current_target()
            .and_then(|target| target.dyn_into::<HtmlElement>().ok())
        {
            let _ = target.release_pointer_capture(event.pointer_id());
        }

        marker_drag_state.set(None);
    });

    view! {
        <div
            id=id
            node_ref=root_ref
            class=class_name
            style=format!("--birei-map-picker-height: {height}px;")
        >
            <div
                class="birei-map-picker__surface"
                on:pointerdown=handle_pointer_down
                on:pointermove=handle_pointer_move
                on:pointerup=handle_pointer_up
                on:pointercancel=handle_pointer_up
                on:wheel=handle_wheel
            >
                {render_tiles(tiles)}
                {render_marker(MarkerViewProps {
                    marker_style,
                    disabled,
                    readonly,
                    start_marker_drag,
                    move_marker,
                    stop_marker_drag,
                })}
                {render_zoom_controls(disabled, zoom_in, zoom_out)}
                {render_attribution()}
            </div>

            {render_hidden_inputs(name, value)}
        </div>
    }
}
