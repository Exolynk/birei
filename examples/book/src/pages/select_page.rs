use birei::{Card, Label, Select, SelectOption, Size};
use leptos::prelude::*;

#[component]
pub fn SelectPage() -> impl IntoView {
    let role = RwSignal::new(Some(String::from("designer")));
    let timezone = RwSignal::new(Some(String::from("europe-zurich")));
    let status = RwSignal::new(Some(String::from("active")));
    let tags = RwSignal::new(vec![String::from("ui"), String::from("rust")]);

    let role_options = vec![
        SelectOption::new("designer", "Product designer"),
        SelectOption::new("engineer", "Frontend engineer"),
        SelectOption::new("writer", "Technical writer"),
        SelectOption::new("producer", "Launch producer"),
    ];
    let timezone_options = vec![
        SelectOption::new("europe-zurich", "Europe / Zurich"),
        SelectOption::new("asia-tokyo", "Asia / Tokyo"),
        SelectOption::new("america-new-york", "America / New York"),
        SelectOption::new("australia-sydney", "Australia / Sydney"),
    ];
    let status_options = vec![
        SelectOption::new("active", "Active").icon("badge-check"),
        SelectOption::new("paused", "Paused").icon("pause"),
        SelectOption::new("invited", "Invited").icon("mail-plus"),
        SelectOption::new("archived", "Archived")
            .icon("archive")
            .disabled(true),
    ];
    let tag_options = vec![
        SelectOption::new("ui", "UI systems").icon("palette"),
        SelectOption::new("rust", "Rust").icon("code"),
        SelectOption::new("docs", "Documentation").icon("book-open"),
        SelectOption::new("launch", "Launch planning").icon("rocket"),
        SelectOption::new("research", "Research").icon("search"),
    ];
    let long_options = (1..=100)
        .map(|index| {
            SelectOption::new(
                format!("entry-{index:03}"),
                format!("Long example entry {index:03}"),
            )
        })
        .collect::<Vec<_>>();
    let long_value = RwSignal::new(Some(String::from("entry-042")));
    let role_options_for_basics = role_options.clone();
    let role_options_for_sizes_small = role_options.clone();
    let role_options_for_sizes_medium = role_options.clone();
    let role_options_for_sizes_large = role_options.clone();
    let status_options_for_nullable = status_options.clone();
    let status_options_for_state_readonly = status_options.clone();
    let status_options_for_state_disabled = status_options.clone();
    let status_options_for_state_invalid = status_options.clone();

    let selected_tags = move || {
        let selected = tags.get();
        if selected.is_empty() {
            String::from("None")
        } else {
            selected.join(", ")
        }
    };

    view! {
        <section class="page-header">
            <div class="page-header__eyebrow">"Component"</div>
            <h2>"Select"</h2>
            <p class="page-header__lede">
                "Searchable select controls with the same sizing, underline treatment, clear affordance, and popup list selection for single or multiple values."
            </p>
        </section>

        <section class="doc-grid">
            <Card header="Single selection" class="doc-card">
                <span class="doc-card__kicker">"Basics"</span>
                <div class="doc-card__preview doc-card__preview--stack">
                    <div class="field">
                        <Label text="Role" required=true for_id="book-select-role"/>
                        <Select
                            id="book-select-role"
                            options=role_options_for_basics.clone()
                            value=role
                            required=true
                            placeholder="Search roles"
                            on_value_change=Callback::new(move |next| role.set(next))
                        />
                    </div>
                    <Select
                        options=timezone_options.clone()
                        value=timezone
                        nullable=true
                        placeholder="No timezone yet"
                        on_value_change=Callback::new(move |next| timezone.set(next))
                    />
                    <p class="doc-card__copy">
                        "Role preview: "
                        <strong>{move || role.get().unwrap_or_else(|| String::from("None"))}</strong>
                    </p>
                </div>
                <pre class="doc-card__code"><code>{r#"<Select
    id="role"
    options=role_options.clone()
    value=role
    required=true
    placeholder="Search roles"
    on_value_change=Callback::new(move |next| role.set(next))
/>
<Select
    options=timezone_options.clone()
    value=timezone
    nullable=true
    placeholder="No timezone yet"
    on_value_change=Callback::new(move |next| timezone.set(next))
/>"#}</code></pre>
            </Card>

            <Card header="Shared with buttons and inputs" class="doc-card">
                <span class="doc-card__kicker">"Sizes"</span>
                <div class="doc-card__preview doc-card__preview--stack">
                    <Select options=role_options_for_sizes_small.clone() size=Size::Small nullable=true placeholder="Small select"/>
                    <Select options=role_options_for_sizes_medium.clone() size=Size::Medium nullable=true placeholder="Medium select"/>
                    <Select options=role_options_for_sizes_large.clone() size=Size::Large nullable=true placeholder="Large select"/>
                </div>
                <pre class="doc-card__code"><code>{r#"<Select options=role_options.clone() size=Size::Small/>
<Select options=role_options.clone() size=Size::Medium/>
<Select options=role_options.clone() size=Size::Large/>"#}</code></pre>
            </Card>

            <Card header="Allow clearing and show icons" class="doc-card">
                <span class="doc-card__kicker">"Nullable"</span>
                <div class="doc-card__preview doc-card__preview--stack">
                    <div class="field">
                        <Label text="Member status" for_id="book-select-member-status"/>
                        <Select
                            id="book-select-member-status"
                            options=status_options_for_nullable.clone()
                            value=status
                            nullable=true
                            placeholder="Select status"
                            on_value_change=Callback::new(move |next| status.set(next))
                        />
                    </div>
                    <p class="doc-card__copy">
                        "Current status: "
                        <strong>{move || status.get().unwrap_or_else(|| String::from("None"))}</strong>
                    </p>
                </div>
                <pre class="doc-card__code"><code>{r#"<Select
    options=status_options.clone()
    value=status
    nullable=true
    placeholder="Select status"
    on_value_change=Callback::new(move |next| status.set(next))
/>"#}</code></pre>
            </Card>

            <Card header="Popup list with filtering" class="doc-card">
                <span class="doc-card__kicker">"Multiple"</span>
                <div class="doc-card__preview doc-card__preview--stack">
                    <div class="field">
                        <Label text="Topics" for_id="book-select-topics"/>
                        <Select
                            id="book-select-topics"
                            options=tag_options.clone()
                            values=tags
                            multiple=true
                            nullable=true
                            placeholder="Filter topics"
                            on_values_change=Callback::new(move |next| tags.set(next))
                        />
                    </div>
                    <p class="doc-card__copy">
                        "Selected tags: "
                        <strong>{selected_tags}</strong>
                    </p>
                </div>
                <pre class="doc-card__code"><code>{r#"<Select
    id="topics"
    options=tag_options.clone()
    values=tags
    multiple=true
    nullable=true
    placeholder="Filter topics"
    on_values_change=Callback::new(move |next| tags.set(next))
/>"#}</code></pre>
            </Card>

            <Card header="Long lists stay inside the popup" class="doc-card">
                <span class="doc-card__kicker">"Scrolling"</span>
                <div class="doc-card__preview doc-card__preview--stack">
                    <div class="field">
                        <Label text="Long example" for_id="book-select-long-example"/>
                        <Select
                            id="book-select-long-example"
                            options=long_options.clone()
                            value=long_value
                            placeholder="Filter 100 entries"
                            nullable=true
                            on_value_change=Callback::new(move |next| long_value.set(next))
                        />
                    </div>
                    <p class="doc-card__copy">
                        "Selected long entry: "
                        <strong>{move || long_value.get().unwrap_or_else(|| String::from("None"))}</strong>
                    </p>
                </div>
                <pre class="doc-card__code"><code>{r#"<Select
    id="long-example"
    options=long_options.clone()
    value=long_value
    placeholder="Filter 100 entries"
    nullable=true
    on_value_change=Callback::new(move |next| long_value.set(next))
/>"#}</code></pre>
            </Card>

            <Card header="Readonly, disabled, invalid" class="doc-card">
                <span class="doc-card__kicker">"State"</span>
                <div class="doc-card__preview doc-card__preview--stack">
                    <Select
                        options=status_options_for_state_readonly.clone()
                        value=Some(String::from("paused"))
                        readonly=true
                    />
                    <Select options=status_options_for_state_disabled.clone() placeholder="Disabled select" disabled=true nullable=true/>
                    <Select
                        options=status_options_for_state_invalid.clone()
                        value=Some(String::from("archived"))
                        invalid=true
                    />
                </div>
                <pre class="doc-card__code"><code>{r#"<Select options=status_options.clone() value=Some(String::from("paused")) readonly=true/>
<Select options=status_options.clone() disabled=true nullable=true/>
<Select options=status_options value=Some(String::from("archived")) invalid=true/>"#}</code></pre>
            </Card>
        </section>
    }
}
