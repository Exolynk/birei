use crate::code_example::CodeExample;
use birei::{Button, ButtonVariant, Card, Icon, Input, InputType, Label, Size};
use leptos::ev;
use leptos::prelude::*;
use web_sys::HtmlInputElement;

#[component]
pub fn InputPage() -> impl IntoView {
    let name = RwSignal::new(String::from("Aiko"));
    let email_address = RwSignal::new(String::from("aiko@birei.dev"));
    let password_value = RwSignal::new(String::from("atelier-2026"));
    let search = RwSignal::new(String::new());
    let phone_number = RwSignal::new(String::from("+41 44 555 01 23"));
    let website_url = RwSignal::new(String::from("https://birei.dev"));
    let email = RwSignal::new(String::new());
    let invite_code = RwSignal::new(String::from("TOKYO-24"));
    let newsletter_email = RwSignal::new(String::new());

    let update_signal = |signal: RwSignal<String>| {
        move |event: ev::Event| {
            signal.set(event_target::<HtmlInputElement>(&event).value());
        }
    };

    view! {
        <section class="page-header">
            <div class="page-header__eyebrow">"Component"</div>
            <h2>"Input"</h2>
            <p class="page-header__lede">
                "Text inputs with affix slots, animated focus line treatment, and sizes aligned to the button system."
            </p>
        </section>

        <section class="doc-grid">
            <Card header="Supported input types" class="doc-card">
                <span class="doc-card__kicker">"Basics"</span>
                <div class="doc-card__preview doc-card__preview--stack">
                    <div class="field">
                        <Label text="Display name" required=true for_id="book-input-display-name"/>
                        <Input
                            id="book-input-display-name"
                            value=name
                            required=true
                            placeholder="Display name"
                            on_input=Callback::new(update_signal(name))
                        />
                    </div>
                    <Input
                        value=email_address
                        input_type=InputType::Email
                        placeholder="name@birei.dev"
                        on_input=Callback::new(update_signal(email_address))
                    />
                    <Input
                        value=password_value
                        input_type=InputType::Password
                        placeholder="Password"
                        on_input=Callback::new(update_signal(password_value))
                    />
                    <Input
                        value=search
                        input_type=InputType::Search
                        placeholder="Search components"
                        on_input=Callback::new(update_signal(search))
                    />
                    <Input
                        value=phone_number
                        input_type=InputType::Tel
                        placeholder="+41 44 555 01 23"
                        on_input=Callback::new(update_signal(phone_number))
                    />
                    <Input
                        value=website_url
                        input_type=InputType::Url
                        placeholder="https://birei.dev"
                        on_input=Callback::new(update_signal(website_url))
                    />
                    <p class="doc-card__copy">
                        "Text preview: "
                        <strong>{move || name.get()}</strong>
                    </p>
                </div>
                <CodeExample code={r#"<Input
    id="display-name"
    value=name
    required=true
    placeholder="Display name"
    on_input=Callback::new(update_signal(name))
/>
<Input input_type=InputType::Email placeholder="name@birei.dev"/>
<Input input_type=InputType::Password placeholder="Password"/>
<Input input_type=InputType::Search placeholder="Search components"/>
<Input input_type=InputType::Tel placeholder="+41 44 555 01 23"/>
<Input input_type=InputType::Url placeholder="https://birei.dev"/>"#}/>
            </Card>

            <Card header="Aligned with buttons" class="doc-card">
                <span class="doc-card__kicker">"Sizes"</span>
                <div class="doc-card__preview doc-card__preview--stack">
                    <Input size=Size::Small placeholder="Small · 1.5rem / 24px"/>
                    <Input size=Size::Medium placeholder="Medium · 2rem / 32px"/>
                    <Input size=Size::Large placeholder="Large · 2.5rem / 40px"/>
                </div>
                <CodeExample code={r#"<Input size=Size::Small placeholder="Small · 1.5rem / 24px"/>
<Input size=Size::Medium placeholder="Medium · 2rem / 32px"/>
<Input size=Size::Large placeholder="Large · 2.5rem / 40px"/>"#}/>
            </Card>

            <Card header="Prefix and suffix content" class="doc-card">
                <span class="doc-card__kicker">"Affixes"</span>
                <div class="doc-card__preview doc-card__preview--stack">
                    <Input
                        value=search
                        input_type=InputType::Search
                        placeholder="Search the component book"
                        prefix=|| view! { <Icon name="search" label="Search"/> }
                        suffix=move || view! { <span>{move || format!("{} chars", search.get().len())}</span> }
                        on_input=Callback::new(update_signal(search))
                    />
                    <Input
                        value=email
                        input_type=InputType::Email
                        placeholder="work@birei.dev"
                        prefix=|| view! { <Icon name="mail" label="Email"/> }
                        suffix=|| view! { <Icon name="at-sign" label="Domain"/> }
                        on_input=Callback::new(update_signal(email))
                    />
                </div>
                <CodeExample code={r#"<Input
    input_type=InputType::Search
    placeholder="Search the component book"
    prefix=|| view! { <Icon name="search" label="Search"/> }
    suffix=|| view! { <span>"12 chars"</span> }
/>

<Input
    input_type=InputType::Email
    placeholder="work@birei.dev"
    prefix=|| view! { <Icon name="mail" label="Email"/> }
    suffix=|| view! { <Icon name="at-sign" label="Domain"/> }
/>"#}/>
            </Card>

            <Card header="Works with buttons too" class="doc-card">
                <span class="doc-card__kicker">"Suffix action"</span>
                <div class="doc-card__preview doc-card__preview--stack">
                    <Input
                        value=invite_code
                        placeholder="Invite code"
                        prefix=|| view! { <span>"Code"</span> }
                        suffix=move || {
                            view! {
                                <Button
                                    size=Size::Small
                                    variant=ButtonVariant::Transparent
                                    on_click=Callback::new(move |_| {
                                        invite_code.set(String::new());
                                    })
                                >
                                    "Clear"
                                </Button>
                            }
                        }
                        on_input=Callback::new(update_signal(invite_code))
                    />
                    <div class="field">
                        <Label text="Newsletter email" for_id="book-input-newsletter-email"/>
                        <Input
                            id="book-input-newsletter-email"
                            value=newsletter_email
                            input_type=InputType::Email
                            placeholder="name@studio.dev"
                            prefix=|| {
                                view! {
                                    <Button size=Size::Small variant=ButtonVariant::Secondary>
                                        "Email"
                                    </Button>
                                }
                            }
                            suffix=|| {
                                view! {
                                    <Button size=Size::Small>
                                        "Join"
                                    </Button>
                                }
                            }
                            on_input=Callback::new(update_signal(newsletter_email))
                        />
                    </div>
                </div>
                <CodeExample code={r#"<Input
    value=invite_code
    suffix=move || {
        view! {
            <Button size=Size::Small variant=ButtonVariant::Transparent>
                "Clear"
            </Button>
        }
    }
/>

<Input
    input_type=InputType::Email
    prefix=|| {
        view! {
            <Button size=Size::Small variant=ButtonVariant::Secondary>
                "Email"
            </Button>
        }
    }
    suffix=|| {
        view! {
            <Button size=Size::Small>
                "Join"
            </Button>
        }
    }
/>"#}/>
            </Card>

            <Card header="Disabled, readonly, invalid" class="doc-card">
                <span class="doc-card__kicker">"State"</span>
                <div class="doc-card__preview doc-card__preview--stack">
                    <Input placeholder="Disabled input" disabled=true/>
                    <Input value="Read-only value" readonly=true/>
                    <Input
                        value="invalid-address"
                        input_type=InputType::Email
                        invalid=true
                        suffix=|| view! { <span>"Required"</span> }
                    />
                </div>
                <CodeExample code={r#"<Input placeholder="Disabled input" disabled=true/>
<Input value="Read-only value" readonly=true/>
<Input value="invalid-address" input_type=InputType::Email invalid=true/>"#}/>
            </Card>
        </section>
    }
}
