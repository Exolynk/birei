use crate::code_example::CodeExample;
use birei::{Card, Loading, Size};
use leptos::prelude::*;

#[component]
pub fn LoadingPage() -> impl IntoView {
    view! {
        <section class="page-header">
            <div class="page-header__eyebrow">"Component"</div>
            <h2>"Loading"</h2>
            <p class="page-header__lede">
                "Theme-aligned loading indicators for inline waits, centered panels, and status text."
            </p>
        </section>

        <section class="doc-grid">
            <Card header="Spinner" class="doc-card">
                <span class="doc-card__kicker">"Inline"</span>
                <div class="doc-card__preview doc-card__preview--stack">
                    <Loading label="Loading"/>
                    <Loading label="Loading records" show_label=true/>
                </div>
                <CodeExample code={r#"<Loading />
<Loading label="Loading records" show_label=true />"#}/>
            </Card>

            <Card header="Shared sizes" class="doc-card">
                <span class="doc-card__kicker">"Size"</span>
                <div class="doc-card__preview doc-card__preview--stack">
                    <Loading size=Size::Small label="Loading small" show_label=true/>
                    <Loading size=Size::Medium label="Loading medium" show_label=true/>
                    <Loading size=Size::Large label="Loading large" show_label=true/>
                </div>
                <CodeExample code={r#"<Loading size=Size::Small />
<Loading size=Size::Medium />
<Loading size=Size::Large />"#}/>
            </Card>

            <Card header="Centered loading state" class="doc-card">
                <span class="doc-card__kicker">"Block"</span>
                <div class="doc-card__preview">
                    <Loading block=true label="Loading environment" show_label=true/>
                </div>
                <CodeExample code={r#"<Loading block=true label="Loading environment" show_label=true />"#}/>
            </Card>
        </section>
    }
}
