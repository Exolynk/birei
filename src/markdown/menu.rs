use crate::ButtonBarItem;

/// Built-in toolbar actions shown by default in the markdown editor.
pub(crate) fn default_toolbar_items() -> Vec<ButtonBarItem> {
    vec![
        ButtonBarItem::new("heading", "Heading").icon("heading"),
        ButtonBarItem::new("bold", "Bold").icon("bold"),
        ButtonBarItem::new("italic", "Italic").icon("italic"),
        ButtonBarItem::new("link", "Link").icon("link"),
        ButtonBarItem::new("unordered-list", "Bullets").icon("list"),
        ButtonBarItem::new("ordered-list", "Numbers").icon("list-ordered"),
        ButtonBarItem::new("table", "Table").icon("table"),
        ButtonBarItem::new("image", "Image").icon("image-up"),
        ButtonBarItem::new("toggle-markdown-view", ".md").icon("code-xml"),
    ]
}

/// Heading submenu options used by the heading toolbar trigger.
pub(crate) fn heading_menu_items() -> Vec<ButtonBarItem> {
    vec![
        ButtonBarItem::new("heading-1", "Heading 1").icon("heading-1"),
        ButtonBarItem::new("heading-2", "Heading 2").icon("heading-2"),
        ButtonBarItem::new("heading-3", "Heading 3").icon("heading-3"),
        ButtonBarItem::new("heading-4", "Heading 4").icon("heading-4"),
        ButtonBarItem::new("heading-paragraph", "Normal text").icon("text"),
    ]
}

/// Table submenu options used when the selection is currently inside a table.
pub(crate) fn table_menu_items() -> Vec<ButtonBarItem> {
    vec![
        ButtonBarItem::new("table-row-above", "Row above").icon("arrow-up-to-line"),
        ButtonBarItem::new("table-row-below", "Row below").icon("arrow-down-to-line"),
        ButtonBarItem::new("table-col-left", "Column left").icon("arrow-left-to-line"),
        ButtonBarItem::new("table-col-right", "Column right").icon("arrow-right-to-line"),
        ButtonBarItem::new("table-row-delete", "Delete row").icon("rows-3"),
        ButtonBarItem::new("table-col-delete", "Delete column").icon("columns-3"),
        ButtonBarItem::new("table-delete", "Delete table").icon("trash-2"),
    ]
}

/// Shared popup class composition for markdown dropdown-style menus.
pub(crate) fn menu_popup_class_name(base: &str, open_upward: bool) -> String {
    let mut classes = String::from(base);
    classes.push_str(" birei-dropdown-button__menu birei-dropdown-button__menu--content-width");
    if open_upward {
        classes.push_str(" birei-dropdown-button__menu--upward");
    }
    classes
}

/// Maps menu item values back to the internal table action identifiers.
pub(crate) fn table_action_from_value(value: &str) -> &'static str {
    match value {
        "table-row-above" => "table-row-above",
        "table-row-below" => "table-row-below",
        "table-col-left" => "table-col-left",
        "table-col-right" => "table-col-right",
        "table-row-delete" => "table-row-delete",
        "table-col-delete" => "table-col-delete",
        _ => "table-delete",
    }
}
