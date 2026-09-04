use std::future::Future;
use std::pin::Pin;
use std::sync::Arc;

type DownloadFuture = Pin<Box<dyn Future<Output = Result<String, String>> + 'static>>;

/// Cloneable async hook that resolves a markdown image source to a display URL.
#[derive(Clone)]
pub struct MarkdownImageDownloadHandler(Arc<dyn Fn(String) -> DownloadFuture + Send + Sync>);

impl MarkdownImageDownloadHandler {
    /// Wraps an async source resolver into the editor's shared download abstraction.
    pub fn new<F, Fut>(handler: F) -> Self
    where
        F: Fn(String) -> Fut + Send + Sync + 'static,
        Fut: Future<Output = Result<String, String>> + 'static,
    {
        Self(Arc::new(move |source| Box::pin(handler(source))))
    }

    /// Resolves an image source to the URL used by the rendered image element.
    pub async fn run(&self, source: String) -> Result<String, String> {
        (self.0)(source).await
    }
}
