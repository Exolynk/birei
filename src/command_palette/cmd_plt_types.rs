use crate::{ArcOneCallback, IcnName};

/// Payload passed when a command item executes its own action.
#[derive(Clone)]
pub struct CommandExecution {
    /// Command definition that was activated.
    ///
    /// The `item.value` field remains the internal command id, while
    /// `item.name` is the user-facing label that was shown in the palette.
    pub item: CommandItem,
    /// Parameter values collected before the command executed.
    ///
    /// Each entry is keyed by [`CommandParameter::name`], which is an internal
    /// parameter id chosen by the host application.
    pub parameters: Vec<CommandParameterValue>,
}

/// Text parameter requested before a command executes.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct CommandParameter {
    /// Internal parameter id returned in [`CommandExecution::parameters`].
    ///
    /// This value is not shown to users. Use a stable machine-readable key,
    /// for example `"category"` or `"target_user"`.
    pub name: String,
    /// User-facing prompt shown while the palette asks for this parameter.
    ///
    /// The host application should provide this in the active UI language.
    pub placeholder: String,
    /// Optional selectable values for this parameter.
    ///
    /// When empty, the palette accepts free text. When non-empty, the palette
    /// shows a filtered option list and submits the selected
    /// [`CommandParameterOption::value`].
    pub options: Vec<CommandParameterOption>,
}

impl CommandParameter {
    /// Creates a named text parameter.
    pub fn new(name: impl Into<String>, placeholder: impl Into<String>) -> Self {
        Self {
            name: name.into(),
            placeholder: placeholder.into(),
            options: Vec::new(),
        }
    }

    /// Creates a named parameter whose value is selected from options.
    pub fn options(
        name: impl Into<String>,
        placeholder: impl Into<String>,
        options: Vec<CommandParameterOption>,
    ) -> Self {
        Self {
            name: name.into(),
            placeholder: placeholder.into(),
            options,
        }
    }
}

/// Selectable value for an option-backed command parameter.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct CommandParameterOption {
    /// Internal value submitted to the command action.
    ///
    /// This value is not shown to users. It is returned as
    /// [`CommandParameterValue::value`] when the option is selected.
    pub value: String,
    /// User-facing option label shown in the command palette.
    ///
    /// The host application should provide this in the active UI language.
    pub label: String,
}

impl CommandParameterOption {
    /// Creates a selectable parameter option.
    pub fn new(value: impl Into<String>, label: impl Into<String>) -> Self {
        Self {
            value: value.into(),
            label: label.into(),
        }
    }
}

/// Named parameter value collected by the command palette.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct CommandParameterValue {
    /// Internal parameter id copied from [`CommandParameter::name`].
    pub name: String,
    /// Submitted parameter value.
    ///
    /// For free-text parameters this is the entered text. For option-backed
    /// parameters this is the selected [`CommandParameterOption::value`].
    pub value: String,
}

/// Action or destination rendered by [`CommandPalette`](super::CommandPalette).
#[derive(Clone)]
pub struct CommandItem {
    /// Internal stable command id.
    ///
    /// This value is not shown to users. Use it to identify the command in
    /// callbacks, tests, analytics, or host-side routing.
    pub value: String,
    /// User-facing command name shown as the primary row text.
    ///
    /// The host application should provide this in the active UI language.
    pub name: String,
    /// Optional user-facing secondary text shown below [`name`](Self::name).
    pub description: Option<String>,
    /// Optional leading icon shown before the command text.
    pub icon: Option<IcnName>,
    /// Optional user-facing section title used to group commands.
    ///
    /// The host application should provide this in the active UI language.
    pub group: Option<String>,
    /// Optional user-facing shortcut hint and searchable shortcut.
    ///
    /// Whitespace is stripped, labels are normalized to uppercase for display,
    /// and matching is case-insensitive.
    pub shortcut: Option<String>,
    /// Additional search terms that are not rendered in the command row.
    pub keywords: Vec<String>,
    /// Parameters collected before the command action executes.
    ///
    /// Parameters are requested in order. If this list is empty, activating the
    /// command immediately runs [`action`](Self::action).
    pub parameters: Vec<CommandParameter>,
    /// Callback invoked after all parameters have been collected.
    ///
    /// When absent, activating the command only closes the palette.
    pub action: Option<ArcOneCallback<CommandExecution>>,
    /// Keeps the command visible but prevents selection and execution.
    pub disabled: bool,
}

impl CommandItem {
    /// Creates a command with a stable value and visible name.
    pub fn new(value: impl Into<String>, name: impl Into<String>) -> Self {
        Self {
            value: value.into(),
            name: name.into(),
            description: None,
            icon: None,
            group: None,
            shortcut: None,
            keywords: Vec::new(),
            parameters: Vec::new(),
            action: None,
            disabled: false,
        }
    }

    /// Adds secondary text shown below the command name.
    pub fn description(mut self, description: impl Into<String>) -> Self {
        self.description = Some(description.into());
        self
    }

    /// Adds a leading icon.
    pub fn icon(mut self, icon: impl Into<IcnName>) -> Self {
        self.icon = Some(icon.into());
        self
    }

    /// Places the command inside a named section.
    pub fn group(mut self, group: impl Into<String>) -> Self {
        self.group = Some(group.into());
        self
    }

    /// Adds an optional trailing keyboard shortcut hint.
    pub fn shortcut(mut self, shortcut: impl Into<String>) -> Self {
        self.shortcut = Some(normalize_shortcut_label(&shortcut.into()));
        self
    }

    /// Adds a non-visible term that can be used to find the command.
    pub fn keyword(mut self, keyword: impl Into<String>) -> Self {
        self.keywords.push(keyword.into());
        self
    }

    /// Adds one text parameter requested before the command executes.
    pub fn parameter(mut self, name: impl Into<String>, placeholder: impl Into<String>) -> Self {
        self.parameters
            .push(CommandParameter::new(name, placeholder));
        self
    }

    /// Adds one option-backed parameter requested before the command executes.
    pub fn parameter_options(
        mut self,
        name: impl Into<String>,
        placeholder: impl Into<String>,
        options: Vec<CommandParameterOption>,
    ) -> Self {
        self.parameters
            .push(CommandParameter::options(name, placeholder, options));
        self
    }

    /// Adds the action callback executed when the command is activated.
    pub fn action(mut self, action: impl Into<ArcOneCallback<CommandExecution>>) -> Self {
        self.action = Some(action.into());
        self
    }

    /// Marks the command as visible but non-interactive.
    pub fn disabled(mut self, disabled: bool) -> Self {
        self.disabled = disabled;
        self
    }
}

impl PartialEq for CommandItem {
    fn eq(&self, other: &Self) -> bool {
        self.value == other.value
            && self.name == other.name
            && self.description == other.description
            && self.icon == other.icon
            && self.group == other.group
            && self.shortcut == other.shortcut
            && self.keywords == other.keywords
            && self.parameters == other.parameters
            && self.disabled == other.disabled
    }
}

impl Eq for CommandItem {}

fn normalize_shortcut_label(shortcut: &str) -> String {
    shortcut
        .chars()
        .filter(|character| !character.is_whitespace())
        .flat_map(char::to_uppercase)
        .collect()
}
