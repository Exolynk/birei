// Markdown editing is split into DOM, popup-effects, table, upload, and view
// helpers so the main editor component can stay readable.
mod dom;
mod download;
mod effects;
mod images;
mod legacy;
mod md;
mod menu;
mod plain;
mod read_view;
mod table;
mod upload;
mod view;

pub use download::MarkdownImageDownloadHandler;
pub use md::MarkdownEditor;
pub use plain::markdown_plain_text;
pub use upload::MarkdownImageUploadHandler;
mod renderer;
mod safe;
pub use renderer::MarkdownRenderer;
