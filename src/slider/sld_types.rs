/// Optional labeled stop rendered beneath a [`Slider`](super::Slider).
#[derive(Debug, Clone, PartialEq)]
pub struct SliderStepLabel {
    pub value: f64,
    pub label: String,
}

impl SliderStepLabel {
    pub fn new(value: f64, label: impl Into<String>) -> Self {
        Self {
            value,
            label: label.into(),
        }
    }
}
