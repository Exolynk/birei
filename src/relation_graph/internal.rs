use uuid::Uuid;

use super::types::{RelationGraphEdge, RelationGraphNode};

pub(crate) const NODE_WIDTH: f64 = 248.0;
pub(crate) const NODE_HEIGHT: f64 = 36.0;
pub(crate) const LAYER_GAP: f64 = 156.0;
pub(crate) const ROW_GAP: f64 = 28.0;
pub(crate) const GRAPH_PADDING_X: f64 = 36.0;
pub(crate) const GRAPH_PADDING_Y: f64 = 32.0;
pub(crate) const GRAPH_TRAILING_PADDING: f64 = 32.0;

#[derive(Clone, Debug, PartialEq)]
pub(crate) struct NodeLayout {
    pub(crate) node: RelationGraphNode,
    pub(crate) x: f64,
    pub(crate) y: f64,
}

#[derive(Clone, Debug, PartialEq)]
pub(crate) struct EdgePath {
    pub(crate) key: String,
    pub(crate) d: String,
    pub(crate) arrow: bool,
}

#[derive(Clone, Debug, PartialEq)]
pub(crate) struct EdgeLayout {
    pub(crate) edge: RelationGraphEdge,
    pub(crate) paths: Vec<EdgePath>,
}

#[derive(Clone, Debug, PartialEq)]
pub(crate) struct RelationGraphLayout {
    pub(crate) width: f64,
    pub(crate) height: f64,
    pub(crate) nodes: Vec<NodeLayout>,
    pub(crate) edges: Vec<EdgeLayout>,
}

#[derive(Clone, Debug, PartialEq)]
pub(crate) struct HoverPopup {
    pub(crate) title: String,
    pub(crate) body: Option<String>,
    pub(crate) left: f64,
    pub(crate) top: f64,
}

#[derive(Clone, Debug, PartialEq)]
pub(crate) struct PositionedNode {
    pub(crate) id: Uuid,
    pub(crate) x: f64,
    pub(crate) y: f64,
    pub(crate) layer: usize,
    pub(crate) row: usize,
}
