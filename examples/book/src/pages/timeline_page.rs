use crate::code_example::CodeExample;
use birei::{NotificationManager, Tag, Timeline, TimelineItem};
use leptos::prelude::*;

#[component]
pub fn TimelinePage() -> impl IntoView {
    view! {
        <section class="page-header">
            <div class="page-header__eyebrow">"Component"</div>
            <h2>"Timeline"</h2>
            <p class="page-header__lede">
                "Vertical event chronology with connected icons, card-backed entries, and optional clickable names."
            </p>
        </section>

        <section class="doc-grid">
            <birei::Card header="Event stream" class="doc-card">
                <span class="doc-card__kicker">"Mixed entries"</span>
                <Timeline>
                    <TimelineItem
                        icon="rocket"
                        name="Release"
                        title="Version 1.4 shipped"
                        subtitle="Today · 09:12"
                    >
                        <p>
                            "The new notification manager, sign pad, and timeline components are now available in the shared library."
                        </p>
                        <div class="book-timeline-tags">
                            <Tag>"components"</Tag>
                            <Tag>"release"</Tag>
                            <Tag>"ui"</Tag>
                        </div>
                    </TimelineItem>

                    <TimelineItem
                        icon="message-square-text"
                        name="Designer"
                        title="Mina approved the final review"
                        subtitle="Yesterday · 16:40"
                        on_name_click=Callback::new(move |_| {
                            NotificationManager::global().info("Clicked timeline author name.");
                        })
                    >
                        <p>
                            "Spacing was tightened around the metadata row and the connector line now terminates cleanly on the last item."
                        </p>
                    </TimelineItem>

                    <TimelineItem
                        icon="package-check"
                        name="Ops"
                        title="Deployment completed"
                        subtitle="Yesterday · 14:05"
                    >
                        <p>
                            "Production rollout finished without manual intervention. Monitoring stayed within the normal baseline for the entire publish window."
                        </p>
                    </TimelineItem>
                </Timeline>
                <CodeExample code={r#"<Timeline>
    <TimelineItem
        icon="rocket"
        name="Release"
        title="Version 1.4 shipped"
        subtitle="Today · 09:12"
    >
        <p>"Release notes go here."</p>
    </TimelineItem>

    <TimelineItem
        icon="message-square-text"
        name="Designer"
        title="Mina approved the final review"
        on_name_click=Callback::new(move |_| {
            NotificationManager::global().info("Clicked timeline author name.");
        })
    >
        <p>"Feedback summary."</p>
    </TimelineItem>
</Timeline>"#}/>
            </birei::Card>
        </section>
    }
}
