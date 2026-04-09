use super::chart_types::ChartDatumKind;

#[derive(Clone, PartialEq)]
pub(crate) struct BarSegment {
    pub(crate) key: String,
    pub(crate) title: String,
    pub(crate) group: String,
    pub(crate) color: String,
    pub(crate) x: f64,
    pub(crate) y: f64,
    pub(crate) width: f64,
    pub(crate) height: f64,
    pub(crate) value: f64,
}

#[derive(Clone, PartialEq)]
pub(crate) struct LinePoint {
    pub(crate) key: String,
    pub(crate) title: String,
    pub(crate) group: String,
    pub(crate) color: String,
    pub(crate) x: f64,
    pub(crate) y: f64,
    pub(crate) value: f64,
}

#[derive(Clone, PartialEq)]
pub(crate) struct LineSeries {
    pub(crate) key: String,
    pub(crate) title: String,
    pub(crate) color: String,
    pub(crate) path: String,
    pub(crate) points: Vec<LinePoint>,
}

#[derive(Clone, PartialEq)]
pub(crate) struct LegendEntry {
    pub(crate) key: String,
    pub(crate) label: String,
    pub(crate) color: String,
    pub(crate) kind: ChartDatumKind,
}

#[derive(Clone, PartialEq)]
pub(crate) struct SliceLayout {
    pub(crate) key: String,
    pub(crate) title: String,
    pub(crate) color: String,
    pub(crate) value: f64,
    pub(crate) percentage: f64,
    pub(crate) path: String,
    pub(crate) group: String,
}

#[derive(Clone, PartialEq)]
pub(crate) struct BarLayout {
    pub(crate) bars: Vec<BarSegment>,
    pub(crate) lines: Vec<LineSeries>,
    pub(crate) legend: Vec<LegendEntry>,
    pub(crate) grid: Vec<(f64, String)>,
    pub(crate) x_labels: Vec<(f64, String)>,
    pub(crate) chart_left: f64,
    pub(crate) chart_bottom: f64,
    pub(crate) chart_right: f64,
}

#[derive(Clone, PartialEq)]
pub(crate) struct PieLayout {
    pub(crate) slices: Vec<SliceLayout>,
    pub(crate) legend: Vec<LegendEntry>,
    pub(crate) total: f64,
}

#[derive(Clone, PartialEq)]
pub(crate) enum ChartLayout {
    Bar(BarLayout),
    Pie(PieLayout),
}

#[derive(Clone, PartialEq)]
pub(crate) struct HoverPayload {
    pub(crate) title: String,
    pub(crate) group: String,
    pub(crate) value: f64,
}

#[derive(Clone, PartialEq)]
pub(crate) struct HoverPopup {
    pub(crate) title: String,
    pub(crate) group: String,
    pub(crate) value: f64,
    pub(crate) left: f64,
    pub(crate) top: f64,
}
