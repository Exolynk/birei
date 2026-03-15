use leptos::prelude::*;

use super::icon_types::IcnName;
use crate::Size;

/// Lucide font icon rendered through the bundled icon font classes.
#[component]
pub fn Icon(
    /// Lucide icon name without the `icon-` prefix, for example `search` or `arrow-right`.
    #[prop(into)]
    name: IcnName,
    /// Shared sizing token aligned with the rest of the component library.
    #[prop(optional)]
    size: Size,
    /// Accessible label announced by assistive technologies.
    #[prop(optional, into)]
    label: Option<String>,
    /// Additional CSS class names applied to the root element.
    #[prop(optional, into)]
    class: Option<String>,
) -> impl IntoView {
    let mut classes = vec!["birei-icon", size.icon_class_name()];
    let icon_class = format!("icon-{}", name.as_str());
    classes.push(icon_class.as_str());

    if let Some(class) = class.as_deref() {
        classes.push(class);
    }

    let class_name = classes.join(" ");
    let aria_hidden = label.is_none();

    view! {
        <span
            class=class_name
            role=if aria_hidden { None } else { Some("img") }
            aria-label=label
            aria-hidden=if aria_hidden { Some("true") } else { None }
        ></span>
    }
}
