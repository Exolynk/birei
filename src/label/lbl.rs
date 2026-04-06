use leptos::prelude::*;

/// Shared field label with optional required indicator.
#[component]
pub fn Label(
    /// Text shown inside the label.
    #[prop(into)]
    text: String,
    /// Optional target control id for native label association.
    #[prop(optional, into)]
    for_id: Option<String>,
    /// Renders the required asterisk.
    #[prop(optional)]
    required: bool,
    /// Additional CSS class names applied to the label element.
    #[prop(optional, into)]
    class: Option<String>,
) -> impl IntoView {
    // The label only carries an optional external hook class; required
    // rendering is handled inline so it stays semantically tied to the text.
    let mut classes = vec!["birei-label"];
    if let Some(class) = class.as_deref() {
        classes.push(class);
    }

    view! {
        <label class=classes.join(" ") for=for_id>
            <span>{text}</span>
            {required.then(|| {
                view! { <span class="birei-label__required" aria-hidden="true">"*"</span> }
            })}
        </label>
    }
}
