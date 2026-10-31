use std::collections::{HashMap, HashSet};
use std::sync::{Arc, Mutex};

use leptos::prelude::*;
use leptos::task::spawn_local;
use wasm_bindgen::JsCast;
use web_sys::{Element, HtmlElement};

use super::dom::{markdown_image_source, set_markdown_image_display_source};
use super::download::MarkdownImageDownloadHandler;

/// Resolves deferred markdown image sources to display URLs through the download hook.
///
/// Resolved URLs are cached per source and blob URLs are revoked when the owner unmounts.
#[derive(Clone)]
pub(super) struct MarkdownImages {
    handler: Option<MarkdownImageDownloadHandler>,
    resolved: Arc<Mutex<HashMap<String, String>>>,
    resolving: Arc<Mutex<HashSet<String>>>,
}

impl MarkdownImages {
    /// Creates a resolver for the current component; without a handler images stay as rendered.
    pub(super) fn new(handler: Option<MarkdownImageDownloadHandler>) -> Self {
        let images = Self {
            handler,
            resolved: Arc::new(Mutex::new(HashMap::new())),
            resolving: Arc::new(Mutex::new(HashSet::new())),
        };
        let resolved = Arc::clone(&images.resolved);
        on_cleanup(move || {
            if let Ok(sources) = resolved.lock() {
                for display_source in sources.values() {
                    if display_source.starts_with("blob:") {
                        let _ = web_sys::Url::revoke_object_url(display_source);
                    }
                }
            }
        });
        images
    }

    /// Returns whether rendered HTML should defer image loading to [`Self::resolve`].
    pub(super) fn defers_downloads(&self) -> bool {
        self.handler.is_some()
    }

    /// Resolves all images below `root` that are not resolved or being resolved yet.
    ///
    /// `root` is looked up again when a download finishes, as the DOM may have been re-rendered.
    pub(super) fn resolve(
        &self,
        root: &HtmlElement,
        current_root: impl Fn() -> Option<HtmlElement> + Clone + 'static,
        on_error: impl Fn(String) + Clone + 'static,
    ) {
        let Some(handler) = self.handler.clone() else {
            return;
        };
        for image in images_below(root) {
            let Some(source) = markdown_image_source(&image) else {
                continue;
            };
            let display_source = self
                .resolved
                .lock()
                .ok()
                .and_then(|sources| sources.get(&source).cloned());
            if let Some(display_source) = display_source {
                set_markdown_image_display_source(&image, &display_source);
                continue;
            }
            let should_resolve = self
                .resolving
                .lock()
                .is_ok_and(|mut sources| sources.insert(source.clone()));
            if !should_resolve {
                continue;
            }

            let handler = handler.clone();
            let resolved = Arc::clone(&self.resolved);
            let resolving = Arc::clone(&self.resolving);
            let current_root = current_root.clone();
            let on_error = on_error.clone();
            spawn_local(async move {
                match handler.run(source.clone()).await {
                    Ok(display_source) => {
                        if let Ok(mut sources) = resolved.lock() {
                            sources.insert(source.clone(), display_source.clone());
                        }
                        if let Some(root) = current_root() {
                            for image in images_below(&root) {
                                if markdown_image_source(&image).as_deref() == Some(source.as_str())
                                {
                                    set_markdown_image_display_source(&image, &display_source);
                                }
                            }
                        }
                    }
                    Err(error) => on_error(error),
                }
                if let Ok(mut sources) = resolving.lock() {
                    sources.remove(&source);
                }
            });
        }
    }
}

/// Collects the image elements below `root`.
fn images_below(root: &HtmlElement) -> Vec<Element> {
    let Ok(nodes) = root.query_selector_all("img") else {
        return Vec::new();
    };
    (0..nodes.length())
        .filter_map(|index| nodes.item(index))
        .filter_map(|node| node.dyn_into::<Element>().ok())
        .collect()
}
