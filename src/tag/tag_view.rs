use leptos::ev;
use leptos::prelude::*;

/// Small inline tag with optional remove action.
#[component]
pub fn Tag(
    /// Text rendered inside the tag.
    #[prop(into)]
    label: String,
    /// Optional remove handler. When present, a remove affordance is shown.
    #[prop(optional)]
    on_remove: Option<Callback<ev::MouseEvent>>,
    /// Additional CSS class names applied to the tag root.
    #[prop(optional, into)]
    class: Option<String>,
) -> impl IntoView {
    let mut classes = vec!["birei-tag"];
    if let Some(class) = class.as_deref() {
        classes.push(class);
    }

    view! {
        <span class=classes.join(" ")>
            <span>{label}</span>
            {on_remove.map(|on_remove| {
                view! {
                    <button
                        type="button"
                        class="birei-tag__remove"
                        tabindex="-1"
                        on:mousedown=move |event| {
                            event.prevent_default();
                            event.stop_propagation();
                        }
                        on:click=move |event| on_remove.run(event)
                    >
                        "x"
                    </button>
                }
            })}
        </span>
    }
}
