use leptos::ev;
use leptos::prelude::*;

use crate::{Button, ButtonVariant, Size};

use super::geo::{MapCoordinate, MapTile};

pub(crate) struct MarkerViewProps {
    pub(crate) marker_style: Signal<Option<String>>,
    pub(crate) disabled: bool,
    pub(crate) readonly: bool,
    pub(crate) start_marker_drag: Callback<ev::PointerEvent>,
    pub(crate) move_marker: Callback<ev::PointerEvent>,
    pub(crate) stop_marker_drag: Callback<ev::PointerEvent>,
}

pub(crate) fn render_tiles(tiles: Memo<Vec<MapTile>>) -> AnyView {
    view! {
        {move || {
            tiles.get()
                .into_iter()
                .map(|tile| {
                    view! {
                        <img
                            class="birei-map-picker__tile"
                            src=tile.url
                            alt=""
                            draggable="false"
                            data-key=tile.key
                            style=format!("left: {:.3}px; top: {:.3}px;", tile.left, tile.top)
                        />
                    }
                })
                .collect_view()
        }}
    }
    .into_any()
}

pub(crate) fn render_marker(props: MarkerViewProps) -> AnyView {
    let MarkerViewProps {
        marker_style,
        disabled,
        readonly,
        start_marker_drag,
        move_marker,
        stop_marker_drag,
    } = props;

    view! {
        {move || {
            marker_style.get().map(|style| {
                view! {
                    <div
                        class="birei-map-picker__marker"
                        style=style
                        tabindex=if disabled || readonly { "-1" } else { "0" }
                        role="button"
                        aria-label="Selected map marker"
                        on:pointerdown=move |event| start_marker_drag.run(event)
                        on:pointermove=move |event| move_marker.run(event)
                        on:pointerup=move |event| stop_marker_drag.run(event)
                        on:pointercancel=move |event| stop_marker_drag.run(event)
                        on:click=move |event| event.stop_propagation()
                    >
                        <span class="birei-map-picker__marker-pin" aria-hidden="true"></span>
                        <span class="birei-map-picker__marker-shadow" aria-hidden="true"></span>
                    </div>
                }
            })
        }}
    }
    .into_any()
}

pub(crate) fn render_zoom_controls(
    disabled: bool,
    zoom_in: Callback<ev::MouseEvent>,
    zoom_out: Callback<ev::MouseEvent>,
) -> AnyView {
    view! {
        <div
            class="birei-map-picker__controls"
            on:pointerdown=move |event| event.stop_propagation()
            on:click=move |event| event.stop_propagation()
        >
            <Button
                variant=ButtonVariant::Secondary
                size=Size::Small
                circle=true
                class="birei-map-picker__zoom-button"
                disabled=disabled
                on_click=zoom_in
            >
                <span aria-hidden="true">"+"</span>
                <span class="birei-map-picker__sr-only">"Zoom in"</span>
            </Button>
            <Button
                variant=ButtonVariant::Secondary
                size=Size::Small
                circle=true
                class="birei-map-picker__zoom-button"
                disabled=disabled
                on_click=zoom_out
            >
                <span aria-hidden="true">"−"</span>
                <span class="birei-map-picker__sr-only">"Zoom out"</span>
            </Button>
        </div>
    }
    .into_any()
}

pub(crate) fn render_attribution() -> AnyView {
    view! {
        <a
            class="birei-map-picker__attribution"
            href="https://www.openstreetmap.org/copyright"
            target="_blank"
            rel="noopener noreferrer"
            on:pointerdown=move |event| event.stop_propagation()
            on:click=move |event| event.stop_propagation()
        >
            "© OpenStreetMap"
        </a>
    }
    .into_any()
}

pub(crate) fn render_hidden_inputs(
    name: Option<String>,
    value: MaybeProp<Option<MapCoordinate>>,
) -> AnyView {
    view! {
        {move || {
            let marker = value.get().flatten();
            name.as_ref().map(|name| {
                view! {
                    <input type="hidden" name=format!("{name}[lat]") value=marker.map(|value| value.lat.to_string()).unwrap_or_default()/>
                    <input type="hidden" name=format!("{name}[lng]") value=marker.map(|value| value.lng.to_string()).unwrap_or_default()/>
                }
            })
        }}
    }
    .into_any()
}
