use leptos::ev;
use leptos::prelude::*;

use super::chart_types::{ChartData, ChartLegendPosition, ChartType};
use super::chart_utils::request_animation_frame_once;
use super::internal::{ChartLayout, HoverPayload, HoverPopup};
use super::layout::{build_bar_layout, build_pie_layout};
use super::render::{render_layout, render_legend};

/// Single configurable chart component that renders bar, pie, or doughnut charts.
#[component]
pub fn Chart(
    /// Selects how the chart is rendered.
    #[prop(optional)]
    chart_type: ChartType,
    /// Flat chart input model shared across all render modes.
    #[prop(into)]
    data: MaybeProp<Vec<ChartData>>,
    /// Accessible label announced for the chart region.
    #[prop(optional, into)]
    aria_label: Option<String>,
    /// Optional fixed y-axis maximum for bar charts.
    #[prop(optional, into)]
    y_max: Option<f64>,
    /// Controls where the legend is rendered.
    #[prop(optional)]
    legend_position: ChartLegendPosition,
    /// Inline chart height in CSS units.
    #[prop(optional, default = String::from("20rem"))]
    height: String,
    /// Toggles the mount animation.
    #[prop(optional, default = true)]
    animated: bool,
    /// Additional class names applied to the root element.
    #[prop(optional, into)]
    class: Option<String>,
) -> impl IntoView {
    let entered = RwSignal::new(!animated);
    let hover_popup = RwSignal::new(None::<HoverPopup>);

    Effect::new(move |_| {
        if animated && !entered.get_untracked() {
            request_animation_frame_once({
                let entered = entered;
                move || entered.set(true)
            });
        }
    });

    let chart_class = move || {
        let mut classes = vec!["birei-chart", legend_position.class_name()];

        classes.push(match chart_type {
            ChartType::Bar => "birei-chart--bar",
            ChartType::Pie => "birei-chart--pie",
            ChartType::Doughnut => "birei-chart--doughnut",
        });

        if entered.get() {
            classes.push("birei-chart--entered");
        }
        if let Some(class) = class.as_deref() {
            classes.push(class);
        }

        classes.join(" ")
    };

    let layout = Memo::new(move |_| match chart_type {
        ChartType::Bar => ChartLayout::Bar(build_bar_layout(data.get().unwrap_or_default(), y_max)),
        ChartType::Pie | ChartType::Doughnut => {
            ChartLayout::Pie(build_pie_layout(data.get().unwrap_or_default(), chart_type))
        }
    });

    let legend_view = move || render_legend(layout.get(), legend_position);
    let show_popup = Callback::new(move |(event, payload): (ev::PointerEvent, HoverPayload)| {
        hover_popup.set(Some(HoverPopup {
            title: payload.title,
            group: payload.group,
            value: payload.value,
            left: f64::from(event.client_x()) + 14.0,
            top: f64::from(event.client_y()) - 18.0,
        }));
    });
    let hide_popup = Callback::new(move |_| {
        hover_popup.set(None);
    });

    view! {
        <div class=chart_class style=format!("--birei-chart-height: {height};")>
            {move || render_layout(
                layout.get(),
                chart_type,
                aria_label.clone(),
                show_popup,
                hide_popup,
            )}

            {legend_view}

            {move || {
                hover_popup.get().map(|popup| {
                    let show_group = !popup.group.trim().is_empty();
                    let group_text = popup.group.clone();

                    view! {
                        <div
                            class="birei-chart__popup"
                            role="status"
                            aria-live="polite"
                            style=format!("left: {:.1}px; top: {:.1}px;", popup.left, popup.top)
                        >
                            <div class="birei-chart__popup-title">{popup.title}</div>
                            {show_group.then(|| {
                                view! {
                                    <div class="birei-chart__popup-row">
                                        <span class="birei-chart__popup-label">"Group"</span>
                                        <span class="birei-chart__popup-value">{group_text}</span>
                                    </div>
                                }
                            })}
                            <div class="birei-chart__popup-row">
                                <span class="birei-chart__popup-label">"Value"</span>
                                <span class="birei-chart__popup-value">{format!("{:.0}", popup.value)}</span>
                            </div>
                        </div>
                    }
                })
            }}
        </div>
    }
}
