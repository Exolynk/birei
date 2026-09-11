use birei::{Card, Illustration, IllustrationKind, IllustrationTextLocation};
use leptos::prelude::*;

use crate::code_example::CodeExample;

/// Documents Birei's bundled Exolynk companion illustrations.
#[component]
pub fn IllustrationPage() -> impl IntoView {
    view! {
        <section class="page-header">
            <div class="page-header__eyebrow">"Component"</div>
            <h2>"Illustration"</h2>
            <p class="page-header__lede">
                "Animated companion scenes for loading, error, permission, and empty states."
            </p>
        </section>

        <section class="doc-grid">
            <Card header="Empty search state" class="doc-card">
                <div class="doc-card__preview">
                    <Illustration
                        kind=IllustrationKind::NoResults
                        label="EMPTY STATE"
                        title="No matches found"
                        description="We looked everywhere, but nothing matches your search. Try a different keyword, or clear your filters and start again."
                        location=IllustrationTextLocation::Auto
                    />
                </div>
                <CodeExample code={r#"<Illustration
    kind=IllustrationKind::NoResults
    label="EMPTY STATE"
    title="No matches found"
    description="We looked everywhere, but nothing matches your search."
    location=IllustrationTextLocation::Auto
/>"#}/>
            </Card>
            <IllustrationCard kind=IllustrationKind::BrandBuilding title="Brand building" />
            <IllustrationCard kind=IllustrationKind::ConnectionLost title="Connection lost" />
            <IllustrationCard kind=IllustrationKind::NoAccess title="No access" />
            <IllustrationCard kind=IllustrationKind::NoData title="No data yet" />
        </section>
    }
}

/// Shows one illustration kind and its corresponding component invocation.
#[component]
fn IllustrationCard(kind: IllustrationKind, title: &'static str) -> impl IntoView {
    let code = format!("<Illustration kind=IllustrationKind::{kind:?} />");

    view! {
        <Card header=title class="doc-card">
            <div class="doc-card__preview">
                <Illustration kind=kind />
            </div>
            <CodeExample code=code />
        </Card>
    }
}
