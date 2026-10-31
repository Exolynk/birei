use leptos::{ev, html, prelude::*};
use wasm_bindgen::JsCast;
use web_sys::HtmlElement;

use super::SelectOption;
use crate::common::FloatingPopupLayout;
use crate::{ArcOneCallback, IcnName, Icon, Size};

/// Fixed height of one option row, used for virtualization and scrolling.
pub(super) const SELECT_OPTION_ROW_HEIGHT: f64 = 40.0;
/// Rows rendered above and below the visible window.
const SELECT_OPTION_OVERSCAN: usize = 6;

/// Portaled option list of a select, virtualized to the visible rows.
///
/// Rows are keyed, so hovering or selecting only updates the classes of the affected rows
/// instead of rebuilding the visible window.
#[component]
pub(super) fn SelectMenu(
    /// Options after filtering, in popup order.
    options: Memo<Vec<SelectOption>>,
    /// Values that are currently selected.
    selected: Signal<Vec<String>>,
    /// Keyboard or hover highlighted option.
    active_index: RwSignal<Option<usize>>,
    /// Floating position of the menu.
    layout: RwSignal<FloatingPopupLayout>,
    /// Scroll offset of the menu, drives virtualization.
    scroll_top: RwSignal<f64>,
    /// Visible height of the menu, drives virtualization.
    viewport_height: RwSignal<f64>,
    /// Reference used for scrolling and measuring the menu.
    menu_ref: NodeRef<html::Div>,
    /// Allows several selected values.
    multiple: bool,
    /// Called with the value of a chosen enabled option.
    on_select: ArcOneCallback<String>,
) -> impl IntoView {
    let range = Memo::new(move |_| {
        let len = options.try_with(Vec::len).unwrap_or_default();
        visible_range(
            len,
            scroll_top.try_get().unwrap_or_default(),
            viewport_height.try_get().unwrap_or_default(),
        )
    });
    let visible_rows = move || {
        let (start, end) = range.try_get().unwrap_or_default();
        options
            .try_with(|options| {
                options[start..end]
                    .iter()
                    .cloned()
                    .enumerate()
                    .map(|(offset, option)| (start + offset, option))
                    .collect::<Vec<_>>()
            })
            .unwrap_or_default()
    };
    let spacer = move |top: bool| {
        let (start, end) = range.try_get().unwrap_or_default();
        let len = options.try_with(Vec::len).unwrap_or_default();
        let rows = if top { start } else { len.saturating_sub(end) };
        format!("height: {}px;", rows as f64 * SELECT_OPTION_ROW_HEIGHT)
    };

    view! {
        <div
            class=move || {
                if layout.try_get().unwrap_or_default().open_upward {
                    "birei-select__menu birei-select__menu--portal birei-select__menu--upward"
                } else {
                    "birei-select__menu birei-select__menu--portal"
                }
            }
            style=move || {
                let layout = layout.try_get().unwrap_or_default();
                format!(
                    "left: {}px; top: {}px; width: {}px; max-height: {}px;",
                    layout.left, layout.top, layout.width, layout.max_height
                )
            }
            node_ref=menu_ref
            role="listbox"
            aria-multiselectable=if multiple { "true" } else { "false" }
            on:scroll=move |event: ev::Event| {
                let Some(target) = event
                    .current_target()
                    .and_then(|target| target.dyn_into::<HtmlElement>().ok())
                else {
                    return;
                };
                let next_scroll_top = f64::from(target.scroll_top());
                let next_viewport_height = f64::from(target.client_height());
                if scroll_top.try_get_untracked() != Some(next_scroll_top) {
                    let _ = scroll_top.try_set(next_scroll_top);
                }
                if viewport_height.try_get_untracked() != Some(next_viewport_height) {
                    let _ = viewport_height.try_set(next_viewport_height);
                }
            }
        >
            <Show
                when=move || options.try_with(|options| !options.is_empty()).unwrap_or_default()
                fallback=|| view! { <div class="birei-select__empty">"No matching values"</div> }
            >
                <div class="birei-select__menu-spacer" style=move || spacer(true)></div>
                <For
                    each=visible_rows
                    key=|(index, option)| (*index, option.value.clone())
                    children=move |(index, option)| {
                        let value = option.value.clone();
                        let is_selected = Signal::derive(move || {
                            selected
                                .try_with(|selected| selected.contains(&value))
                                .unwrap_or_default()
                        });
                        let is_active = Signal::derive(move || {
                            active_index.try_get().flatten() == Some(index)
                        });
                        let disabled = option.disabled;
                        let value = option.value.clone();
                        view! {
                            <SelectMenuOption
                                option_index=index
                                label=option.label
                                icon=option.icon
                                disabled=disabled
                                selected=is_selected
                                active=is_active
                                on_hover=move |_| {
                                    if !disabled {
                                        let _ = active_index.try_set(Some(index));
                                    }
                                }
                                on_select=move |_| {
                                    if !disabled {
                                        on_select.run(value.clone());
                                    }
                                }
                            />
                        }
                    }
                />
                <div class="birei-select__menu-spacer" style=move || spacer(false)></div>
            </Show>
        </div>
    }
}

/// Render a popup row including optional icon, selection mark, and active styling.
#[component]
fn SelectMenuOption(
    option_index: usize,
    label: String,
    icon: Option<IcnName>,
    disabled: bool,
    selected: Signal<bool>,
    active: Signal<bool>,
    #[prop(into)] on_hover: ArcOneCallback<()>,
    #[prop(into)] on_select: ArcOneCallback<()>,
) -> impl IntoView {
    view! {
        <button
            type="button"
            data-option-index=option_index.to_string()
            class=move || {
                option_class_name(
                    selected.try_get().unwrap_or_default(),
                    active.try_get().unwrap_or_default(),
                    disabled,
                )
            }
            disabled=disabled
            on:mousedown=move |event| {
                event.prevent_default();
            }
            on:mouseenter=move |_| on_hover.run(())
            on:click=move |_| on_select.run(())
        >
            <span class="birei-select__option-content">
                {icon.map(|icon| {
                    view! {
                        <Icon
                            name=icon
                            size=Size::Small
                            label=label.clone()
                        />
                    }
                })}
                <span>{label.clone()}</span>
            </span>
            <span class="birei-select__option-mark" aria-hidden="true">
                {move || if selected.try_get().unwrap_or_default() { "✓" } else { "" }}
            </span>
        </button>
    }
}

/// Builds popup option classes from selected, active, and disabled state.
fn option_class_name(selected: bool, active: bool, disabled: bool) -> String {
    let mut classes = String::from("birei-select__option");
    if selected {
        classes.push_str(" birei-select__option--selected");
    }
    if active {
        classes.push_str(" birei-select__option--active");
    }
    if disabled {
        classes.push_str(" birei-select__option--disabled");
    }
    classes
}

/// Computes the rendered row window `(start, end)` for the scroll position, with overscan.
fn visible_range(len: usize, scroll_top: f64, viewport_height: f64) -> (usize, usize) {
    if len == 0 {
        return (0, 0);
    }
    let first = (scroll_top / SELECT_OPTION_ROW_HEIGHT).floor().max(0.0) as usize;
    let last = ((scroll_top + viewport_height) / SELECT_OPTION_ROW_HEIGHT)
        .ceil()
        .max(0.0) as usize;
    let start = first.saturating_sub(SELECT_OPTION_OVERSCAN).min(len);
    let end = (last + SELECT_OPTION_OVERSCAN).min(len);
    (start, end.max(start))
}

#[cfg(test)]
mod tests {
    use super::visible_range;

    /// The window covers the viewport plus overscan and stays inside the list.
    #[test]
    fn window_covers_viewport_with_overscan() {
        assert_eq!(visible_range(0, 0.0, 400.0), (0, 0));
        assert_eq!(visible_range(100, 0.0, 400.0), (0, 16));
        assert_eq!(visible_range(100, 800.0, 400.0), (14, 36));
        assert_eq!(visible_range(10, 800.0, 400.0), (10, 10));
    }
}
