use crate::code_example::CodeExample;
use birei::{Button, ButtonVariant, Card, Input, Label, Popup};
use leptos::prelude::*;

#[component]
pub fn PopupPage() -> impl IntoView {
    let basic_open = RwSignal::new(false);
    let form_open = RwSignal::new(false);

    view! {
        <section class="page-header">
            <div class="page-header__eyebrow">"Component"</div>
            <h2>"Popup"</h2>
            <p class="page-header__lede">
                "Controlled modal dialog with a sticky header, scrollable body, always-visible close action, and a standard footer slot for custom buttons."
            </p>
        </section>

        <section class="doc-grid">
            <Card header="Basic popup" class="doc-card">
                <span class="doc-card__kicker">"Controlled"</span>
                <div class="doc-card__preview doc-card__preview--stack">
                    <Button on_click=Callback::new(move |_| basic_open.set(true))>
                        "Open popup"
                    </Button>
                    <Popup
                        open=basic_open
                        header="Delete collection"
                        on_open_change=Callback::new(move |next| basic_open.set(next))
                        actions=move || view! {
                            <>
                                <Button
                                    variant=ButtonVariant::Secondary
                                    on_click=Callback::new(move |_| basic_open.set(false))
                                >
                                    "Cancel"
                                </Button>
                                <Button on_click=Callback::new(move |_| basic_open.set(false))>
                                    "Delete"
                                </Button>
                            </>
                        }
                    >
                        <p>
                            "This popup stays fully controlled by the parent. Clicking outside, pressing Escape, or using the close icon all request `open = false` through the callback."
                        </p>
                    </Popup>
                </div>
                <CodeExample code={r#"let open = RwSignal::new(false);

view! {
    <Popup
        open=open
        header="Delete collection"
        on_open_change=Callback::new(move |next| open.set(next))
        actions=move || view! {
            <>
                <Button variant=ButtonVariant::Secondary>"Cancel"</Button>
                <Button>"Delete"</Button>
            </>
        }
    >
        <p>"Controlled popup content."</p>
    </Popup>
}"#}/>
            </Card>

            <Card header="Scroll body and custom footer" class="doc-card">
                <span class="doc-card__kicker">"Layout"</span>
                <div class="doc-card__preview doc-card__preview--stack">
                    <Button
                        variant=ButtonVariant::Secondary
                        on_click=Callback::new(move |_| form_open.set(true))
                    >
                        "Open editor"
                    </Button>
                    <Popup
                        open=form_open
                        header="Edit deployment details"
                        on_open_change=Callback::new(move |next| form_open.set(next))
                        actions=move || view! {
                            <>
                                <Button
                                    variant=ButtonVariant::Transparent
                                    on_click=Callback::new(move |_| form_open.set(false))
                                >
                                    "Close"
                                </Button>
                                <Button variant=ButtonVariant::Secondary>"Previous"</Button>
                                <Button on_click=Callback::new(move |_| form_open.set(false))>
                                    "Next"
                                </Button>
                            </>
                        }
                    >
                        <div class="doc-card__preview doc-card__preview--stack">
                            <div class="field">
                                <Label text="Service name" for_id="book-popup-service-name"/>
                                <Input id="book-popup-service-name" placeholder="Payments API"/>
                            </div>
                            <div class="field">
                                <Label text="Namespace" for_id="book-popup-namespace"/>
                                <Input id="book-popup-namespace" placeholder="production-eu"/>
                            </div>
                            <p>
                                "The body area scrolls independently when the content grows, while the header and action row stay pinned."
                            </p>
                            <p>
                                "Mobile view switches the popup to a full-screen panel so forms and long review flows have enough space."
                            </p>
                            <p>
                                "Custom footer actions are just regular library buttons or any other view content passed through the standard slot."
                            </p>
                        </div>
                    </Popup>
                </div>
                <CodeExample code={r#"<Popup
    open=open
    header="Edit deployment details"
    on_open_change=Callback::new(move |next| open.set(next))
    actions=move || view! {
        <>
            <Button variant=ButtonVariant::Transparent>"Close"</Button>
            <Button variant=ButtonVariant::Secondary>"Previous"</Button>
            <Button>"Next"</Button>
        </>
    }
>
    <Label text="Service name" for_id="service-name"/>
    <Input id="service-name" placeholder="Payments API"/>
</Popup>"#}/>
            </Card>
        </section>
    }
}
