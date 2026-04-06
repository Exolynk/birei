use super::types::{
    CompletionRequest, CompletionResponse, DiagnosticsRequest, DiagnosticsResponse,
    HighlightRequest, HighlightResponse, IndentRequest, IndentResponse, LocalBoxFuture,
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
}
