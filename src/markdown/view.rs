use std::rc::Rc;

use leptos::html;
use leptos::prelude::*;

use crate::common::FloatingPopupLayout;
use crate::{ButtonBarItem, Icon, Size};

use super::menu::{
    heading_menu_items, menu_popup_class_name, table_action_from_value, table_menu_items,
};

pub(crate) struct ToolbarViewProps {
    pub(crate) toolbar_buttons: Vec<ButtonBarItem>,
    pub(crate) toolbar_button_class: String,
    pub(crate) heading_button_ref: NodeRef<html::Button>,
    pub(crate) table_button_ref: NodeRef<html::Button>,
    pub(crate) heading_popup_open: RwSignal<bool>,
    pub(crate) table_button_is_menu: RwSignal<bool>,
    pub(crate) disabled: bool,
    pub(crate) readonly: bool,
    pub(crate) handle_toolbar_action: Rc<dyn Fn(String)>,
}

pub(crate) fn render_toolbar_view(props: ToolbarViewProps) -> AnyView {
    let ToolbarViewProps {
        toolbar_buttons,
        toolbar_button_class,
        heading_button_ref,
        table_button_ref,
        heading_popup_open,
        table_button_is_menu,
        disabled,
        readonly,
        handle_toolbar_action,
    } = props;

    toolbar_buttons
        .into_iter()
        .map(|item| {
            let value = item.value.clone();
            let label = item.label.clone();
            let icon = item.icon.clone();
            let handle_toolbar_action = Rc::clone(&handle_toolbar_action);

            if value == "heading" {
                let heading_button_class = toolbar_button_class.clone();
                return view! {
                    <button
                        type="button"
                        node_ref=heading_button_ref
                        class=heading_button_class.clone() + " birei-dropdown-button__trigger"
                        aria-expanded=move || if heading_popup_open.get() { "true" } else { "false" }
                        disabled=disabled || readonly || item.disabled
                        on:click=move |_| handle_toolbar_action(value.clone())
                    >
                        {icon.clone().map(|icon_name| view! { <Icon name=icon_name size=Size::Small/> })}
                        <span>{label.clone()}</span>
                        <span class="birei-dropdown-button__divider" aria-hidden="true"></span>
                        <span class="birei-dropdown-button__caret" aria-hidden="true">
                            <Icon name="chevron-down" size=Size::Small/>
                        </span>
                    </button>
                }
                .into_any();
            }

            if value == "table" {
                let table_button_class = toolbar_button_class.clone();
                return view! {
                    <button
                        type="button"
                        node_ref=table_button_ref
                        class=move || {
                            let mut classes = table_button_class.clone();
                            if table_button_is_menu.get() {
                                classes.push_str(" birei-dropdown-button__trigger");
                            }
                            classes
                        }
                        disabled=disabled || readonly || item.disabled
                        on:click=move |_| handle_toolbar_action(value.clone())
                    >
                        {icon.clone().map(|icon_name| view! { <Icon name=icon_name size=Size::Small/> })}
                        <span>{label.clone()}</span>
                        <Show when=move || table_button_is_menu.get()>
                            <span class="birei-dropdown-button__divider" aria-hidden="true"></span>
                            <span class="birei-dropdown-button__caret" aria-hidden="true">
                                <Icon name="chevron-down" size=Size::Small/>
                            </span>
                        </Show>
                    </button>
                }
                .into_any();
            }

            let button = view! {
                <button
                    type="button"
                    class=toolbar_button_class.clone()
                    disabled=disabled || readonly || item.disabled
                    on:click=move |_| handle_toolbar_action(value.clone())
                >
                    {icon.map(|icon_name| view! { <Icon name=icon_name size=Size::Small/> })}
                    <span>{label}</span>
                </button>
            };
            button.into_any()
        })
        .collect_view()
        .into_any()
}

pub(crate) fn render_heading_popup(
    heading_popup_ref: NodeRef<html::Div>,
    heading_popup_open: RwSignal<bool>,
    heading_popup_layout: RwSignal<FloatingPopupLayout>,
    handle_toolbar_action: Rc<dyn Fn(String)>,
) -> AnyView {
    view! {
        <div
            node_ref=heading_popup_ref
            class=move || menu_popup_class_name("birei-markdown__menu-popup", heading_popup_layout.get().open_upward)
            style=move || {
                let layout = heading_popup_layout.get();
                if heading_popup_open.get() {
                    format!(
                        "left: {}px; top: {}px; max-height: {}px;",
                        layout.left, layout.top, layout.max_height
                    )
                } else {
                    String::from("display: none;")
                }
            }
            on:mousedown=move |event| event.stop_propagation()
        >
            {heading_menu_items()
                .into_iter()
                .map(|item| {
                    let value = item.value.clone();
                    let label = item.label.clone();
                    let icon = item.icon.clone();
                    let handle_toolbar_action = Rc::clone(&handle_toolbar_action);

                    view! {
                        <button
                            type="button"
                            class="birei-dropdown-button__item"
                            on:mousedown=move |event| event.prevent_default()
                            on:click=move |_| handle_toolbar_action(value.clone())
                        >
                            <span class="birei-dropdown-button__item-content">
                                {icon.map(|icon_name| view! { <Icon name=icon_name size=Size::Small/> })}
                                <span>{label}</span>
                            </span>
                        </button>
                    }
                })
                .collect_view()}
        </div>
    }
    .into_any()
}

pub(crate) fn render_table_popup(
    table_popup_ref: NodeRef<html::Div>,
    table_popup_open: RwSignal<bool>,
    table_popup_layout: RwSignal<FloatingPopupLayout>,
    handle_table_action: Rc<dyn Fn(&'static str)>,
) -> AnyView {
    view! {
        <div
            node_ref=table_popup_ref
            class=move || menu_popup_class_name("birei-markdown__menu-popup", table_popup_layout.get().open_upward)
            style=move || {
                let layout = table_popup_layout.get();
                if table_popup_open.get() {
                    format!(
                        "left: {}px; top: {}px; max-height: {}px;",
                        layout.left, layout.top, layout.max_height
                    )
                } else {
                    String::from("display: none;")
                }
            }
            on:mousedown=move |event| event.stop_propagation()
        >
            {table_menu_items()
                .into_iter()
                .map(|item| {
                    let value = item.value.clone();
                    let label = item.label.clone();
                    let icon = item.icon.clone();
                    let handle_table_action = Rc::clone(&handle_table_action);

                    view! {
                        <button
                            type="button"
                            class="birei-dropdown-button__item"
                            on:mousedown=move |event| event.prevent_default()
                            on:click=move |_| handle_table_action(table_action_from_value(&value))
                        >
                            <span class="birei-dropdown-button__item-content">
                                {icon.map(|icon_name| view! { <Icon name=icon_name size=Size::Small/> })}
                                <span>{label}</span>
                            </span>
                        </button>
                    }
                })
                .collect_view()}
        </div>
    }
    .into_any()
}

pub(crate) fn render_link_popup(
    link_popup_open: RwSignal<bool>,
    link_popup_layout: RwSignal<FloatingPopupLayout>,
    link_input_ref: NodeRef<html::Input>,
    link_url: RwSignal<String>,
    apply_link: Rc<dyn Fn()>,
    close_link_popup: Rc<dyn Fn()>,
) -> AnyView {
    view! {
        <div
            class=move || {
                let layout = link_popup_layout.get();
                let mut classes = String::from("birei-markdown__link-popup");
                if layout.open_upward {
                    classes.push_str(" birei-markdown__link-popup--upward");
                }
                classes
            }
            style=move || {
                let layout = link_popup_layout.get();
                if link_popup_open.get() {
                    format!("left: {}px; top: {}px;", layout.left, layout.top)
                } else {
                    String::from("display: none;")
                }
            }
            on:mousedown=move |event| event.stop_propagation()
        >
            <label class="birei-markdown__link-label" for="birei-markdown-link-input">
                "Link URL"
            </label>
            <input
                node_ref=link_input_ref
                id="birei-markdown-link-input"
                class="birei-markdown__link-input"
                type="url"
                prop:value=move || link_url.get()
                placeholder="https://example.com"
                on:input=move |event| link_url.set(event_target_value(&event))
                on:keydown={
                    let apply_link = Rc::clone(&apply_link);
                    let close_link_popup = Rc::clone(&close_link_popup);
                    move |event| match event.key().as_str() {
                        "Enter" => {
                            event.prevent_default();
                            apply_link();
                        }
                        "Escape" => {
                            event.prevent_default();
                            close_link_popup();
                        }
                        _ => {}
                    }
                }
            />
            <div class="birei-markdown__link-actions">
                <button
                    type="button"
                    class="birei-button birei-button--transparent birei-button--small"
                    on:click={
                        let close_link_popup = Rc::clone(&close_link_popup);
                        move |_| close_link_popup()
                    }
                >
                    "Cancel"
                </button>
                <button
                    type="button"
                    class="birei-button birei-button--secondary birei-button--small"
                    on:click={
                        let apply_link = Rc::clone(&apply_link);
                        move |_| apply_link()
                    }
                >
                    "Apply"
                </button>
            </div>
        </div>
    }
    .into_any()
}
