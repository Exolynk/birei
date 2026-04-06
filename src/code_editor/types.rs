use std::ops::Range;

/// Caret position in both raw byte offset and user-facing coordinates.
#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub struct CodeCursor {
    pub offset: usize,
    pub line: usize,
    pub column: usize,
}

/// Inclusive-exclusive selection range inside the raw text buffer.
#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub struct CodeSelection {
    pub start: usize,
    pub end: usize,
}

/// Styled text range returned by a highlighter implementation.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct HighlightSpan {
    pub range: Range<usize>,
    pub class_name: &'static str,
}

/// Highlight result consumed by the mirrored overlay renderer.
#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub struct HighlightResponse {
    pub spans: Vec<HighlightSpan>,
}

/// Minimal highlight request: the full source text at the current revision.
#[derive(Clone, Debug)]
pub struct HighlightRequest<'a> {
    pub text: &'a str,
}

/// High-level completion categories for UI styling and future behavior hooks.
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum CodeCompletionKind {
    Keyword,
    Snippet,
    Tag,
    Attribute,
}

/// One completion option shown in the popup.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct CodeCompletionItem {
    pub label: String,
    pub detail: Option<String>,
    pub insert_text: String,
    pub kind: CodeCompletionKind,
}

/// Completion result plus the replacement range that each item targets.
#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub struct CompletionResponse {
    pub items: Vec<CodeCompletionItem>,
    pub replace: Option<Range<usize>>,
}

/// Completion request with the current text and selection context.
#[derive(Clone, Debug)]
pub struct CompletionRequest<'a> {
    pub text: &'a str,
    pub cursor: CodeCursor,
    pub selection: CodeSelection,
}

/// Distinguishes which editor action requested indentation logic.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum IndentAction {
    Indent,
    Outdent,
    NewLine,
}

/// Indentation request sent to the active language service.
#[derive(Clone, Debug)]
pub struct IndentRequest<'a> {
    pub text: &'a str,
    pub cursor: CodeCursor,
    pub selection: CodeSelection,
    pub action: IndentAction,
}

/// Text replacement used by completions, indentation, and history restores.
#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub struct TextEdit {
    pub range: Range<usize>,
    pub replacement: String,
    pub cursor: Option<usize>,
}

/// Optional edit returned from indentation logic.
#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub struct IndentResponse {
    pub edit: Option<TextEdit>,
}

/// Severity used by optional diagnostics output below the editor.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CodeDiagnosticSeverity {
    Error,
    Warning,
    Info,
}

/// Structured diagnostic tied to a text range.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct CodeDiagnostic {
    pub message: String,
    pub severity: CodeDiagnosticSeverity,
    pub range: Range<usize>,
}

/// Diagnostics request for the full current document.
#[derive(Clone, Debug)]
pub struct DiagnosticsRequest<'a> {
    pub text: &'a str,
}

/// Diagnostics payload rendered by the editor status area.
#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub struct DiagnosticsResponse {
    pub items: Vec<CodeDiagnostic>,
}

/// Shared boxed future type so services can stay async without `async_trait`.
pub type LocalBoxFuture<'a, T> = std::pin::Pin<Box<dyn std::future::Future<Output = T> + 'a>>;
