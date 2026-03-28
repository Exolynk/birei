/// Identifies one of the three fixed columns in [`FlexibleColumns`](super::FlexibleColumns).
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub enum FlexibleColumn {
    Start,
    #[default]
    Middle,
    End,
}

impl FlexibleColumn {
    pub(crate) const fn index(self) -> usize {
        match self {
            Self::Start => 0,
            Self::Middle => 1,
            Self::End => 2,
        }
    }

    pub(crate) const fn from_index(index: usize) -> Self {
        match index {
            0 => Self::Start,
            1 => Self::Middle,
            _ => Self::End,
        }
    }

    pub(crate) const fn aria_label(self) -> &'static str {
        match self {
            Self::Start => "Start column",
            Self::Middle => "Middle column",
            Self::End => "End column",
        }
    }
}
