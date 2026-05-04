use leptos::ev;
use leptos::html;
use leptos::prelude::*;
use wasm_bindgen::JsCast;
use web_sys::{File, FileList, HtmlElement, HtmlInputElement};

use crate::{IcnName, Icon, Size};

/// Action-card styled file upload target with click and drag/drop support.
#[component]
pub fn ActionCardUpload(
    /// Primary label shown beneath the upload icon.
    #[prop(into)]
    title: String,
    /// Secondary text shown beneath the title.
    #[prop(into)]
    subtitle: String,
    /// Icon shown in the card hero area.
    #[prop(optional, into)]
    icon: Option<IcnName>,
    /// Native file accept filter, for example `image/*` or `.pdf`.
    #[prop(optional, into)]
    accept: Option<String>,
    /// Allows selecting or dropping more than one file.
    #[prop(optional)]
    multiple: bool,
    /// Disables click and drag/drop interaction.
    #[prop(optional)]
    disabled: bool,
    /// Additional class names applied to the card button.
    #[prop(optional, into)]
    class: Option<String>,
    /// Called whenever files are selected or dropped.
    #[prop(optional)]
    on_files: Option<Callback<Vec<File>>>,
) -> impl IntoView {
    let input_ref = NodeRef::<html::Input>::new();
    let is_dragging = RwSignal::new(false);
    let ripple_style = RwSignal::new(String::from(
        "--birei-ripple-x: 50%; --birei-ripple-y: 50%; --birei-ripple-size: 0px;",
    ));
    let ripple_phase = RwSignal::new(None::<bool>);

    let emit_files = move |files: FileList| {
        let files = files_from_file_list(files, multiple);
        if files.is_empty() {
            return;
        }

        if let Some(on_files) = on_files.as_ref() {
            on_files.run(files);
        }
    };

    let class_name = move || {
        let mut classes = vec![
            "birei-action-card",
            "birei-action-card--interactive",
            "birei-action-card--icon",
            "birei-action-card-upload__card",
        ];

        if is_dragging.get() {
            classes.push("birei-action-card-upload__card--dragging");
        }
        if disabled {
            classes.push("birei-action-card-upload__card--disabled");
        }
        if let Some(class) = class.as_deref() {
            classes.push(class);
        }

        let mut classes = classes.join(" ");
        if let Some(phase) = ripple_phase.get() {
            classes.push(' ');
            classes.push_str(if phase {
                "birei-action-card--ripple-a"
            } else {
                "birei-action-card--ripple-b"
            });
        }

        classes
    };

    let open_file_dialog = move || {
        if disabled {
            return;
        }

        if let Some(input) = input_ref.get_untracked() {
            input.set_value("");
            if let Ok(element) = input.dyn_into::<HtmlElement>() {
                element.click();
            }
        }
    };

    let handle_click = move |event: ev::MouseEvent| {
        if disabled {
            return;
        }

        if let Some(target) = event
            .current_target()
            .and_then(|target| target.dyn_into::<HtmlElement>().ok())
        {
            let rect = target.get_bounding_client_rect();
            let x = f64::from(event.client_x()) - rect.left();
            let y = f64::from(event.client_y()) - rect.top();
            let size = rect.width().max(rect.height()) * 1.35;

            ripple_style.set(format!(
                "--birei-ripple-x: {x}px; --birei-ripple-y: {y}px; --birei-ripple-size: {size}px;"
            ));
            ripple_phase.update(|phase| {
                *phase = Some(!phase.unwrap_or(false));
            });
        }

        open_file_dialog();
    };

    let handle_input = move |event: ev::Event| {
        if disabled {
            return;
        }

        if let Some(files) = event_target::<HtmlInputElement>(&event).files() {
            emit_files(files);
        }
    };

    let handle_drag_enter = move |event: ev::DragEvent| {
        if disabled {
            return;
        }

        event.prevent_default();
        is_dragging.set(true);
    };

    let handle_drag_over = move |event: ev::DragEvent| {
        if disabled {
            return;
        }

        event.prevent_default();
    };

    let handle_drag_leave = move |event: ev::DragEvent| {
        if disabled {
            return;
        }

        event.prevent_default();
        is_dragging.set(false);
    };

    let handle_drop = move |event: ev::DragEvent| {
        if disabled {
            return;
        }

        event.prevent_default();
        is_dragging.set(false);

        let Some(files) = event.data_transfer().and_then(|data| data.files()) else {
            return;
        };

        emit_files(files);
    };

    let icon = icon.unwrap_or_else(|| "upload".into());

    view! {
        <div class="birei-action-card-upload">
            <button
                type="button"
                class=class_name
                style=move || ripple_style.get()
                disabled=disabled
                on:click=handle_click
                on:dragenter=handle_drag_enter
                on:dragover=handle_drag_over
                on:dragleave=handle_drag_leave
                on:drop=handle_drop
            >
                <div class="birei-action-card__hero" aria-hidden="true">
                    <span class="birei-action-card__icon">
                        <Icon name=icon size=Size::Large/>
                    </span>
                </div>
                <div class="birei-action-card__copy">
                    <div class="birei-action-card__title">{title}</div>
                    <div class="birei-action-card__subtitle">{subtitle}</div>
                </div>
            </button>
            <input
                node_ref=input_ref
                class="birei-action-card-upload__input"
                type="file"
                accept=accept
                multiple=multiple
                disabled=disabled
                tabindex="-1"
                on:change=handle_input
            />
        </div>
    }
}

fn files_from_file_list(files: FileList, multiple: bool) -> Vec<File> {
    let limit = if multiple { files.length() } else { files.length().min(1) };

    (0..limit)
        .filter_map(|index| files.get(index))
        .collect::<Vec<_>>()
}
