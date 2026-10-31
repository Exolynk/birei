use crate::ArcOneCallback;
use leptos::prelude::*;

use super::CommandItem;
use crate::{Icon, Size, Tag};

#[component]
pub(super) fn CommandGroups(
    items: Vec<CommandItem>,
    start_index: usize,
    active_index: RwSignal<Option<usize>>,
    #[prop(into)] on_hover: ArcOneCallback<usize>,
    #[prop(into)] on_select: ArcOneCallback<CommandItem>,
) -> impl IntoView {
    let mut groups = Vec::<(String, Vec<(usize, CommandItem)>)>::new();
    for (offset, item) in items.into_iter().enumerate() {
        let absolute_index = start_index + offset;
        let group = item
            .group
            .clone()
            .unwrap_or_else(|| String::from("Commands"));
        if let Some((_, group_items)) = groups.iter_mut().find(|(name, _)| name == &group) {
            group_items.push((absolute_index, item));
        } else {
            groups.push((group, vec![(absolute_index, item)]));
        }
    }

    view! {
        <>
            {groups.into_iter().map(|(title, items)| {
                view! {
                    <CommandSection
                        title=title
                        items=items
                        active_index=active_index
                        on_hover=on_hover
                        on_select=on_select
                    />
                }
            }).collect_view()}
        </>
    }
}

#[component]
pub(super) fn CommandSection(
    title: impl Into<String>,
    items: Vec<(usize, CommandItem)>,
    active_index: RwSignal<Option<usize>>,
    #[prop(into)] on_hover: ArcOneCallback<usize>,
    #[prop(into)] on_select: ArcOneCallback<CommandItem>,
) -> impl IntoView {
    let title = title.into();

    view! {
        <section class="birei-command__section">
            <div class="birei-command__section-title">{title}</div>
            {items.into_iter().map(|(index, item)| {
                let item_for_execute = item.clone();
                let disabled = item.disabled;
                let has_shortcut = item.shortcut.is_some();
                let hover_on_enter = on_hover;
                let hover_on_move = on_hover;
                let select_on_click = on_select;

                view! {
                    <button
                        type="button"
                        data-command-index=index.to_string()
                        class=move || command_item_class_name(active_index.try_get().flatten() == Some(index), disabled)
                        role="option"
                        tabindex="-1"
                        aria-selected=move || if active_index.try_get().flatten() == Some(index) { "true" } else { "false" }
                        disabled=disabled
                        on:mousedown=move |event| event.prevent_default()
                        on:mouseenter=move |_| {
                            if !disabled {
                                hover_on_enter.run(index);
                            }
                        }
                        on:mousemove=move |_| {
                            if !disabled {
                                hover_on_move.run(index);
                            }
                        }
                        on:click=move |_| select_on_click.run(item_for_execute.clone())
                    >
                        <span class=if has_shortcut {
                            "birei-command__item-icon birei-command__item-icon--has-shortcut"
                        } else {
                            "birei-command__item-icon"
                        }>
                            {item.icon.map(|icon| {
                                view! {
                                    <span class="birei-command__item-icon-glyph">
                                        <Icon name=icon size=Size::Small label=item.name.clone()/>
                                    </span>
                                }
                            })}
                            {item.shortcut.clone().map(|shortcut| {
                                view! {
                                    <Tag class="birei-command__item-shortcut birei-command__item-shortcut--leading">
                                        {shortcut}
                                    </Tag>
                                }
                            })}
                        </span>
                        <span class="birei-command__item-body">
                            <span class="birei-command__item-name">{item.name}</span>
                            {item.description.map(|description| {
                                view! {
                                    <span class="birei-command__item-description">{description}</span>
                                }
                            })}
                        </span>
                        {item.shortcut.map(|shortcut| {
                            view! {
                                <Tag class="birei-command__item-shortcut">{shortcut}</Tag>
                            }
                        })}
                    </button>
                }
            }).collect_view()}
        </section>
    }
}

pub(super) fn command_item_class_name(active: bool, disabled: bool) -> String {
    let mut classes = String::from("birei-command__item");
    if active {
        classes.push_str(" birei-command__item--active");
    }
    if disabled {
        classes.push_str(" birei-command__item--disabled");
    }
    classes
}
