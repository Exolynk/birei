use std::collections::HashMap;

use super::chart_types::{ChartData, ChartDatumKind, ChartType};
use super::chart_utils::{
    describe_arc, describe_full_circle, describe_full_ring, describe_ring_segment,
    CHART_VIEWBOX_HEIGHT, CHART_VIEWBOX_WIDTH,
};
use super::internal::{
    BarLayout, BarSegment, LegendEntry, LinePoint, LineSeries, PieLayout, SliceLayout,
};

pub(crate) fn build_bar_layout(data: Vec<ChartData>, y_max: Option<f64>) -> BarLayout {
    let groups = ordered_groups(&data);
    let chart_left = 84.0;
    let chart_right = CHART_VIEWBOX_WIDTH - 28.0;
    let chart_top = 26.0;
    let chart_bottom = CHART_VIEWBOX_HEIGHT - 56.0;
    let chart_width = (chart_right - chart_left).max(1.0);
    let chart_height = (chart_bottom - chart_top).max(1.0);
    let category_count = groups.len().max(1) as f64;
    let step = chart_width / category_count;
    let slot_width = step * 0.6;
    let bar_width = slot_width.min(56.0);
    let group_offset = (step - bar_width) * 0.5;

    let stack_totals = groups
        .iter()
        .map(|group| {
            data.iter()
                .filter(|datum| datum.kind == ChartDatumKind::Bar)
                .filter(|datum| normalized_group(datum) == *group)
                .map(|datum| datum.value.max(0.0))
                .sum::<f64>()
        })
        .collect::<Vec<_>>();
    let line_max = data
        .iter()
        .filter(|datum| datum.kind == ChartDatumKind::Line)
        .map(|datum| datum.value.max(0.0))
        .fold(0.0_f64, f64::max);
    let derived_y_max = y_max.unwrap_or_else(|| {
        stack_totals
            .iter()
            .copied()
            .fold(line_max.max(1.0), f64::max)
            .max(1.0)
    });

    let bars = groups
        .iter()
        .enumerate()
        .flat_map(|(index, group)| {
            let x = chart_left + (index as f64 * step) + group_offset;
            let mut stacked_height = 0.0;

            data.iter()
                .filter(|datum| datum.kind == ChartDatumKind::Bar)
                .filter(|datum| normalized_group(datum) == *group)
                .enumerate()
                .map(move |(bar_index, datum)| {
                    let value = datum.value.max(0.0);
                    let height = chart_height * (value / derived_y_max).clamp(0.0, 1.0);
                    let y = chart_bottom - stacked_height - height;
                    stacked_height += height;

                    BarSegment {
                        key: format!("{group}-{}-{bar_index}", datum.title),
                        title: datum.title.clone(),
                        group: group.clone(),
                        color: resolve_color(
                            datum.color.as_deref(),
                            datum.title.as_str(),
                            bar_index,
                        ),
                        x,
                        y,
                        width: bar_width,
                        height,
                        value,
                    }
                })
                .collect::<Vec<_>>()
        })
        .collect::<Vec<_>>();

    let lines = ordered_line_titles(&data)
        .into_iter()
        .enumerate()
        .map(|(series_index, title)| {
            let color = data
                .iter()
                .find(|datum| datum.kind == ChartDatumKind::Line && datum.title == title)
                .and_then(|datum| datum.color.clone())
                .unwrap_or_else(|| palette_color(series_index + 5).to_string());

            let points = groups
                .iter()
                .enumerate()
                .filter_map(|(group_index, group)| {
                    let value = data
                        .iter()
                        .filter(|datum| datum.kind == ChartDatumKind::Line)
                        .filter(|datum| datum.title == title && normalized_group(datum) == *group)
                        .map(|datum| datum.value.max(0.0))
                        .reduce(|acc, next| acc + next)?;
                    let x = chart_left + (group_index as f64 * step) + (step * 0.5);
                    let y = chart_bottom - (chart_height * (value / derived_y_max).clamp(0.0, 1.0));

                    Some(LinePoint {
                        key: format!("{title}-{group_index}"),
                        title: title.clone(),
                        group: group.clone(),
                        color: color.clone(),
                        x,
                        y,
                        value,
                    })
                })
                .collect::<Vec<_>>();

            let path = points
                .iter()
                .enumerate()
                .map(|(index, point)| {
                    if index == 0 {
                        format!("M {:.3} {:.3}", point.x, point.y)
                    } else {
                        format!("L {:.3} {:.3}", point.x, point.y)
                    }
                })
                .collect::<Vec<_>>()
                .join(" ");

            LineSeries {
                key: title.clone(),
                title,
                color,
                path,
                points,
            }
        })
        .filter(|series| !series.points.is_empty())
        .collect::<Vec<_>>();

    let grid = (0..=4)
        .map(|index| {
            let ratio = index as f64 / 4.0;
            let y = chart_bottom - (chart_height * ratio);
            let value = derived_y_max * ratio;
            (y, format!("{value:.0}"))
        })
        .collect::<Vec<_>>();

    let x_labels = groups
        .iter()
        .enumerate()
        .map(|(index, group)| {
            (
                chart_left + (index as f64 * step) + (step * 0.5),
                group.clone(),
            )
        })
        .collect::<Vec<_>>();

    let legend = build_legend(&data);

    BarLayout {
        bars,
        lines,
        legend,
        grid,
        x_labels,
        chart_left,
        chart_bottom,
        chart_right,
    }
}

pub(crate) fn build_pie_layout(data: Vec<ChartData>, chart_type: ChartType) -> PieLayout {
    let slices = data
        .into_iter()
        .filter(|datum| datum.kind != ChartDatumKind::Line)
        .filter(|datum| datum.value > 0.0)
        .collect::<Vec<_>>();
    let total = slices
        .iter()
        .map(|datum| datum.value.max(0.0))
        .sum::<f64>()
        .max(1.0);
    let cx = 180.0;
    let cy = 180.0;
    let outer_radius = 124.0;
    let explicit_groups = ordered_explicit_groups(&slices);
    let slices = if matches!(chart_type, ChartType::Pie | ChartType::Doughnut)
        && !explicit_groups.is_empty()
    {
        let groups = explicit_groups;
        let ring_count = groups.len().max(1);
        let center_hole = match chart_type {
            ChartType::Doughnut if ring_count > 1 => 40.0,
            ChartType::Doughnut => 70.0,
            ChartType::Pie => 0.0,
            ChartType::Bar => 0.0,
        };
        let ring_gap = if ring_count > 1 { 6.0 } else { 0.0 };
        let band_width = ((outer_radius - center_hole)
            - (ring_gap * (ring_count.saturating_sub(1) as f64)))
            / ring_count as f64;
        let mut slice_index = 0usize;

        groups
            .into_iter()
            .enumerate()
            .flat_map(|(group_index, group)| {
                let group_items = slices
                    .iter()
                    .filter(|datum| normalized_group(datum) == group)
                    .cloned()
                    .collect::<Vec<_>>();
                let group_total = group_items
                    .iter()
                    .map(|datum| datum.value.max(0.0))
                    .sum::<f64>()
                    .max(1.0);
                let outer = outer_radius - (group_index as f64 * (band_width + ring_gap));
                let inner = outer - band_width;
                let mut angle = -90.0;

                group_items.into_iter().map(move |datum| {
                    let sweep = 360.0 * (datum.value / group_total);
                    let start_angle = angle;
                    let end_angle = angle + sweep;
                    angle = end_angle;

                    let path = if inner <= 0.001 && sweep >= 359.999 {
                        describe_full_circle(cx, cy, outer)
                    } else if inner <= 0.001 {
                        let arc = describe_arc(cx, cy, outer, start_angle, end_angle);
                        format!("{arc} L {cx:.3} {cy:.3} Z")
                    } else if sweep >= 359.999 {
                        describe_full_ring(cx, cy, outer, inner)
                    } else {
                        describe_ring_segment(cx, cy, outer, inner, start_angle, end_angle)
                    };

                    let current_index = slice_index;
                    slice_index += 1;

                    SliceLayout {
                        key: format!("slice-{current_index}-{}", datum.title),
                        title: datum.title.clone(),
                        color: resolve_color(
                            datum.color.as_deref(),
                            datum.title.as_str(),
                            current_index,
                        ),
                        value: datum.value,
                        percentage: (datum.value / total) * 100.0,
                        path,
                        group: datum.group,
                    }
                })
            })
            .collect::<Vec<_>>()
    } else {
        let mut angle = -90.0;
        let inner_radius = if chart_type == ChartType::Doughnut {
            70.0
        } else {
            0.0
        };

        slices
            .into_iter()
            .enumerate()
            .map(|(index, datum)| {
                let sweep = 360.0 * (datum.value / total);
                let start_angle = angle;
                let end_angle = angle + sweep;
                angle = end_angle;

                let path = if chart_type == ChartType::Doughnut && sweep >= 359.999 {
                    describe_full_ring(cx, cy, outer_radius, inner_radius)
                } else if chart_type == ChartType::Doughnut {
                    describe_ring_segment(
                        cx,
                        cy,
                        outer_radius,
                        inner_radius,
                        start_angle,
                        end_angle,
                    )
                } else if sweep >= 359.999 {
                    describe_full_circle(cx, cy, outer_radius)
                } else {
                    let arc = describe_arc(cx, cy, outer_radius, start_angle, end_angle);
                    format!("{arc} L {cx:.3} {cy:.3} Z")
                };

                SliceLayout {
                    key: format!("slice-{index}-{}", datum.title),
                    title: datum.title.clone(),
                    color: resolve_color(datum.color.as_deref(), datum.title.as_str(), index),
                    value: datum.value,
                    percentage: (datum.value / total) * 100.0,
                    path,
                    group: datum.group,
                }
            })
            .collect::<Vec<_>>()
    };

    let legend = slices
        .iter()
        .map(|slice| LegendEntry {
            key: slice.key.clone(),
            label: slice.title.clone(),
            color: slice.color.clone(),
            kind: ChartDatumKind::Bar,
        })
        .collect::<Vec<_>>();

    PieLayout {
        slices,
        legend,
        total,
    }
}

pub(crate) fn resolve_color(explicit: Option<&str>, title: &str, offset: usize) -> String {
    explicit.map(str::to_owned).unwrap_or_else(|| {
        palette_color(title.bytes().fold(offset, |acc, byte| acc + byte as usize)).to_string()
    })
}

fn build_legend(data: &[ChartData]) -> Vec<LegendEntry> {
    let mut seen = HashMap::<(String, ChartDatumKind), usize>::new();
    let mut legend = Vec::new();

    for datum in data {
        let key = (datum.title.clone(), datum.kind);
        if seen.contains_key(&key) {
            continue;
        }

        let index = legend.len();
        seen.insert(key, index);
        legend.push(LegendEntry {
            key: format!(
                "{}-{}",
                datum.title,
                if datum.kind == ChartDatumKind::Line {
                    "line"
                } else {
                    "bar"
                }
            ),
            label: datum.title.clone(),
            color: resolve_color(datum.color.as_deref(), datum.title.as_str(), index),
            kind: datum.kind,
        });
    }

    legend
}

fn ordered_groups(data: &[ChartData]) -> Vec<String> {
    let mut seen = HashMap::<String, usize>::new();
    let mut groups = Vec::new();

    for datum in data {
        let group = normalized_group(datum);
        if seen.contains_key(&group) {
            continue;
        }

        seen.insert(group.clone(), groups.len());
        groups.push(group);
    }

    groups
}

fn ordered_explicit_groups(data: &[ChartData]) -> Vec<String> {
    let mut seen = HashMap::<String, usize>::new();
    let mut groups = Vec::new();

    for datum in data {
        let group = datum.group.trim();
        if group.is_empty() {
            continue;
        }

        let group = group.to_string();
        if seen.contains_key(&group) {
            continue;
        }

        seen.insert(group.clone(), groups.len());
        groups.push(group);
    }

    groups
}

fn ordered_line_titles(data: &[ChartData]) -> Vec<String> {
    let mut seen = HashMap::<String, usize>::new();
    let mut titles = Vec::new();

    for datum in data
        .iter()
        .filter(|datum| datum.kind == ChartDatumKind::Line)
    {
        if seen.contains_key(&datum.title) {
            continue;
        }

        seen.insert(datum.title.clone(), titles.len());
        titles.push(datum.title.clone());
    }

    titles
}

fn normalized_group(datum: &ChartData) -> String {
    let trimmed = datum.group.trim();
    if trimmed.is_empty() {
        datum.title.clone()
    } else {
        trimmed.to_string()
    }
}

fn palette_color(index: usize) -> &'static str {
    const COLORS: [&str; 8] = [
        "#1d7dfa", "#69b578", "#f28f3b", "#f25f5c", "#6c9a8b", "#8b5cf6", "#f2c14e", "#06b6d4",
    ];

    COLORS[index % COLORS.len()]
}
