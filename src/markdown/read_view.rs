use leptos::{html, prelude::*};

use super::dom::{decorate_rendered_content, defer_markdown_image_downloads, markdown_to_html};
use super::download::MarkdownImageDownloadHandler;
use super::images::MarkdownImages;

/// Read-only rendering of [`MarkdownEditor`](super::MarkdownEditor) with `render_only`.
///
/// Uses the same markup and classes as the editor, without toolbar, popups or selection
/// handling, so read-only fields stay cheap.
#[component]
pub(super) fn MarkdownReadView(
    /// Markdown rendered into the view.
    value: MaybeProp<String>,
    /// Optional id applied to the content element.
    id: Option<String>,
    /// Root class names, the same the editor would use.
    class_name: String,
    /// Content shell class names, the same the editor would use.
    shell_class_name: String,
    /// Height of the content area in any valid CSS size.
    height: String,
    /// Placeholder shown while the value is empty.
    placeholder: MaybeProp<String>,
    /// Optional async download hook used to resolve rendered image sources.
    on_image_download: Option<MarkdownImageDownloadHandler>,
) -> impl IntoView {
    let content_ref = NodeRef::<html::Div>::new();
    let images = MarkdownImages::new(on_image_download);

    // Render once the element exists and again whenever the value changes.
    Effect::new(move |_| {
        let Some(content) = content_ref.try_get().flatten() else {
            return;
        };
        let html = value
            .try_with(|value| markdown_to_html(value.as_deref().unwrap_or_default()))
            .unwrap_or_default();
        let html = if images.defers_downloads() {
            defer_markdown_image_downloads(html)
        } else {
            html
        };
        content.set_inner_html(&html);
        decorate_rendered_content(&content, false);
        images.resolve(
            &content,
            move || content_ref.try_get_untracked().flatten().map(Into::into),
            |_| {},
        );
    });

    view! {
        <div class=class_name style=format!("--birei-markdown-editor-height: {height};")>
            <div class=shell_class_name>
                <div
                    id=id
                    node_ref=content_ref
                    class="birei-markdown__editor"
                    role="document"
                    aria-readonly="true"
                    data-placeholder=move || placeholder.try_get().flatten().unwrap_or_default()
                    data-birei-markdown-editor="true"
                    tabindex=-1
                    contenteditable="false"
                ></div>
            </div>
        </div>
    }
}
