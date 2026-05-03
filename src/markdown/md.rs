use std::cell::RefCell;
use std::rc::Rc;

use leptos::ev;
use leptos::html;
use leptos::prelude::*;
use leptos::task::spawn_local;
use wasm_bindgen::JsCast;
use web_sys::{window, DomRect, HtmlElement, HtmlInputElement, HtmlTextAreaElement, KeyboardEvent, Range};

use crate::common::{
    measure_floating_popup_layout, measure_floating_popup_layout_in_container, FloatingPopupLayout,
};
use crate::{ButtonBarItem, ButtonGroup, ButtonVariant, Size};

use super::dom::{
    decorate_rendered_content, escape_html_attribute, escape_html_text, exec_document_command,
    insert_html_at_saved_range, markdown_from_html, markdown_to_html,
};
use super::effects::{
    setup_heading_popup_effects, setup_link_popup_effects, setup_table_popup_effects,
};
use super::menu::default_toolbar_items;
use super::table::{
    apply_table_action, current_table_selection, move_to_adjacent_cell, table_selection_from_range,
};
use super::upload::MarkdownImageUploadHandler;
use super::view::{
    render_heading_popup, render_link_popup, render_table_popup, render_toolbar_view,
    ToolbarViewProps,
};

/// WYSIWYG markdown editor that renders markdown as editable HTML and emits normalized markdown on blur.
#[component]
pub fn MarkdownEditor(
    /// Current markdown value rendered into editable HTML.
    #[prop(optional, into)]
    value: MaybeProp<String>,
    /// Placeholder text shown while the editor is empty.
    #[prop(optional, into)]
    placeholder: MaybeProp<String>,
    /// Optional id applied to the editor root.
    #[prop(optional, into)]
    id: Option<String>,
    /// Height of the editor surface in any valid CSS size, for example `14rem` or `320px`.
    #[prop(optional, into)]
    height: Option<String>,
    /// Additional CSS class names applied to the component root.
    #[prop(optional, into)]
    class: Option<String>,
    /// Disables the editor and toolbar.
    #[prop(optional)]
    disabled: bool,
    /// Marks the editor as read-only while still rendering the toolbar.
    #[prop(optional)]
    readonly: bool,
    /// Marks the editor as invalid for styling and accessibility.
    #[prop(optional)]
    invalid: bool,
    /// Shows the built-in formatting buttons.
    #[prop(optional, default = true)]
    show_default_toolbar: bool,
    /// Additional toolbar buttons appended after the built-in formatting actions.
    #[prop(optional, into)]
    toolbar_items: MaybeProp<Vec<ButtonBarItem>>,
    /// Shared button styling used by the toolbar.
    #[prop(optional, default = ButtonVariant::Secondary)]
    toolbar_variant: ButtonVariant,
    /// Emits normalized markdown after the editor loses focus and the content changed.
    #[prop(optional)]
    on_change: Option<Callback<String>>,
    /// Receives unknown toolbar item values so consumers can implement custom buttons.
    #[prop(optional)]
    on_toolbar_action: Option<Callback<String>>,
    /// Optional async upload hook used by the built-in image button.
    #[prop(optional)]
    on_image_upload: Option<MarkdownImageUploadHandler>,
) -> impl IntoView {
    fn update_markdown_source_textarea(
        textarea: &HtmlTextAreaElement,
        markdown_source: RwSignal<String>,
        next_value: String,
        selection_start: usize,
        selection_end: usize,
    ) {
        textarea.set_value(&next_value);
        markdown_source.set(next_value);
        let _ = textarea.set_selection_range(selection_start as u32, selection_end as u32);
        let _ = textarea.focus();
    }

    fn wrap_markdown_selection(
        textarea: &HtmlTextAreaElement,
        markdown_source: RwSignal<String>,
        prefix: &str,
        suffix: &str,
        placeholder: &str,
    ) {
        let value = textarea.value();
        let start = textarea
            .selection_start()
            .ok()
            .flatten()
            .unwrap_or(value.len() as u32) as usize;
        let end = textarea
            .selection_end()
            .ok()
            .flatten()
            .unwrap_or(start as u32) as usize;
        let selected = &value[start..end];
        let inner = if selected.is_empty() { placeholder } else { selected };
        let replacement = format!("{prefix}{inner}{suffix}");
        let next = format!("{}{}{}", &value[..start], replacement, &value[end..]);
        let selection_start = if selected.is_empty() {
            start + prefix.len()
        } else {
            start
        };
        let selection_end = if selected.is_empty() {
            start + prefix.len() + inner.len()
        } else {
            start + replacement.len()
        };
        update_markdown_source_textarea(
            textarea,
            markdown_source,
            next,
            selection_start,
            selection_end,
        );
    }

    fn prefix_markdown_lines(
        textarea: &HtmlTextAreaElement,
        markdown_source: RwSignal<String>,
        prefix_builder: impl Fn(usize) -> String,
    ) {
        let value = textarea.value();
        let start = textarea
            .selection_start()
            .ok()
            .flatten()
            .unwrap_or(value.len() as u32) as usize;
        let end = textarea
            .selection_end()
            .ok()
            .flatten()
            .unwrap_or(start as u32) as usize;
        let line_start = value[..start].rfind('\n').map(|index| index + 1).unwrap_or(0);
        let line_end = value[end..]
            .find('\n')
            .map(|index| end + index)
            .unwrap_or(value.len());
        let block = &value[line_start..line_end];
        let prefixed = block
            .split('\n')
            .enumerate()
            .map(|(index, line)| format!("{}{}", prefix_builder(index), line))
            .collect::<Vec<_>>()
            .join("\n");
        let next = format!("{}{}{}", &value[..line_start], prefixed, &value[line_end..]);
        update_markdown_source_textarea(
            textarea,
            markdown_source,
            next,
            line_start,
            line_start + prefixed.len(),
        );
    }

    fn insert_markdown_text(
        textarea: &HtmlTextAreaElement,
        markdown_source: RwSignal<String>,
        text: &str,
        cursor_offset: usize,
    ) {
        let value = textarea.value();
        let start = textarea
            .selection_start()
            .ok()
            .flatten()
            .unwrap_or(value.len() as u32) as usize;
        let end = textarea
            .selection_end()
            .ok()
            .flatten()
            .unwrap_or(start as u32) as usize;
        let next = format!("{}{}{}", &value[..start], text, &value[end..]);
        let cursor = start + cursor_offset;
        update_markdown_source_textarea(textarea, markdown_source, next, cursor, cursor);
    }

    // DOM refs and transient popup state are kept local because the editor
    // bridges markdown, HTML, browser selection ranges, and file input flows.
    let initial_markdown = value.get_untracked().unwrap_or_default();
    let editor_ref = NodeRef::<html::Div>::new();
    let root_ref = NodeRef::<html::Div>::new();
    let markdown_source_ref = NodeRef::<html::Textarea>::new();
    let file_input_ref = NodeRef::<html::Input>::new();
    let link_input_ref = NodeRef::<html::Input>::new();
    let heading_popup_ref = NodeRef::<html::Div>::new();
    let table_popup_ref = NodeRef::<html::Div>::new();
    let heading_button_ref = NodeRef::<html::Button>::new();
    let link_button_ref = NodeRef::<html::Button>::new();
    let table_button_ref = NodeRef::<html::Button>::new();
    let has_focus = RwSignal::new(false);
    let last_committed_markdown = RwSignal::new(initial_markdown.clone());
    let upload_error = RwSignal::new(None::<String>);
    let saved_range = Rc::new(RefCell::new(None::<Range>));
    let link_popup_open = RwSignal::new(false);
    let link_popup_layout = RwSignal::new(FloatingPopupLayout::default());
    let link_url = RwSignal::new(String::new());
    let heading_popup_open = RwSignal::new(false);
    let heading_popup_layout = RwSignal::new(FloatingPopupLayout::default());
    let table_popup_open = RwSignal::new(false);
    let table_popup_layout = RwSignal::new(FloatingPopupLayout::default());
    let table_button_is_menu = RwSignal::new(false);
    let image_picker_open = RwSignal::new(false);
    let markdown_view_open = RwSignal::new(false);
    let markdown_source = RwSignal::new(initial_markdown);
    let editor_line_style = RwSignal::new(String::from("--birei-markdown-line-origin: 50%;"));
    let editor_height = height.unwrap_or_else(|| String::from("14rem"));

    let measure_popup_layout: Rc<dyn Fn(&DomRect) -> FloatingPopupLayout> = Rc::new({
        let root_ref = root_ref;
        move |anchor_rect: &DomRect| {
            root_ref
                .get_untracked()
                .map(|root| {
                    measure_floating_popup_layout_in_container(
                        anchor_rect,
                        &root.get_bounding_client_rect(),
                    )
                })
                .unwrap_or_else(|| measure_floating_popup_layout(anchor_rect))
        }
    });

    // Root classes mirror the shared textarea sizing tokens plus editor state.
    let class_name = move || {
        let mut classes = vec!["birei-markdown"];
        if disabled {
            classes.push("birei-markdown--disabled");
        }
        if readonly {
            classes.push("birei-markdown--readonly");
        }
        if invalid {
            classes.push("birei-markdown--invalid");
        }
        if let Some(class) = class.as_deref() {
            classes.push(class);
        }
        classes.join(" ")
    };

    // Toolbar composition happens once from the built-in and caller-supplied
    // button definitions.
    let toolbar_buttons = {
        let mut items = if show_default_toolbar {
            default_toolbar_items()
        } else {
            Vec::new()
        };
        items.extend(toolbar_items.get_untracked().unwrap_or_default());
        items
    };

    // Rendering markdown into the editor always reapplies contenteditable flags
    // because replacing inner HTML discards them.
    let render_editor_value = move |markdown: &str| {
        let Some(editor) = editor_ref.get_untracked() else {
            return;
        };

        let html = markdown_to_html(markdown);
        editor.set_inner_html(&html);
        decorate_rendered_content(&editor, !(disabled || readonly));
    };

    // Committing reads the live HTML back into markdown, normalizes it, and
    // emits changes only when the value actually changed.
    let commit_editor_value = Rc::new(move || {
        let Some(editor) = editor_ref.get_untracked() else {
            return;
        };

        let markdown = markdown_from_html(&editor.inner_html());
        let previous = last_committed_markdown.get_untracked();

        render_editor_value(&markdown);
        markdown_source.set(markdown.clone());
        last_committed_markdown.set(markdown.clone());

        if markdown != previous {
            if let Some(on_change) = on_change.as_ref() {
                on_change.run(markdown);
            }
        }
    });

    // Keep the rich-text DOM synchronized from the shared markdown state once
    // the editor node exists and while the user is not actively editing it.
    Effect::new(move |_| {
        let _ = editor_ref.get();
        if has_focus.get() || markdown_view_open.get() {
            return;
        }

        render_editor_value(&markdown_source.get());
    });

    // Controlled external values replace the editor only while the user is not
    // actively interacting with it.
    Effect::new(move |_| {
        let next_markdown = value.get().unwrap_or_default();
        if has_focus.get() || next_markdown == last_committed_markdown.get_untracked() {
            return;
        }

        markdown_source.set(next_markdown.clone());
        last_committed_markdown.set(next_markdown);
    });

    // Selection is saved into a reusable DOM range so toolbar actions and
    // popups can restore it after focus moves away from the editor.
    let save_selection: Rc<dyn Fn()> = Rc::new({
        let saved_range = Rc::clone(&saved_range);
        move || {
            let Some(selection) = window().and_then(|window| window.get_selection().ok().flatten())
            else {
                return;
            };

            if selection.range_count() == 0 {
                return;
            }

            if let Ok(range) = selection.get_range_at(0) {
                *saved_range.borrow_mut() = Some(range);
            }
        }
    });

    // Restores the last saved DOM selection back into the browser selection.
    let restore_selection: Rc<dyn Fn()> = Rc::new({
        let saved_range = Rc::clone(&saved_range);
        move || {
            let Some(range) = saved_range.borrow().clone() else {
                return;
            };
            let Some(selection) = window().and_then(|window| window.get_selection().ok().flatten())
            else {
                return;
            };

            let _ = selection.remove_all_ranges();
            let _ = selection.add_range(&range);
        }
    });

    // Popups share the same range-based positioning strategy but each one owns
    // its own open/close lifecycle and commit timing.
    let restore_selection_for_toolbar = Rc::clone(&restore_selection);
    let save_selection_for_toolbar = Rc::clone(&save_selection);
    let commit_after_popup_close = Rc::clone(&commit_editor_value);
    let open_link_popup: Rc<dyn Fn()> = Rc::new({
        let measure_popup_layout = Rc::clone(&measure_popup_layout);
        move || {
            if let Some(button) = link_button_ref.get_untracked() {
                link_popup_layout.set(measure_popup_layout(&button.get_bounding_client_rect()));
                link_url.set(String::new());
                link_popup_open.set(true);
            }
        }
    });
    let open_heading_popup: Rc<dyn Fn()> = Rc::new({
        let measure_popup_layout = Rc::clone(&measure_popup_layout);
        move || {
            if let Some(button) = heading_button_ref.get_untracked() {
                heading_popup_layout.set(measure_popup_layout(&button.get_bounding_client_rect()));
                heading_popup_open.set(true);
            }
        }
    });
    let commit_after_heading_popup_close = Rc::clone(&commit_editor_value);
    let close_heading_popup: Rc<dyn Fn()> = Rc::new(move || {
        heading_popup_open.set(false);
        if !has_focus.get_untracked() {
            commit_after_heading_popup_close();
        }
    });
    let close_heading_popup_for_toolbar = Rc::clone(&close_heading_popup);
    let close_link_popup: Rc<dyn Fn()> = Rc::new(move || {
        link_popup_open.set(false);
        if !has_focus.get_untracked() {
            commit_after_popup_close();
        }
    });
    let commit_after_table_popup_close = Rc::clone(&commit_editor_value);
    let saved_range_for_table_popup = Rc::clone(&saved_range);
    let open_table_popup: Rc<dyn Fn()> = Rc::new({
        let measure_popup_layout = Rc::clone(&measure_popup_layout);
        move || {
            if let Some(button) = table_button_ref.get_untracked() {
                table_popup_layout.set(measure_popup_layout(&button.get_bounding_client_rect()));
            } else {
                let Some(range) = saved_range_for_table_popup.borrow().clone() else {
                    return;
                };
                let Some(selection) = table_selection_from_range(&range) else {
                    return;
                };
                table_popup_layout
                    .set(measure_popup_layout(&selection.cell.get_bounding_client_rect()));
            }
            table_popup_open.set(true);
        }
    });
    let close_table_popup: Rc<dyn Fn()> = Rc::new(move || {
        table_popup_open.set(false);
        if !has_focus.get_untracked() {
            commit_after_table_popup_close();
        }
    });
    let close_heading_popup_on_toggle = Rc::clone(&close_heading_popup);
    let close_link_popup_on_toggle = Rc::clone(&close_link_popup);
    let close_table_popup_on_toggle = Rc::clone(&close_table_popup);
    // Table insertion and link insertion reuse the last saved selection range.
    let insert_at_saved_range = Rc::new({
        let saved_range = Rc::clone(&saved_range);
        move |html: &str| {
            insert_html_at_saved_range(&saved_range, html);
        }
    });

    let apply_link: Rc<dyn Fn()> = Rc::new({
        let saved_range = Rc::clone(&saved_range);
        let close_link_popup = Rc::clone(&close_link_popup);
        move || {
            let href = link_url.get_untracked().trim().to_owned();
            if href.is_empty() {
                return;
            }

            if markdown_view_open.get_untracked() {
                if let Some(textarea) = markdown_source_ref.get_untracked() {
                    let value = textarea.value();
                    let start = textarea
                        .selection_start()
                        .ok()
                        .flatten()
                        .unwrap_or(value.len() as u32) as usize;
                    let end = textarea
                        .selection_end()
                        .ok()
                        .flatten()
                        .unwrap_or(start as u32) as usize;
                    let selected = &value[start..end];
                    let text = if selected.trim().is_empty() {
                        href.clone()
                    } else {
                        selected.to_string()
                    };
                    let replacement = format!("[{text}]({href})");
                    let next = format!("{}{}{}", &value[..start], replacement, &value[end..]);
                    update_markdown_source_textarea(
                        &textarea,
                        markdown_source,
                        next,
                        start,
                        start + replacement.len(),
                    );
                }
                close_link_popup();
                return;
            }

            let link_text = saved_range
                .borrow()
                .as_ref()
                .map(|range| range.to_string().as_string().unwrap_or_default())
                .filter(|value| !value.trim().is_empty())
                .unwrap_or_else(|| href.clone());
            let link_html = format!(
                r#"<a href="{}" target="_blank" rel="noopener noreferrer">{}</a>"#,
                escape_html_attribute(&href),
                escape_html_text(&link_text)
            );
            insert_html_at_saved_range(&saved_range, &link_html);
            close_link_popup();
        }
    });

    // Toolbar actions centralize every editor command, popup open, and custom
    // extension hook into one dispatch point.
    let commit_editor_value_for_toolbar = Rc::clone(&commit_editor_value);
    let handle_toolbar_action: Rc<dyn Fn(String)> = Rc::new(move |action: String| {
        if disabled || readonly {
            return;
        }

        upload_error.set(None);
        match action.as_str() {
            "toggle-markdown-view" => {
                close_heading_popup_on_toggle();
                close_link_popup_on_toggle();
                close_table_popup_on_toggle();

                if markdown_view_open.get_untracked() {
                    let markdown = markdown_source.get_untracked();
                    let previous = last_committed_markdown.get_untracked();
                    render_editor_value(&markdown);
                    last_committed_markdown.set(markdown.clone());
                    if markdown != previous {
                        if let Some(on_change) = on_change.as_ref() {
                            on_change.run(markdown.clone());
                        }
                    }
                    markdown_view_open.set(false);
                } else {
                    commit_editor_value_for_toolbar();
                    markdown_view_open.set(true);
                }
            }
            "bold" => {
                if markdown_view_open.get_untracked() {
                    if let Some(textarea) = markdown_source_ref.get_untracked() {
                        wrap_markdown_selection(&textarea, markdown_source, "**", "**", "bold");
                    }
                    return;
                }
                restore_selection_for_toolbar();
                exec_document_command("bold", None);
            }
            "italic" => {
                if markdown_view_open.get_untracked() {
                    if let Some(textarea) = markdown_source_ref.get_untracked() {
                        wrap_markdown_selection(&textarea, markdown_source, "*", "*", "italic");
                    }
                    return;
                }
                restore_selection_for_toolbar();
                exec_document_command("italic", None);
            }
            "heading" => {
                save_selection_for_toolbar();
                open_heading_popup();
            }
            "heading-1" | "heading-2" | "heading-3" | "heading-4" | "heading-paragraph" => {
                if markdown_view_open.get_untracked() {
                    if let Some(textarea) = markdown_source_ref.get_untracked() {
                        if action == "heading-paragraph" {
                            let value = textarea.value();
                            let start = textarea
                                .selection_start()
                                .ok()
                                .flatten()
                                .unwrap_or(value.len() as u32) as usize;
                            let end = textarea
                                .selection_end()
                                .ok()
                                .flatten()
                                .unwrap_or(start as u32) as usize;
                            let line_start =
                                value[..start].rfind('\n').map(|index| index + 1).unwrap_or(0);
                            let line_end = value[end..]
                                .find('\n')
                                .map(|index| end + index)
                                .unwrap_or(value.len());
                            let block = &value[line_start..line_end];
                            let unheaded = block
                                .split('\n')
                                .map(|line| {
                                    let trimmed = line.trim_start_matches('#');
                                    trimmed
                                        .strip_prefix(' ')
                                        .unwrap_or(trimmed)
                                        .to_string()
                                })
                                .collect::<Vec<_>>()
                                .join("\n");
                            let next = format!(
                                "{}{}{}",
                                &value[..line_start],
                                unheaded,
                                &value[line_end..]
                            );
                            update_markdown_source_textarea(
                                &textarea,
                                markdown_source,
                                next,
                                line_start,
                                line_start + unheaded.len(),
                            );
                        } else {
                            let prefix = match action.as_str() {
                                "heading-1" => "# ",
                                "heading-2" => "## ",
                                "heading-3" => "### ",
                                _ => "#### ",
                            };
                            prefix_markdown_lines(&textarea, markdown_source, move |_| {
                                prefix.to_string()
                            });
                        }
                    }
                    close_heading_popup_for_toolbar();
                    return;
                }
                restore_selection_for_toolbar();
                exec_document_command(
                    "formatBlock",
                    Some(match action.as_str() {
                        "heading-1" => "h1",
                        "heading-2" => "h2",
                        "heading-3" => "h3",
                        "heading-4" => "h4",
                        _ => "p",
                    }),
                );
                close_heading_popup_for_toolbar();
            }
            "unordered-list" => {
                if markdown_view_open.get_untracked() {
                    if let Some(textarea) = markdown_source_ref.get_untracked() {
                        prefix_markdown_lines(&textarea, markdown_source, |_| String::from("- "));
                    }
                    return;
                }
                restore_selection_for_toolbar();
                exec_document_command("insertUnorderedList", None);
            }
            "ordered-list" => {
                if markdown_view_open.get_untracked() {
                    if let Some(textarea) = markdown_source_ref.get_untracked() {
                        prefix_markdown_lines(&textarea, markdown_source, |index| {
                            format!("{}. ", index + 1)
                        });
                    }
                    return;
                }
                restore_selection_for_toolbar();
                exec_document_command("insertOrderedList", None);
            }
            "link" => {
                if markdown_view_open.get_untracked() {
                    open_link_popup();
                } else {
                    save_selection_for_toolbar();
                    open_link_popup();
                }
            }
            "table" => {
                if markdown_view_open.get_untracked() {
                    if let Some(textarea) = markdown_source_ref.get_untracked() {
                        insert_markdown_text(
                            &textarea,
                            markdown_source,
                            "| Column 1 | Column 2 |\n| --- | --- |\n| Value 1 | Value 2 |",
                            0,
                        );
                    }
                    return;
                }
                save_selection_for_toolbar();
                if current_table_selection().is_some() {
                    table_button_is_menu.set(true);
                    open_table_popup();
                } else {
                    table_button_is_menu.set(false);
                    restore_selection_for_toolbar();
                    insert_at_saved_range(
                        r#"<table><thead><tr><th contenteditable="true">Column 1</th><th contenteditable="true">Column 2</th></tr></thead><tbody><tr><td contenteditable="true">Value 1</td><td contenteditable="true">Value 2</td></tr></tbody></table>"#,
                    );
                }
            }
            "insert-divider" => {
                if markdown_view_open.get_untracked() {
                    if let Some(textarea) = markdown_source_ref.get_untracked() {
                        insert_markdown_text(&textarea, markdown_source, "\n---\n", 5);
                    }
                    return;
                }
                restore_selection_for_toolbar();
                insert_at_saved_range(r#"<hr />"#);
            }
            "image" => {
                if !markdown_view_open.get_untracked() {
                    save_selection_for_toolbar();
                }
                image_picker_open.set(true);
                if let Some(input) = file_input_ref.get_untracked() {
                    input.set_value("");
                    input.click();
                }
            }
            _ => {
                if let Some(on_toolbar_action) = on_toolbar_action.as_ref() {
                    on_toolbar_action.run(action);
                }
            }
        }
    });

    // Image uploads support both synchronous filename insertion and async
    // upload handlers that resolve to final image URLs.
    let handle_image_change = {
        let on_image_upload = on_image_upload.clone();
        let saved_range = Rc::clone(&saved_range);
        let commit_editor_value = Rc::clone(&commit_editor_value);
        move |event: ev::Event| {
            image_picker_open.set(false);
            if disabled || readonly {
                return;
            }

            let Some(target) = event
                .target()
                .and_then(|target| target.dyn_into::<HtmlInputElement>().ok())
            else {
                return;
            };
            let Some(files) = target.files() else {
                if !has_focus.get_untracked() && !link_popup_open.get_untracked() {
                    commit_editor_value();
                }
                return;
            };
            let Some(file) = files.get(0) else {
                if !has_focus.get_untracked() && !link_popup_open.get_untracked() {
                    commit_editor_value();
                }
                return;
            };
            let file_name = file.name();

            if let Some(handler) = on_image_upload.clone() {
                let restore_selection = Rc::clone(&restore_selection);
                let saved_range = Rc::clone(&saved_range);
                let commit_after_image = Rc::clone(&commit_editor_value);
                let markdown_source_ref = markdown_source_ref;
                let upload_error = upload_error;
                spawn_local(async move {
                    match handler.run(file).await {
                        Ok(url) => {
                            if markdown_view_open.get_untracked() {
                                if let Some(textarea) = markdown_source_ref.get_untracked() {
                                    insert_markdown_text(
                                        &textarea,
                                        markdown_source,
                                        &format!("![{}]({})", file_name, url),
                                        0,
                                    );
                                }
                            } else {
                                restore_selection();
                                insert_html_at_saved_range(
                                    &saved_range,
                                    &format!(
                                        r#"<img src="{}" alt="{}" />"#,
                                        escape_html_attribute(&url),
                                        escape_html_attribute(&file_name)
                                    ),
                                );
                            }
                        }
                        Err(error) => {
                            upload_error.set(Some(error));
                        }
                    }

                    if !has_focus.get_untracked() && !link_popup_open.get_untracked() {
                        commit_after_image();
                    }
                });
            } else {
                if markdown_view_open.get_untracked() {
                    if let Some(textarea) = markdown_source_ref.get_untracked() {
                        insert_markdown_text(
                            &textarea,
                            markdown_source,
                            &format!("![{}]({})", file_name, file_name),
                            0,
                        );
                    }
                } else {
                    restore_selection();
                    insert_html_at_saved_range(
                        &saved_range,
                        &format!(
                            r#"<img src="{}" alt="{}" />"#,
                            escape_html_attribute(&file_name),
                            escape_html_attribute(&file_name)
                        ),
                    );
                }

                if !has_focus.get_untracked() && !link_popup_open.get_untracked() {
                    commit_editor_value();
                }
            }
        }
    };

    // Each popup installs its own browser-level listeners through small setup
    // helpers to keep the main component body manageable.
    setup_link_popup_effects(
        link_popup_open,
        link_button_ref,
        link_input_ref,
        link_popup_layout,
        Rc::clone(&close_link_popup),
        Rc::clone(&measure_popup_layout),
    );
    setup_heading_popup_effects(
        heading_popup_open,
        heading_button_ref,
        heading_popup_ref,
        heading_popup_layout,
        Rc::clone(&close_heading_popup),
        Rc::clone(&measure_popup_layout),
    );
    setup_table_popup_effects(
        table_popup_open,
        table_button_ref,
        table_popup_ref,
        table_popup_layout,
        Rc::clone(&close_table_popup),
        Rc::clone(&measure_popup_layout),
    );

    // Toolbar buttons share a common button class string derived from the
    // configured toolbar variant and a fixed small size.
    let toolbar_button_class = format!(
        "birei-button {} {}",
        toolbar_variant.class_name(),
        Size::Small.button_class_name()
    );
    let toolbar_selected_button_class = format!(
        "birei-button {} {}",
        ButtonVariant::Primary.class_name(),
        Size::Small.button_class_name()
    );
    let refresh_table_button_state = move || {
        table_button_is_menu.set(current_table_selection().is_some());
    };
    let save_selection_on_mouseup = Rc::clone(&save_selection);
    let save_selection_on_keyup = Rc::clone(&save_selection);
    let save_selection_on_input = Rc::clone(&save_selection);
    let save_selection_after_table_move = Rc::clone(&save_selection);
    let handle_editor_pointer_down = move |event: ev::PointerEvent| {
        if let Some(target) = event
            .current_target()
            .and_then(|target| target.dyn_into::<HtmlElement>().ok())
        {
            let rect = target.get_bounding_client_rect();
            let x = f64::from(event.client_x()) - rect.left();
            editor_line_style.set(format!("--birei-markdown-line-origin: {x}px;"));
        }
    };

    // Tab handling inside tables moves between cells instead of leaving the
    // editor, including creating a new row when needed.
    let handle_editor_keydown = move |event: KeyboardEvent| {
        if disabled || readonly || event.key() != "Tab" {
            return;
        }

        let Some(selection) = current_table_selection() else {
            return;
        };

        event.prevent_default();
        if move_to_adjacent_cell(&selection, event.shift_key()).is_some() {
            if table_popup_open.get_untracked() {
                if let Some(button) = table_button_ref.get_untracked() {
                    table_popup_layout.set(measure_floating_popup_layout(
                        &button.get_bounding_client_rect(),
                    ));
                }
            }
            save_selection_after_table_move();
            table_button_is_menu.set(true);
        }
    };
    let saved_range_for_table_actions = Rc::clone(&saved_range);
    let save_selection_after_table_action = Rc::clone(&save_selection);
    let close_table_popup_after_action = Rc::clone(&close_table_popup);

    // Table menu actions operate on the saved cell selection and then restore
    // selection state for continued editing.
    let handle_table_action: Rc<dyn Fn(&'static str)> = Rc::new(move |action: &'static str| {
        let Some(range) = saved_range_for_table_actions.borrow().clone() else {
            return;
        };
        let Some(selection) = table_selection_from_range(&range) else {
            return;
        };
        if apply_table_action(&selection, action).is_some() {
            save_selection_after_table_action();
            close_table_popup_after_action();
        }
    });

    let toolbar_view = render_toolbar_view(ToolbarViewProps {
        toolbar_buttons,
        toolbar_button_class,
        toolbar_selected_button_class,
        heading_button_ref,
        link_button_ref,
        table_button_ref,
        heading_popup_open,
        link_popup_open,
        markdown_view_open,
        table_button_is_menu,
        disabled,
        readonly,
        handle_toolbar_action: Rc::clone(&handle_toolbar_action),
    });

    view! {
        <div
            class=class_name
            node_ref=root_ref
            style=format!("--birei-markdown-editor-height: {editor_height};")
        >
            <div class="birei-markdown__toolbar" on:mousedown=move |event| event.prevent_default()>
                <ButtonGroup variant=toolbar_variant size=Size::Small class="birei-markdown__toolbar-group">
                    {toolbar_view}
                </ButtonGroup>
                <input
                    node_ref=file_input_ref
                    class="birei-markdown__file-input"
                    type="file"
                    accept="image/*"
                    on:change=handle_image_change
                />
            </div>

            <div
                class=move || {
                    let mut classes = vec!["birei-markdown__editor-shell"];
                    if disabled {
                        classes.push("birei-markdown__editor-shell--disabled");
                    }
                    if readonly {
                        classes.push("birei-markdown__editor-shell--readonly");
                    }
                    if invalid {
                        classes.push("birei-markdown__editor-shell--invalid");
                    }
                    classes.join(" ")
                }
                style=move || {
                    let mut style = editor_line_style.get();
                    if markdown_view_open.get() {
                        style.push_str(" display: none;");
                    }
                    style
                }
                on:pointerdown=handle_editor_pointer_down
            >
                <div
                    id=id.clone()
                    node_ref=editor_ref
                    class="birei-markdown__editor"
                    role="textbox"
                    aria-multiline="true"
                    aria-invalid=move || if invalid { "true" } else { "false" }
                    aria-disabled=move || if disabled { "true" } else { "false" }
                    aria-readonly=move || if readonly { "true" } else { "false" }
                    data-placeholder=move || placeholder.get().unwrap_or_default()
                    data-birei-markdown-editor="true"
                    tabindex=if disabled { -1 } else { 0 }
                    contenteditable=if disabled || readonly { "false" } else { "true" }
                    on:focus=move |_| has_focus.set(true)
                    on:mouseup=move |_| {
                        save_selection_on_mouseup();
                        refresh_table_button_state();
                    }
                    on:keyup=move |_| {
                        save_selection_on_keyup();
                        refresh_table_button_state();
                    }
                    on:input=move |_| {
                        save_selection_on_input();
                        refresh_table_button_state();
                    }
                    on:keydown=handle_editor_keydown
                    on:blur=move |_| {
                        has_focus.set(false);
                        refresh_table_button_state();
                        if !heading_popup_open.get_untracked()
                            && !link_popup_open.get_untracked()
                            && !table_popup_open.get_untracked()
                            && !image_picker_open.get_untracked()
                        {
                            commit_editor_value();
                        }
                    }
                ></div>
            </div>

            <div
                style=move || {
                    if markdown_view_open.get() {
                        String::new()
                    } else {
                        String::from("display: none;")
                    }
                }
            >
                <div
                    class=move || {
                        let mut classes = vec!["birei-textarea"];
                        if disabled {
                            classes.push("birei-textarea--disabled");
                        }
                        if readonly {
                            classes.push("birei-textarea--readonly");
                        }
                        if invalid {
                            classes.push("birei-textarea--invalid");
                        }
                        classes.push("birei-markdown__source");
                        classes.join(" ")
                    }
                    style="--birei-textarea-line-origin: 50%;"
                >
                    <textarea
                        node_ref=markdown_source_ref
                        class="birei-textarea__field"
                        prop:value=move || markdown_source.get()
                        rows=12
                        placeholder=move || placeholder.get().unwrap_or_default()
                        disabled=disabled
                        readonly=readonly
                        aria-invalid=move || if invalid { "true" } else { "false" }
                        on:focus=move |_| has_focus.set(true)
                        on:input=move |event| markdown_source.set(event_target_value(&event))
                        on:blur=move |_| {
                            has_focus.set(false);
                            let markdown = markdown_source.get_untracked();
                            let previous = last_committed_markdown.get_untracked();
                            last_committed_markdown.set(markdown.clone());
                            if markdown != previous {
                                if let Some(on_change) = on_change.as_ref() {
                                    on_change.run(markdown);
                                }
                            }
                        }
                    ></textarea>
                </div>
            </div>

            {render_heading_popup(
                heading_popup_ref,
                heading_popup_open,
                heading_popup_layout,
                Rc::clone(&handle_toolbar_action),
            )}

            {render_table_popup(
                table_popup_ref,
                table_popup_open,
                table_popup_layout,
                Rc::clone(&handle_table_action),
            )}

            {render_link_popup(
                link_popup_open,
                link_popup_layout,
                link_input_ref,
                link_url,
                Rc::clone(&apply_link),
                Rc::clone(&close_link_popup),
            )}

            <Show when=move || upload_error.get().is_some()>
                <p class="birei-markdown__status">
                    {move || upload_error.get().unwrap_or_default()}
                </p>
            </Show>
        </div>
    }
}
