use std::sync::Arc;

use leptos::prelude::*;

use super::types::NotificationVariant;

#[derive(Clone, Debug)]
pub(crate) struct NotificationRecord {
    pub(crate) id: usize,
    pub(crate) text: String,
    pub(crate) variant: NotificationVariant,
    pub(crate) duration_ms: i32,
}

#[derive(Clone, Debug)]
pub(crate) struct HostedNotification {
    pub(crate) record: NotificationRecord,
    pub(crate) exiting: RwSignal<bool>,
    pub(crate) removal_scheduled: RwSignal<bool>,
}

#[derive(Clone)]
pub(crate) struct NotificationBindings {
    pub(crate) add: Arc<dyn Fn(NotificationRecord) + Send + Sync>,
    pub(crate) remove: Arc<dyn Fn(usize) + Send + Sync>,
    pub(crate) clear: Arc<dyn Fn() + Send + Sync>,
}

#[derive(Default)]
pub(crate) struct NotificationManagerState {
    pub(crate) bindings: Option<NotificationBindings>,
    pub(crate) mounted: bool,
    pub(crate) pending: Vec<NotificationRecord>,
}
