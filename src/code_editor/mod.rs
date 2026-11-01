// Internal editor plumbing is split into focused modules so the main
// component stays navigable as the interaction model grows.
mod completion;
mod diagnostics;
mod editor;
mod history;
mod hover;
mod keyboard;
pub mod langs;
mod popup;
mod scroll;
mod service;
mod text;
mod types;

pub use editor::CodeEditor;
pub use langs::{HtmlCodeLanguageService, PlainTextCodeLanguageService};
pub use service::CodeLanguageService;
pub use types::{
    CodeCompletionItem, CodeCompletionKind, CodeCursor, CodeDiagnostic, CodeDiagnosticSeverity,
    CodeSelection, CompletionRequest, CompletionResponse, DiagnosticsRequest, DiagnosticsResponse,
    HighlightRequest, HighlightResponse, HighlightSpan, HoverRequest, HoverResponse, IndentAction,
    IndentRequest, IndentResponse, TextEdit,
};
