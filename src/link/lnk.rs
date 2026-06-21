use crate::{ArcOneCallback, IcnName, Icon, Size};
use leptos::ev;
use leptos::prelude::*;

/// Inline control rendered as a native link.
#[component]
pub fn Link(
    /// Content rendered inside the link control.
    children: Children,
    /// Link destination for native anchor behavior.
    #[prop(optional, into)]
    href: Option<String>,
    /// Optional native anchor target.
    #[prop(optional, into)]
    target: Option<String>,
    /// Optional native anchor rel attribute.
    #[prop(optional, into)]
    rel: Option<String>,
    /// Optional Lucide icon name rendered before the content.
    #[prop(optional, into)]
    icon: MaybeProp<IcnName>,
    /// Disables the link and prevents user interaction.
    #[prop(optional)]
    disabled: bool,
    /// Additional CSS class names applied to the root element.
    #[prop(optional, into)]
    class: Option<String>,
    /// Explicit tab order override. Use `-1` to remove the link from tab navigation.
    #[prop(optional)]
    tabindex: Option<i32>,
    /// Click handler for the underlying anchor element.
    #[prop(optional, into)]
    on_click: Option<ArcOneCallback<ev::MouseEvent>>,
) -> impl IntoView {
    let mut classes = vec!["birei-link"];
    if disabled {
        classes.push("birei-link--disabled");
    }
    if let Some(class) = class.as_deref() {
        classes.push(class);
    }

    let class_name = classes.join(" ");
    let href = href
        .filter(|href| !href.is_empty())
        .unwrap_or_else(|| String::from("#"));
    let tabindex = tabindex
        .unwrap_or(if disabled { -1 } else { 0 })
        .to_string();

    let handle_click = move |event: ev::MouseEvent| {
        if disabled {
            event.prevent_default();
            return;
        }

        if let Some(on_click) = on_click.as_ref() {
            on_click.run(event);
        }
    };

    view! {
        <a
            href=href
            target=target
            rel=rel
            class=class_name
            aria-disabled=if disabled { "true" } else { "false" }
            tabindex=tabindex
            on:click=handle_click
        >
            <span class="birei-link__inner">
                {move || {
                    icon.get()
                        .filter(|icon| !icon.is_empty())
                        .map(|icon| view! { <Icon name=icon size=Size::Small /> })
                }}
                <span class="birei-link__content">{children()}</span>
            </span>
        </a>
    }
}
