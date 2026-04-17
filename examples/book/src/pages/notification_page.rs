use crate::code_example::CodeExample;
use birei::{
    Button, ButtonVariant, Card, NotificationManager, NotificationOptions, NotificationVariant,
};
use leptos::prelude::*;

#[component]
pub fn NotificationPage() -> impl IntoView {
    view! {
        <section class="page-header">
            <div class="page-header__eyebrow">"Component"</div>
            <h2>"Notification"</h2>
            <p class="page-header__lede">
                "Global bottom-right toast notifications with variant styling, auto-hide, hover pause, and manual dismiss."
            </p>
        </section>

        <section class="doc-grid">
            <Card header="Variants" class="doc-card">
                <span class="doc-card__kicker">"Manager helpers"</span>
                <div class="doc-card__preview">
                    <Button
                        on_click=Callback::new({
                            move |_| {
                                NotificationManager::global().info("Inventory sync finished successfully.");
                            }
                        })
                    >
                        "Info"
                    </Button>
                    <Button
                        variant=ButtonVariant::Secondary
                        on_click=Callback::new({
                            move |_| {
                                NotificationManager::global().success("Payment captured and receipt sent.");
                            }
                        })
                    >
                        "Success"
                    </Button>
                    <Button
                        variant=ButtonVariant::Secondary
                        on_click=Callback::new({
                            move |_| {
                                NotificationManager::global().warning("Only two approval seats remain.");
                            }
                        })
                    >
                        "Warning"
                    </Button>
                    <Button
                        variant=ButtonVariant::Transparent
                        on_click=Callback::new({
                            move |_| {
                                NotificationManager::global().error("Signature upload failed. Try again.");
                            }
                        })
                    >
                        "Error"
                    </Button>
                </div>
                <CodeExample code={r#"let manager = NotificationManager::global().clone();

manager.info("Inventory sync finished successfully.");
manager.success("Payment captured and receipt sent.");
manager.warning("Only two approval seats remain.");
manager.error("Signature upload failed. Try again.");"#}/>
            </Card>

            <Card header="Custom options" class="doc-card">
                <span class="doc-card__kicker">"notify(...)"</span>
                <div class="doc-card__preview">
                    <Button
                        on_click=Callback::new({
                            move |_| {
                                NotificationManager::global().notify(
                                    NotificationOptions::new("Publishing will restart all active preview sessions.")
                                        .variant(NotificationVariant::Warning)
                                        .duration_ms(9000),
                                );
                            }
                        })
                    >
                        "Long warning"
                    </Button>
                    <Button
                        variant=ButtonVariant::Secondary
                        on_click=Callback::new({
                            move |_| {
                                for index in 1..=6 {
                                    NotificationManager::global().notify(
                                        NotificationOptions::new(format!("Queued background job #{index}."))
                                            .variant(NotificationVariant::Info)
                                            .duration_ms(3200 + (index * 180)),
                                    );
                                }
                            }
                        })
                    >
                        "Burst"
                    </Button>
                    <Button
                        variant=ButtonVariant::Transparent
                        on_click=Callback::new(move |_| NotificationManager::global().clear())
                    >
                        "Clear all"
                    </Button>
                </div>
                <CodeExample code={r#"NotificationManager::global().notify(
    NotificationOptions::new("Publishing will restart all active preview sessions.")
        .variant(NotificationVariant::Warning)
        .duration_ms(9000),
);"#}/>
            </Card>
        </section>
    }
}
