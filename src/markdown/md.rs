use std::cell::RefCell;
use std::rc::Rc;

use leptos::ev;
use leptos::html;
use leptos::prelude::*;
use leptos::task::spawn_local;
use wasm_bindgen::JsCast;
use web_sys::{window, HtmlInputElement, KeyboardEvent, Range};

use crate::common::{measure_floating_popup_layout, FloatingPopupLayout};
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
    /// Shared sizing token aligned with the rest of the component library.
    #[prop(optional)]
    size: Size,
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
    let editor_ref = NodeRef::<html::Div>::new();
    let file_input_ref = NodeRef::<html::Input>::new();
    let link_input_ref = NodeRef::<html::Input>::new();
    let heading_popup_ref = NodeRef::<html::Div>::new();
    let table_popup_ref = NodeRef::<html::Div>::new();
    let heading_button_ref = NodeRef::<html::Button>::new();
    let table_button_ref = NodeRef::<html::Button>::new();
    let has_focus = RwSignal::new(false);
    let last_committed_markdown = RwSignal::new(String::new());
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

    let class_name = move || {
        let mut classes = vec!["birei-markdown", size.textarea_class_name()];
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

    let toolbar_buttons = {
        let mut items = if show_default_toolbar {
            default_toolbar_items()
        } else {
            Vec::new()
        };
        items.extend(toolbar_items.get_untracked().unwrap_or_default());
        items
    };

    let render_editor_value = move |markdown: &str| {
        let Some(editor) = editor_ref.get_untracked() else {
            return;
        };

        let html = markdown_to_html(markdown);
        editor.set_inner_html(&html);
        decorate_rendered_content(&editor, !(disabled || readonly));
    };

    let commit_editor_value = Rc::new(move || {
        let Some(editor) = editor_ref.get_untracked() else {
            return;
        };

        let markdown = markdown_from_html(&editor.inner_html());
        let previous = last_committed_markdown.get_untracked();

        render_editor_value(&markdown);
        last_committed_markdown.set(markdown.clone());

        if markdown != previous {
            if let Some(on_change) = on_change.as_ref() {
                on_change.run(markdown);
            }
        }
    });

    Effect::new(move |_| {
        let next_markdown = value.get().unwrap_or_default();
        if has_focus.get() || next_markdown == last_committed_markdown.get_untracked() {
            return;
        }

        render_editor_value(&next_markdown);
        last_committed_markdown.set(next_markdown);
    });

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

    let restore_selection_for_toolbar = Rc::clone(&restore_selection);
    let save_selection_for_toolbar = Rc::clone(&save_selection);
    let commit_after_popup_close = Rc::clone(&commit_editor_value);
    let open_link_popup: Rc<dyn Fn()> = Rc::new({
        let saved_range = Rc::clone(&saved_range);
        move || {
            let Some(range) = saved_range.borrow().clone() else {
                return;
            };
            link_popup_layout.set(measure_floating_popup_layout(
                &range.get_bounding_client_rect(),
            ));
            link_url.set(String::new());
            link_popup_open.set(true);
        }
    });
    let open_heading_popup: Rc<dyn Fn()> = Rc::new(move || {
        if let Some(button) = heading_button_ref.get_untracked() {
            heading_popup_layout.set(measure_floating_popup_layout(
                &button.get_bounding_client_rect(),
            ));
            heading_popup_open.set(true);
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
    let open_table_popup: Rc<dyn Fn()> = Rc::new(move || {
        if let Some(button) = table_button_ref.get_untracked() {
            table_popup_layout.set(measure_floating_popup_layout(
                &button.get_bounding_client_rect(),
            ));
        } else {
            let Some(range) = saved_range_for_table_popup.borrow().clone() else {
                return;
            };
            let Some(selection) = table_selection_from_range(&range) else {
                return;
            };
            table_popup_layout.set(measure_floating_popup_layout(
                &selection.cell.get_bounding_client_rect(),
            ));
        }
        table_popup_open.set(true);
    });
    let close_table_popup: Rc<dyn Fn()> = Rc::new(move || {
        table_popup_open.set(false);
        if !has_focus.get_untracked() {
            commit_after_table_popup_close();
        }
    });
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
    let handle_toolbar_action: Rc<dyn Fn(String)> = Rc::new(move |action: String| {
        if disabled || readonly {
            return;
        }

        upload_error.set(None);
        match action.as_str() {
            "bold" => {
                restore_selection_for_toolbar();
                exec_document_command("bold", None);
            }
            "italic" => {
                restore_selection_for_toolbar();
                exec_document_command("italic", None);
            }
            "heading" => {
                save_selection_for_toolbar();
                open_heading_popup();
            }
            "heading-1" | "heading-2" | "heading-3" => {
                restore_selection_for_toolbar();
                exec_document_command(
                    "formatBlock",
                    Some(match action.as_str() {
                        "heading-1" => "h1",
                        "heading-2" => "h2",
                        _ => "h3",
                    }),
                );
                close_heading_popup_for_toolbar();
            }
            "unordered-list" => {
                restore_selection_for_toolbar();
                exec_document_command("insertUnorderedList", None);
            }
            "ordered-list" => {
                restore_selection_for_toolbar();
                exec_document_command("insertOrderedList", None);
            }
            "link" => {
                save_selection_for_toolbar();
                open_link_popup();
            }
            "table" => {
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
                restore_selection_for_toolbar();
                insert_at_saved_range(r#"<hr />"#);
            }
            "image" => {
                save_selection_for_toolbar();
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
                let upload_error = upload_error;
                spawn_local(async move {
                    match handler.run(file).await {
                        Ok(url) => {
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
                        Err(error) => {
                            upload_error.set(Some(error));
                        }
                    }

                    if !has_focus.get_untracked() && !link_popup_open.get_untracked() {
                        commit_after_image();
                    }
                });
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

                if !has_focus.get_untracked() && !link_popup_open.get_untracked() {
                    commit_editor_value();
                }
            }
        }
    };

    setup_link_popup_effects(
        link_popup_open,
        link_input_ref,
        link_popup_layout,
        Rc::clone(&saved_range),
        Rc::clone(&close_link_popup),
    );
    setup_heading_popup_effects(
        heading_popup_open,
        heading_button_ref,
        heading_popup_ref,
        heading_popup_layout,
        Rc::clone(&close_heading_popup),
    );
    setup_table_popup_effects(
        table_popup_open,
        table_button_ref,
        table_popup_ref,
        table_popup_layout,
        Rc::clone(&close_table_popup),
    );

    let toolbar_button_class = format!(
        "birei-button {} {}",
        toolbar_variant.class_name(),
        Size::Small.button_class_name()
    );
    let refresh_table_button_state = move || {
        table_button_is_menu.set(current_table_selection().is_some());
    };
    let save_selection_on_mouseup = Rc::clone(&save_selection);
    let save_selection_on_keyup = Rc::clone(&save_selection);
    let save_selection_on_input = Rc::clone(&save_selection);
    let save_selection_after_table_move = Rc::clone(&save_selection);
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
        heading_button_ref,
        table_button_ref,
        heading_popup_open,
        table_button_is_menu,
        disabled,
        readonly,
        handle_toolbar_action: Rc::clone(&handle_toolbar_action),
    });

    view! {
        <div class=class_name>
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
                id=id
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
