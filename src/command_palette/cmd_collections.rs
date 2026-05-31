use std::cell::{Cell, RefCell};
use std::sync::Arc;

use leptos::prelude::*;

use crate::{ArcOneCallback, ButtonBarItem, TabItem};

/// Default text and shortcut policy for one kind of command-enabled component.
pub trait CommandCollectionDefaults: Clone + 'static {
    /// User-facing default group name for generated commands.
    fn default_group() -> String;
    /// User-facing default name for the aggregate selection command.
    fn default_select_name() -> String;
    /// User-facing default placeholder for the aggregate selection prompt.
    fn default_select_placeholder() -> String;
    /// User-facing default shortcut for the aggregate selection command.
    fn default_select_shortcut() -> String;
    /// User-facing default name for one direct item command.
    fn default_item_name(context: Self) -> String;
    /// User-facing default option label for one item in the aggregate selector.
    fn default_item_option_label(context: Self) -> String;
    /// User-facing default shortcut for one direct item command.
    fn default_item_shortcut(global_index: usize, context: Self) -> String;
}

/// User-facing text and shortcut policy for commands generated from a
/// command-enabled collection component.
///
/// A collection is a component that exposes several selectable items, such as a
/// tab list. The config defines one aggregate command, for example `"Select
/// tab"` with shortcut `"TS"`, and one direct command per item, for example
/// `"Open Overview"` with shortcut `"T1"`.
#[derive(Clone)]
pub struct CommandCollectionConfig<Context: Clone + 'static> {
    /// User-facing group name used for generated collection commands.
    ///
    /// Rendered as the command palette section title, for example
    /// `"Navigation"` for tabs. The host application should provide this in
    /// the active UI language.
    pub group: String,
    /// User-facing name of the aggregate selection command.
    ///
    /// This command uses [`select_shortcut`](Self::select_shortcut) and opens a
    /// parameter prompt with every enabled registered item in the collection.
    pub select_name: String,
    /// Optional user-facing secondary text for the aggregate selection command.
    ///
    /// Rendered below [`select_name`](Self::select_name) when present.
    pub select_description: Option<String>,
    /// User-facing placeholder shown while choosing an item in the aggregate
    /// selection command.
    ///
    /// Example: `"Choose tab"`. The host application should provide this in
    /// the active UI language.
    pub select_placeholder: String,
    /// User-facing shortcut hint and searchable shortcut for the aggregate
    /// selection command.
    ///
    /// Example: `"TS"`. Whitespace is ignored and matching is
    /// case-insensitive.
    pub select_shortcut: String,
    /// Builds the user-facing command name for one direct item command.
    ///
    /// Called for every enabled registered item whenever the command palette
    /// rebuilds generated collection commands.
    pub item_name: Arc<dyn Fn(Context) -> String + Send + Sync>,
    /// Optionally builds user-facing secondary text for one direct item
    /// command.
    ///
    /// Rendered below the generated direct item command name when present.
    pub item_description: Option<Arc<dyn Fn(Context) -> Option<String> + Send + Sync>>,
    /// Builds the user-facing option label for one item inside the aggregate
    /// selection command.
    ///
    /// The returned text is shown in the option list opened by
    /// [`select_name`](Self::select_name). For tabs, this is typically the tab
    /// label.
    pub item_option_label: Arc<dyn Fn(Context) -> String + Send + Sync>,
    /// Builds the user-facing shortcut for one direct item command.
    ///
    /// Receives a zero-based global index across all enabled registered items
    /// of the same collection kind in registration order. For tabs, the default
    /// produces `"T1"`, `"T2"`, `"T3"`, and so on.
    pub item_shortcut: Arc<dyn Fn(usize, Context) -> String + Send + Sync>,
}

impl<Context> Default for CommandCollectionConfig<Context>
where
    Context: CommandCollectionDefaults,
{
    fn default() -> Self {
        Self::new()
    }
}

impl<Context> CommandCollectionConfig<Context>
where
    Context: CommandCollectionDefaults,
{
    /// Creates collection command text with defaults for the context type.
    ///
    /// Host applications with runtime language switching can pass a derived
    /// config to [`CommandPalette`](super::CommandPalette) and rebuild these
    /// values whenever the active language changes.
    pub fn new() -> Self {
        Self {
            group: Context::default_group(),
            select_name: Context::default_select_name(),
            select_description: None,
            select_placeholder: Context::default_select_placeholder(),
            select_shortcut: Context::default_select_shortcut(),
            item_name: Arc::new(Context::default_item_name),
            item_description: None,
            item_option_label: Arc::new(Context::default_item_option_label),
            item_shortcut: Arc::new(Context::default_item_shortcut),
        }
    }
}

impl<Context: Clone + 'static> CommandCollectionConfig<Context> {
    /// Sets the user-facing group name used for generated collection commands.
    pub fn group(mut self, group: impl Into<String>) -> Self {
        self.group = group.into();
        self
    }

    /// Sets the user-facing name of the aggregate selection command.
    pub fn select_name(mut self, name: impl Into<String>) -> Self {
        self.select_name = name.into();
        self
    }

    /// Sets the optional user-facing description of the aggregate selection
    /// command.
    pub fn select_description(mut self, description: impl Into<String>) -> Self {
        self.select_description = Some(description.into());
        self
    }

    /// Sets the user-facing placeholder shown by the aggregate selection
    /// prompt.
    pub fn select_placeholder(mut self, placeholder: impl Into<String>) -> Self {
        self.select_placeholder = placeholder.into();
        self
    }

    /// Sets the user-facing shortcut for the aggregate selection command.
    pub fn select_shortcut(mut self, shortcut: impl Into<String>) -> Self {
        self.select_shortcut = shortcut.into();
        self
    }

    /// Sets the user-facing command name builder for direct item commands.
    pub fn item_name(
        mut self,
        item_name: impl Fn(Context) -> String + Send + Sync + 'static,
    ) -> Self {
        self.item_name = Arc::new(item_name);
        self
    }

    /// Sets the user-facing description builder for direct item commands.
    pub fn item_description(
        mut self,
        item_description: impl Fn(Context) -> Option<String> + Send + Sync + 'static,
    ) -> Self {
        self.item_description = Some(Arc::new(item_description));
        self
    }

    /// Sets the user-facing option label builder used by the aggregate
    /// selection command.
    pub fn item_option_label(
        mut self,
        item_option_label: impl Fn(Context) -> String + Send + Sync + 'static,
    ) -> Self {
        self.item_option_label = Arc::new(item_option_label);
        self
    }

    /// Sets the user-facing shortcut builder for direct item commands.
    pub fn item_shortcut(
        mut self,
        item_shortcut: impl Fn(usize, Context) -> String + Send + Sync + 'static,
    ) -> Self {
        self.item_shortcut = Arc::new(item_shortcut);
        self
    }
}

/// Tab metadata passed to generated tab command text callbacks.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct TabCommandContext {
    /// Zero-based index of the tab within its own [`TabList`](crate::TabList).
    ///
    /// This is structural metadata, not user-facing text. It is only shown if
    /// the host application includes it in a generated label.
    pub local_index: usize,
    /// The tab entry being converted into command palette entries.
    ///
    /// `tab.value` is the internal selection value. `tab.label` is the
    /// user-facing label shown by the tab trigger.
    pub tab: TabItem,
}

impl CommandCollectionDefaults for TabCommandContext {
    fn default_group() -> String {
        String::from("Navigation")
    }

    fn default_select_name() -> String {
        String::from("Select tab")
    }

    fn default_select_placeholder() -> String {
        String::from("Choose tab")
    }

    fn default_select_shortcut() -> String {
        String::from("TS")
    }

    fn default_item_name(context: Self) -> String {
        format!("Open {}", context.tab.label)
    }

    fn default_item_option_label(context: Self) -> String {
        context.tab.label
    }

    fn default_item_shortcut(global_index: usize, _context: Self) -> String {
        format!("T{}", global_index + 1)
    }
}

/// Button bar metadata passed to generated button bar command text callbacks.
#[derive(Clone)]
pub struct ButtonBarCommandContext {
    /// Zero-based index of the action within its own [`ButtonBar`](crate::ButtonBar).
    ///
    /// This is structural metadata, not user-facing text. It is only shown if
    /// the host application includes it in a generated label.
    pub local_index: usize,
    /// The button bar item being converted into command palette entries.
    ///
    /// `item.value` is the internal action value. `item.label` is the
    /// user-facing label shown by the button trigger.
    pub item: ButtonBarItem,
}

impl CommandCollectionDefaults for ButtonBarCommandContext {
    fn default_group() -> String {
        String::from("Actions")
    }

    fn default_select_name() -> String {
        String::from("Select action")
    }

    fn default_select_placeholder() -> String {
        String::from("Choose action")
    }

    fn default_select_shortcut() -> String {
        String::from("AS")
    }

    fn default_item_name(context: Self) -> String {
        context.item.label.get().unwrap_or_default()
    }

    fn default_item_option_label(context: Self) -> String {
        context.item.label.get().unwrap_or_default()
    }

    fn default_item_shortcut(global_index: usize, _context: Self) -> String {
        format!("A{}", global_index + 1)
    }
}

/// Command-generation config for command-enabled tab lists.
pub type TabCommandPaletteConfig = CommandCollectionConfig<TabCommandContext>;

/// Command-generation config for command-enabled button bars.
pub type ButtonBarCommandPaletteConfig = CommandCollectionConfig<ButtonBarCommandContext>;

#[derive(Clone)]
pub(crate) struct RegisteredTabList {
    pub id: usize,
    pub tabs: RwSignal<Vec<TabItem>>,
    pub on_select: ArcOneCallback<String>,
}

#[derive(Clone)]
pub(crate) struct RegisteredButtonBar {
    pub id: usize,
    pub items: RwSignal<Vec<ButtonBarItem>>,
    pub on_select: ArcOneCallback<String>,
}

thread_local! {
    static NEXT_TAB_REGISTRATION_ID: Cell<usize> = const { Cell::new(1) };
    static NEXT_BUTTON_BAR_REGISTRATION_ID: Cell<usize> = const { Cell::new(1) };
    static NEXT_COMMAND_LISTENER_ID: Cell<usize> = const { Cell::new(1) };
    static REGISTERED_TAB_LISTS: RefCell<Vec<RegisteredTabList>> = const { RefCell::new(Vec::new()) };
    static REGISTERED_BUTTON_BARS: RefCell<Vec<RegisteredButtonBar>> = const { RefCell::new(Vec::new()) };
    static COMMAND_COLLECTION_LISTENERS: RefCell<Vec<(usize, RwSignal<u64>)>> = const { RefCell::new(Vec::new()) };
}

pub(crate) fn register_tab_list(
    tabs: RwSignal<Vec<TabItem>>,
    on_select: ArcOneCallback<String>,
) -> usize {
    let id = NEXT_TAB_REGISTRATION_ID.with(|next| {
        let id = next.get();
        next.set(id + 1);
        id
    });

    REGISTERED_TAB_LISTS.with(|registered| {
        registered.borrow_mut().push(RegisteredTabList {
            id,
            tabs,
            on_select,
        });
    });
    notify_command_collection_registry();
    id
}

pub(crate) fn unregister_tab_list(id: usize) {
    REGISTERED_TAB_LISTS.with(|registered| {
        registered.borrow_mut().retain(|entry| entry.id != id);
    });
    notify_command_collection_registry();
}

pub(crate) fn registered_tab_lists() -> Vec<RegisteredTabList> {
    REGISTERED_TAB_LISTS.with(|registered| registered.borrow().clone())
}

pub(crate) fn register_button_bar(
    items: RwSignal<Vec<ButtonBarItem>>,
    on_select: ArcOneCallback<String>,
) -> usize {
    let id = NEXT_BUTTON_BAR_REGISTRATION_ID.with(|next| {
        let id = next.get();
        next.set(id + 1);
        id
    });

    REGISTERED_BUTTON_BARS.with(|registered| {
        registered.borrow_mut().push(RegisteredButtonBar {
            id,
            items,
            on_select,
        });
    });
    notify_command_collection_registry();
    id
}

pub(crate) fn unregister_button_bar(id: usize) {
    REGISTERED_BUTTON_BARS.with(|registered| {
        registered.borrow_mut().retain(|entry| entry.id != id);
    });
    notify_command_collection_registry();
}

pub(crate) fn registered_button_bars() -> Vec<RegisteredButtonBar> {
    REGISTERED_BUTTON_BARS.with(|registered| registered.borrow().clone())
}

pub(crate) fn notify_command_collection_registry() {
    COMMAND_COLLECTION_LISTENERS.with(|listeners| {
        for (_, listener) in listeners.borrow().iter() {
            listener.update(|version| *version += 1);
        }
    });
}

pub(crate) fn register_command_collection_listener(listener: RwSignal<u64>) -> usize {
    let id = NEXT_COMMAND_LISTENER_ID.with(|next| {
        let id = next.get();
        next.set(id + 1);
        id
    });

    COMMAND_COLLECTION_LISTENERS.with(|listeners| {
        listeners.borrow_mut().push((id, listener));
    });
    id
}

pub(crate) fn unregister_command_collection_listener(id: usize) {
    COMMAND_COLLECTION_LISTENERS.with(|listeners| {
        listeners
            .borrow_mut()
            .retain(|(listener_id, _)| *listener_id != id);
    });
}
