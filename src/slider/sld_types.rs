/// Optional labeled stop rendered beneath a [`Slider`](super::Slider).
#[derive(Debug, Clone, PartialEq)]
pub struct SliderStepLabel {
    pub value: f64,
    pub label: String,
}

impl SliderStepLabel {
    /// Small builder used by examples and callers so labels stay ergonomic to declare.
    pub fn new(value: f64, label: impl Into<String>) -> Self {
        Self {
            value,
            label: label.into(),
        }
    }
}
