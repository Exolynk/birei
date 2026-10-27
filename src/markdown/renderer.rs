//! Read-only Markdown for untrusted text, without raw HTML or remote images.

use leptos::prelude::*;

use super::safe::{safe_markdown_html, Images};

/// Renders Markdown without exposing the editor or interpreting embedded HTML.
#[component]
pub fn MarkdownRenderer(#[prop(into)] value: Signal<String>) -> impl IntoView {
    view! { <div class="birei-markdown-renderer" inner_html=move || safe_markdown_html(&value.get(), Images::Remove) /> }
}
