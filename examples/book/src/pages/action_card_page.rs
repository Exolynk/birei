use crate::code_example::CodeExample;
use birei::{ActionCard, ActionCardUpload, Button, ButtonVariant, Card};
use leptos::prelude::*;

#[component]
pub fn ActionCardPage() -> impl IntoView {
    let revenue = RwSignal::new(12850.0);
    let balance = RwSignal::new(-3250.0);
    let clicks = RwSignal::new(0usize);
    let uploaded_files = RwSignal::new(Vec::<String>::new());

    view! {
        <section class="page-header">
            <div class="page-header__eyebrow">"Component"</div>
            <h2>"Action Card"</h2>
            <p class="page-header__lede">
                "Compact highlight surface for home screens and overview panels, showing either an icon or an animated numeric value with title and subtitle."
            </p>
        </section>

        <section class="doc-grid">
            <Card header="Icon and number cards" class="doc-card">
                <span class="doc-card__kicker">"Overview"</span>
                <div class="doc-card__preview doc-card__preview--stack">
                    <div class="book-action-card-grid">
                        <ActionCard
                            icon="rocket"
                            title="Quick start"
                            subtitle="Open the deployment checklist"
                            on_click=Callback::new(move |_| {
                                clicks.update(|value| *value += 1);
                            })
                        />
                        <ActionCard
                            value=revenue
                            title="Revenue"
                            subtitle="Current monthly total"
                            abbreviate=true
                        />
                        <ActionCard
                            value=balance
                            title="Balance"
                            subtitle="Outstanding difference"
                            precision=1
                            abbreviate=true
                        />
                    </div>
                    <p>{move || format!("Quick start clicks: {}", clicks.get())}</p>
                    <div class="demo-form__actions">
                        <Button
                            variant=ButtonVariant::Secondary
                            on_click=Callback::new(move |_| revenue.update(|value| *value += 1750.0))
                        >
                            "Increase revenue"
                        </Button>
                        <Button
                            variant=ButtonVariant::Transparent
                            on_click=Callback::new(move |_| balance.update(|value| *value -= 420.0))
                        >
                            "Decrease balance"
                        </Button>
                    </div>
                </div>
                <CodeExample code={r#"let revenue = RwSignal::new(12850.0);

view! {
    <ActionCard
        value=revenue
        title="Revenue"
        subtitle="Current monthly total"
        abbreviate=true
    />
}"#}/>
            </Card>

            <Card header="Clickable action card" class="doc-card">
                <span class="doc-card__kicker">"Button semantics"</span>
                <div class="doc-card__preview doc-card__preview--stack">
                    <ActionCard
                        icon="external-link"
                        title="Open workspace"
                        subtitle="Jump to the live dashboard"
                        on_click=Callback::new(move |_| {
                            clicks.update(|value| *value += 1);
                        })
                    />
                </div>
                <CodeExample code={r#"<ActionCard
    icon="external-link"
    title="Open workspace"
    subtitle="Jump to the live dashboard"
    on_click=Callback::new(move |_| {
        // handle open
    })
/>"#}/>
            </Card>

            <Card header="Upload action card" class="doc-card">
                <span class="doc-card__kicker">"File input"</span>
                <div class="doc-card__preview doc-card__preview--stack">
                    <ActionCardUpload
                        icon="cloud-upload"
                        title="Upload files"
                        subtitle="Click or drop files"
                        multiple=true
                        on_files=Callback::new(move |files: Vec<web_sys::File>| {
                            uploaded_files.set(files.into_iter().map(|file| file.name()).collect());
                        })
                    />
                    <p>{move || {
                        let files = uploaded_files.get();
                        if files.is_empty() {
                            String::from("No files selected.")
                        } else {
                            format!("Selected: {}", files.join(", "))
                        }
                    }}</p>
                </div>
                <CodeExample code={r#"<ActionCardUpload
    icon="cloud-upload"
    title="Upload files"
    subtitle="Click or drop files"
    multiple=true
    on_files=Callback::new(move |files| {
        // upload files
    })
/>"#}/>
            </Card>
        </section>
    }
}
