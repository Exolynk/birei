use leptos::ev;
use leptos::html;
use leptos::prelude::*;
use wasm_bindgen::closure::Closure;
use wasm_bindgen::JsCast;
use web_sys::{window, ResizeObserver};

use super::FlexibleColumn;
use crate::{IcnName, Icon, Size};

const DEFAULT_RATIOS: [f32; 3] = [20.0, 60.0, 20.0];
const DIVIDER_WIDTH_PX: f64 = 18.0;

/// Breakpoints used to decide how many columns can be shown at once.
#[derive(Clone, Copy, PartialEq, Eq)]
enum ResponsiveTier {
    Phone,
    Tablet,
    Desktop,
}

/// Active drag data for one divider interaction.
#[derive(Clone, Copy)]
struct DragState {
    left_index: usize,
    right_index: usize,
    pair_start_px: f64,
    pair_width_px: f64,
    available_total: f32,
}

/// One column as it should be rendered after responsive filtering.
#[derive(Clone)]
struct RenderColumn {
    index: usize,
    width: f32,
    focused: bool,
}

/// Precomputed layout consumed directly by the view.
#[derive(Clone, Default)]
struct RenderLayout {
    columns: Vec<RenderColumn>,
    divider_count: usize,
    template: String,
}

/// Responsive three-column layout with draggable separators and focus-aware collapse.
#[component]
pub fn FlexibleColumns(
    #[prop(into)] start: ViewFn,
    #[prop(into)] middle: ViewFn,
    #[prop(into)] end: ViewFn,
    #[prop(optional, into)] focused: MaybeProp<FlexibleColumn>,
    #[prop(optional, into)] initial_ratios: MaybeProp<[f32; 3]>,
    #[prop(optional, into)] available_columns: MaybeProp<[bool; 3]>,
    #[prop(optional)] on_focus_change: Option<Callback<FlexibleColumn>>,
    #[prop(optional)] on_ratios_change: Option<Callback<[f32; 3]>>,
    #[prop(optional, into)] class: Option<String>,
) -> impl IntoView {
    // The root element is measured so the layout can respond to its actual
    // width, not just the global viewport size.
    let root_ref = NodeRef::<html::Div>::new();
    let resize_observer_attached = RwSignal::new(false);
    let container_width = RwSignal::new(0_f64);
    let initial_focus = focused.get_untracked().unwrap_or_default();
    let focused_column = RwSignal::new(initial_focus);
    let initial_layout = initial_ratios
        .get_untracked()
        .unwrap_or_else(|| initial_focus_preset(initial_focus));
    let ratios = RwSignal::new(normalize_ratios(initial_layout));
    let drag_state = RwSignal::new(None::<DragState>);
    let resize_observer = StoredValue::new_local(None::<ResizeObserver>);
    let resize_callback =
        StoredValue::new_local(None::<Closure<dyn FnMut(js_sys::Array, ResizeObserver)>>);
    let pending_drag_ratios = StoredValue::new_local(None::<[f32; 3]>);
    let drag_frame = StoredValue::new_local(None::<i32>);
    let drag_frame_callback = StoredValue::new_local(None::<Closure<dyn FnMut(f64)>>);

    // Root classes reflect drag state and any optional external hook class.
    let class_name = move || {
        let mut classes = vec!["birei-flex-columns"];
        if drag_state.get().is_some() {
            classes.push("birei-flex-columns--dragging");
        }
        if let Some(class) = class.as_deref() {
            classes.push(class);
        }
        classes.join(" ")
    };

    // Rendering is driven by a precomputed layout struct so the template and
    // panel metadata stay in sync.
    let render_layout = move || {
        compute_render_layout(
            container_width.get(),
            ratios.get(),
            available_columns.get().unwrap_or([true, true, true]),
            focused_column.get(),
        )
    };

    // Local ratio updates are used during drag for responsive preview, while
    // committed updates also notify the consumer.
    let set_ratios_local = move |next: [f32; 3]| {
        let normalized = normalize_ratios(next);
        ratios.set(normalized);
    };

    // Persist a new ratio set and emit the controlled callback if present.
    let commit_ratios = move |next: [f32; 3]| {
        let normalized = normalize_ratios(next);
        ratios.set(normalized);
        if let Some(on_ratios_change) = on_ratios_change.as_ref() {
            on_ratios_change.run(normalized);
        }
    };

    // Focus changes are tracked internally and optionally exposed to the
    // consumer that owns the surrounding workflow.
    let announce_focus = move |next: FlexibleColumn| {
        focused_column.set(next);
        if let Some(on_focus_change) = on_focus_change.as_ref() {
            on_focus_change.run(next);
        }
    };

    // Controlled focus updates replace the local focused column when provided.
    Effect::new(move |_| {
        if let Some(next) = focused.get() {
            focused_column.set(next);
        }
    });

    // Controlled ratio updates replace the local layout state when provided.
    Effect::new(move |_| {
        if let Some(next) = initial_ratios.get() {
            ratios.set(normalize_ratios(next));
        }
    });

    // A resize observer keeps the responsive layout synced to the component's
    // actual rendered width. The effect tracks the node ref so delayed mounts
    // still attach an observer instead of getting stuck with width 0.
    Effect::new(move |_| {
        let Some(root) = root_ref.get() else {
            return;
        };

        container_width.set(f64::from(root.client_width()));

        let callback = Closure::wrap(Box::new(
            move |_entries: js_sys::Array, _observer: ResizeObserver| {
                if let Some(root) = root_ref.get_untracked() {
                    container_width.set(f64::from(root.client_width()));
                }
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

    // Dragging is handled at window scope so pointer movement continues even
    // when the cursor leaves the divider hit area.
    Effect::new(move |_| {
        let Some(state) = drag_state.get() else {
            return;
        };

        let frame_callback = Closure::wrap(Box::new(move |_timestamp: f64| {
            drag_frame.update_value(|frame| *frame = None);
            if let Some(next) = pending_drag_ratios.get_value() {
                set_ratios_local(next);
            }
        }) as Box<dyn FnMut(f64)>);
        drag_frame_callback.update_value(|stored| *stored = Some(frame_callback));

        let move_handle = window_event_listener_untyped("mousemove", {
            move |event| {
                let Ok(event) = event.dyn_into::<web_sys::MouseEvent>() else {
                    return;
                };

                let pair_width = state.pair_width_px.max(1.0);
                let raw_position = f64::from(event.client_x()) - state.pair_start_px;
                let ratio = (raw_position / pair_width).clamp(0.0, 1.0) as f32;
                let mut next = ratios.get_untracked();
                next[state.left_index] = state.available_total * ratio;
                next[state.right_index] = state.available_total - next[state.left_index];
                pending_drag_ratios.update_value(|pending| *pending = Some(next));

                if drag_frame.get_value().is_some() {
                    return;
                }

                let Some(window) = window() else {
                    set_ratios_local(next);
                    return;
                };

                let scheduled = drag_frame_callback.try_with_value(|stored| {
                    let Some(callback) = stored.as_ref() else {
                        return false;
                    };

                    if let Ok(frame_id) =
                        window.request_animation_frame(callback.as_ref().unchecked_ref())
                    {
                        drag_frame.update_value(|frame| *frame = Some(frame_id));
                        true
                    } else {
                        false
                    }
                });

                if scheduled != Some(true) {
                    set_ratios_local(next);
                }
            }
        });
        let up_handle = window_event_listener_untyped("mouseup", move |_| {
            if let Some(next) = pending_drag_ratios.get_value() {
                set_ratios_local(next);
            }
            pending_drag_ratios.update_value(|pending| *pending = None);
            if let Some(frame_id) = drag_frame.get_value() {
                if let Some(window) = window() {
                    let _ = window.cancel_animation_frame(frame_id);
                }
                drag_frame.update_value(|frame| *frame = None);
            }
            commit_ratios(ratios.get_untracked());
            drag_state.set(None);
        });

        on_cleanup(move || {
            move_handle.remove();
            up_handle.remove();
            pending_drag_ratios.update_value(|pending| *pending = None);
            if let Some(frame_id) = drag_frame.get_value() {
                if let Some(window) = window() {
                    let _ = window.cancel_animation_frame(frame_id);
                }
                drag_frame.update_value(|frame| *frame = None);
            }
            drag_frame_callback.update_value(|stored| {
                stored.take();
            });
        });
    });

    view! {
        <div
            node_ref=root_ref
            class=class_name
            style=move || format!("grid-template-columns: {};", render_layout().template)
        >
            {move || {
                let layout = render_layout();
                let columns = layout.columns.clone();

                columns
                    .into_iter()
                    .enumerate()
                    .flat_map(|(position, column)| {
                        let column_index = column.index;
                        let column_label = FlexibleColumn::from_index(column_index).aria_label();
                        let children = match column_index {
                            0 => start.clone(),
                            1 => middle.clone(),
                            _ => end.clone(),
                        };

                        let mut views = vec![view! {
                            <section
                                class=move || {
                                    let mut classes = String::from("birei-flex-columns__panel");
                                    if column.focused {
                                        classes.push_str(" birei-flex-columns__panel--focused");
                                    }
                                    classes
                                }
                                data-column=column_index.to_string()
                                aria-label=column_label
                            >
                                <div class="birei-flex-columns__panel-body">
                                    {children.run()}
                                </div>
                            </section>
                        }
                        .into_any()];

                        if let Some(next_column) = layout.columns.get(position + 1) {
                            let left_index = column.index;
                            let right_index = next_column.index;

                            views.push(
                                view! {
                                    <div
                                        class="birei-flex-columns__divider"
                                        on:mousedown=move |event: ev::MouseEvent| {
                                            event.prevent_default();

                                            let layout = render_layout();
                                            let total_divider_width =
                                                DIVIDER_WIDTH_PX * layout.divider_count as f64;
                                            let usable_width =
                                                (container_width.get_untracked() - total_divider_width)
                                                    .max(1.0);
                                            let root_left = root_ref
                                                .get_untracked()
                                                .map(|root| root.get_bounding_client_rect().left())
                                                .unwrap_or(0.0);
                                            let Some(left_position) = layout
                                                .columns
                                                .iter()
                                                .position(|candidate| candidate.index == left_index)
                                            else {
                                                return;
                                            };
                                            let Some(right_position) = layout
                                                .columns
                                                .iter()
                                                .position(|candidate| candidate.index == right_index)
                                            else {
                                                return;
                                            };

                                            let left_width = f64::from(layout.columns[left_position].width)
                                                / 100.0
                                                * usable_width;
                                            let right_width =
                                                f64::from(layout.columns[right_position].width)
                                                    / 100.0
                                                    * usable_width;
                                            let preceding_width = layout.columns[..left_position]
                                                .iter()
                                                .map(|column| {
                                                    f64::from(column.width) / 100.0 * usable_width
                                                })
                                                .sum::<f64>()
                                                + DIVIDER_WIDTH_PX * left_position as f64;
                                            let available_total = (100.0
                                                - ratios
                                                    .get_untracked()
                                                    .iter()
                                                    .enumerate()
                                                    .filter(|(index, _)| {
                                                        *index != left_index && *index != right_index
                                                    })
                                                    .map(|(_, ratio)| *ratio)
                                                    .sum::<f32>())
                                            .max(0.0);

                                            drag_state.set(Some(DragState {
                                                left_index,
                                                right_index,
                                                pair_start_px: root_left + preceding_width,
                                                pair_width_px: left_width + right_width,
                                                available_total,
                                            }));
                                        }
                                    >
                                        <div class="birei-flex-columns__divider-rail">
                                            <button
                                                type="button"
                                                class="birei-button birei-button--transparent birei-flex-columns__divider-action"
                                                aria-label="Toggle left-side column"
                                                on:click=move |_| {
                                                    let next = divider_action_ratios(
                                                        ratios.get_untracked(),
                                                        left_index,
                                                        right_index,
                                                        false,
                                                    );
                                                    commit_ratios(next);
                                                    announce_focus(FlexibleColumn::Middle);
                                                }
                                            >
                                                <Icon
                                                    name=divider_icon_name(left_index, false)
                                                    size=Size::Small
                                                    label="Maximize left column"
                                                />
                                            </button>
                                            <span class="birei-flex-columns__divider-handle" aria-hidden="true">
                                                <Icon name="grip-vertical" size=Size::Small label="Resize columns"/>
                                            </span>
                                            <button
                                                type="button"
                                                class="birei-button birei-button--transparent birei-flex-columns__divider-action"
                                                aria-label="Toggle right-side column"
                                                on:click=move |_| {
                                                    let next = divider_action_ratios(
                                                        ratios.get_untracked(),
                                                        left_index,
                                                        right_index,
                                                        true,
                                                    );
                                                    commit_ratios(next);
                                                    announce_focus(FlexibleColumn::Middle);
                                                }
                                            >
                                                <Icon
                                                    name=divider_icon_name(right_index, true)
                                                    size=Size::Small
                                                    label="Maximize right column"
                                                />
                                            </button>
                                        </div>
                                    </div>
                                }
                                .into_any(),
                            );
                        }

                        views
                    })
                    .collect_view()
            }}
        </div>
    }
}

/// Starting ratios are biased toward the initially focused column.
fn initial_focus_preset(focused: FlexibleColumn) -> [f32; 3] {
    match focused {
        FlexibleColumn::Start => [60.0, 25.0, 15.0],
        FlexibleColumn::Middle => DEFAULT_RATIOS,
        FlexibleColumn::End => [15.0, 25.0, 60.0],
    }
}

/// Maps the current width to the rendering rules used by the component.
fn responsive_tier(width: f64) -> ResponsiveTier {
    if width < 599.0 {
        ResponsiveTier::Phone
    } else if width < 1024.0 {
        ResponsiveTier::Tablet
    } else {
        ResponsiveTier::Desktop
    }
}

/// Keeps ratios non-negative and normalized to a 100% total.
fn normalize_ratios(ratios: [f32; 3]) -> [f32; 3] {
    let clamped = ratios.map(|ratio| ratio.max(0.0));
    let total = clamped.iter().sum::<f32>();

    if total <= f32::EPSILON {
        return DEFAULT_RATIOS;
    }

    clamped.map(|ratio| ratio / total * 100.0)
}

/// Builds the visible column list and CSS grid template for the current width,
/// availability, focus, and ratios.
fn compute_render_layout(
    width: f64,
    ratios: [f32; 3],
    available_columns: [bool; 3],
    focused: FlexibleColumn,
) -> RenderLayout {
    let tier = responsive_tier(width);
    let focus_index = focused.index();

    if tier == ResponsiveTier::Phone {
        let phone_index = if available_columns[focus_index] {
            focus_index
        } else {
            available_columns
                .iter()
                .position(|available| *available)
                .unwrap_or(focus_index)
        };
        return RenderLayout {
            columns: vec![RenderColumn {
                index: phone_index,
                width: 100.0,
                focused: phone_index == focus_index,
            }],
            divider_count: 0,
            template: String::from("minmax(0, 1fr)"),
        };
    }

    let mut visible = ratios
        .iter()
        .enumerate()
        .filter_map(|(index, ratio)| {
            (available_columns[index] && (*ratio > 0.01 || *ratio <= 0.01)).then_some(index)
        })
        .collect::<Vec<_>>();

    if visible.is_empty() {
        if let Some(index) = available_columns.iter().position(|available| *available) {
            visible.push(index);
        } else {
            visible.push(focus_index);
        }
    }

    if tier == ResponsiveTier::Tablet && visible.len() > 2 {
        visible = match focused {
            FlexibleColumn::Start => vec![0, 1],
            FlexibleColumn::Middle => vec![1, 2],
            FlexibleColumn::End => vec![1, 2],
        }
        .into_iter()
        .filter(|index| available_columns[*index])
        .collect();
    }

    let visible = match tier {
        ResponsiveTier::Desktop => visible,
        ResponsiveTier::Tablet => visible,
        ResponsiveTier::Phone => unreachable!(),
    };

    let visible_total = visible.iter().map(|index| ratios[*index]).sum::<f32>();

    let columns = visible
        .iter()
        .map(|index| RenderColumn {
            index: *index,
            width: if visible_total <= f32::EPSILON {
                if *index == focus_index {
                    100.0
                } else {
                    0.0
                }
            } else {
                ratios[*index] / visible_total * 100.0
            },
            focused: *index == focus_index,
        })
        .collect::<Vec<_>>();

    let mut template_parts = Vec::new();
    for (index, column) in columns.iter().enumerate() {
        template_parts.push(format!("minmax(0, {}fr)", column.width.max(0.001)));
        if index + 1 < columns.len() {
            template_parts.push(format!("{DIVIDER_WIDTH_PX}px"));
        }
    }

    RenderLayout {
        columns,
        divider_count: visible.len().saturating_sub(1),
        template: template_parts.join(" "),
    }
}

/// Picks the divider action icon based on which neighboring side is being
/// emphasized by the quick-toggle button.
fn divider_icon_name(column_index: usize, emphasize_right: bool) -> IcnName {
    match (column_index, emphasize_right) {
        (0, _) => "arrow-left".into(),
        (1, false) => "arrow-left".into(),
        (1, true) => "arrow-right".into(),
        (_, _) => "arrow-right".into(),
    }
}

/// Applies one divider action button press by snapping to the next meaningful
/// stop while respecting the collapsed-middle-column behavior.
fn divider_action_ratios(
    current: [f32; 3],
    left_index: usize,
    right_index: usize,
    toward_right: bool,
) -> [f32; 3] {
    const EPSILON: f32 = 0.01;

    let mut boundaries = [current[0], current[0] + current[1]];
    let boundary_index = left_index.min(boundaries.len() - 1);
    let current_position = boundaries[boundary_index];
    let target_position = next_divider_stop(current_position, toward_right);
    let mut delta = target_position - current_position;

    // If the middle column is collapsed, only drag both dividers together when the move
    // would push further into the collapsed column. Moving away from the collapse should
    // separate the dividers normally and reopen the middle column.
    let pushes_into_collapsed_middle = current[1] <= EPSILON
        && ((left_index == 0 && right_index == 1 && toward_right)
            || (left_index == 1 && right_index == 2 && !toward_right));

    if pushes_into_collapsed_middle {
        delta = delta.clamp(-boundaries[0], 100.0 - boundaries[1]);
        boundaries[0] += delta;
        boundaries[1] += delta;
    } else {
        let lower_bound = if boundary_index == 0 {
            0.0
        } else {
            boundaries[boundary_index - 1]
        };
        let upper_bound = if boundary_index + 1 >= boundaries.len() {
            100.0
        } else {
            boundaries[boundary_index + 1]
        };

        boundaries[boundary_index] =
            (boundaries[boundary_index] + delta).clamp(lower_bound, upper_bound);
    }

    normalize_ratios([
        boundaries[0],
        (boundaries[1] - boundaries[0]).max(0.0),
        (100.0 - boundaries[1]).max(0.0),
    ])
}

/// Snaps divider actions to coarse percentage stops so toggle buttons feel
/// intentional instead of moving by tiny deltas.
fn next_divider_stop(current_position: f32, toward_right: bool) -> f32 {
    const STEP: f32 = 20.0;
    const MIN_MOVE: f32 = 10.0;
    const EPSILON: f32 = 0.01;

    let mut stop = if toward_right {
        ((current_position / STEP).floor() + 1.0) * STEP
    } else {
        ((current_position / STEP).ceil() - 1.0) * STEP
    };

    stop = stop.clamp(0.0, 100.0);

    let mut movement = (stop - current_position).abs();
    if movement < MIN_MOVE {
        stop = if toward_right {
            (stop + STEP).clamp(0.0, 100.0)
        } else {
            (stop - STEP).clamp(0.0, 100.0)
        };
        movement = (stop - current_position).abs();
    }

    if movement < EPSILON {
        if toward_right {
            100.0
        } else {
            0.0
        }
    } else {
        stop
    }
}
