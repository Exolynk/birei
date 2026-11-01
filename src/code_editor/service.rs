use super::types::{
    CompletionRequest, CompletionResponse, DiagnosticsRequest, DiagnosticsResponse,
    HighlightRequest, HighlightResponse, HoverRequest, HoverResponse, IndentRequest,
    IndentResponse, LocalBoxFuture,
};

/// Async language capability boundary consumed by `CodeEditor`.
/// Implementors can provide lightweight local logic or delegate to heavier
/// parsers/engines without changing the editor core.
pub trait CodeLanguageService: Send + Sync + 'static {
    fn language_id(&self) -> &'static str;

    fn highlight<'a>(&'a self, req: HighlightRequest<'a>) -> LocalBoxFuture<'a, HighlightResponse>;

    fn complete<'a>(&'a self, req: CompletionRequest<'a>)
        -> LocalBoxFuture<'a, CompletionResponse>;

    fn indent<'a>(&'a self, req: IndentRequest<'a>) -> LocalBoxFuture<'a, IndentResponse>;

    fn diagnostics<'a>(
        &'a self,
        _req: DiagnosticsRequest<'a>,
    ) -> LocalBoxFuture<'a, DiagnosticsResponse> {
        Box::pin(async { DiagnosticsResponse::default() })
    }

    /// Describes the code element at the requested position, or `None` when there is
    /// nothing to describe. The editor shows it after the pointer or caret rests there.
    fn hover<'a>(&'a self, _req: HoverRequest<'a>) -> LocalBoxFuture<'a, Option<HoverResponse>> {
        Box::pin(async { None })
    }
}
