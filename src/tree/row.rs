use crate::{ArcOneCallback, IcnName, Icon, Size};
use leptos::prelude::*;

/// Renders one virtual tree item and its disclosure control.
#[component]
pub(super) fn TreeRow(
    index: usize,
    title: String,
    icon: Option<IcnName>,
    meta: Option<String>,
    highlight: Option<String>,
    depth: usize,
    position: usize,
    sibling_count: usize,
    has_children: bool,
    expanded: bool,
    selected: bool,
    active: bool,
    on_toggle: ArcOneCallback<()>,
    on_select: ArcOneCallback<()>,
    on_hover: ArcOneCallback<()>,
) -> impl IntoView {
    let class = if selected {
        "birei-tree__row birei-tree__row--selected"
    } else if active {
        "birei-tree__row birei-tree__row--active"
    } else {
        "birei-tree__row"
    };
    let action = if expanded { "Collapse" } else { "Expand" };
    view! {
        <div
            id=format!("birei-tree-row-{index}")
            class=class
            style=move || {
                let mut style = format!("--birei-tree-depth: {};", depth.saturating_sub(1));
                if let Some(color) = highlight.as_deref().filter(|color| !color.trim().is_empty()) {
                    style.push_str(&format!("--birei-tree-row-highlight: {color};"));
                }
                style
            }
            role="treeitem"
            attr:aria-level=depth.to_string()
            attr:aria-posinset=position.to_string()
            attr:aria-setsize=sibling_count.to_string()
            aria-expanded=has_children.then_some(if expanded { "true" } else { "false" })
            aria-selected=if selected { "true" } else { "false" }
            on:mousemove=move |_| on_hover.run(())
            on:click=move |_| on_select.run(())
        >
            <span class="birei-tree__indent" aria-hidden="true"></span>
            {if has_children {
                view! {
                    <button
                        type="button"
                        class="birei-tree__toggle"
                        tabindex="-1"
                        aria-label=format!("{action} {title}")
                        on:click=move |event| {
                            event.stop_propagation();
                            on_toggle.run(());
                        }
                    >
                        <Icon name=if expanded { "chevron-down" } else { "chevron-right" } size=Size::Small />
                    </button>
                }.into_any()
            } else {
                view! { <span class="birei-tree__toggle-spacer" aria-hidden="true"></span> }.into_any()
            }}
            {icon.map(|icon| view! { <Icon name=icon size=Size::Small class="birei-tree__icon" /> })}
            <span class="birei-tree__title">{title}</span>
            {meta.map(|meta| view! { <span class="birei-tree__meta">{meta}</span> })}
        </div>
    }
}
