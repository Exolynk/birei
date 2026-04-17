use leptos::prelude::*;

/// Vertical timeline container that stacks [`TimelineItem`](super::TimelineItem) entries.
#[component]
pub fn Timeline(
    children: Children,
    #[prop(optional, into)] class: Option<String>,
) -> impl IntoView {
    let class_name = move || {
        let mut classes = vec!["birei-timeline"];
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
