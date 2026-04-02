use std::cell::RefCell;
use std::rc::Rc;

use html2md::parse_html;
use js_sys::{Function, Reflect};
use pulldown_cmark::{html as markdown_html, Options, Parser};
use wasm_bindgen::JsCast;
use wasm_bindgen::JsValue;
use web_sys::{window, HtmlElement, Node, Range};

pub(crate) fn markdown_from_html(html: &str) -> String {
    normalize_markdown(parse_html(html))
}

pub(crate) fn markdown_to_html(markdown: &str) -> String {
    if markdown.trim().is_empty() {
        return String::new();
    }

    let mut options = Options::empty();
    options.insert(Options::ENABLE_TABLES);
    options.insert(Options::ENABLE_STRIKETHROUGH);
    let parser = Parser::new_ext(markdown, options);
    let mut html = String::new();
    markdown_html::push_html(&mut html, parser);
    html
}

pub(crate) fn decorate_rendered_content(editor: &HtmlElement, is_editable: bool) {
    let _ = editor.set_attribute(
        "contenteditable",
        if is_editable { "true" } else { "false" },
    );

    if let Ok(nodes) = editor.query_selector_all("h1, h2, h3, h4, h5, h6, p, li, td, th, a") {
        for index in 0..nodes.length() {
            let Some(node) = nodes.item(index) else {
                continue;
            };
            let Some(element) = node.dyn_ref::<web_sys::Element>() else {
                continue;
            };
            let _ = element.set_attribute(
                "contenteditable",
                if is_editable { "true" } else { "false" },
            );
            if element.tag_name() == "A" {
                let _ = element.set_attribute("target", "_blank");
                let _ = element.set_attribute("rel", "noopener noreferrer");
            }
        }
    }

    if let Ok(nodes) = editor.query_selector_all("img") {
        for index in 0..nodes.length() {
            let Some(node) = nodes.item(index) else {
                continue;
            };
            let Some(element) = node.dyn_ref::<web_sys::Element>() else {
                continue;
            };
            let _ = element.set_attribute("contenteditable", "false");
        }
    }
}

pub(crate) fn exec_document_command(command: &str, value: Option<&str>) {
    let Some(document) = window().and_then(|window| window.document()) else {
        return;
    };

    let Ok(exec_command) = Reflect::get(document.as_ref(), &JsValue::from_str("execCommand"))
    else {
        return;
    };
    let Ok(function) = exec_command.dyn_into::<Function>() else {
        return;
    };

    let show_ui = JsValue::from_bool(false);
    let value = value.map(JsValue::from_str).unwrap_or(JsValue::NULL);
    let _ = function.call3(
        document.as_ref(),
        &JsValue::from_str(command),
        &show_ui,
        &value,
    );
}

pub(crate) fn escape_html_attribute(value: &str) -> String {
    value
        .replace('&', "&amp;")
        .replace('"', "&quot;")
        .replace('<', "&lt;")
        .replace('>', "&gt;")
}

pub(crate) fn escape_html_text(value: &str) -> String {
    value
        .replace('&', "&amp;")
        .replace('<', "&lt;")
        .replace('>', "&gt;")
}

pub(crate) fn insert_html_at_saved_range(saved_range: &Rc<RefCell<Option<Range>>>, html: &str) {
    let Some(document) = window().and_then(|window| window.document()) else {
        return;
    };
    let Some(range) = saved_range.borrow().clone() else {
        exec_document_command("insertHTML", Some(html));
        return;
    };

    let Ok(container) = document.create_element("div") else {
        return;
    };
    container.set_inner_html(html);

    let _ = range.delete_contents();
    let mut last_inserted = None::<Node>;

    while let Some(node) = container.first_child() {
        let _ = range.insert_node(&node);
        let _ = range.set_start_after(&node);
        let _ = range.set_end_after(&node);
        last_inserted = Some(node);
    }

    if let Some(last_inserted) = last_inserted {
        let _ = range.set_start_after(&last_inserted);
        let _ = range.set_end_after(&last_inserted);
    }

    if let Some(selection) = window().and_then(|window| window.get_selection().ok().flatten()) {
        let _ = selection.remove_all_ranges();
        let _ = selection.add_range(&range);
    }

    *saved_range.borrow_mut() = Some(range);
}

fn normalize_markdown(markdown: String) -> String {
    markdown.trim().to_owned()
}
