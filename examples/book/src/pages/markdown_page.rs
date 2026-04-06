use birei::{ButtonBarItem, Card, MarkdownEditor, MarkdownImageUploadHandler, Size};
use leptos::prelude::*;
use web_sys::Url;
use crate::code_example::{CodeExample, CodeExampleLanguage};

#[component]
pub fn MarkdownPage() -> impl IntoView {
    let markdown = RwSignal::new(String::from(
        "# Project brief\n\nWrite directly in the rendered content.\n\n## Checklist\n\n- Format text with the toolbar\n- Add links and images\n- Edit table cells inline\n\n| Area | Status |\n| --- | --- |\n| Content | Ready |\n| Review | Pending |\n\n![Preview](https://placehold.co/640x240/png)\n",
    ));

    let extra_toolbar = vec![ButtonBarItem::new("insert-divider", "Divider").icon("minus")];
    let image_upload = MarkdownImageUploadHandler::new(|file| async move {
        Url::create_object_url_with_blob(&file)
            .map_err(|_| String::from("Image upload failed while creating a preview URL."))
    });

    view! {
        <section class="page-header">
            <div class="page-header__eyebrow">"Component"</div>
            <h2>"Markdown Editor"</h2>
            <p class="page-header__lede">
                "A WYSIWYG markdown editor that renders markdown as editable HTML and emits normalized markdown when the editor blurs."
            </p>
        </section>

        <section class="doc-grid">
            <Card class="doc-card">
                <span class="doc-card__kicker">"WYSIWYG"</span>
                <div class="doc-card__preview doc-card__preview--stack">
                    <MarkdownEditor
                        value=markdown
                        size=Size::Medium
                        placeholder="Start writing markdown..."
                        toolbar_items=extra_toolbar
                        on_change=Callback::new(move |next| markdown.set(next))
                        on_toolbar_action=Callback::new(move |action| {
                            if action == "insert-divider" {
                                markdown.update(|value| {
                                    if !value.ends_with("\n") {
                                        value.push('\n');
                                    }
                                    value.push_str("\n---\n");
                                });
                            }
                        })
                        on_image_upload=image_upload
                    />
                    <p class="doc-card__copy">
                        "Blur the editor to receive normalized markdown back from the component."
                    </p>
                    <CodeExample
                        code_signal=Signal::derive(move || markdown.get())
                        language=CodeExampleLanguage::PlainText
                        title="Markdown Source"
                    />
                </div>
                <CodeExample code={r#"<MarkdownEditor
    value=markdown
    toolbar_items=vec![
        ButtonBarItem::new("insert-divider", "Divider").icon("minus"),
    ]
    on_change=Callback::new(move |next| markdown.set(next))
    on_toolbar_action=Callback::new(move |action| {
        if action == "insert-divider" {
            markdown.update(|value| value.push_str("\n---\n"));
        }
    })
    on_image_upload=MarkdownImageUploadHandler::new(|file| async move {
        web_sys::Url::create_object_url_with_blob(&file)
            .map_err(|_| String::from("Image upload failed."))
    })
/>"#}/>
            </Card>
        </section>
    }
}
