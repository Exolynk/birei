use std::cell::Cell;
use std::rc::Rc;
use std::sync::Arc;

use leptos::ev;
use leptos::html;
use leptos::portal::Portal;
use leptos::prelude::*;
use leptos::task::spawn_local;
use wasm_bindgen::JsCast;
use web_sys::{HtmlElement, HtmlTextAreaElement, KeyboardEvent};

use crate::common::{measure_floating_popup_layout, FloatingPopupLayout};
use crate::Size;

use super::completion::{
    accept_selected_completion, apply_completion_response, selection_after_edit,
};
use super::history::HistoryEntry;
use super::keyboard::{is_redo_shortcut, is_undo_shortcut, should_skip_completion_refresh};
use super::scroll::sync_completion_scroll;
use super::service::CodeLanguageService;
use super::text::{
    byte_index_to_utf16_offset, current_selection, cursor_from_text, escape_html_with_breaks,
    indent_selection, outdent_at_cursor, render_highlight_html,
};
use super::types::{
    CodeCompletionItem, CodeCursor, CodeSelection, CompletionRequest, DiagnosticsRequest,
    DiagnosticsResponse, HighlightRequest, IndentAction, IndentRequest, TextEdit,
};

/// Plain-text code editor with async language services for highlighting and completion.
#[component]
pub fn CodeEditor(
    #[prop(optional, into)] value: MaybeProp<String>,
    #[prop(optional, into)] placeholder: MaybeProp<String>,
    #[prop(optional, into)] id: Option<String>,
    #[prop(optional, into)] name: Option<String>,
    #[prop(optional)] size: Size,
    #[prop(optional)] disabled: bool,
    #[prop(optional)] readonly: bool,
    #[prop(optional)] invalid: bool,
    #[prop(optional)] line_numbers: bool,
    #[prop(optional, default = 4)] rows: u32,
    #[prop(optional, default = 2)] tab_size: usize,
    service: Arc<dyn CodeLanguageService>,
    #[prop(optional)] on_change: Option<Callback<String>>,
) -> impl IntoView {
    const HISTORY_LIMIT: usize = 200;

    // DOM references are used for direct browser integrations: selection,
    // scroll sync, popup positioning, and portal menu scrolling.
    let textarea_ref = NodeRef::<html::Textarea>::new();
    let root_ref = NodeRef::<html::Div>::new();
    let highlight_content_ref = NodeRef::<html::Div>::new();
    let measure_content_ref = NodeRef::<html::Div>::new();
    let completion_list_ref = NodeRef::<html::Div>::new();
    let gutter_ref = NodeRef::<html::Div>::new();
    let has_focus = RwSignal::new(false);
    let text = RwSignal::new(value.get_untracked().unwrap_or_default());
    let highlight_html = RwSignal::new(render_highlight_html(&text.get_untracked(), &[]));
    let completions = RwSignal::new(Vec::<CodeCompletionItem>::new());
    let completion_replace = RwSignal::new(None::<std::ops::Range<usize>>);
    let completion_open = RwSignal::new(false);
    let completion_index = RwSignal::new(0usize);
    let completion_layout = RwSignal::new(FloatingPopupLayout::default());
    let completion_theme_style = RwSignal::new(String::new());
    let completion_scroll_request = RwSignal::new(0u64);
    let accept_completion_nonce = RwSignal::new(0u64);
    let diagnostics = RwSignal::new(DiagnosticsResponse::default());
    let selection_state = RwSignal::new(CodeSelection::default());
    let scroll_top_state = RwSignal::new(0i32);
    let scroll_left_state = RwSignal::new(0i32);
    let line_style = RwSignal::new(String::from("--birei-code-editor-line-origin: 50%;"));
    let overlay_transform_style = RwSignal::new(String::new());
    let undo_history = RwSignal::new(Vec::<HistoryEntry>::new());
    let redo_history = RwSignal::new(Vec::<HistoryEntry>::new());
    let highlight_request_id = Rc::new(Cell::new(0u64));
    let completion_request_id = Rc::new(Cell::new(0u64));
    let diagnostics_request_id = Rc::new(Cell::new(0u64));

    // Root class list mirrors the existing input/textarea sizing and state
    // tokens so the editor fits the rest of the component library.
    let class_name = move || {
        let mut classes = vec!["birei-code-editor", size.textarea_class_name()];
        if disabled {
            classes.push("birei-code-editor--disabled");
        }
        if readonly {
            classes.push("birei-code-editor--readonly");
        }
        if invalid {
            classes.push("birei-code-editor--invalid");
        }
        classes.join(" ")
    };
    // Readonly and disabled editors still render their content, but all edit
    // affordances and completion triggers are suppressed.
    let is_interactive = move || !disabled && !readonly;
    // The root style carries both the focus-line origin and a CSS row count
    // used to derive compact readonly example heights.
    let root_style = move || {
        format!(
            "{} --birei-code-editor-row-count: {};",
            line_style.get(),
            rows.max(1)
        )
    };

    // Capture the pointer origin so the animated underline grows from the
    // click position, matching the rest of the form controls.
    let handle_pointer_down = move |event: ev::PointerEvent| {
        if let Some(target) = event
            .current_target()
            .and_then(|target| target.dyn_into::<HtmlElement>().ok())
        {
            let rect = target.get_bounding_client_rect();
            let x = f64::from(event.client_x()) - rect.left();
            line_style.set(format!("--birei-code-editor-line-origin: {x}px;"));
        }
    };

    // External controlled values should replace the local buffer only while
    // the user is not actively editing the textarea.
    Effect::new(move |_| {
        let next = value.get().unwrap_or_default();
        if has_focus.get() || next == text.get_untracked() {
            return;
        }
        text.set(next);
    });

    // Any transition into readonly/disabled state must immediately hide the
    // completion popup to avoid stale interactive UI.
    Effect::new(move |_| {
        if !is_interactive() {
            completion_open.set(false);
        }
    });

    // Highlight requests are async and versioned so slower responses from an
    // older text revision cannot repaint newer editor content.
    let run_highlight: Rc<dyn Fn(String)> = Rc::new({
        let service = Arc::clone(&service);
        let highlight_request_id = Rc::clone(&highlight_request_id);
        move |next_text: String| {
            let request_id = highlight_request_id.get() + 1;
            highlight_request_id.set(request_id);
            let service = Arc::clone(&service);
            let highlight_request_id_for_task = Rc::clone(&highlight_request_id);
            spawn_local({
                let highlight_html = highlight_html;
                async move {
                    let response = service
                        .highlight(HighlightRequest { text: &next_text })
                        .await;
                    if highlight_request_id_for_task.get() != request_id {
                        return;
                    }
                    highlight_html.set(render_highlight_html(&next_text, &response.spans));
                }
            });
        }
    });

    // Diagnostics follow the same request-versioning pattern as highlighting.
    let run_diagnostics: Rc<dyn Fn(String)> = Rc::new({
        let service = Arc::clone(&service);
        let diagnostics_request_id = Rc::clone(&diagnostics_request_id);
        move |next_text: String| {
            let request_id = diagnostics_request_id.get() + 1;
            diagnostics_request_id.set(request_id);
            let service = Arc::clone(&service);
            let diagnostics_request_id_for_task = Rc::clone(&diagnostics_request_id);
            spawn_local({
                let diagnostics = diagnostics;
                async move {
                    let response = service
                        .diagnostics(DiagnosticsRequest { text: &next_text })
                        .await;
                    if diagnostics_request_id_for_task.get() != request_id {
                        return;
                    }
                    diagnostics.set(response);
                }
            });
        }
    });

    // Completion requests also carry an `allow_open` flag so cursor movement
    // can refresh or close an existing popup without creating a new one.
    let run_completion: Rc<dyn Fn(String, CodeCursor, CodeSelection, bool)> = Rc::new({
        let service = Arc::clone(&service);
        let completion_request_id = Rc::clone(&completion_request_id);
        move |next_text: String, cursor: CodeCursor, selection: CodeSelection, allow_open: bool| {
            let request_id = completion_request_id.get() + 1;
            completion_request_id.set(request_id);
            let service = Arc::clone(&service);
            let completion_request_id_for_task = Rc::clone(&completion_request_id);
            spawn_local({
                let completions = completions;
                let completion_replace = completion_replace;
                let completion_open = completion_open;
                let completion_index = completion_index;
                async move {
                    let response = service
                        .complete(CompletionRequest {
                            text: &next_text,
                            cursor,
                            selection,
                        })
                        .await;
                    if completion_request_id_for_task.get() != request_id {
                        return;
                    }
                    apply_completion_response(
                        response,
                        completions,
                        completion_replace,
                        completion_open,
                        completion_index,
                        completion_scroll_request,
                        allow_open,
                    );
                }
            });
        }
    });

    // Every text revision refreshes highlight and diagnostics in parallel.
    let run_highlight_effect = Rc::clone(&run_highlight);
    let run_diagnostics_effect = Rc::clone(&run_diagnostics);
    Effect::new(move |_| {
        let next_text = text.get();
        run_highlight_effect.as_ref()(next_text.clone());
        run_diagnostics_effect.as_ref()(next_text);
    });

    // The mirrored overlay does not scroll independently. Instead, its inner
    // content is translated to stay pixel-aligned with the real textarea.
    let sync_scroll = move |textarea: &HtmlTextAreaElement| {
        let scroll_top = textarea.scroll_top();
        let scroll_left = textarea.scroll_left();
        overlay_transform_style.set(format!(
            "transform: translate({}px, {}px);",
            -scroll_left, -scroll_top
        ));
        if let Some(gutter) = gutter_ref.get_untracked() {
            gutter.set_scroll_top(scroll_top);
        }
    };

    // Popup placement is measured from a hidden mirror that renders the text
    // up to the current caret with matching typography and whitespace rules.
    let position_completion_popup = move |textarea: &HtmlTextAreaElement, cursor_offset: usize| {
        let Some(measure_content) = measure_content_ref.get_untracked() else {
            return;
        };
        let textarea_value = textarea.value();
        let safe_offset = cursor_offset.min(textarea_value.len());

        measure_content.set_inner_html(&format!(
            r#"{}<span class="birei-code-editor__measure-caret"></span>"#,
            escape_html_with_breaks(&textarea_value[..safe_offset])
        ));
        let Ok(Some(marker)) = measure_content.query_selector(".birei-code-editor__measure-caret")
        else {
            return;
        };

        let marker_rect = marker.get_bounding_client_rect();
        let mut layout = measure_floating_popup_layout(&marker_rect);
        let popup_width = 352.0;
        let viewport_width = web_sys::window()
            .and_then(|window| window.inner_width().ok())
            .and_then(|value| value.as_f64())
            .unwrap_or(marker_rect.right() + popup_width);
        layout.left = layout
            .left
            .clamp(8.0, (viewport_width - popup_width - 8.0).max(8.0));
        layout.width = popup_width;
        completion_layout.set(layout);

        if let Some(root) = root_ref.get_untracked() {
            if let Some(window) = web_sys::window() {
                if let Ok(Some(computed_style)) = window.get_computed_style(&root) {
                    let background = computed_style
                        .get_property_value("--birei-code-editor-bg")
                        .unwrap_or_default();
                    let border = computed_style
                        .get_property_value("--birei-code-editor-border")
                        .unwrap_or_default();
                    let color = computed_style
                        .get_property_value("--birei-code-editor-color")
                        .unwrap_or_default();
                    let muted = computed_style
                        .get_property_value("--birei-code-editor-muted")
                        .unwrap_or_default();
                    completion_theme_style.set(format!(
                        "--birei-code-editor-popup-bg: color-mix(in srgb, {background} 98%, white 2%); --birei-code-editor-popup-border: color-mix(in srgb, {border} 88%, rgba(37, 84, 89, 0.18)); --birei-code-editor-color: {color}; --birei-code-editor-muted: {muted};"
                    ));
                }
            }
        }
    };

    // All outward-facing text changes flow through one closure so controlled
    // consumers and local state stay in lockstep.
    let emit_change = move |next_text: String| {
        text.set(next_text.clone());
        if let Some(on_change) = on_change.as_ref() {
            on_change.run(next_text);
        }
    };

    // Selection and scroll are cached in signals so history entries and async
    // completions can reason about the current editor state.
    let sync_editor_state = move |textarea: &HtmlTextAreaElement| {
        selection_state.set(current_selection(textarea));
        scroll_top_state.set(textarea.scroll_top());
        scroll_left_state.set(textarea.scroll_left());
    };

    // History snapshots store both text and viewport state so undo/redo feels
    // like moving back through actual editing moments.
    let capture_history_entry: Rc<dyn Fn() -> HistoryEntry> = Rc::new(move || HistoryEntry {
        text: text.get_untracked(),
        selection: selection_state.get_untracked(),
        scroll_top: scroll_top_state.get_untracked(),
        scroll_left: scroll_left_state.get_untracked(),
    });

    // Push the current state into undo history while deduplicating identical
    // consecutive snapshots and trimming the history size.
    let push_undo_snapshot: Rc<dyn Fn()> = Rc::new({
        let capture_history_entry = Rc::clone(&capture_history_entry);
        move || {
            let entry = capture_history_entry.as_ref()();
            undo_history.update(|history| {
                if history.last() == Some(&entry) {
                    return;
                }
                history.push(entry);
                if history.len() > HISTORY_LIMIT {
                    history.remove(0);
                }
            });
            redo_history.set(Vec::new());
        }
    });

    // Central text-edit application used by completions, tab handling,
    // indentation, and future programmatic commands.
    let run_highlight_apply = Rc::clone(&run_highlight);
    let run_diagnostics_apply = Rc::clone(&run_diagnostics);
    let run_completion_apply = Rc::clone(&run_completion);
    let push_undo_snapshot_apply = Rc::clone(&push_undo_snapshot);
    let apply_edit: Rc<dyn Fn(TextEdit)> = Rc::new(move |edit: TextEdit| {
        let Some(textarea) = textarea_ref.get_untracked() else {
            return;
        };
        let mut next_text = textarea.value();
        next_text.replace_range(edit.range.clone(), &edit.replacement);
        push_undo_snapshot_apply.as_ref()();
        textarea.set_value(&next_text);
        let next_cursor = edit
            .cursor
            .unwrap_or(edit.range.start.saturating_add(edit.replacement.len()))
            .min(next_text.len());
        let next_cursor_utf16 = byte_index_to_utf16_offset(&next_text, next_cursor);
        let _ = textarea.set_selection_start(Some(next_cursor_utf16));
        let _ = textarea.set_selection_end(Some(next_cursor_utf16));
        sync_editor_state(&textarea);
        emit_change(next_text.clone());
        run_highlight_apply.as_ref()(next_text.clone());
        run_diagnostics_apply.as_ref()(next_text.clone());
        run_completion_apply.as_ref()(
            next_text.clone(),
            cursor_from_text(&next_text, next_cursor),
            selection_after_edit(next_cursor),
            true,
        );
        sync_scroll(&textarea);
    });

    // Undo/redo restore a full history snapshot, including selection and
    // scroll state, instead of trying to replay incremental operations.
    let run_highlight_restore = Rc::clone(&run_highlight);
    let run_diagnostics_restore = Rc::clone(&run_diagnostics);
    let restore_history_entry: Rc<dyn Fn(HistoryEntry)> = Rc::new(move |entry: HistoryEntry| {
        let Some(textarea) = textarea_ref.get_untracked() else {
            return;
        };

        textarea.set_value(&entry.text);
        let selection_start = byte_index_to_utf16_offset(&entry.text, entry.selection.start);
        let selection_end = byte_index_to_utf16_offset(&entry.text, entry.selection.end);
        let _ = textarea.set_selection_start(Some(selection_start));
        let _ = textarea.set_selection_end(Some(selection_end));
        textarea.set_scroll_top(entry.scroll_top);
        textarea.set_scroll_left(entry.scroll_left);

        selection_state.set(entry.selection);
        scroll_top_state.set(entry.scroll_top);
        scroll_left_state.set(entry.scroll_left);
        completion_open.set(false);
        emit_change(entry.text.clone());
        run_highlight_restore.as_ref()(entry.text.clone());
        run_diagnostics_restore.as_ref()(entry.text);
        sync_scroll(&textarea);
    });

    // Mouse acceptance of a completion increments a nonce; this effect turns
    // that signal edge into a single completion acceptance action.
    let apply_edit_effect = Rc::clone(&apply_edit);
    Effect::new(move |_| {
        if accept_completion_nonce.get() == 0 {
            return;
        }
        let _ = accept_selected_completion(
            Rc::clone(&apply_edit_effect),
            completions,
            completion_replace,
            completion_open,
            completion_index,
        );
    });

    // Scroll the active popup option into view after the DOM reflects the
    // latest active-state class.
    Effect::new(move |_| {
        let _ = completion_scroll_request.get();

        if !completion_open.get() || completions.get().is_empty() {
            return;
        }

        let Some(list) = completion_list_ref.get() else {
            return;
        };
        let list: HtmlElement = list.unchecked_into();
        if let Some(window) = web_sys::window() {
            let callback = wasm_bindgen::closure::Closure::once_into_js(move || {
                let Ok(Some(option)) =
                    list.query_selector(".birei-code-editor__completion--active")
                else {
                    return;
                };
                let Ok(option) = option.dyn_into::<HtmlElement>() else {
                    return;
                };

                sync_completion_scroll(&list, &option);
            });

            let _ = window
                .set_timeout_with_callback_and_timeout_and_arguments_0(callback.unchecked_ref(), 0);
        }
    });

    // Shared completion request entry point for keyboard and mouse-driven
    // caret movement. The caller decides whether the popup may open.
    let run_completion_request = Rc::clone(&run_completion);
    let request_completion_from_textarea: Rc<dyn Fn(bool)> = Rc::new(move |allow_open: bool| {
        if !is_interactive() {
            completion_open.set(false);
            return;
        }
        let Some(textarea) = textarea_ref.get_untracked() else {
            return;
        };
        let next_text = textarea.value();
        let selection = current_selection(&textarea);
        let cursor = cursor_from_text(&next_text, selection.end);
        position_completion_popup(&textarea, selection.end);
        run_completion_request.as_ref()(next_text, cursor, selection, allow_open);
    });

    // Direct text input is the only path that may open a fresh completion
    // popup. It also seeds undo history before mutating the buffer.
    let run_highlight_input = Rc::clone(&run_highlight);
    let run_diagnostics_input = Rc::clone(&run_diagnostics);
    let run_completion_input = Rc::clone(&run_completion);
    let push_undo_snapshot_input = Rc::clone(&push_undo_snapshot);
    let handle_input = move |event: ev::Event| {
        if !is_interactive() {
            completion_open.set(false);
            return;
        }
        let textarea = event_target::<HtmlTextAreaElement>(&event);
        let next_text = textarea.value();
        if next_text != text.get_untracked() {
            push_undo_snapshot_input.as_ref()();
        }
        let selection = current_selection(&textarea);
        let cursor = cursor_from_text(&next_text, selection.end);
        sync_editor_state(&textarea);
        emit_change(next_text.clone());
        run_highlight_input.as_ref()(next_text.clone());
        run_diagnostics_input.as_ref()(next_text.clone());
        position_completion_popup(&textarea, selection.end);
        run_completion_input.as_ref()(next_text, cursor, selection, true);
        sync_scroll(&textarea);
    };

    // Scroll events only keep overlays and popup placement aligned with the
    // real textarea viewport.
    let handle_scroll = move |event: ev::Event| {
        let textarea = event_target::<HtmlTextAreaElement>(&event);
        sync_scroll(&textarea);
        sync_editor_state(&textarea);
        let selection = current_selection(&textarea);
        position_completion_popup(&textarea, selection.end);
    };

    // Keyup is used to refresh an already-open popup after caret movement
    // without opening new popups from navigation alone.
    let handle_keyup = {
        let request_completion_from_textarea = Rc::clone(&request_completion_from_textarea);
        move |event: KeyboardEvent| {
            if !is_interactive() {
                completion_open.set(false);
                return;
            }
            if completion_open.get_untracked()
                && matches!(event.key().as_str(), "ArrowDown" | "ArrowUp")
            {
                return;
            }
            if should_skip_completion_refresh(&event.key()) {
                return;
            }
            request_completion_from_textarea.as_ref()(false);
        }
    };

    // Keydown owns undo/redo, popup navigation, acceptance, indentation, and
    // newline behavior before the browser mutates the textarea.
    let apply_edit_keydown = Rc::clone(&apply_edit);
    let restore_history_entry_keydown = Rc::clone(&restore_history_entry);
    let handle_keydown = move |event: KeyboardEvent| {
        if disabled || readonly {
            return;
        }

        if is_undo_shortcut(&event) {
            event.prevent_default();
            if let Some(previous) = undo_history.get_untracked().last().cloned() {
                let current = capture_history_entry.as_ref()();
                undo_history.update(|history| {
                    history.pop();
                });
                redo_history.update(|history| {
                    history.push(current);
                    if history.len() > HISTORY_LIMIT {
                        history.remove(0);
                    }
                });
                restore_history_entry_keydown.as_ref()(previous);
            }
            return;
        }

        if is_redo_shortcut(&event) {
            event.prevent_default();
            if let Some(next) = redo_history.get_untracked().last().cloned() {
                let current = capture_history_entry.as_ref()();
                redo_history.update(|history| {
                    history.pop();
                });
                undo_history.update(|history| {
                    history.push(current);
                    if history.len() > HISTORY_LIMIT {
                        history.remove(0);
                    }
                });
                restore_history_entry_keydown.as_ref()(next);
            }
            return;
        }

        if event.key() == "ArrowDown" && completion_open.get_untracked() {
            event.prevent_default();
            let len = completions.get_untracked().len();
            if len > 0 {
                completion_index.update(|index| *index = (*index + 1).min(len - 1));
                completion_scroll_request.update(|value| *value += 1);
            }
            return;
        }

        if event.key() == "ArrowUp" && completion_open.get_untracked() {
            event.prevent_default();
            completion_index.update(|index| *index = index.saturating_sub(1));
            completion_scroll_request.update(|value| *value += 1);
            return;
        }

        if event.key() == "Escape" && completion_open.get_untracked() {
            completion_open.set(false);
            return;
        }

        if matches!(event.key().as_str(), "Enter" | "Tab")
            && accept_selected_completion(
                Rc::clone(&apply_edit_keydown),
                completions,
                completion_replace,
                completion_open,
                completion_index,
            )
        {
            event.prevent_default();
            return;
        }

        if event.key() == "Tab" {
            event.prevent_default();
            if let Some(textarea) = textarea_ref.get_untracked() {
                let selection = current_selection(&textarea);
                let next_text = textarea.value();
                let indent = " ".repeat(tab_size.max(1));
                let edit = if selection.start != selection.end {
                    indent_selection(&next_text, selection, &indent, event.shift_key())
                } else if event.shift_key() {
                    outdent_at_cursor(&next_text, selection.end, &indent)
                } else {
                    TextEdit {
                        range: selection.end..selection.end,
                        replacement: indent.clone(),
                        cursor: Some(selection.end + indent.len()),
                    }
                };
                apply_edit_keydown.as_ref()(edit);
            }
            return;
        }

        if event.key() == "Enter" {
            event.prevent_default();
            if let Some(textarea) = textarea_ref.get_untracked() {
                let next_text = textarea.value();
                let selection = current_selection(&textarea);
                let cursor = cursor_from_text(&next_text, selection.end);
                let service = Arc::clone(&service);
                let apply_edit = Rc::clone(&apply_edit_keydown);
                spawn_local(async move {
                    let response = service
                        .indent(IndentRequest {
                            text: &next_text,
                            cursor,
                            selection: selection.clone(),
                            action: IndentAction::NewLine,
                        })
                        .await;
                    if let Some(edit) = response.edit {
                        apply_edit.as_ref()(edit);
                    } else {
                        apply_edit.as_ref()(TextEdit {
                            range: selection.end..selection.end,
                            replacement: String::from("\n"),
                            cursor: Some(selection.end + 1),
                        });
                    }
                });
            }
        }
    };

    // Line numbers are derived from the current buffer, never from DOM state.
    let line_count = move || text.get().lines().count().max(1);

    view! {
        <div
            node_ref=root_ref
            class=class_name
            style=root_style
            on:pointerdown=handle_pointer_down
        >
            <div
                class="birei-code-editor__surface"
                class:birei-code-editor__surface--with-gutter=line_numbers
            >
                <Show when=move || line_numbers>
                    <div node_ref=gutter_ref class="birei-code-editor__gutter" aria-hidden="true">
                        {move || {
                            (1..=line_count())
                                .map(|line| view! { <span class="birei-code-editor__line-number">{line}</span> })
                                .collect_view()
                        }}
                    </div>
                </Show>

                <div class="birei-code-editor__viewport">
                    <div
                        class="birei-code-editor__highlight"
                        aria-hidden="true"
                    >
                        <div
                            node_ref=highlight_content_ref
                            class="birei-code-editor__highlight-content"
                            style=move || overlay_transform_style.get()
                            inner_html=move || highlight_html.get()
                        ></div>
                    </div>

                    <div class="birei-code-editor__measure" aria-hidden="true">
                        <div
                            node_ref=measure_content_ref
                            class="birei-code-editor__measure-content"
                            style=move || overlay_transform_style.get()
                        ></div>
                    </div>

                    <textarea
                        node_ref=textarea_ref
                        class="birei-code-editor__input"
                        id=id
                        name=name
                        rows=rows
                        spellcheck="false"
                        autocapitalize="off"
                        autocomplete="off"
                        placeholder=move || placeholder.get().unwrap_or_default()
                        prop:value=move || text.get()
                        disabled=disabled
                        readonly=readonly
                        aria-invalid=move || if invalid { "true" } else { "false" }
                        on:focus=move |_| {
                            has_focus.set(true);
                            if let Some(textarea) = textarea_ref.get_untracked() {
                                sync_editor_state(&textarea);
                            }
                        }
                        on:blur=move |_| {
                            has_focus.set(false);
                            completion_open.set(false);
                        }
                        on:input=handle_input
                        on:scroll=handle_scroll
                        on:mouseup=move |event: ev::MouseEvent| {
                            let textarea = event_target::<HtmlTextAreaElement>(&event);
                            sync_editor_state(&textarea);
                        }
                        on:click={
                            let request_completion_from_textarea =
                                Rc::clone(&request_completion_from_textarea);
                            move |_| {
                                if completion_open.get_untracked() {
                                    request_completion_from_textarea.as_ref()(false);
                                }
                            }
                        }
                        on:keyup=handle_keyup
                        on:keydown=handle_keydown
                    ></textarea>

                </div>
            </div>

            {move || {
                (completion_open.get() && !completions.get().is_empty()).then(|| {
                    view! {
                        <Portal>
                            <div
                                node_ref=completion_list_ref
                                class=move || {
                                    let layout = completion_layout.get();
                                    if layout.open_upward {
                                        "birei-code-editor__completions birei-code-editor__completions--portal birei-code-editor__completions--upward"
                                    } else {
                                        "birei-code-editor__completions birei-code-editor__completions--portal"
                                    }
                                }
                                role="listbox"
                                style=move || {
                                    let layout = completion_layout.get();
                                    format!(
                                        "left: {}px; top: {}px; width: {}px; max-height: {}px; {}",
                                        layout.left,
                                        layout.top,
                                        layout.width,
                                        layout.max_height,
                                        completion_theme_style.get()
                                    )
                                }
                            >
                                {move || {
                                    completions
                                        .get()
                                        .into_iter()
                                        .enumerate()
                                        .map(|(index, item)| {
                                            let label = item.label.clone();
                                            let detail = item.detail.clone().unwrap_or_default();
                                            view! {
                                                <button
                                                    class="birei-code-editor__completion"
                                                    class:birei-code-editor__completion--active=move || {
                                                        completion_index.get() == index
                                                    }
                                                    on:mousedown=move |event| {
                                                        if !is_interactive() {
                                                            return;
                                                        }
                                                        event.prevent_default();
                                                        completion_index.set(index);
                                                        accept_completion_nonce.update(|value| *value += 1);
                                                    }
                                                >
                                                    <span class="birei-code-editor__completion-label">{label}</span>
                                                    <span class="birei-code-editor__completion-detail">{detail}</span>
                                                </button>
                                            }
                                        })
                                        .collect_view()
                                }}
                            </div>
                        </Portal>
                    }
                })
            }}

            <Show when=move || !diagnostics.get().items.is_empty()>
                <div class="birei-code-editor__status">
                    {move || {
                        diagnostics
                            .get()
                            .items
                            .into_iter()
                            .map(|item| view! { <p>{item.message}</p> })
                            .collect_view()
                    }}
                </div>
            </Show>
        </div>
    }
}
