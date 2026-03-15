#[cfg(feature = "embedded-icons")]
use base64::Engine;

const EMBEDDED_STYLE_ID: &str = "birei-embedded-css";
#[cfg(feature = "embedded-css")]
const CSS: &str = include_str!("../dist/birei.css");
#[cfg(feature = "embedded-icons")]
const LUCIDE_FONT_WOFF2: &[u8] = include_bytes!("../deps/lucide0-577-0/lucide.woff2");

pub fn embed_assets() -> Result<(), wasm_bindgen::JsValue> {
    let window = web_sys::window()
        .ok_or_else(|| wasm_bindgen::JsValue::from_str("window is not available"))?;
    let document = window
        .document()
        .ok_or_else(|| wasm_bindgen::JsValue::from_str("document is not available"))?;
    let head = document
        .head()
        .ok_or_else(|| wasm_bindgen::JsValue::from_str("document.head is not available"))?;

    if document.get_element_by_id(EMBEDDED_STYLE_ID).is_some() {
        return Ok(());
    }

    let style = document.create_element("style")?;
    style.set_attribute("id", EMBEDDED_STYLE_ID)?;
    style.set_text_content(Some(embedded_css().as_str()));
    head.append_child(&style)?;

    Ok(())
}

fn embedded_css() -> String {
    #[cfg(feature = "embedded-icons")]
    {
        let encoded_font = base64::engine::general_purpose::STANDARD.encode(LUCIDE_FONT_WOFF2);

        CSS.replace(
            r#"url("lucide.woff2")"#,
            &format!(r#"url("data:font/woff2;base64,{encoded_font}")"#),
        )
    }

    #[cfg(not(feature = "embedded-icons"))]
    {
        CSS.to_owned()
    }
}
