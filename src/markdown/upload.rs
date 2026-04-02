use std::future::Future;
use std::pin::Pin;
use std::sync::Arc;

type UploadFuture = Pin<Box<dyn Future<Output = Result<String, String>> + 'static>>;

#[derive(Clone)]
pub struct MarkdownImageUploadHandler(Arc<dyn Fn(web_sys::File) -> UploadFuture + Send + Sync>);

impl MarkdownImageUploadHandler {
    pub fn new<F, Fut>(handler: F) -> Self
    where
        F: Fn(web_sys::File) -> Fut + Send + Sync + 'static,
        Fut: Future<Output = Result<String, String>> + 'static,
    {
        Self(Arc::new(move |file| Box::pin(handler(file))))
    }

    pub async fn run(&self, file: web_sys::File) -> Result<String, String> {
        (self.0)(file).await
    }
}
