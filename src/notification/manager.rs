use std::sync::atomic::{AtomicUsize, Ordering};
use std::sync::{Arc, Mutex, OnceLock};

use leptos::mount::mount_to_body;
use leptos::prelude::*;

use super::host::NotificationHost;
use super::state::{NotificationBindings, NotificationManagerState, NotificationRecord};
use super::types::{NotificationOptions, NotificationVariant};

static NOTIFICATION_MANAGER: OnceLock<NotificationManager> = OnceLock::new();
static NOTIFICATION_ID_COUNTER: AtomicUsize = AtomicUsize::new(1);

/// Global notification manager with implicit host mounting.
#[derive(Clone, Default)]
pub struct NotificationManager(Arc<Mutex<NotificationManagerState>>);

impl NotificationManager {
    /// Returns the singleton global notification manager.
    pub fn global() -> &'static Self {
        NOTIFICATION_MANAGER.get_or_init(Self::default)
    }

    /// Creates a notification from a full options object and returns its identifier.
    pub fn notify(&self, options: NotificationOptions) -> usize {
        self.ensure_host();

        let record = NotificationRecord {
            id: NOTIFICATION_ID_COUNTER.fetch_add(1, Ordering::Relaxed),
            text: options.text,
            variant: options.variant,
            duration_ms: options
                .duration_ms
                .unwrap_or_else(|| options.variant.default_duration_ms()),
        };

        if let Some(bindings) = self.bindings() {
            (bindings.add)(record.clone());
        } else if let Ok(mut state) = self.0.lock() {
            state.pending.insert(0, record.clone());
        }

        record.id
    }

    /// Convenience helper for an info notification.
    pub fn info(&self, text: impl Into<String>) -> usize {
        self.notify(NotificationOptions::new(text).variant(NotificationVariant::Info))
    }

    /// Convenience helper for a success notification.
    pub fn success(&self, text: impl Into<String>) -> usize {
        self.notify(NotificationOptions::new(text).variant(NotificationVariant::Success))
    }

    /// Convenience helper for a warning notification.
    pub fn warning(&self, text: impl Into<String>) -> usize {
        self.notify(NotificationOptions::new(text).variant(NotificationVariant::Warning))
    }

    /// Convenience helper for an error notification.
    pub fn error(&self, text: impl Into<String>) -> usize {
        self.notify(NotificationOptions::new(text).variant(NotificationVariant::Error))
    }

    /// Dismisses one notification by identifier.
    pub fn remove(&self, id: usize) {
        if let Some(bindings) = self.bindings() {
            (bindings.remove)(id);
        } else if let Ok(mut state) = self.0.lock() {
            state.pending.retain(|entry| entry.id != id);
        }
    }

    /// Dismisses every currently queued or visible notification.
    pub fn clear(&self) {
        if let Some(bindings) = self.bindings() {
            (bindings.clear)();
        }

        if let Ok(mut state) = self.0.lock() {
            state.pending.clear();
        }
    }

    pub(crate) fn attach(&self, bindings: NotificationBindings) {
        let pending = if let Ok(mut state) = self.0.lock() {
            state.bindings = Some(bindings.clone());
            std::mem::take(&mut state.pending)
        } else {
            Vec::new()
        };

        for notification in pending.into_iter().rev() {
            (bindings.add)(notification);
        }
    }

    fn bindings(&self) -> Option<NotificationBindings> {
        self.0.lock().ok().and_then(|state| state.bindings.clone())
    }

    fn ensure_host(&self) {
        let should_mount = if let Ok(mut state) = self.0.lock() {
            if state.mounted {
                false
            } else {
                state.mounted = true;
                true
            }
        } else {
            false
        };

        if !should_mount || web_sys::window().is_none() {
            return;
        }

        let manager = self.clone();
        mount_to_body(move || view! { <NotificationHost manager=manager.clone()/> });
    }
}
