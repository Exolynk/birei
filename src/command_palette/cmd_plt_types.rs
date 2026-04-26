use crate::IcnName;

/// Payload passed when a command item executes its own action.
#[derive(Clone)]
pub struct CommandExecution {
    pub item: CommandItem,
    pub parameters: Vec<CommandParameterValue>,
}

/// Text parameter requested before a command executes.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct CommandParameter {
    pub name: String,
    pub placeholder: String,
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
    pub value: String,
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
    pub name: String,
    pub value: String,
}

/// Action or destination rendered by [`CommandPalette`](super::CommandPalette).
#[derive(Clone)]
pub struct CommandItem {
    pub value: String,
    pub name: String,
    pub description: Option<String>,
    pub icon: Option<IcnName>,
    pub group: Option<String>,
    pub shortcut: Option<String>,
    pub parameters: Vec<CommandParameter>,
    pub action: Option<leptos::prelude::Callback<CommandExecution>>,
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
    pub fn action(mut self, action: leptos::prelude::Callback<CommandExecution>) -> Self {
        self.action = Some(action);
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
