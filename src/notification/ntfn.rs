use leptos::prelude::*;

use super::types::NotificationVariant;
use crate::{Button, ButtonVariant, Icon, Size};

/// Plain-text notification row with variant icon and optional dismiss control.
#[component]
pub fn Notification(
    #[prop(into)] text: String,
    #[prop(optional)] variant: NotificationVariant,
    #[prop(optional)] dismissible: bool,
    #[prop(optional)] on_dismiss: Option<Callback<()>>,
    #[prop(optional, into)] class: Option<String>,
) -> impl IntoView {
    let class_name = move || {
        let mut classes = vec!["birei-notification", variant.class_name()];
        if let Some(class) = class.as_deref() {
            classes.push(class);
        }
        classes.join(" ")
    };

    view! {
        <div class=class_name role="status" aria-live="polite">
            <div class="birei-notification__icon">
                <Icon name=variant.icon_name() label=format!("{variant:?} notification") size=Size::Small/>
            </div>
            <div class="birei-notification__text">{text}</div>
            <Show when=move || dismissible>
                <Button
                    size=Size::Small
                    circle=true
                    variant=ButtonVariant::Transparent
                    class="birei-notification__dismiss"
                    on_click=Callback::new(move |_| {
                        if let Some(on_dismiss) = on_dismiss.as_ref() {
                            on_dismiss.run(());
                        }
                    })
                >
                    <Icon name="x" label="Dismiss notification" size=Size::Small/>
                </Button>
            </Show>
        </div>
    }
}
