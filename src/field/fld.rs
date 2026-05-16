use leptos::prelude::*;

use crate::Label;

/// Responsive label/control wrapper for composing existing form inputs.
#[component]
pub fn Field(
    /// Text shown in the left-side label.
    #[prop(into)]
    label: MaybeProp<String>,
    /// Existing control rendered to the right of, or below, the label.
    children: Children,
    /// Optional target control id for native label association.
    #[prop(optional, into)]
    for_id: Option<String>,
    /// Renders the required asterisk in the label.
    #[prop(optional)]
    required: bool,
    /// Additional CSS class names applied to the root element.
    #[prop(optional, into)]
    class: Option<String>,
) -> impl IntoView {
    let mut classes = vec!["birei-field"];
    if let Some(class) = class.as_deref() {
        classes.push(class);
    }
    let label_view = if let Some(for_id) = for_id {
        view! { <Label text=label for_id=for_id required=required /> }.into_any()
    } else {
        view! { <Label text=label required=required /> }.into_any()
    };

    view! {
        <div class=classes.join(" ")>
            <div class="birei-field__layout">
                <div class="birei-field__label">
                    {label_view}
                </div>
                <div class="birei-field__control">
                    {children()}
                </div>
            </div>
        </div>
    }
}
