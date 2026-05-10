use leptos::prelude::*;

use crate::Size;

/// Loading spinner with optional visible status text.
#[component]
pub fn Loading(
    /// Accessible loading label. Rendered visibly when `show_label` is set.
    #[prop(optional, into)]
    label: MaybeProp<String>,
    /// Shared spinner size.
    #[prop(optional)]
    size: Size,
    /// Centers the loader in a full-width block.
    #[prop(optional)]
    block: bool,
    /// Shows the label text below the spinner.
    #[prop(optional)]
    show_label: bool,
    /// Additional CSS class names applied to the root element.
    #[prop(optional, into)]
    class: Option<String>,
) -> impl IntoView {
    let mut classes = vec!["birei-loading", loading_size_class_name(size)];

    if block {
        classes.push("birei-loading--block");
    }
    if let Some(class) = class.as_deref() {
        classes.push(class);
    }

    let class_name = classes.join(" ");
    let aria_label = label;
    let visible_label = label;

    view! {
        <div
            class=class_name
            role="status"
            aria-live="polite"
            aria-label=move || aria_label.get().unwrap_or_else(|| String::from("Loading"))
        >
            <span class="birei-loading__spinner" aria-hidden="true">
                <span></span>
                <span></span>
                <span></span>
                <span></span>
            </span>
            {show_label.then(|| {
                view! {
                    <span class="birei-loading__label">
                        {move || visible_label.get().unwrap_or_else(|| String::from("Loading"))}
                    </span>
                }
            })}
        </div>
    }
}

const fn loading_size_class_name(size: Size) -> &'static str {
    match size {
        Size::Small => "birei-loading--small",
        Size::Medium => "birei-loading--medium",
        Size::Large => "birei-loading--large",
    }
}
