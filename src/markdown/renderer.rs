//! Read-only Markdown for untrusted text, without raw HTML or remote images.

use leptos::prelude::*;
use pulldown_cmark::{html, Event, Options, Parser, Tag, TagEnd};

/// Renders Markdown without exposing the editor or interpreting embedded HTML.
#[component]
pub fn MarkdownRenderer(#[prop(into)] value: Signal<String>) -> impl IntoView {
    view! { <div class="birei-markdown-renderer" inner_html=move || safe_markdown_html(&value.get()) /> }
}

/// Allows explicit web, email, root-relative and fragment links only.
fn safe_url(url: &str) -> bool {
    if url.chars().any(|c| c.is_control() || c == '\\') {
        return false;
    }
    let lower = url.to_ascii_lowercase();
    lower.starts_with("https://")
        || lower.starts_with("http://")
        || lower.starts_with("mailto:")
        || lower.starts_with('#')
        || (lower.starts_with('/') && !lower.starts_with("//"))
}

/// Escapes raw HTML, removes image fetching and strips unsafe link destinations.
fn safe_markdown_html(value: &str) -> String {
    let mut links = Vec::new();
    let events = Parser::new_ext(
        value,
        Options::ENABLE_TABLES | Options::ENABLE_STRIKETHROUGH,
    )
    .filter_map(|event| match event {
        Event::Html(text) | Event::InlineHtml(text) => Some(Event::Text(text)),
        Event::Start(Tag::Image { .. }) | Event::End(TagEnd::Image) => None,
        Event::Start(Tag::Link { ref dest_url, .. }) => {
            let allowed = safe_url(dest_url);
            links.push(allowed);
            allowed.then_some(event)
        }
        Event::End(TagEnd::Link) => links.pop().unwrap_or(false).then_some(event),
        event => Some(event),
    });
    let mut output = String::new();
    html::push_html(&mut output, events);
    output
}

#[cfg(test)]
mod tests {
    use super::*;

    /// Keeps standard Markdown formatting and safe links.
    #[test]
    fn renders_formatting() {
        let html =
            safe_markdown_html("**Bold** and `code`\n\n- item\n\n[web](https://example.com)");
        assert!(html.contains("<strong>Bold</strong>"));
        assert!(html.contains("<code>code</code>"));
        assert!(html.contains("<li>item</li>"));
        assert!(html.contains("href=\"https://example.com\""));
    }

    /// Untrusted markup and images cannot execute code or load remote assets.
    #[test]
    fn blocks_active_content() {
        let html = safe_markdown_html("<script>alert(1)</script>\n\n<img src=x onerror=alert(1)>\n\n![alt](https://example.com/pixel)\n\n[bad](javascript:alert%281%29)");
        assert!(!html.contains("<script>"));
        assert!(!html.contains("<img"));
        assert!(!html.contains("href="));
        assert!(html.contains("alt"));
        assert!(html.contains("&lt;script&gt;"));
    }

    /// Rejects browser URL normalization tricks and non-web schemes.
    #[test]
    fn validates_urls() {
        for url in [
            "javascript:alert(1)",
            "data:text/html,x",
            "//evil.com",
            "/\\evil.com",
            "https://good.com\n",
        ] {
            assert!(!safe_url(url));
        }
        for url in [
            "HTTPS://example.com",
            "mailto:a@example.com",
            "/record",
            "#name",
        ] {
            assert!(safe_url(url));
        }
    }
}
