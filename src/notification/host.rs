use std::sync::Arc;

use leptos::prelude::*;
use wasm_bindgen::closure::Closure;
use wasm_bindgen::JsCast;

use super::manager::NotificationManager;
use super::ntfn::Notification;
use super::state::{HostedNotification, NotificationBindings};

#[component]
pub(crate) fn NotificationHost(manager: NotificationManager) -> impl IntoView {
    let entries = RwSignal::new(Vec::<HostedNotification>::new());
    let hovered_count = RwSignal::new(0usize);

    let schedule_host_removal = move |id: usize| {
        let entries = entries;
        let Some(window) = web_sys::window() else {
            entries.update(|items| items.retain(|item| item.record.id != id));
            return;
        };

        let callback = Closure::once_into_js(move || {
            entries.update(|items| items.retain(|item| item.record.id != id));
        });
        let _ = window
            .set_timeout_with_callback_and_timeout_and_arguments_0(callback.unchecked_ref(), 220);
    };

    let schedule_exiting_removals = move || {
        if hovered_count.get_untracked() > 0 {
            return;
        }

        let exiting_ids = entries
            .get_untracked()
            .into_iter()
            .filter_map(|item| {
                if item.exiting.get_untracked() && !item.removal_scheduled.get_untracked() {
                    item.removal_scheduled.set(true);
                    Some(item.record.id)
                } else {
                    None
                }
            })
            .collect::<Vec<_>>();

        for id in exiting_ids {
            schedule_host_removal(id);
        }
    };

    manager.attach(NotificationBindings {
        add: Arc::new(move |notification| {
            entries.update(|items| {
                items.insert(
                    0,
                    HostedNotification {
                        record: notification,
                        exiting: RwSignal::new(false),
                        removal_scheduled: RwSignal::new(false),
                    },
                );
                if items.len() > 48 {
                    items.truncate(48);
                }
            });
        }),
        remove: Arc::new(move |id| {
            let mut should_schedule = false;
            entries.update(|items| {
                if let Some(item) = items.iter_mut().find(|item| item.record.id == id) {
                    if !item.exiting.get_untracked() {
                        item.exiting.set(true);
                        item.removal_scheduled.set(false);
                        should_schedule = hovered_count.get_untracked() == 0;
                    }
                }
            });
            if should_schedule {
                schedule_exiting_removals();
            }
        }),
        clear: Arc::new(move || {
            let mut should_schedule = false;
            entries.update(|items| {
                if items.iter().any(|item| !item.exiting.get_untracked()) {
                    should_schedule = hovered_count.get_untracked() == 0;
                }
                for item in items.iter_mut() {
                    item.exiting.set(true);
                    item.removal_scheduled.set(false);
                }
            });
            if should_schedule {
                schedule_exiting_removals();
            }
        }),
    });

    Effect::new(move |_| {
        if hovered_count.get() == 0 {
            schedule_exiting_removals();
        }
    });

    view! {
        <div class="birei-notification-stack" aria-live="polite" aria-atomic="false">
            <For
                each=move || entries.get()
                key=|entry| entry.record.id
                children=move |entry| {
                    view! {
                        <ManagedNotification
                            entry=entry
                            manager=manager.clone()
                            stack_paused=Signal::derive(move || hovered_count.get() > 0)
                            on_hover_change=Callback::new(move |hovered| {
                                hovered_count.update(|count| {
                                    if hovered {
                                        *count += 1;
                                    } else if *count > 0 {
                                        *count -= 1;
                                    }
                                });
                            })
                        />
                    }
                }
            />
        </div>
    }
}

#[component]
fn ManagedNotification(
    entry: HostedNotification,
    manager: NotificationManager,
    stack_paused: Signal<bool>,
    on_hover_change: Callback<bool>,
) -> impl IntoView {
    let remaining_ms = RwSignal::new(entry.record.duration_ms.max(0));
    let timeout_id = RwSignal::new(None::<i32>);
    let started_at = RwSignal::new(None::<f64>);
    let entered = RwSignal::new(false);

    let clear_timeout = Arc::new(move || {
        let Some(active_timeout_id) = timeout_id.get_untracked() else {
            return;
        };
        timeout_id.set(None);
        if let Some(window) = web_sys::window() {
            window.clear_timeout_with_handle(active_timeout_id);
        }
    });

    let dismiss = Arc::new({
        let clear_timeout = clear_timeout.clone();
        let manager = manager.clone();
        let id = entry.record.id;
        move || {
            if entry.exiting.get_untracked() {
                return;
            }

            clear_timeout();
            manager.remove(id);
        }
    });

    let schedule_timeout = Arc::new({
        let clear_timeout = clear_timeout.clone();
        let dismiss = dismiss.clone();
        move || {
            let dismiss_now = dismiss.clone();
            if entry.exiting.get_untracked() || remaining_ms.get_untracked() <= 0 {
                dismiss_now();
                return;
            }

            let Some(window) = web_sys::window() else {
                return;
            };

            clear_timeout();
            started_at.set(Some(js_sys::Date::now()));

            let callback = Closure::once_into_js(move || {
                dismiss_now();
            });
            if let Ok(timeout_id_value) = window
                .set_timeout_with_callback_and_timeout_and_arguments_0(
                    callback.unchecked_ref(),
                    remaining_ms.get_untracked(),
                )
            {
                timeout_id.set(Some(timeout_id_value));
            }
        }
    });

    let pause_timeout = Arc::new({
        let clear_timeout = clear_timeout.clone();
        move || {
            if entry.exiting.get_untracked() {
                return;
            }

            clear_timeout();
            if let Some(started_at_ms) = started_at.get_untracked() {
                let elapsed = (js_sys::Date::now() - started_at_ms).round() as i32;
                remaining_ms.update(|remaining| {
                    *remaining = (*remaining - elapsed).max(0);
                });
            }
            started_at.set(None);
        }
    });

    let toast_class = move || {
        let mut classes = vec!["birei-notification-toast"];
        if entered.get() && !entry.exiting.get() {
            classes.push("birei-notification-toast--entered");
        }
        if entry.exiting.get() {
            classes.push("birei-notification-toast--exiting");
        }
        classes.join(" ")
    };

    let pause_timeout_for_effect = pause_timeout.clone();
    let schedule_timeout_for_effect = schedule_timeout.clone();
    Effect::new(move |_| {
        if stack_paused.get() {
            pause_timeout_for_effect();
        } else if entered.get() && !entry.exiting.get_untracked() {
            schedule_timeout_for_effect();
        }
    });

    if let Some(window) = web_sys::window() {
        let callback = Closure::once_into_js({
            let schedule_timeout = schedule_timeout.clone();
            move || {
                entered.set(true);
                schedule_timeout();
            }
        });
        let _ = window.request_animation_frame(callback.unchecked_ref());
    } else {
        entered.set(true);
        schedule_timeout();
    }

    on_cleanup(move || clear_timeout());

    view! {
        <div
            class=toast_class
            on:pointerenter={
                let pause_timeout = pause_timeout.clone();
                move |_| {
                    pause_timeout();
                    on_hover_change.run(true);
                }
            }
            on:pointerleave={
                move |_| {
                    on_hover_change.run(false);
                }
            }
        >
            <Notification
                text=entry.record.text
                variant=entry.record.variant
                dismissible=true
                on_dismiss=Callback::new(move |_| dismiss())
            />
        </div>
    }
}
