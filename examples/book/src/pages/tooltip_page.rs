use birei::{Button, Card, Input, Label, Tooltip, TooltipPlacement};
use leptos::prelude::*;

#[component]
pub fn TooltipPage() -> impl IntoView {
    view! {
        <section class="page-header">
            <div class="page-header__eyebrow">"Component"</div>
            <h2>"Tooltip"</h2>
            <p class="page-header__lede">
                "Hover or focus helper text that opens after a short delay, renders in a popup, and can be positioned around the trigger."
            </p>
        </section>

        <section class="doc-grid">
            <Card class="doc-card">
                    <span class="doc-card__kicker">"Basics"</span>
                    <div class="doc-card__preview">
                        <Tooltip content="Used for account recovery and security alerts.">
                            <Label text="Email address"/>
                        </Tooltip>
                    </div>
                    <pre class="doc-card__code"><code>{r#"<Tooltip content="Used for account recovery and security alerts.">
    <Label text="Email address"/>
</Tooltip>"#}</code></pre>
            </Card>

            <Card class="doc-card">
                    <span class="doc-card__kicker">"Placement"</span>
                    <div class="doc-card__preview">
                        <Tooltip content="Opens above by default.">
                            <span>"Top"</span>
                        </Tooltip>
                        <Tooltip content="Can also open below." placement=TooltipPlacement::Bottom>
                            <span>"Bottom"</span>
                        </Tooltip>
                        <Tooltip content="Left side placement." placement=TooltipPlacement::Left>
                            <span>"Left"</span>
                        </Tooltip>
                        <Tooltip content="Right side placement." placement=TooltipPlacement::Right>
                            <span>"Right"</span>
                        </Tooltip>
                    </div>
                    <pre class="doc-card__code"><code>{r#"<Tooltip content="Can also open below." placement=TooltipPlacement::Bottom>
    <span>"Bottom"</span>
</Tooltip>"#}</code></pre>
            </Card>

            <Card class="doc-card">
                    <span class="doc-card__kicker">"Common Triggers"</span>
                    <div class="doc-card__preview">
                        <Tooltip content="Saves changes without leaving the current step.">
                            <Button>"Save draft"</Button>
                        </Tooltip>
                        <Tooltip content="We only use this to send shipping updates.">
                            <Input placeholder="Email address"/>
                        </Tooltip>
                    </div>
                    <pre class="doc-card__code"><code>{r#"<Tooltip content="Saves changes without leaving the current step.">
    <Button>"Save draft"</Button>
</Tooltip>

<Tooltip content="We only use this to send shipping updates.">
    <Input placeholder="Email address"/>
</Tooltip>"#}</code></pre>
            </Card>
        </section>
    }
}
