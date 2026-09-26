use std::collections::HashSet;

use super::TreeNode;
use crate::IcnName;

pub(crate) const ROW_HEIGHT: f64 = 48.0;

/// Render and navigation data for one item in the flattened, visible tree.
#[derive(PartialEq, Eq)]
pub(crate) struct VisibleNode {
    pub(crate) value: String,
    pub(crate) title: String,
    pub(crate) has_children: bool,
    pub(crate) icon: Option<IcnName>,
    pub(crate) meta: Option<String>,
    pub(crate) highlight: Option<String>,
    pub(crate) expanded: bool,
    pub(crate) depth: usize,
    pub(crate) parent: Option<usize>,
    pub(crate) position: usize,
    pub(crate) sibling_count: usize,
}

/// Flattens expanded branches in document order for keyboard and scroll calculations.
pub(crate) fn visible_nodes(roots: &[TreeNode], expanded: &[String]) -> Vec<VisibleNode> {
    let expanded: HashSet<&str> = expanded.iter().map(String::as_str).collect();
    let mut visible = Vec::new();
    append_visible(roots, 1, None, &expanded, &mut visible);
    visible
}

/// Appends one level and visits children only when their parent is expanded.
fn append_visible(
    nodes: &[TreeNode],
    depth: usize,
    parent: Option<usize>,
    expanded: &HashSet<&str>,
    visible: &mut Vec<VisibleNode>,
) {
    for (position, node) in nodes.iter().enumerate() {
        let index = visible.len();
        let is_expanded = expanded.contains(node.value.as_str());
        visible.push(VisibleNode {
            value: node.value.clone(),
            title: node.title.clone(),
            has_children: node.has_children || !node.children.is_empty(),
            icon: node.icon.clone(),
            meta: node.meta.clone(),
            highlight: node.highlight.clone(),
            expanded: is_expanded,
            depth,
            parent,
            position: position + 1,
            sibling_count: nodes.len(),
        });
        if is_expanded {
            append_visible(&node.children, depth + 1, Some(index), expanded, visible);
        }
    }
}

/// Returns the overscanned range for fixed-height tree rows.
pub(crate) fn visible_range(
    count: usize,
    overscan: usize,
    scroll_top: f64,
    viewport_height: f64,
) -> (usize, usize) {
    if count == 0 {
        return (0, 0);
    }
    let start = ((scroll_top / ROW_HEIGHT).floor() as usize).saturating_sub(overscan);
    let end = (((scroll_top + viewport_height) / ROW_HEIGHT).ceil() as usize)
        .saturating_add(overscan)
        .min(count);
    (start.min(count), end)
}

/// Returns a new expansion list with one branch added or removed.
pub(crate) fn toggle_expanded(expanded: &[String], value: &str) -> Vec<String> {
    if expanded.iter().any(|item| item == value) {
        expanded
            .iter()
            .filter(|item| item.as_str() != value)
            .cloned()
            .collect()
    } else {
        let mut next = expanded.to_vec();
        next.push(value.to_owned());
        next
    }
}

/// Reports whether a tree occurrence is in the controlled selection.
pub(crate) fn is_selected(selected: &[String], value: &str) -> bool {
    selected.iter().any(|item| item == value)
}

#[cfg(test)]
mod tests {
    use super::{is_selected, toggle_expanded, visible_nodes, visible_range};
    use crate::TreeNode;

    /// Expansion keeps descendants in document order with usable parent indexes.
    #[test]
    fn flatten_expanded_tree() {
        let roots = vec![
            TreeNode::new("a", "A").children(vec![
                TreeNode::new("b", "B").children(vec![TreeNode::new("c", "C")]),
                TreeNode::new("d", "D"),
            ]),
            TreeNode::new("e", "E"),
        ];
        let visible = visible_nodes(&roots, &["a".into(), "b".into()]);
        let values: Vec<_> = visible.iter().map(|item| item.value.as_str()).collect();
        assert_eq!(values, ["a", "b", "c", "d", "e"]);
        assert_eq!(visible[2].parent, Some(1));
        assert_eq!(visible[3].depth, 2);
        assert_eq!(visible[4].position, 2);
        assert_eq!(visible[4].sibling_count, 2);
    }

    /// Collapsed descendants do not consume virtual row space.
    #[test]
    fn collapsed_nodes_are_hidden() {
        let roots = vec![TreeNode::new("a", "A").children(vec![TreeNode::new("b", "B")])];
        assert_eq!(visible_nodes(&roots, &[]).len(), 1);
        assert_eq!(visible_range(100, 2, 240.0, 144.0), (3, 10));
    }

    /// A node can remain expandable while its children have not arrived.
    #[test]
    fn unloaded_children_keep_disclosure() {
        let roots = vec![TreeNode::new("a", "A").has_children(true)];
        let visible = visible_nodes(&roots, &[]);
        assert!(visible[0].has_children);
    }

    /// Toggling preserves the order of other externally controlled branches.
    #[test]
    fn expansion_toggle_preserves_other_values() {
        let expanded = vec!["a".into(), "b".into()];
        assert_eq!(toggle_expanded(&expanded, "a"), ["b"]);
        assert_eq!(toggle_expanded(&expanded, "c"), ["a", "b", "c"]);
    }

    /// Every path for one logical record can be highlighted independently.
    #[test]
    fn selection_matches_multiple_occurrences() {
        let selected = vec!["root-a/record".into(), "root-b/record".into()];
        assert!(is_selected(&selected, "root-a/record"));
        assert!(is_selected(&selected, "root-b/record"));
        assert!(!is_selected(&selected, "root-a/other"));
    }
}
