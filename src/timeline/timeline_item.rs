use leptos::prelude::*;

use crate::{Card, IcnName, Icon};

/// One card-backed entry inside a [`Timeline`](super::Timeline).
#[component]
pub fn TimelineItem(
    children: Children,
    #[prop(optional, into)] icon: Option<IcnName>,
    #[prop(optional, into)] name: Option<String>,
    #[prop(optional, into)] title: Option<String>,
    #[prop(optional, into)] subtitle: Option<String>,
    #[prop(optional)] on_name_click: Option<Callback<()>>,
    #[prop(optional, into)] class: Option<String>,
) -> impl IntoView {
    let class_name = move || {
        let mut classes = vec!["birei-timeline__item"];
        if let Some(class) = class.as_deref() {
            classes.push(class);
        }
        classes.join(" ")
    };
    let icon_name = icon.unwrap_or_else(|| IcnName::from("circle"));
    let clickable_name = on_name_click.is_some() && name.is_some();

    view! {
        <article class=class_name>
            <div class="birei-timeline__rail" aria-hidden="true">
                <div class="birei-timeline__icon-shell">
                    <Icon name=icon_name label="Timeline entry icon"/>
                </div>
            </div>
            <Card class="birei-timeline__card">
                <div class="birei-timeline__header">
                    <div class="birei-timeline__headline">
                        {name.as_ref().map(|name| {
                            let name = name.clone();

                            if clickable_name {
                                view! {
                                    <button
                                        type="button"
                                        class="birei-timeline__name birei-timeline__name--clickable"
                                        on:click=move |_| {
                                            if let Some(on_name_click) = on_name_click.as_ref() {
                                                on_name_click.run(());
                                            }
                                        }
                                    >
                                        {name}
                                    </button>
                                }
                                    .into_any()
                            } else {
                                view! {
                                    <span class="birei-timeline__name">{name}</span>
                                }
                                    .into_any()
                            }
                        })}
                        {title.as_ref().map(|title| {
                            view! {
                                <span class="birei-timeline__title">{title.clone()}</span>
                            }
                        })}
                    </div>
                    {subtitle.as_ref().map(|subtitle| {
                        view! {
                            <div class="birei-timeline__subtitle">{subtitle.clone()}</div>
                        }
                    })}
                </div>
                <div class="birei-timeline__content">
                    {children()}
                </div>
            </Card>
        </article>
    }
}
