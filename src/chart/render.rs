use leptos::ev;
use leptos::prelude::*;

use super::chart_types::{ChartDatumKind, ChartLegendPosition, ChartType};
use super::chart_utils::{CHART_VIEWBOX_HEIGHT, CHART_VIEWBOX_WIDTH};
use super::internal::{ChartLayout, HoverPayload, LegendEntry, PieLayout};

pub(crate) fn render_layout(
    layout: ChartLayout,
    chart_type: ChartType,
    aria_label: Option<String>,
    on_hover: Callback<(ev::PointerEvent, HoverPayload)>,
    on_leave: Callback<()>,
) -> AnyView {
    match layout {
        ChartLayout::Bar(layout) => {
            let label = aria_label.unwrap_or_else(|| String::from("Bar chart"));

            view! {
                <svg
                    class="birei-chart__svg"
                    viewBox=format!("0 0 {CHART_VIEWBOX_WIDTH} {CHART_VIEWBOX_HEIGHT}")
                    role="img"
                    aria-label=label
                    preserveAspectRatio="xMidYMid meet"
                >
                    <defs>
                        <linearGradient id="birei-chart-bar-gradient" x1="0%" y1="0%" x2="0%" y2="100%">
                            <stop offset="0%" stop-color="rgba(255,255,255,0.16)"></stop>
                            <stop offset="100%" stop-color="rgba(255,255,255,0)"></stop>
                        </linearGradient>
                    </defs>

                    <g class="birei-chart__grid" aria-hidden="true">
                        <line
                            class="birei-chart__axis"
                            x1=layout.chart_left
                            y1=layout.chart_bottom
                            x2=layout.chart_right
                            y2=layout.chart_bottom
                        ></line>

                        {layout
                            .grid
                            .into_iter()
                            .map(|(y, value)| {
                                view! {
                                    <g>
                                        <line
                                            class="birei-chart__grid-line"
                                            x1=layout.chart_left
                                            y1=y
                                            x2=layout.chart_right
                                            y2=y
                                        ></line>
                                        <text
                                            class="birei-chart__axis-label birei-chart__axis-label--y"
                                            x=layout.chart_left - 16.0
                                            y=y + 4.0
                                            text-anchor="end"
                                        >
                                            {value}
                                        </text>
                                    </g>
                                }
                            })
                            .collect_view()}
                    </g>

                    <g class="birei-chart__bars">
                        {layout
                            .bars
                            .into_iter()
                            .map(|bar| {
                                let payload = HoverPayload {
                                    title: bar.title.clone(),
                                    group: bar.group.clone(),
                                    value: bar.value,
                                };

                                view! {
                                    <g class="birei-chart__bar-group" data-key=bar.key.clone()>
                                        <rect
                                            class="birei-chart__bar"
                                            x=bar.x
                                            y=bar.y
                                            width=bar.width
                                            height=bar.height
                                            rx="4"
                                            ry="4"
                                            style=format!("--birei-chart-series-color: {};", bar.color)
                                            on:pointerenter={
                                                let payload = payload.clone();
                                                move |event| on_hover.run((event, payload.clone()))
                                            }
                                            on:pointermove={
                                                let payload = payload.clone();
                                                move |event| on_hover.run((event, payload.clone()))
                                            }
                                            on:pointerleave=move |_| on_leave.run(())
                                        ></rect>
                                        <rect
                                            class="birei-chart__bar-sheen"
                                            x=bar.x
                                            y=bar.y
                                            width=bar.width
                                            height=bar.height
                                            rx="4"
                                            ry="4"
                                        ></rect>
                                    </g>
                                }
                            })
                            .collect_view()}
                    </g>

                    <g class="birei-chart__lines">
                        {layout
                            .lines
                            .into_iter()
                            .map(|series| {
                                view! {
                                    <g class="birei-chart__line-group" data-key=series.key>
                                        <path
                                            class="birei-chart__line"
                                            d=series.path
                                            pathLength="1"
                                            style=format!("--birei-chart-series-color: {};", series.color)
                                        >
                                            <title>{series.title.clone()}</title>
                                        </path>
                                        {series
                                            .points
                                            .into_iter()
                                            .map(|point| {
                                                let payload = HoverPayload {
                                                    title: point.title.clone(),
                                                    group: point.group.clone(),
                                                    value: point.value,
                                                };

                                                view! {
                                                    <circle
                                                        class="birei-chart__point"
                                                        cx=point.x
                                                        cy=point.y
                                                        r="6"
                                                        style=format!("--birei-chart-series-color: {};", point.color)
                                                        data-key=point.key
                                                        on:pointerenter={
                                                            let payload = payload.clone();
                                                            move |event| on_hover.run((event, payload.clone()))
                                                        }
                                                        on:pointermove={
                                                            let payload = payload.clone();
                                                            move |event| on_hover.run((event, payload.clone()))
                                                        }
                                                        on:pointerleave=move |_| on_leave.run(())
                                                    ></circle>
                                                }
                                            })
                                            .collect_view()}
                                    </g>
                                }
                            })
                            .collect_view()}
                    </g>

                    <g class="birei-chart__x-axis">
                        {layout
                            .x_labels
                            .into_iter()
                            .map(|(x, label)| {
                                view! {
                                    <text
                                        class="birei-chart__axis-label"
                                        x=x
                                        y=layout.chart_bottom + 24.0
                                        text-anchor="middle"
                                    >
                                        {label}
                                    </text>
                                }
                            })
                            .collect_view()}
                    </g>
                </svg>
            }
            .into_any()
        }
        ChartLayout::Pie(layout) => {
            render_pie_layout(layout, chart_type, aria_label, on_hover, on_leave).into_any()
        }
    }
}

pub(crate) fn render_legend(layout: ChartLayout, legend_position: ChartLegendPosition) -> AnyView {
    if legend_position == ChartLegendPosition::None {
        return ().into_any();
    }

    let legend = match layout {
        ChartLayout::Bar(layout) => layout.legend,
        ChartLayout::Pie(layout) => layout.legend,
    };

    view! {
        <div class="birei-chart__legend" role="list">
            {legend
                .into_iter()
                .map(render_legend_entry)
                .collect_view()}
        </div>
    }
    .into_any()
}

fn render_pie_layout(
    layout: PieLayout,
    chart_type: ChartType,
    aria_label: Option<String>,
    on_hover: Callback<(ev::PointerEvent, HoverPayload)>,
    on_leave: Callback<()>,
) -> AnyView {
    let label = aria_label.unwrap_or_else(|| match chart_type {
        ChartType::Doughnut => String::from("Doughnut chart"),
        _ => String::from("Pie chart"),
    });
    if matches!(chart_type, ChartType::Doughnut) {
        view! {
            <div class="birei-chart__pie-wrap" data-chart-type="doughnut">
                <svg
                    class="birei-chart__svg birei-chart__svg--pie"
                    viewBox="0 0 360 360"
                    role="img"
                    aria-label=label
                    preserveAspectRatio="xMidYMid meet"
                >
                    <g class="birei-chart__pie-group">
                        {layout
                            .slices
                            .into_iter()
                            .map(|slice| {
                                let payload = HoverPayload {
                                    title: slice.title.clone(),
                                    group: slice.group.clone(),
                                    value: slice.value,
                                };

                                view! {
                                    <path
                                        class="birei-chart__slice"
                                        d=slice.path
                                        style=format!("--birei-chart-series-color: {};", slice.color)
                                        data-key=slice.key
                                        on:pointerenter={
                                            let payload = payload.clone();
                                            move |event| on_hover.run((event, payload.clone()))
                                        }
                                        on:pointermove={
                                            let payload = payload.clone();
                                            move |event| on_hover.run((event, payload.clone()))
                                        }
                                        on:pointerleave=move |_| on_leave.run(())
                                    ></path>
                                }
                            })
                            .collect_view()}
                    </g>
                </svg>

                <div class="birei-chart__center">
                    <span class="birei-chart__center-label">"Total"</span>
                    <strong class="birei-chart__center-value">
                        {format!("{:.0}", layout.total)}
                    </strong>
                </div>
            </div>
        }
        .into_any()
    } else {
        view! {
            <div class="birei-chart__pie-wrap" data-chart-type="pie">
                <svg
                    class="birei-chart__svg birei-chart__svg--pie"
                    viewBox="0 0 360 360"
                    role="img"
                    aria-label=label
                    preserveAspectRatio="xMidYMid meet"
                >
                    <g class="birei-chart__pie-group">
                        {layout
                            .slices
                            .into_iter()
                            .map(|slice| {
                                let payload = HoverPayload {
                                    title: slice.title.clone(),
                                    group: slice.group.clone(),
                                    value: slice.value,
                                };

                                view! {
                                    <path
                                        class="birei-chart__slice"
                                        d=slice.path
                                        style=format!("--birei-chart-series-color: {};", slice.color)
                                        data-key=slice.key
                                        on:pointerenter={
                                            let payload = payload.clone();
                                            move |event| on_hover.run((event, payload.clone()))
                                        }
                                        on:pointermove={
                                            let payload = payload.clone();
                                            move |event| on_hover.run((event, payload.clone()))
                                        }
                                        on:pointerleave=move |_| on_leave.run(())
                                    ></path>
                                }
                            })
                            .collect_view()}
                    </g>
                </svg>
            </div>
        }
        .into_any()
    }
}

fn render_legend_entry(entry: LegendEntry) -> impl IntoView {
    let item_class = if entry.kind == ChartDatumKind::Line {
        "birei-chart__legend-item birei-chart__legend-item--line"
    } else {
        "birei-chart__legend-item"
    };
    let swatch_class = if entry.kind == ChartDatumKind::Line {
        "birei-chart__legend-swatch birei-chart__legend-swatch--line"
    } else {
        "birei-chart__legend-swatch"
    };

    view! {
        <div class=item_class role="listitem" data-key=entry.key>
            <span
                class=swatch_class
                aria-hidden="true"
                style=format!("--birei-chart-series-color: {};", entry.color)
            ></span>
            <span class="birei-chart__legend-label">{entry.label}</span>
        </div>
    }
}
