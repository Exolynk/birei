use crate::code_example::CodeExample;
use birei::{
    Card, Chart, ChartData, ChartLegendPosition, ChartType, Label, Select, SelectOption,
};
use leptos::prelude::*;

#[component]
pub fn ChartPage() -> impl IntoView {
    let bar_data = vec![
        ChartData::new("Revenue", 18.0).group("Jan").color("#1d7dfa"),
        ChartData::new("Costs", 11.0).group("Jan").color("#69b578"),
        ChartData::new("Forecast", 16.0)
            .group("Jan")
            .color("#f28f3b")
            .line(),
        ChartData::new("Revenue", 24.0).group("Feb").color("#1d7dfa"),
        ChartData::new("Costs", 15.0).group("Feb").color("#69b578"),
        ChartData::new("Forecast", 20.0)
            .group("Feb")
            .color("#f28f3b")
            .line(),
        ChartData::new("Revenue", 29.0).group("Mar").color("#1d7dfa"),
        ChartData::new("Costs", 19.0).group("Mar").color("#69b578"),
        ChartData::new("Forecast", 24.0)
            .group("Mar")
            .color("#f28f3b")
            .line(),
        ChartData::new("Revenue", 26.0).group("Apr").color("#1d7dfa"),
        ChartData::new("Costs", 17.0).group("Apr").color("#69b578"),
        ChartData::new("Forecast", 27.0)
            .group("Apr")
            .color("#f28f3b")
            .line(),
        ChartData::new("Revenue", 34.0).group("May").color("#1d7dfa"),
        ChartData::new("Costs", 22.0).group("May").color("#69b578"),
        ChartData::new("Forecast", 30.0)
            .group("May")
            .color("#f28f3b")
            .line(),
        ChartData::new("Revenue", 39.0).group("Jun").color("#1d7dfa"),
        ChartData::new("Costs", 25.0).group("Jun").color("#69b578"),
        ChartData::new("Forecast", 35.0)
            .group("Jun")
            .color("#f28f3b")
            .line(),
    ];

    let pie_data = vec![
        ChartData::new("Design", 22.0).color("#f25f5c"),
        ChartData::new("Product", 31.0).color("#1d7dfa"),
        ChartData::new("Ops", 18.0).color("#6c9a8b"),
        ChartData::new("Sales", 29.0).color("#f2c14e"),
    ];
    let doughnut_data = pie_data.clone();
    let grouped_doughnut_data = vec![
        ChartData::new("Enterprise", 32.0)
            .group("North")
            .color("#1d7dfa"),
        ChartData::new("Retail", 18.0).group("North").color("#69b578"),
        ChartData::new("Target", 44.0)
            .group("North")
            .color("#8b5cf6")
            .line(),
        ChartData::new("Enterprise", 27.0)
            .group("South")
            .color("#f28f3b"),
        ChartData::new("Retail", 23.0).group("South").color("#f25f5c"),
        ChartData::new("Target", 39.0)
            .group("South")
            .color("#8b5cf6")
            .line(),
    ];
    let playground_chart_type = RwSignal::new(Some(String::from("doughnut")));
    let playground_legend = RwSignal::new(Some(String::from("right")));
    let chart_type_options = vec![
        SelectOption::new("bar", "Bar"),
        SelectOption::new("pie", "Pie"),
        SelectOption::new("doughnut", "Doughnut"),
    ];
    let legend_options = vec![
        SelectOption::new("top", "Top"),
        SelectOption::new("top-left", "Top Left"),
        SelectOption::new("top-right", "Top Right"),
        SelectOption::new("left", "Left"),
        SelectOption::new("right", "Right"),
        SelectOption::new("bottom", "Bottom"),
        SelectOption::new("bottom-left", "Bottom Left"),
        SelectOption::new("bottom-right", "Bottom Right"),
        SelectOption::new("none", "None"),
    ];

    view! {
        <section class="page-header">
            <div class="page-header__eyebrow">"Component"</div>
            <h2>"Charts"</h2>
            <p class="page-header__lede">
                "One configurable SVG chart component that can render stacked bars, pie, or doughnut charts from the same flat data model."
            </p>
        </section>

        <section class="doc-grid">
            <Card header="Stacked bars with line overlay" class="doc-card">
                <span class="doc-card__kicker">"Bar"</span>
                <div class="doc-card__preview doc-card__preview--stack">
                    <Chart
                        aria_label="Monthly revenue, costs, and forecast"
                        chart_type=ChartType::Bar
                        legend_position=ChartLegendPosition::TopLeft
                        data=bar_data.clone()
                    />
                </div>
                <CodeExample code={r##"let data = vec![
    ChartData::new("Revenue", 18.0).group("Jan").color("#1d7dfa"),
    ChartData::new("Costs", 11.0).group("Jan").color("#69b578"),
    ChartData::new("Forecast", 16.0).group("Jan").color("#f28f3b").line(),
    ChartData::new("Revenue", 24.0).group("Feb").color("#1d7dfa"),
    ChartData::new("Costs", 15.0).group("Feb").color("#69b578"),
    ChartData::new("Forecast", 20.0).group("Feb").color("#f28f3b").line(),
];

view! {
    <Chart
        chart_type=ChartType::Bar
        legend_position=ChartLegendPosition::TopLeft
        data=data
    />
}"##}/>
            </Card>

            <Card header="Pie chart" class="doc-card">
                <span class="doc-card__kicker">"Distribution"</span>
                <div class="doc-card__preview doc-card__preview--stack">
                    <Chart
                        aria_label="Team allocation by department"
                        chart_type=ChartType::Pie
                        legend_position=ChartLegendPosition::Right
                        data=pie_data.clone()
                    />
                </div>
                <CodeExample code={r##"let data = vec![
    ChartData::new("Design", 22.0).color("#f25f5c"),
    ChartData::new("Product", 31.0).color("#1d7dfa"),
    ChartData::new("Ops", 18.0).color("#6c9a8b"),
];

view! {
    <Chart chart_type=ChartType::Pie data=data/>
}"##}/>
            </Card>

            <Card header="Doughnut chart" class="doc-card">
                <span class="doc-card__kicker">"Variant"</span>
                <div class="doc-card__preview doc-card__preview--stack">
                    <Chart
                        aria_label="Team allocation by department"
                        chart_type=ChartType::Doughnut
                        legend_position=ChartLegendPosition::Bottom
                        data=doughnut_data
                    />
                </div>
                <CodeExample code={r#"<Chart chart_type=ChartType::Doughnut data=data/>"#}/>
            </Card>

            <Card header="Interactive grouped chart" class="doc-card">
                <span class="doc-card__kicker">"Playground"</span>
                <div class="doc-card__preview doc-card__preview--stack">
                    <div class="field">
                        <Label text="Chart type" for_id="book-chart-playground-type"/>
                        <Select
                            id="book-chart-playground-type"
                            options=chart_type_options
                            value=playground_chart_type
                            on_value_change=Callback::new(move |next| playground_chart_type.set(next))
                        />
                    </div>
                    <div class="field">
                        <Label text="Legend position" for_id="book-chart-playground-legend"/>
                        <Select
                            id="book-chart-playground-legend"
                            options=legend_options
                            value=playground_legend
                            on_value_change=Callback::new(move |next| playground_legend.set(next))
                        />
                    </div>
                    {move || {
                        let chart_type = match playground_chart_type.get().as_deref() {
                            Some("bar") => ChartType::Bar,
                            Some("pie") => ChartType::Pie,
                            _ => ChartType::Doughnut,
                        };
                        let legend_position = match playground_legend.get().as_deref() {
                            Some("top") => ChartLegendPosition::Top,
                            Some("top-left") => ChartLegendPosition::TopLeft,
                            Some("top-right") => ChartLegendPosition::TopRight,
                            Some("left") => ChartLegendPosition::Left,
                            Some("bottom") => ChartLegendPosition::Bottom,
                            Some("bottom-left") => ChartLegendPosition::BottomLeft,
                            Some("bottom-right") => ChartLegendPosition::BottomRight,
                            Some("none") => ChartLegendPosition::None,
                            _ => ChartLegendPosition::Right,
                        };

                        view! {
                            <Chart
                                aria_label="Revenue mix by region"
                                chart_type=chart_type
                                legend_position=legend_position
                                data=grouped_doughnut_data.clone()
                            />
                        }
                    }}
                </div>
                <CodeExample code={r##"let data = vec![
    ChartData::new("Enterprise", 32.0).group("North").color("#1d7dfa"),
    ChartData::new("Retail", 18.0).group("North").color("#69b578"),
    ChartData::new("Target", 44.0).group("North").color("#8b5cf6").line(),
    ChartData::new("Enterprise", 27.0).group("South").color("#f28f3b"),
    ChartData::new("Retail", 23.0).group("South").color("#f25f5c"),
    ChartData::new("Target", 39.0).group("South").color("#8b5cf6").line(),
];

let chart_type = RwSignal::new(Some(String::from("doughnut")));
let legend_position = RwSignal::new(Some(String::from("right")));

view! {
    <Select value=chart_type options=chart_type_options/>
    <Select value=legend_position options=legend_options/>
    <Chart data=data/>
}"##}/>
            </Card>
        </section>
    }
}
