use std::rc::Rc;

use leptos::prelude::*;

use super::types::{CodeCompletionItem, CodeSelection, CompletionResponse, TextEdit};

/// Applies a completion response into the editor state while respecting
/// whether the caller is allowed to open a new popup or only refresh one
/// that is already visible.
pub(crate) fn apply_completion_response(
    response: CompletionResponse,
    completions: RwSignal<Vec<CodeCompletionItem>>,
    completion_replace: RwSignal<Option<std::ops::Range<usize>>>,
    completion_open: RwSignal<bool>,
    completion_index: RwSignal<usize>,
    completion_scroll_request: RwSignal<u64>,
    allow_open: bool,
) {
    let is_empty = response.items.is_empty();
    completions.set(response.items);
    completion_replace.set(response.replace);
    completion_index.set(0);
    let should_open = !is_empty && (allow_open || completion_open.get_untracked());
    completion_open.set(should_open);
    if should_open {
        completion_scroll_request.update(|value| *value += 1);
    }
}

/// Replaces the currently targeted completion range with the active item.
/// Returns `false` when the popup has nothing actionable to accept.
pub(crate) fn accept_selected_completion(
    apply_edit: Rc<dyn Fn(TextEdit)>,
    completions: RwSignal<Vec<CodeCompletionItem>>,
    completion_replace: RwSignal<Option<std::ops::Range<usize>>>,
    completion_open: RwSignal<bool>,
    completion_index: RwSignal<usize>,
) -> bool {
    if !completion_open.get_untracked() {
        return false;
    }

    let items = completions.get_untracked();
    let Some(item) = items.get(completion_index.get_untracked()).cloned() else {
        return false;
    };
    let Some(range) = completion_replace.get_untracked() else {
        return false;
    };

    let cursor = range.start + item.insert_text.len();
    apply_edit(TextEdit {
        range,
        replacement: item.insert_text,
        cursor: Some(cursor),
    });
    completion_open.set(false);
    true
}

/// Programmatic edits collapse the selection to a single caret position.
pub(crate) fn selection_after_edit(cursor: usize) -> CodeSelection {
    CodeSelection {
        start: cursor,
        end: cursor,
    }
}
