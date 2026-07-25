use leptos::prelude::*;

/// Visual orientation for a [`Timeline`] container.
#[derive(Clone, Copy, Default, PartialEq, Eq)]
pub enum TimelineLayout {
    /// Stack entries from top to bottom.
    #[default]
    Vertical,
    /// Arrange entries from left to right with horizontal scrolling as needed.
    Horizontal,
}

impl TimelineLayout {
    /// Returns the root modifier class for this orientation.
    fn class_name(self) -> &'static str {
        match self {
            Self::Vertical => "birei-timeline--vertical",
            Self::Horizontal => "birei-timeline--horizontal",
        }
    }
}

/// Timeline container that arranges [`TimelineItem`](super::TimelineItem) entries by layout.
#[component]
pub fn Timeline(
    children: Children,
    /// Selects the entry orientation.
    #[prop(optional)]
    layout: TimelineLayout,
    /// Additional classes applied to the container.
    #[prop(optional, into)]
    class: Option<String>,
) -> impl IntoView {
    let class_name = move || {
        let mut classes = vec!["birei-timeline", layout.class_name()];
        if let Some(class) = class.as_deref() {
            classes.push(class);
        }
        classes.join(" ")
    };

    view! {
        <div class=class_name>
            {children()}
        </div>
    }
}
