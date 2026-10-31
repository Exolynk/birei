use super::SelectOption;
use crate::common::step_enabled_index;

/// Label/value pair used to render selected tags in multi-select mode.
#[derive(Clone)]
pub(super) struct SelectedTagData {
    pub(super) value: String,
    pub(super) label: String,
}

/// Find the first selectable option in the current filtered set.
pub(super) fn first_enabled_index(options: &[SelectOption]) -> Option<usize> {
    options.iter().position(|option| !option.disabled)
}

/// Find the first selected option in popup order so opening the menu can scroll to it.
pub(super) fn first_selected_index(
    options: &[SelectOption],
    selected_value: Option<&str>,
    selected_values: &[String],
    multiple: bool,
) -> Option<usize> {
    options.iter().position(|option| {
        !option.disabled
            && if multiple {
                selected_values.iter().any(|value| value == &option.value)
            } else {
                selected_value == Some(option.value.as_str())
            }
    })
}

/// Advances to the next enabled option in popup order, wrapping around the list.
pub(super) fn next_enabled_index(
    options: &[SelectOption],
    current: Option<usize>,
    direction: i32,
) -> Option<usize> {
    step_enabled_index(options.len(), current, direction, |index| {
        !options[index].disabled
    })
}

/// Resolve the display label for a selected single value.
pub(super) fn find_option_label(options: &[SelectOption], value: Option<&str>) -> String {
    options
        .iter()
        .find(|option| Some(option.value.as_str()) == value)
        .map(|option| option.label.clone())
        .unwrap_or_default()
}

/// Filter options by label or value against the free-text query.
pub(super) fn filter_options_by_query(options: &[SelectOption], query: &str) -> Vec<SelectOption> {
    let needle = query.trim().to_lowercase();

    options
        .iter()
        .filter(|option| {
            needle.is_empty()
                || option.label.to_lowercase().contains(&needle)
                || option.value.to_lowercase().contains(&needle)
        })
        .cloned()
        .collect()
}

/// Map selected values back to their labels for multi-select token rendering.
pub(super) fn collect_selected_tags(
    options: &[SelectOption],
    selected_values: &[String],
) -> Vec<SelectedTagData> {
    selected_values
        .iter()
        .filter_map(|selected_value| {
            options
                .iter()
                .find(|option| option.value == *selected_value)
                .map(|option| SelectedTagData {
                    value: option.value.clone(),
                    label: option.label.clone(),
                })
        })
        .collect()
}
