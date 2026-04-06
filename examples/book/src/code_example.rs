use std::sync::Arc;

use birei::code_editor::{
    CodeEditor, CodeLanguageService, HtmlCodeLanguageService, PlainTextCodeLanguageService,
};
use leptos::prelude::*;

#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub enum CodeExampleLanguage {
    #[default]
    Html,
    PlainText,
}

#[component]
pub fn CodeExample(
    #[prop(optional, into)]
    code: Option<String>,
    #[prop(optional, into)]
    code_signal: Option<Signal<String>>,
    #[prop(optional, into)]
    title: Option<String>,
    #[prop(optional)]
    rows: Option<u32>,
    #[prop(optional)]
    language: CodeExampleLanguage,
) -> impl IntoView {
    let service: Arc<dyn CodeLanguageService> = match language {
        CodeExampleLanguage::Html => Arc::new(HtmlCodeLanguageService),
        CodeExampleLanguage::PlainText => Arc::new(PlainTextCodeLanguageService),
    };
    let static_code = code.clone();
    let code_value = Signal::derive(move || {
        code_signal
            .as_ref()
            .map(Signal::get)
            .or_else(|| static_code.clone())
            .unwrap_or_default()
    });
    let computed_rows = rows.unwrap_or_else(|| {
        code_signal
            .as_ref()
            .map(Signal::get_untracked)
            .or_else(|| code.clone())
            .unwrap_or_default()
            .lines()
            .count()
            .clamp(3, 12) as u32
    });
    let title = title.unwrap_or_else(|| match language {
        CodeExampleLanguage::Html => String::from("Leptos Component Code"),
        CodeExampleLanguage::PlainText => String::from("Component Source"),
    });

    view! {
        <div class="doc-card__code-example">
            <div class="doc-card__code-example-title">{title}</div>
            <CodeEditor value=code_value service=service size=birei::Size::Small rows=computed_rows readonly=true/>
        </div>
    }
}
