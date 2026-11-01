use leptos::ev;
use leptos::prelude::*;
use qrcodegen::{QrCode as EncoderQrCode, QrCodeEcc};

use crate::ArcOneCallback;

/// SVG QR code on a transparent background with a color per theme, center image, and click
/// action.
///
/// The modules are black in light mode and white in dark mode unless `color` or `dark_color`
/// set another color for that mode.
#[component]
pub fn QrCode(
    /// Text or URL encoded by the QR code.
    #[prop(into)]
    data: String,
    /// Color of the modules in light mode.
    #[prop(optional, into)]
    color: Option<String>,
    /// Color of the modules in dark mode.
    #[prop(optional, into)]
    dark_color: Option<String>,
    /// Optional image URL shown in the QR code center.
    #[prop(optional, into)]
    image: Option<String>,
    /// Rendered QR code size in CSS pixels.
    #[prop(optional, default = 200)]
    size: u32,
    /// Accessible description of the QR code.
    #[prop(optional, into)]
    aria_label: Option<String>,
    /// Additional classes applied to the QR code root.
    #[prop(optional, into)]
    class: Option<String>,
    /// Optional action invoked when the QR code is clicked.
    #[prop(optional, into)]
    on_click: Option<ArcOneCallback<ev::MouseEvent>>,
) -> impl IntoView {
    let style = color_style(resolve_color(color), resolve_color(dark_color));
    let image = image.filter(|image| !image.trim().is_empty());
    let modules = encode_modules(&data);
    let label = aria_label.unwrap_or_else(|| String::from("QR code"));
    let interactive = on_click.is_some();
    let class_name = qr_code_class_name(class, interactive);

    if let Some(on_click) = on_click {
        view! {
            <button
                class=class_name
                type="button"
                aria-label=label
                style=style
                on:click=move |event| on_click.run(event)
            >
                {render_svg(modules, image, size)}
            </button>
        }
        .into_any()
    } else {
        view! {
            <div class=class_name role="img" aria-label=label style=style>
                {render_svg(modules, image, size)}
            </div>
        }
        .into_any()
    }
}

/// Distinguishes round data modules from the square modules of the finder patterns.
#[derive(Clone, Copy)]
enum ModuleKind {
    Dot,
    Finder,
}

/// One filled QR module positioned in the encoder's square matrix.
#[derive(Clone, Copy)]
struct Module {
    x: i32,
    y: i32,
    kind: ModuleKind,
}

/// Converts optional color properties into non-empty CSS colors.
fn resolve_color(color: Option<String>) -> Option<String> {
    color.filter(|color| !color.trim().is_empty())
}

/// Passes the configured colors to the stylesheet, which picks the one of the active theme.
fn color_style(color: Option<String>, dark_color: Option<String>) -> String {
    [
        ("--birei-qr-code-color", color),
        ("--birei-qr-code-dark-color", dark_color),
    ]
    .into_iter()
    .filter_map(|(name, color)| Some(format!("{name}: {};", color?)))
    .collect::<Vec<_>>()
    .join(" ")
}

/// Encodes text with high error correction and classifies visible matrix modules.
fn encode_modules(data: &str) -> Vec<Module> {
    let Ok(code) = EncoderQrCode::encode_text(data, QrCodeEcc::High) else {
        return Vec::new();
    };

    let size = code.size();
    let mut modules = Vec::new();
    for y in 0..size {
        for x in 0..size {
            if code.get_module(x, y) {
                modules.push(Module {
                    x,
                    y,
                    kind: module_kind(x, y, size),
                });
            }
        }
    }
    modules
}

/// Returns whether a module belongs to one of the three finder patterns.
fn module_kind(x: i32, y: i32, size: i32) -> ModuleKind {
    let in_finder =
        [(0, 0), (size - 7, 0), (0, size - 7)]
            .into_iter()
            .any(|(origin_x, origin_y)| {
                (origin_x..origin_x + 7).contains(&x) && (origin_y..origin_y + 7).contains(&y)
            });
    if in_finder {
        ModuleKind::Finder
    } else {
        ModuleKind::Dot
    }
}

/// Builds the SVG content for an encoded QR matrix.
///
/// Modules are filled with `currentColor`, which the stylesheet sets per theme.
fn render_svg(modules: Vec<Module>, image: Option<String>, size: u32) -> AnyView {
    let matrix_size = modules
        .iter()
        .map(|module| module.x.max(module.y) + 1)
        .max()
        .unwrap_or(1);
    let module_views = modules
        .into_iter()
        .map(|module| match module.kind {
            ModuleKind::Dot => view! {
                <circle
                    cx=format!("{}.5", module.x)
                    cy=format!("{}.5", module.y)
                    r="0.42"
                    fill="currentColor"
                />
            }
            .into_any(),
            ModuleKind::Finder => view! {
                <rect x=module.x y=module.y width="1" height="1" fill="currentColor" />
            }
            .into_any(),
        })
        .collect_view();
    let image_overlay = image.map(|image| {
        let image_size = (f64::from(matrix_size) * 0.18).clamp(5.0, 10.0);
        let image_start = (f64::from(matrix_size) - image_size) / 2.0;
        let image_background_size = image_size + 1.2;
        let image_background_start = image_start - 0.6;
        view! {
            <rect
                x=image_background_start
                y=image_background_start
                width=image_background_size
                height=image_background_size
                rx="1"
                class="birei-qr-code__image-backdrop"
            />
            <image
                href=image
                x=image_start
                y=image_start
                width=image_size
                height=image_size
                preserveAspectRatio="xMidYMid meet"
            />
        }
    });

    view! {
        <svg
            class="birei-qr-code__svg"
            viewBox=format!("0 0 {matrix_size} {matrix_size}")
            width=size
            height=size
            aria-hidden="true"
            focusable="false"
            xmlns="http://www.w3.org/2000/svg"
        >
            {module_views}
            {image_overlay}
        </svg>
    }
    .into_any()
}

/// Builds stable root classes for interactive and static QR code renderings.
fn qr_code_class_name(class: Option<String>, interactive: bool) -> String {
    let mut classes = vec!["birei-qr-code"];
    if interactive {
        classes.push("birei-qr-code--interactive");
    }
    if let Some(class) = class.as_deref() {
        classes.push(class);
    }
    classes.join(" ")
}
