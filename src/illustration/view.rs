use leptos::prelude::*;

const BRAND_BUILDING: &str = include_str!("assets/brand-building.svg");
const CONNECTION_LOST: &str = include_str!("assets/connection-lost.svg");
const NO_ACCESS: &str = include_str!("assets/no-access.svg");
const NO_DATA: &str = include_str!("assets/no-data.svg");
const NO_RESULTS: &str = include_str!("assets/no-results.svg");

/// Identifies an Exolynk companion illustration included with Birei.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum IllustrationKind {
    /// Companion building the Exolynk mark.
    BrandBuilding,
    /// Companion holding a broken connection.
    ConnectionLost,
    /// Companion encountering a locked door.
    NoAccess,
    /// Companion facing an empty canvas.
    NoData,
    /// Companion searching without a result.
    NoResults,
}

/// Controls where optional illustration copy appears relative to the artwork.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum IllustrationTextLocation {
    /// Places copy above the artwork.
    Top,
    /// Places copy to the left of the artwork.
    Left,
    /// Places copy to the right of the artwork.
    Right,
    /// Places copy below the artwork.
    Bottom,
    /// Places copy right of the artwork in a landscape viewport and below it otherwise.
    #[default]
    Auto,
}

impl IllustrationTextLocation {
    /// Returns the CSS modifier used to arrange optional illustration copy.
    const fn class_name(self) -> &'static str {
        match self {
            Self::Top => "top",
            Self::Left => "left",
            Self::Right => "right",
            Self::Bottom => "bottom",
            Self::Auto => "auto",
        }
    }
}

impl IllustrationKind {
    /// Returns the trusted inline SVG source for this illustration.
    const fn markup(self) -> &'static str {
        match self {
            Self::BrandBuilding => BRAND_BUILDING,
            Self::ConnectionLost => CONNECTION_LOST,
            Self::NoAccess => NO_ACCESS,
            Self::NoData => NO_DATA,
            Self::NoResults => NO_RESULTS,
        }
    }
}

/// Renders a responsive animated Exolynk companion illustration.
#[component]
pub fn Illustration(
    /// Illustration selected from Birei's bundled companion scenes.
    kind: IllustrationKind,
    /// Optional compact context label displayed above the title.
    #[prop(optional, into)]
    label: Option<String>,
    /// Optional heading that describes the illustrated state.
    #[prop(optional, into)]
    title: Option<String>,
    /// Optional supporting text displayed below the title.
    #[prop(optional, into)]
    description: Option<String>,
    /// Optional interactive content rendered beneath the illustration copy.
    #[prop(optional)]
    children: Option<ChildrenFn>,
    /// Placement of the optional copy relative to the illustration.
    #[prop(optional)]
    location: IllustrationTextLocation,
    /// Additional CSS class names applied to the illustration container.
    #[prop(optional, into)]
    class: Option<String>,
) -> impl IntoView {
    let label = non_empty(label);
    let title = non_empty(title);
    let description = non_empty(description);
    let has_copy =
        label.is_some() || title.is_some() || description.is_some() || children.is_some();
    let mut class_name = format!(
        "birei-illustration birei-illustration--{}",
        location.class_name()
    );
    if let Some(class) = class.filter(|class| !class.trim().is_empty()) {
        class_name.push(' ');
        class_name.push_str(&class);
    }

    let copy = has_copy.then(|| {
        view! {
            <div class="birei-illustration__copy">
                {label.map(|label| view! { <div class="birei-illustration__label">{label}</div> })}
                {title.map(|title| view! { <h2 class="birei-illustration__title">{title}</h2> })}
                {description.map(|description| view! { <p class="birei-illustration__description">{description}</p> })}
                {children.map(|children| view! {
                    <div class="birei-illustration__actions">{children()}</div>
                })}
            </div>
        }
    });

    view! {
        <div class=class_name>
            <div class="birei-illustration__art" inner_html=kind.markup()></div>
            {copy}
        </div>
    }
}

/// Removes optional copy values that contain only whitespace.
fn non_empty(value: Option<String>) -> Option<String> {
    value.filter(|value| !value.trim().is_empty())
}
