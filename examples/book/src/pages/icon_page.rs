use birei::{icon, Card, Icon, Input, InputType, Size};
use leptos::ev;
use leptos::prelude::*;
use web_sys::HtmlInputElement;

#[component]
pub fn IconPage() -> impl IntoView {
    let filter = RwSignal::new(String::new());
    let filtered_icons = Memo::new(move |_| {
        let query = filter.get().trim().to_ascii_lowercase();

        icon::ICON_NAMES
            .iter()
            .enumerate()
            .filter(|(_, name)| query.is_empty() || name.to_ascii_lowercase().contains(&query))
            .map(|(index, name)| (index, *name))
            .collect::<Vec<_>>()
    });

    view! {
        <section class="page-header">
            <div class="page-header__eyebrow">"Component"</div>
            <h2>"Icon"</h2>
            <p class="page-header__lede">
                "Lucide font icons wrapped in a typed component with shared sizing tokens and accessible labels."
            </p>
        </section>

        <section class="doc-grid">
            <Card header="String names or generated indices" class="doc-card">
                <span class="doc-card__kicker">"Basics"</span>
                <div class="doc-card__preview doc-card__preview--stack">
                    <div class="book-icon-demo-row">
                        <Icon name="search" label="Search"/>
                        <code>"name=\"search\""</code>
                    </div>
                    <div class="book-icon-demo-row">
                        <Icon name=icon::MAIL label="Mail"/>
                        <code>"name=icon::MAIL"</code>
                    </div>
                    <div class="book-icon-demo-row">
                        <Icon name="arrow-right" label="Arrow right"/>
                        <code>"name=\"arrow-right\""</code>
                    </div>
                    <div class="book-icon-demo-row">
                        <Icon name=icon::SETTINGS_2 label="Settings"/>
                        <code>"name=icon::SETTINGS_2"</code>
                    </div>
                </div>
                <pre class="doc-card__code"><code>{r#"<Icon name="search" label="Search"/>
<Icon name=icon::MAIL label="Mail"/>
<Icon name="arrow-right" label="Arrow right"/>
<Icon name=icon::SETTINGS_2 label="Settings"/>"#}</code></pre>
            </Card>

            <Card header="Aligned with the component scale" class="doc-card">
                <span class="doc-card__kicker">"Sizes"</span>
                <div class="doc-card__preview">
                    <div class="book-icon-demo-grid">
                        <div class="book-icon-demo-tile">
                            <Icon name="star" size=Size::Small label="Star small"/>
                            <span>"Small"</span>
                        </div>
                        <div class="book-icon-demo-tile">
                            <Icon name="star" size=Size::Medium label="Star medium"/>
                            <span>"Medium"</span>
                        </div>
                        <div class="book-icon-demo-tile">
                            <Icon name="star" size=Size::Large label="Star large"/>
                            <span>"Large"</span>
                        </div>
                    </div>
                </div>
                <pre class="doc-card__code"><code>{r#"<Icon name="star" size=Size::Small label="Star small"/>
<Icon name="star" size=Size::Medium label="Star medium"/>
<Icon name="star" size=Size::Large label="Star large"/>"#}</code></pre>
            </Card>

            <Card header="Readable in different contexts" class="doc-card">
                <span class="doc-card__kicker">"Contrast"</span>
                <div class="doc-card__preview">
                    <div class="book-icon-demo-grid">
                        <div class="book-icon-demo-tile">
                            <Icon name="flame" size=Size::Large label="Flame"/>
                            <span>"Default"</span>
                        </div>
                        <div class="book-icon-demo-tile">
                            <Icon name="leaf" size=Size::Large label="Leaf"/>
                            <span>"Default"</span>
                        </div>
                    </div>
                </div>
                <pre class="doc-card__code"><code>{r#"<Icon name="flame" size=Size::Large label="Flame"/>
    <Icon name="leaf" size=Size::Large label="Leaf"/>"#}</code></pre>
            </Card>

            <Card header="Generated from icon::ICON_NAMES" class="doc-card">
                <span class="doc-card__kicker">"Catalog"</span>
                <div class="book-icon-catalog">
                    <div class="book-icon-catalog__toolbar">
                        <Input
                            placeholder="Filter icons by name"
                            input_type=InputType::Search
                            prefix=|| view! { <Icon name="search" label="Search icons"/> }
                            on_input=Callback::new(move |event: ev::Event| {
                                filter.set(event_target::<HtmlInputElement>(&event).value());
                            })
                        />
                        <p class="doc-card__copy book-icon-catalog__count">
                            {move || format!("Showing {} icons", filtered_icons.get().len())}
                        </p>
                    </div>
                    <div class="book-icon-catalog__grid">
                        <For
                            each=move || filtered_icons.get()
                            key=|(index, _)| *index
                            children=move |(index, name)| {
                                view! {
                                    <div class="book-icon-catalog__item">
                                        <Icon name=index size=Size::Large label=name/>
                                        <code class="book-icon-catalog__name">{name}</code>
                                    </div>
                                }
                            }
                        />
                    </div>
                </div>
            </Card>
        </section>
    }
}
