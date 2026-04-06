use crate::code_example::CodeExample;
use birei::{Card, List, ListDensity, ListEntry};
use js_sys::Promise;
use leptos::prelude::*;
use leptos::task::spawn_local;
use wasm_bindgen::prelude::Closure;
use wasm_bindgen::JsCast;
use wasm_bindgen_futures::JsFuture;

const LONG_LIST_BATCH_SIZE: usize = 100;
const LONG_LIST_LIMIT: usize = 10_000;

#[component]
pub fn ListPage() -> impl IntoView {
    let compact_entries = (1..=24)
        .map(|index| {
            ListEntry::new(
                format!("compact-{index:02}"),
                format!("Team member {index:02}"),
            )
            .icon("user-round")
            .meta(if index % 3 == 0 { "Admin" } else { "Member" })
        })
        .collect::<Vec<_>>();
    let detailed_entries = (1..=18)
        .map(|index| {
            ListEntry::new(format!("detail-{index:02}"), format!("Project {index:02}"))
                .description(format!(
                    "Updated {} hours ago with new design notes.",
                    index + 1
                ))
                .icon("folder-open")
                .meta(format!("v{}.{}", index / 3 + 1, index % 3))
        })
        .collect::<Vec<_>>();

    let infinite_entries = RwSignal::new(
        (1..=40)
            .map(|index| {
                ListEntry::new(
                    format!("long-{index:03}"),
                    format!("Activity row {index:03}"),
                )
                .description(format!(
                    "Generated row {index:03} for virtualization preview."
                ))
                .icon("clock-3")
                .meta("Queued")
            })
            .collect::<Vec<_>>(),
    );
    let compact_selected = RwSignal::new(Some(String::from("compact-03")));
    let detailed_selected = RwSignal::new(None::<String>);
    let long_selected = RwSignal::new(Some(String::from("long-00512")));
    let is_loading_more = RwSignal::new(false);
    let has_more = RwSignal::new(true);
    let load_more = move || {
        if is_loading_more.get_untracked() || !has_more.get_untracked() {
            return;
        }

        is_loading_more.set(true);
        spawn_local({
            let infinite_entries = infinite_entries;
            let is_loading_more = is_loading_more;
            let has_more = has_more;
            async move {
                let current_len = infinite_entries.get_untracked().len();
                let next_entries = fetch_more_entries(current_len, LONG_LIST_BATCH_SIZE).await;

                infinite_entries.update(|items| items.extend(next_entries));
                has_more.set(infinite_entries.get_untracked().len() < LONG_LIST_LIMIT);
                is_loading_more.set(false);
            }
        });
    };

    view! {
        <section class="page-header">
            <div class="page-header__eyebrow">"Component"</div>
            <h2>"List"</h2>
            <p class="page-header__lede">
                "Fixed-height virtualized lists with compact or detailed rows, owned scrolling, selection, keyboard navigation, and incremental loading."
            </p>
        </section>

        <section class="doc-grid">
            <Card class="doc-card">
                    <span class="doc-card__kicker">"Compact"</span>
                    <div class="doc-card__preview doc-card__preview--stack">
                        <div class="book-list-demo book-list-demo--compact">
                            <List
                                items=compact_entries
                                density=ListDensity::Compact
                                selected=compact_selected
                                on_selected_change=Callback::new(move |next| compact_selected.set(next))
                            />
                        </div>
                        <p class="doc-card__copy">
                            "Selected compact row: "
                            <strong>{move || compact_selected.get().unwrap_or_else(|| String::from("None"))}</strong>
                        </p>
                    </div>
                    <CodeExample code={r#"<List
    items=entries
    density=ListDensity::Compact
    selected=selected
    on_selected_change=Callback::new(move |next| selected.set(next))
/>"#}/>
            </Card>

            <Card class="doc-card">
                    <span class="doc-card__kicker">"Detailed"</span>
                    <div class="doc-card__preview doc-card__preview--stack">
                        <div class="book-list-demo book-list-demo--detailed">
                            <List
                                items=detailed_entries
                                density=ListDensity::Detailed
                                selected=detailed_selected
                                on_selected_change=Callback::new(move |next| detailed_selected.set(next))
                            />
                        </div>
                        <p class="doc-card__copy">
                            "Selected detailed row: "
                            <strong>{move || detailed_selected.get().unwrap_or_else(|| String::from("None"))}</strong>
                        </p>
                    </div>
                    <CodeExample code={r#"<List
    items=entries
    density=ListDensity::Detailed
    selected=selected
    on_selected_change=Callback::new(move |next| selected.set(next))
/>"#}/>
            </Card>

            <Card class="doc-card">
                    <span class="doc-card__kicker">"Infinite Scroll"</span>
                    <div class="doc-card__preview doc-card__preview--stack">
                        <div class="book-list-demo book-list-demo--infinite">
                            <List
                                items=infinite_entries
                                density=ListDensity::Detailed
                                selected=long_selected
                                has_more=has_more
                                is_loading=is_loading_more
                                on_selected_change=Callback::new(move |next| long_selected.set(next))
                                on_load_more=Callback::new(move |_| load_more())
                            />
                        </div>
                        <p class="doc-card__copy">
                            "Selected long row: "
                            <strong>{move || long_selected.get().unwrap_or_else(|| String::from("None"))}</strong>
                        </p>
                    </div>
                    <CodeExample code={r#"<List
    items=entries
    density=ListDensity::Detailed
    selected=selected
    has_more=has_more
    is_loading=is_loading
    on_selected_change=Callback::new(move |next| selected.set(next))
    on_load_more=Callback::new(move |_| load_more())
/>"#}/>
            </Card>
        </section>
    }
}

async fn fetch_more_entries(start_index: usize, batch_size: usize) -> Vec<ListEntry> {
    sleep_ms(500).await;

    let end_index = (start_index + batch_size).min(LONG_LIST_LIMIT);
    ((start_index + 1)..=end_index)
        .map(|index| {
            ListEntry::new(
                format!("long-{index:05}"),
                format!("Activity row {index:05}"),
            )
            .description(format!(
                "Generated row {index:05} for virtualization preview."
            ))
            .icon("clock-3")
            .meta("Queued")
        })
        .collect()
}

async fn sleep_ms(delay_ms: i32) {
    let promise = Promise::new(&mut |resolve, _reject| {
        let Some(window) = web_sys::window() else {
            let _ = resolve.call0(&wasm_bindgen::JsValue::NULL);
            return;
        };

        let callback = Closure::once_into_js(move || {
            let _ = resolve.call0(&wasm_bindgen::JsValue::NULL);
        });

        let _ = window.set_timeout_with_callback_and_timeout_and_arguments_0(
            callback.as_ref().unchecked_ref(),
            delay_ms,
        );
    });

    let _ = JsFuture::from(promise).await;
}
