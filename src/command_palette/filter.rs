use leptos::prelude::*;

use super::{CommandItem, CommandParameterOption};
use crate::common::step_enabled_index;

pub(super) fn filter_command_items(items: &[CommandItem], query: &str) -> Vec<CommandItem> {
    let needle = query.trim().to_lowercase();
    let compact_needle = compact_shortcut_query(query);
    let (exact_shortcut_matches, other_matches): (Vec<_>, Vec<_>) = items
        .iter()
        .filter(|item| {
            needle.is_empty()
                || item.name.to_lowercase().contains(&needle)
                || item.value.to_lowercase().contains(&needle)
                || item
                    .description
                    .as_ref()
                    .is_some_and(|description| description.to_lowercase().contains(&needle))
                || item
                    .group
                    .as_ref()
                    .is_some_and(|group| group.to_lowercase().contains(&needle))
                || item
                    .keywords
                    .iter()
                    .any(|keyword| keyword.to_lowercase().contains(&needle))
                || item.shortcut.as_ref().is_some_and(|shortcut| {
                    shortcut.to_lowercase().contains(&needle)
                        || shortcut.to_lowercase().contains(&compact_needle)
                })
        })
        .partition(|item| shortcut_matches_exact(item, &compact_needle));

    exact_shortcut_matches
        .into_iter()
        .chain(other_matches)
        .cloned()
        .collect()
}

pub(super) fn visible_items_for_query(
    items: &[CommandItem],
    recent_items: &[CommandItem],
    query: &str,
) -> (Vec<CommandItem>, Vec<CommandItem>) {
    let regular = filter_command_items(items, query);
    let recent = if query.trim().is_empty() {
        recent_items.to_vec()
    } else {
        Vec::new()
    };
    (recent, regular)
}

pub(super) fn flatten_visible_items(
    visible_items: (Vec<CommandItem>, Vec<CommandItem>),
) -> Vec<CommandItem> {
    let (recent, regular) = visible_items;
    recent.into_iter().chain(regular).collect()
}

pub(super) fn exact_shortcut_command(items: &[CommandItem], query: &str) -> Option<CommandItem> {
    let needle = compact_shortcut_query(query);
    if needle.is_empty() {
        return None;
    }

    items
        .iter()
        .find(|item| !item.disabled && shortcut_matches_exact(item, &needle))
        .cloned()
}

pub(super) fn first_enabled_exact_shortcut_index(
    items: &[CommandItem],
    query: &str,
) -> Option<usize> {
    let needle = compact_shortcut_query(query);
    items
        .iter()
        .position(|item| !item.disabled && shortcut_matches_exact(item, &needle))
}

fn shortcut_matches_exact(item: &CommandItem, compact_needle: &str) -> bool {
    !compact_needle.is_empty()
        && item
            .shortcut
            .as_ref()
            .is_some_and(|shortcut| shortcut.to_lowercase() == compact_needle)
}

pub(super) fn filter_parameter_options(
    options: &[CommandParameterOption],
    query: &str,
) -> Vec<CommandParameterOption> {
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

pub(super) fn next_enabled_parameter_option_index(
    options: &[CommandParameterOption],
    current: Option<usize>,
    direction: i32,
) -> Option<usize> {
    step_enabled_index(options.len(), current, direction, |_| true)
}

fn compact_shortcut_query(query: &str) -> String {
    query
        .chars()
        .filter(|character| !character.is_whitespace())
        .flat_map(char::to_lowercase)
        .collect()
}

pub(super) fn sync_active_index(
    active_index: RwSignal<Option<usize>>,
    items: &[CommandItem],
    seed: bool,
) {
    let next = active_index
        .try_get_untracked()
        .unwrap_or_default()
        .filter(|index| items.get(*index).is_some_and(|item| !item.disabled))
        .or_else(|| seed.then(|| first_enabled_command_index(items)).flatten());
    active_index.set(next);
}

pub(super) fn first_enabled_command_index(items: &[CommandItem]) -> Option<usize> {
    items.iter().position(|item| !item.disabled)
}

pub(super) fn next_enabled_command_index(
    items: &[CommandItem],
    current: Option<usize>,
    direction: i32,
) -> Option<usize> {
    step_enabled_index(items.len(), current, direction, |index| {
        !items[index].disabled
    })
}
