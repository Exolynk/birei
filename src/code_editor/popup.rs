use leptos::{html, prelude::*};

use super::types::{CodeCompletionItem, CodeCompletionKind};
use crate::common::FloatingPopupLayout;
use crate::ArcOneCallback;

/// Completion list of the code editor with the documentation of the active entry.
#[component]
pub(super) fn CompletionPopup(
    /// Completion entries of the current request.
    completions: RwSignal<Vec<CodeCompletionItem>>,
    /// Highlighted entry.
    active_index: RwSignal<usize>,
    /// Floating position computed from the caret.
    layout: RwSignal<FloatingPopupLayout>,
    /// Reference used to scroll the active entry into view.
    list_ref: NodeRef<html::Div>,
    /// Called with the index of an entry chosen by pointer.
    #[prop(into)]
    on_accept: ArcOneCallback<usize>,
) -> impl IntoView {
    let completion_documentation = Memo::new(move |_| {
        completions
            .try_get()
            .unwrap_or_default()
            .get(active_index.try_get().unwrap_or_default())
            .and_then(|item| {
                item.documentation.as_ref().map(|documentation| {
                    (
                        item.label.clone(),
                        item.detail.clone().unwrap_or_default(),
                        documentation.clone(),
                    )
                })
            })
    });
    let placement = move || {
        let has_documentation = completion_documentation
            .try_with(Option::is_some)
            .unwrap_or_default();
        completion_documentation_placement(&layout.try_get().unwrap_or_default(), has_documentation)
    };

    view! {
        <div
            class=move || {
                let upward = if layout.try_get().unwrap_or_default().open_upward {
                    " birei-code-editor__completion-popups--upward"
                } else {
                    ""
                };
                format!("birei-code-editor__completion-popups {}{upward}", placement())
            }
            style=move || {
                let layout = layout.try_get().unwrap_or_default();
                format!(
                    "left: {}px; top: {}px;",
                    completion_popup_left(&layout, placement()),
                    layout.top
                )
            }
        >
            <div
                node_ref=list_ref
                class="birei-code-editor__completions birei-code-editor__completions--portal"
                role="listbox"
                style=move || {
                    let layout = layout.try_get().unwrap_or_default();
                    format!("width: {}px; max-height: {}px;", layout.width, layout.max_height)
                }
            >
                {move || {
                    completions
                        .try_get()
                        .unwrap_or_default()
                        .into_iter()
                        .enumerate()
                        .map(|(index, item)| {
                            let detail = completion_kind_label(item.kind);
                            view! {
                                <button
                                    class="birei-code-editor__completion"
                                    class:birei-code-editor__completion--active=move || {
                                        active_index.try_get().unwrap_or_default() == index
                                    }
                                    on:mousedown=move |event| {
                                        event.prevent_default();
                                        on_accept.run(index);
                                    }
                                >
                                    <span class="birei-code-editor__completion-label">{item.label}</span>
                                    <span class="birei-code-editor__completion-detail">{detail}</span>
                                </button>
                            }
                        })
                        .collect_view()
                }}
            </div>
            {move || completion_documentation.try_get().flatten().map(|(label, detail, documentation)| view! {
                <aside
                    class="birei-code-editor__completion-documentation"
                    style=move || {
                        format!("max-height: {}px;", layout.try_get().unwrap_or_default().max_height)
                    }
                >
                    <strong>{label}</strong>
                    <span class="birei-code-editor__completion-documentation-detail">{detail}</span>
                    <p>{documentation}</p>
                </aside>
            })}
        </div>
    }
}

/// Chooses a non-overlapping documentation-panel placement for one completion popup.
fn completion_documentation_placement(
    layout: &FloatingPopupLayout,
    has_documentation: bool,
) -> &'static str {
    if !has_documentation {
        return "";
    }
    const DOCUMENTATION_WIDTH: f64 = 352.0;
    const GAP: f64 = 8.0;
    const EDGE: f64 = 8.0;
    let viewport_width = web_sys::window()
        .and_then(|window| window.inner_width().ok())
        .and_then(|value| value.as_f64())
        .unwrap_or(layout.left + layout.width + DOCUMENTATION_WIDTH + GAP + EDGE);
    if layout.left + layout.width + GAP + DOCUMENTATION_WIDTH <= viewport_width - EDGE {
        " birei-code-editor__completion-popups--documentation-right"
    } else if layout.left >= DOCUMENTATION_WIDTH + GAP + EDGE {
        " birei-code-editor__completion-popups--documentation-left"
    } else {
        " birei-code-editor__completion-popups--documentation-stacked"
    }
}

/// Returns the left edge for a completion popup and its optional left-side documentation panel.
fn completion_popup_left(layout: &FloatingPopupLayout, placement: &str) -> f64 {
    const DOCUMENTATION_WIDTH: f64 = 352.0;
    const GAP: f64 = 8.0;
    if placement.contains("documentation-left") {
        (layout.left - DOCUMENTATION_WIDTH - GAP).max(8.0)
    } else {
        layout.left
    }
}

/// Returns the compact semantic label displayed beside a completion name.
fn completion_kind_label(kind: CodeCompletionKind) -> &'static str {
    match kind {
        CodeCompletionKind::Keyword => "keyword",
        CodeCompletionKind::Snippet => "snippet",
        CodeCompletionKind::Tag => "tag",
        CodeCompletionKind::Attribute => "attribute",
        CodeCompletionKind::Function => "fn",
        CodeCompletionKind::Variable => "var",
        CodeCompletionKind::Module => "module",
        CodeCompletionKind::Type => "type",
        CodeCompletionKind::Enum => "enum",
        CodeCompletionKind::Trait => "trait",
        CodeCompletionKind::Variant => "variant",
    }
}
