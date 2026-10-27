//! Converts untrusted Markdown to HTML without raw HTML or unsafe link and image targets.

use pulldown_cmark::{html, Event, Options, Parser, Tag, TagEnd};

/// How images in the Markdown are rendered.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub(crate) enum Images {
    /// Images are left out and only their alt text is shown, so nothing is fetched.
    Remove,
    /// Images with web or root-relative sources are kept; others show their alt text.
    SafeSources,
}

/// Allows explicit web, email, root-relative and fragment links only.
pub(crate) fn safe_url(url: &str) -> bool {
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

/// Allows web and root-relative image sources, such as uploaded `/api/file/…` images.
fn safe_image_url(url: &str) -> bool {
    safe_url(url) && !url.to_ascii_lowercase().starts_with("mailto:") && !url.starts_with('#')
}

/// Renders Markdown with tables and strikethrough, escaping raw HTML and dropping unsafe targets.
pub(crate) fn safe_markdown_html(value: &str, images: Images) -> String {
    let mut links = Vec::new();
    let mut kept_images = Vec::new();
    let events = Parser::new_ext(
        value,
        Options::ENABLE_TABLES | Options::ENABLE_STRIKETHROUGH,
    )
    .filter_map(|event| match event {
        Event::Html(text) | Event::InlineHtml(text) => Some(Event::Text(text)),
        Event::Start(Tag::Image { ref dest_url, .. }) => {
            let allowed = images == Images::SafeSources && safe_image_url(dest_url);
            kept_images.push(allowed);
            allowed.then_some(event)
        }
        Event::End(TagEnd::Image) => kept_images.pop().unwrap_or(false).then_some(event),
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
        let html = safe_markdown_html(
            "**Bold** and `code`\n\n- item\n\n[web](https://example.com)",
            Images::Remove,
        );
        assert!(html.contains("<strong>Bold</strong>"));
        assert!(html.contains("<code>code</code>"));
        assert!(html.contains("<li>item</li>"));
        assert!(html.contains("href=\"https://example.com\""));
    }

    /// Untrusted markup and images cannot execute code or load remote assets.
    #[test]
    fn blocks_active_content() {
        let html = safe_markdown_html("<script>alert(1)</script>\n\n<img src=x onerror=alert(1)>\n\n![alt](https://example.com/pixel)\n\n[bad](javascript:alert%281%29)", Images::Remove);
        assert!(!html.contains("<script>"));
        assert!(!html.contains("<img"));
        assert!(!html.contains("href="));
        assert!(html.contains("alt"));
        assert!(html.contains("&lt;script&gt;"));
    }

    /// Editor content keeps safe images, while raw HTML, script URLs and event handlers stay text.
    #[test]
    fn editor_html_keeps_safe_images_only() {
        let html = safe_markdown_html(
            "Hi <img src=x onerror=alert(1)>\n\n<iframe src=\"javascript:alert(1)\"></iframe>\n\n[klick](javascript:alert(1))\n\n![upload](/api/file/1)\n\n![web](https://example.com/a.png)\n\n![bad](javascript:alert(1))\n\n![data](data:image/svg+xml,x)",
            Images::SafeSources,
        );
        assert!(!html.contains("<iframe"));
        // The script URL survives only inside the escaped iframe text, never as a link.
        assert!(!html.contains("<a "));
        assert!(!html.contains("src=\"data:"));
        assert!(html.contains("&lt;iframe"));
        assert!(html.contains("&lt;img src=x onerror=alert(1)&gt;"));
        assert!(html.contains("<img src=\"/api/file/1\" alt=\"upload\" />"));
        assert!(html.contains("<img src=\"https://example.com/a.png\" alt=\"web\" />"));
        assert!(html.contains("bad"));
        assert_eq!(html.matches("<img").count(), 2);
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
        assert!(!safe_image_url("mailto:a@example.com"));
        assert!(!safe_image_url("#name"));
        assert!(safe_image_url("/api/file/1"));
    }
}
