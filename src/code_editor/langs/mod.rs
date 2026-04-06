// Bundled language-service implementations live in their own namespace so the
// editor core stays clearly separated from language-specific behavior.
mod html;
mod plain_text;

pub use html::HtmlCodeLanguageService;
pub use plain_text::PlainTextCodeLanguageService;
