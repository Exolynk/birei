use std::sync::Arc;
use crate::code_example::CodeExample;

use birei::code_editor::{CodeEditor, HtmlCodeLanguageService};
use birei::{Card, Size};
use leptos::prelude::*;

#[component]
pub fn CodeEditorPage() -> impl IntoView {
    let code = RwSignal::new(String::from(
        r#"<!doctype html>
<html lang="en">
  <head>
    <meta charset="utf-8" />
    <title>Preview</title>
  </head>
  <body>
    <main class="shell">
      <section class="hero">
        <h1>Hello Birei</h1>
        <p>Edit this HTML directly.</p>
      </section>
    </main>
  </body>
</html>
"#,
    ));
    let service = Arc::new(HtmlCodeLanguageService);

    view! {
        <section class="page-header">
            <div class="page-header__eyebrow">"Component"</div>
            <h2>"Code Editor"</h2>
            <p class="page-header__lede">
                "A textarea-backed code editor with async language services. The bundled implementation highlights and completes HTML."
            </p>
        </section>

        <section class="doc-grid">
            <Card header="HTML service" class="doc-card">
                <span class="doc-card__kicker">"Async service API"</span>
                <div class="doc-card__preview doc-card__preview--stack">
                    <CodeEditor
                        value=code
                        service=service
                        size=Size::Medium
                        placeholder="Write HTML..."
                        on_change=Callback::new(move |next| code.set(next))
                    />
                </div>
                <CodeExample
                    code={r#"let service = Arc::new(HtmlCodeLanguageService);

<CodeEditor
    value=code
    service=service
    placeholder="Write HTML..."
    on_change=Callback::new(move |next| code.set(next))
/>"#}
                    title="Code Editor Setup"
                />
            </Card>
        </section>
    }
}
