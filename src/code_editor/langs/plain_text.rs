use super::super::service::CodeLanguageService;
use super::super::types::{
    CompletionRequest, CompletionResponse, HighlightRequest, HighlightResponse, IndentRequest,
    IndentResponse, LocalBoxFuture,
};

/// Shared no-op language service for code views that should behave like
/// plain source text without syntax intelligence.
#[derive(Clone, Copy, Debug, Default)]
pub struct PlainTextCodeLanguageService;

impl CodeLanguageService for PlainTextCodeLanguageService {
    fn language_id(&self) -> &'static str {
        "plain-text"
    }

    fn highlight<'a>(
        &'a self,
        _req: HighlightRequest<'a>,
    ) -> LocalBoxFuture<'a, HighlightResponse> {
        Box::pin(async { HighlightResponse::default() })
    }

    fn complete<'a>(
        &'a self,
        _req: CompletionRequest<'a>,
    ) -> LocalBoxFuture<'a, CompletionResponse> {
        Box::pin(async { CompletionResponse::default() })
    }

    fn indent<'a>(&'a self, _req: IndentRequest<'a>) -> LocalBoxFuture<'a, IndentResponse> {
        Box::pin(async { IndentResponse::default() })
    }
}
