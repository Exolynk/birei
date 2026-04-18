use std::collections::HashSet;

use leptos::ev;
use leptos::html;
use leptos::prelude::*;
use uuid::Uuid;
use wasm_bindgen::JsCast;
use web_sys::Element;

use super::internal::{HoverPopup, RelationGraphLayout, NODE_HEIGHT, NODE_WIDTH};
use super::layout::build_layout;
use super::types::{RelationGraphEdge, RelationGraphNode};
use crate::{Button, ButtonVariant, Icon, Size};

/// Left-to-right relation graph with lazy node expansion, pan/zoom, and edge hover details.
#[component]
pub fn RelationGraph(
    /// Fully controlled list of nodes shown in the graph.
    #[prop(into)]
    nodes: MaybeProp<Vec<RelationGraphNode>>,
    /// Fully controlled list of directed relation edges.
    #[prop(into)]
    edges: MaybeProp<Vec<RelationGraphEdge>>,
    /// Callback used to request loading additional neighbors for a node.
    #[prop(optional, into)]
    on_load_node: Option<Callback<Uuid>>,
    /// Callback used to open an already loaded node in the surrounding app.
    #[prop(optional, into)]
    on_open_node: Option<Callback<Uuid>>,
    /// Accessible label announced for the interactive graph viewport.
    #[prop(optional, into)]
    aria_label: Option<String>,
    /// Fixed viewport height in CSS units.
    #[prop(optional, default = String::from("20rem"))]
    height: String,
    /// Additional class names applied to the root element.
    #[prop(optional, into)]
    class: Option<String>,
) -> impl IntoView {
    let viewport_ref = NodeRef::<html::Div>::new();
    let scale = RwSignal::new(1.0_f64);
    let pan = RwSignal::new((24.0_f64, 24.0_f64));
    let drag_pointer_id = RwSignal::new(None::<i32>);
    let drag_origin = RwSignal::new(None::<(f64, f64)>);
    let hover_popup = RwSignal::new(None::<HoverPopup>);
    let pending_loads = RwSignal::new(HashSet::<Uuid>::new());
    let nodes_data = Memo::new(move |_| nodes.get().unwrap_or_default());
    let edges_data = Memo::new(move |_| edges.get().unwrap_or_default());
    let layout = Memo::new(move |_| build_layout(nodes_data.get(), edges_data.get()));

    Effect::new(move |_| {
        let _ = nodes_data.get();
        pending_loads.set(HashSet::new());
    });

    let class_name = move || {
        let mut classes = vec!["birei-relation-graph"];
        if drag_pointer_id.get().is_some() {
            classes.push("birei-relation-graph--dragging");
        }
        if let Some(class) = class.as_deref() {
            classes.push(class);
        }
        classes.join(" ")
    };

    let show_popup = move |title: String, body: Option<String>, event: ev::PointerEvent| {
        hover_popup.set(Some(HoverPopup {
            title,
            body,
            left: f64::from(event.client_x()) + 14.0,
            top: f64::from(event.client_y()) - 16.0,
        }));
    };

    let hide_popup = move || {
        hover_popup.set(None);
    };

    let update_zoom = move |factor: f64, client_x: f64, client_y: f64| {
        let Some(viewport) = viewport_ref.get_untracked() else {
            return;
        };

        let rect = viewport.get_bounding_client_rect();
        let local_x = client_x - rect.left();
        let local_y = client_y - rect.top();
        let current_scale = scale.get_untracked();
        let next_scale = (current_scale * factor).clamp(0.45, 2.4);
        if (next_scale - current_scale).abs() < f64::EPSILON {
            return;
        }

        let current_pan = pan.get_untracked();
        let scene_x = (local_x - current_pan.0) / current_scale;
        let scene_y = (local_y - current_pan.1) / current_scale;
        pan.set((
            local_x - scene_x * next_scale,
            local_y - scene_y * next_scale,
        ));
        scale.set(next_scale);
    };

    let reset_view = move |_| {
        scale.set(1.0);
        pan.set((24.0, 24.0));
    };

    let zoom_in = move |_| {
        let Some(viewport) = viewport_ref.get() else {
            return;
        };
        let rect = viewport.get_bounding_client_rect();
        update_zoom(
            1.14,
            rect.left() + rect.width() * 0.5,
            rect.top() + rect.height() * 0.5,
        );
    };
    let zoom_out = move |_| {
        let Some(viewport) = viewport_ref.get() else {
            return;
        };
        let rect = viewport.get_bounding_client_rect();
        update_zoom(
            0.88,
            rect.left() + rect.width() * 0.5,
            rect.top() + rect.height() * 0.5,
        );
    };

    view! {
        <div class=class_name>
            <div
                class="birei-relation-graph__viewport"
                node_ref=viewport_ref
                role="region"
                aria-label=aria_label.unwrap_or_else(|| String::from("Relation graph"))
                style=format!("height: {height};")
                on:wheel=move |event: ev::WheelEvent| {
                    event.prevent_default();
                    let factor = if event.delta_y() < 0.0 { 1.03 } else { 0.97 };
                    update_zoom(
                        factor,
                        f64::from(event.client_x()),
                        f64::from(event.client_y()),
                    );
                }
                on:pointerdown=move |event: ev::PointerEvent| {
                    let is_background = event
                        .target()
                        .and_then(|target| target.dyn_into::<Element>().ok())
                        .and_then(|target| {
                            target
                                .closest(
                                    ".birei-relation-graph__node, .birei-relation-graph__load, .birei-relation-graph__controls",
                                )
                                .ok()
                                .flatten()
                        })
                        .is_none();

                    if !is_background {
                        return;
                    }

                    if let Some(viewport) = viewport_ref.get() {
                        let _ = viewport.set_pointer_capture(event.pointer_id());
                    }

                    event.prevent_default();
                    drag_pointer_id.set(Some(event.pointer_id()));
                    drag_origin.set(Some((
                        f64::from(event.client_x()),
                        f64::from(event.client_y()),
                    )));
                    hide_popup();
                }
                on:pointermove=move |event: ev::PointerEvent| {
                    if drag_pointer_id.get_untracked() != Some(event.pointer_id()) {
                        return;
                    }

                    let Some((last_x, last_y)) = drag_origin.get_untracked() else {
                        return;
                    };

                    let next_x = f64::from(event.client_x());
                    let next_y = f64::from(event.client_y());
                    let delta_x = next_x - last_x;
                    let delta_y = next_y - last_y;

                    pan.update(|(x, y)| {
                        *x += delta_x;
                        *y += delta_y;
                    });
                    drag_origin.set(Some((next_x, next_y)));
                }
                on:pointerup=move |event: ev::PointerEvent| {
                    if drag_pointer_id.get_untracked() == Some(event.pointer_id()) {
                        if let Some(viewport) = viewport_ref.get() {
                            let _ = viewport.release_pointer_capture(event.pointer_id());
                        }
                        drag_pointer_id.set(None);
                        drag_origin.set(None);
                    }
                }
                on:pointercancel=move |event: ev::PointerEvent| {
                    if drag_pointer_id.get_untracked() == Some(event.pointer_id()) {
                        if let Some(viewport) = viewport_ref.get() {
                            let _ = viewport.release_pointer_capture(event.pointer_id());
                        }
                        drag_pointer_id.set(None);
                        drag_origin.set(None);
                    }
                }
                on:pointerleave=move |_| {
                    hide_popup();
                }
            >
                <div class="birei-relation-graph__controls">
                    <Button
                        class="birei-relation-graph__zoom-button"
                        variant=ButtonVariant::Secondary
                        size=Size::Small
                        on_click=Callback::new(zoom_in)
                    >
                        "+"
                    </Button>
                    <Button
                        class="birei-relation-graph__zoom-button"
                        variant=ButtonVariant::Secondary
                        size=Size::Small
                        on_click=Callback::new(zoom_out)
                    >
                        "−"
                    </Button>
                    <Button
                        class="birei-relation-graph__zoom-button"
                        variant=ButtonVariant::Secondary
                        size=Size::Small
                        on_click=Callback::new(reset_view)
                    >
                        <Icon name="rotate-ccw" size=Size::Small/>
                    </Button>
                </div>

                <div
                    class="birei-relation-graph__scene"
                    style=move || {
                        let layout = layout.get();
                        let (pan_x, pan_y) = pan.get();
                        format!(
                            "width: {:.1}px; height: {:.1}px; transform: translate({:.1}px, {:.1}px) scale({:.3});",
                            layout.width,
                            layout.height,
                            pan_x,
                            pan_y,
                            scale.get(),
                        )
                    }
                >
                    {move || {
                        let layout = layout.get();
                        if layout.nodes.is_empty() {
                            return view! {
                                <div class="birei-relation-graph__empty">
                                    "No relations to display."
                                </div>
                            }
                                .into_any();
                        }

                        render_layout(
                            layout,
                            pending_loads,
                            on_load_node,
                            on_open_node,
                            show_popup,
                            hide_popup,
                        )
                        .into_any()
                    }}
                </div>
            </div>

            {move || {
                hover_popup.get().map(|popup| {
                    view! {
                        <div
                            class="birei-relation-graph__popup"
                            role="status"
                            aria-live="polite"
                            style=format!("left: {:.1}px; top: {:.1}px;", popup.left, popup.top)
                        >
                            <div class="birei-relation-graph__popup-title">{popup.title}</div>
                            {popup.body
                                .filter(|body| !body.trim().is_empty())
                                .map(|body| view! {
                                    <div class="birei-relation-graph__popup-body">{body}</div>
                                })}
                        </div>
                    }
                })
            }}
        </div>
    }
}

fn render_layout(
    layout: RelationGraphLayout,
    pending_loads: RwSignal<HashSet<Uuid>>,
    on_load_node: Option<Callback<Uuid>>,
    on_open_node: Option<Callback<Uuid>>,
    show_popup: impl Fn(String, Option<String>, ev::PointerEvent) + Copy + 'static,
    hide_popup: impl Fn() + Copy + 'static,
) -> impl IntoView {
    let width = layout.width;
    let height = layout.height;

    view! {
        <>
            <svg
                class="birei-relation-graph__edges"
                viewBox=format!("0 0 {:.1} {:.1}", width, height)
                aria-hidden="true"
            >
                <defs>
                    <marker
                        id="birei-relation-graph-arrow"
                        markerWidth="10"
                        markerHeight="10"
                        refX="8"
                        refY="5"
                        orient="auto"
                        markerUnits="userSpaceOnUse"
                    >
                        <path
                            class="birei-relation-graph__arrow"
                            d="M 0 1 L 8 5 L 0 9 z"
                        ></path>
                    </marker>
                </defs>

                {layout
                    .edges
                    .into_iter()
                    .map(|edge| {
                        let edge_name = edge.edge.name.clone();

                        view! {
                            <g class="birei-relation-graph__edge-group" data-key=edge.edge.id.to_string()>
                                {edge
                                    .paths
                                    .iter()
                                    .map(|path| {
                                        view! {
                                            <path
                                                class="birei-relation-graph__edge"
                                                d=path.d.clone()
                                                marker-end=path.arrow.then_some("url(#birei-relation-graph-arrow)")
                                            ></path>
                                        }
                                    })
                                    .collect_view()}
                                {edge
                                    .paths
                                    .into_iter()
                                    .map(|path| {
                                        let edge_name_enter = edge_name.clone();
                                        let edge_name_move = edge_name.clone();
                                        view! {
                                            <path
                                                class="birei-relation-graph__edge-hit"
                                                d=path.d
                                                on:pointerenter=move |event| show_popup(edge_name_enter.clone(), None, event)
                                                on:pointermove=move |event| show_popup(edge_name_move.clone(), None, event)
                                                on:pointerleave=move |_| hide_popup()
                                            ></path>
                                        }
                                    })
                                    .collect_view()}
                            </g>
                        }
                    })
                    .collect_view()}
            </svg>

            <div class="birei-relation-graph__nodes">
                {layout
                    .nodes
                    .into_iter()
                    .map(|node_layout| {
                        let node = node_layout.node.clone();
                        let node_id = node.id;
                        let node_name_enter = node.name.clone();
                        let node_name_move = node.name.clone();
                        let node_description_enter = node.description.clone();
                        let node_description_move = node.description.clone();
                        let icon_name = node.icon.clone();
                        let show_load = !node.loaded && !pending_loads.get().contains(&node.id);
                        let show_open = node.loaded;
                        let load_callback = on_load_node;
                        let open_callback = on_open_node;

                        view! {
                            <div
                                class="birei-relation-graph__node"
                                style=format!(
                                    "left: {:.1}px; top: {:.1}px; width: {:.1}px; min-height: {:.1}px;",
                                    node_layout.x,
                                    node_layout.y,
                                    NODE_WIDTH,
                                    NODE_HEIGHT,
                                )
                                on:pointerenter=move |event| {
                                    show_popup(
                                        node_name_enter.clone(),
                                        (!node_description_enter.trim().is_empty())
                                            .then_some(node_description_enter.clone()),
                                        event,
                                    )
                                }
                                on:pointermove=move |event| {
                                    show_popup(
                                        node_name_move.clone(),
                                        (!node_description_move.trim().is_empty())
                                            .then_some(node_description_move.clone()),
                                        event,
                                    )
                                }
                                on:pointerleave=move |_| hide_popup()
                            >
                                <div class="birei-relation-graph__node-main">
                                    <span class="birei-relation-graph__node-icon" aria-hidden="true">
                                        <Icon name=icon_name size=Size::Medium/>
                                    </span>
                                    <div class="birei-relation-graph__node-copy">
                                        <div class="birei-relation-graph__node-name">{node.name}</div>
                                    </div>
                                    {show_load.then(|| {
                                        view! {
                                            <button
                                                type="button"
                                                class="birei-relation-graph__load"
                                                aria-label="Open related nodes"
                                                on:click=move |event| {
                                                    event.stop_propagation();
                                                pending_loads.update(|pending| {
                                                    pending.insert(node_id);
                                                });
                                                if let Some(on_load_node) = load_callback.as_ref() {
                                                    on_load_node.run(node_id);
                                                }
                                            }
                                        >
                                            <Icon name="arrow-right" size=Size::Small/>
                                        </button>
                                    }
                                    })}
                                    {show_open.then(|| {
                                        view! {
                                            <button
                                                type="button"
                                                class="birei-relation-graph__load birei-relation-graph__load--open"
                                                aria-label="Open node"
                                                on:click=move |event| {
                                                    event.stop_propagation();
                                                    if let Some(on_open_node) = open_callback.as_ref() {
                                                        on_open_node.run(node_id);
                                                    }
                                                }
                                        >
                                                <Icon name="external-link" size=Size::Small/>
                                            </button>
                                        }
                                    })}
                                </div>
                            </div>
                        }
                    })
                    .collect_view()}
            </div>
        </>
    }
}
