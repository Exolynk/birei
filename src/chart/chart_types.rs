/// Selects the overall chart rendering mode.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Default)]
pub enum ChartType {
    #[default]
    Bar,
    Pie,
    Doughnut,
}

/// Controls where the legend is rendered relative to the chart.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Default)]
pub enum ChartLegendPosition {
    Top,
    TopLeft,
    TopRight,
    Left,
    BottomLeft,
    BottomRight,
    Right,
    #[default]
    Bottom,
    None,
}

impl ChartLegendPosition {
    /// Maps the legend position to a root class so layout rules stay in SCSS.
    pub const fn class_name(self) -> &'static str {
        match self {
            Self::Top => "birei-chart--legend-top",
            Self::TopLeft => "birei-chart--legend-top-left",
            Self::TopRight => "birei-chart--legend-top-right",
            Self::Left => "birei-chart--legend-left",
            Self::BottomLeft => "birei-chart--legend-bottom-left",
            Self::BottomRight => "birei-chart--legend-bottom-right",
            Self::Right => "birei-chart--legend-right",
            Self::Bottom => "birei-chart--legend-bottom",
            Self::None => "birei-chart--legend-none",
        }
    }
}

/// Distinguishes stacked bar values from line overlays in bar charts.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, Default)]
pub enum ChartDatumKind {
    #[default]
    Bar,
    Line,
}

/// Flat input model shared by all chart render modes.
#[derive(Clone, Debug, PartialEq)]
pub struct ChartData {
    pub(crate) value: f64,
    pub(crate) color: Option<String>,
    pub(crate) group: String,
    pub(crate) title: String,
    pub(crate) kind: ChartDatumKind,
}

impl ChartData {
    /// Creates a new chart datum. Use `group` for bar-chart x-axis buckets.
    pub fn new(title: impl Into<String>, value: f64) -> Self {
        Self {
            value,
            color: None,
            group: String::new(),
            title: title.into(),
            kind: ChartDatumKind::Bar,
        }
    }

    /// Assigns the group/category used by bar charts.
    pub fn group(mut self, group: impl Into<String>) -> Self {
        self.group = group.into();
        self
    }

    /// Overrides the automatically assigned series color.
    pub fn color(mut self, color: impl Into<String>) -> Self {
        self.color = Some(color.into());
        self
    }

    /// Renders this datum as part of a line overlay in bar charts.
    pub fn line(mut self) -> Self {
        self.kind = ChartDatumKind::Line;
        self
    }

    /// Sets the datum rendering kind explicitly.
    pub fn kind(mut self, kind: ChartDatumKind) -> Self {
        self.kind = kind;
        self
    }
}
