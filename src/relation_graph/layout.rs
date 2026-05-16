use std::cmp::Ordering;
use std::collections::{HashMap, HashSet, VecDeque};

use uuid::Uuid;

use super::internal::{
    EdgeLayout, EdgePath, NodeLayout, PositionedNode, RelationGraphLayout, GRAPH_PADDING_X,
    GRAPH_PADDING_Y, GRAPH_TRAILING_PADDING, LAYER_GAP, NODE_FIELD_HEIGHT, NODE_HEADER_HEIGHT,
    NODE_WIDTH, ROW_GAP,
};
use super::types::{RelationGraphEdge, RelationGraphNode};

pub(crate) fn build_layout(
    nodes: Vec<RelationGraphNode>,
    edges: Vec<RelationGraphEdge>,
) -> RelationGraphLayout {
    if nodes.is_empty() {
        return RelationGraphLayout {
            width: GRAPH_PADDING_X * 2.0,
            height: GRAPH_PADDING_Y * 2.0,
            nodes: Vec::new(),
            edges: Vec::new(),
        };
    }

    let valid_nodes = nodes
        .iter()
        .map(|node| (node.id, node.clone()))
        .collect::<HashMap<_, _>>();
    let edges = edges
        .into_iter()
        .filter(|edge| {
            valid_nodes.contains_key(&edge.source) && valid_nodes.contains_key(&edge.target)
        })
        .collect::<Vec<_>>();

    let layers = assign_layers(&nodes, &edges);
    let ordering = order_layers(&nodes, &edges, &layers);
    let max_layer = layers.values().copied().max().unwrap_or_default();
    let node_heights = nodes
        .iter()
        .map(|node| (node.id, node_height(node)))
        .collect::<HashMap<_, _>>();
    let node_field_idents = nodes
        .iter()
        .map(|node| {
            (
                node.id,
                node.fields
                    .iter()
                    .map(|field| field.ident.clone())
                    .collect::<Vec<_>>(),
            )
        })
        .collect::<HashMap<_, _>>();
    let lane_height = ordering
        .iter()
        .map(|layer_ids| layer_height(layer_ids, &node_heights))
        .fold(0.0_f64, f64::max);
    let height = (lane_height + (GRAPH_PADDING_Y * 2.0))
        .max((NODE_HEADER_HEIGHT + (GRAPH_PADDING_Y * 2.0)).ceil());
    let width = ((max_layer as f64 * (NODE_WIDTH + LAYER_GAP))
        + NODE_WIDTH
        + GRAPH_PADDING_X * 2.0
        + GRAPH_TRAILING_PADDING)
        .max((NODE_WIDTH + GRAPH_PADDING_X * 2.0 + GRAPH_TRAILING_PADDING).ceil());

    let mut positioned = HashMap::<Uuid, PositionedNode>::new();

    for (layer_index, layer_ids) in ordering.iter().enumerate() {
        let layer_height = layer_height(layer_ids, &node_heights);
        let base_y = GRAPH_PADDING_Y + ((height - (GRAPH_PADDING_Y * 2.0) - layer_height) * 0.5);
        let mut next_y = base_y;

        for (row_index, id) in layer_ids.iter().enumerate() {
            let x = GRAPH_PADDING_X + layer_index as f64 * (NODE_WIDTH + LAYER_GAP);
            let y = next_y;
            let height = node_heights.get(id).copied().unwrap_or(NODE_HEADER_HEIGHT);

            positioned.insert(
                *id,
                PositionedNode {
                    id: *id,
                    x,
                    y,
                    height,
                    field_idents: node_field_idents.get(id).cloned().unwrap_or_default(),
                    layer: layer_index,
                    row: row_index,
                },
            );

            next_y += height + ROW_GAP;
        }
    }

    let nodes = nodes
        .into_iter()
        .filter_map(|node| {
            positioned.get(&node.id).map(|position| NodeLayout {
                node,
                x: position.x,
                y: position.y,
            })
        })
        .collect::<Vec<_>>();

    let edges = edges
        .into_iter()
        .filter_map(|edge| build_edge_layout(edge, &positioned))
        .collect::<Vec<_>>();

    RelationGraphLayout {
        width,
        height,
        nodes,
        edges,
    }
}

fn assign_layers(nodes: &[RelationGraphNode], edges: &[RelationGraphEdge]) -> HashMap<Uuid, usize> {
    let mut incoming = HashMap::<Uuid, HashSet<Uuid>>::new();
    let mut outgoing = HashMap::<Uuid, HashSet<Uuid>>::new();
    let mut indegree = HashMap::<Uuid, usize>::new();

    for node in nodes {
        incoming.insert(node.id, HashSet::new());
        outgoing.insert(node.id, HashSet::new());
        indegree.insert(node.id, 0);
    }

    for edge in edges {
        if edge.source == edge.target {
            continue;
        }
        if outgoing.entry(edge.source).or_default().insert(edge.target) {
            incoming.entry(edge.target).or_default().insert(edge.source);
            *indegree.entry(edge.target).or_default() += 1;
        }
    }

    let mut queue = nodes
        .iter()
        .filter(|node| indegree.get(&node.id).copied().unwrap_or_default() == 0)
        .map(|node| node.id)
        .collect::<Vec<_>>();
    queue.sort_by(|left, right| node_name_cmp(nodes, left, right));

    let mut queue = VecDeque::from(queue);
    let mut layer = HashMap::<Uuid, usize>::new();
    let mut processed = HashSet::<Uuid>::new();

    while let Some(node_id) = queue.pop_front() {
        processed.insert(node_id);
        let current_layer = layer.get(&node_id).copied().unwrap_or_default();

        let mut next_nodes = outgoing
            .get(&node_id)
            .cloned()
            .unwrap_or_default()
            .into_iter()
            .collect::<Vec<_>>();
        next_nodes.sort_by(|left, right| node_name_cmp(nodes, left, right));

        for next_id in next_nodes {
            let next_layer = current_layer + 1;
            layer
                .entry(next_id)
                .and_modify(|value| *value = (*value).max(next_layer))
                .or_insert(next_layer);

            if let Some(value) = indegree.get_mut(&next_id) {
                *value = value.saturating_sub(1);
                if *value == 0 {
                    queue.push_back(next_id);
                }
            }
        }
    }

    for node in nodes {
        if processed.contains(&node.id) {
            continue;
        }

        let fallback = incoming
            .get(&node.id)
            .into_iter()
            .flatten()
            .filter_map(|source| layer.get(source).copied())
            .max()
            .map_or(0, |value| value + 1);
        layer.entry(node.id).or_insert(fallback);
    }

    layer
}

fn order_layers(
    nodes: &[RelationGraphNode],
    edges: &[RelationGraphEdge],
    layers: &HashMap<Uuid, usize>,
) -> Vec<Vec<Uuid>> {
    let max_layer = layers.values().copied().max().unwrap_or_default();
    let mut incoming = HashMap::<Uuid, Vec<Uuid>>::new();
    let mut outgoing = HashMap::<Uuid, Vec<Uuid>>::new();

    for node in nodes {
        incoming.insert(node.id, Vec::new());
        outgoing.insert(node.id, Vec::new());
    }

    for edge in edges {
        if edge.source == edge.target {
            continue;
        }
        outgoing.entry(edge.source).or_default().push(edge.target);
        incoming.entry(edge.target).or_default().push(edge.source);
    }

    let mut layers_vec = vec![Vec::<Uuid>::new(); max_layer + 1];
    for node in nodes {
        let layer = layers.get(&node.id).copied().unwrap_or_default();
        layers_vec[layer].push(node.id);
    }

    for layer_ids in &mut layers_vec {
        layer_ids.sort_by(|left, right| node_name_cmp(nodes, left, right));
    }

    for _ in 0..4 {
        let positions = positions_in_layers(&layers_vec);
        for layer_ids in layers_vec.iter_mut().skip(1) {
            layer_ids.sort_by(|left, right| {
                compare_barycenter(
                    left,
                    right,
                    incoming.get(left).map_or(&[][..], Vec::as_slice),
                    incoming.get(right).map_or(&[][..], Vec::as_slice),
                    &positions,
                    nodes,
                )
            });
        }

        let positions = positions_in_layers(&layers_vec);
        let layer_count = layers_vec.len().saturating_sub(1);
        for layer_ids in layers_vec[..layer_count].iter_mut().rev() {
            layer_ids.sort_by(|left, right| {
                compare_barycenter(
                    left,
                    right,
                    outgoing.get(left).map_or(&[][..], Vec::as_slice),
                    outgoing.get(right).map_or(&[][..], Vec::as_slice),
                    &positions,
                    nodes,
                )
            });
        }
    }

    layers_vec
}

fn positions_in_layers(layers: &[Vec<Uuid>]) -> HashMap<Uuid, usize> {
    layers
        .iter()
        .flat_map(|layer| {
            layer
                .iter()
                .copied()
                .enumerate()
                .map(|(index, id)| (id, index))
        })
        .collect::<HashMap<_, _>>()
}

fn compare_barycenter(
    left: &Uuid,
    right: &Uuid,
    left_neighbors: &[Uuid],
    right_neighbors: &[Uuid],
    positions: &HashMap<Uuid, usize>,
    nodes: &[RelationGraphNode],
) -> Ordering {
    let left_center = barycenter(left_neighbors, positions);
    let right_center = barycenter(right_neighbors, positions);

    left_center
        .partial_cmp(&right_center)
        .unwrap_or(Ordering::Equal)
        .then_with(|| node_name_cmp(nodes, left, right))
}

fn barycenter(neighbors: &[Uuid], positions: &HashMap<Uuid, usize>) -> f64 {
    if neighbors.is_empty() {
        return f64::INFINITY;
    }

    let (sum, count) = neighbors
        .iter()
        .filter_map(|id| positions.get(id).copied())
        .fold((0.0, 0usize), |(sum, count), value| {
            (sum + value as f64, count + 1)
        });

    if count == 0 {
        f64::INFINITY
    } else {
        sum / count as f64
    }
}

fn build_edge_layout(
    edge: RelationGraphEdge,
    positioned: &HashMap<Uuid, PositionedNode>,
) -> Option<EdgeLayout> {
    let source = positioned.get(&edge.source)?;
    let target = positioned.get(&edge.target)?;
    let paths = vec![build_single_path(&edge, source, target)];

    Some(EdgeLayout { edge, paths })
}

fn build_single_path(
    edge: &RelationGraphEdge,
    source: &PositionedNode,
    target: &PositionedNode,
) -> EdgePath {
    let start_x = source.x + NODE_WIDTH;
    let start_y = anchor_y(source, edge.source_ident.as_deref());
    let end_x = target.x;
    let end_y = anchor_y(target, edge.target_ident.as_deref());
    let span = (end_x - start_x).max(72.0);
    let bend_x = start_x + span * 0.5;
    let d = format!(
        "M {:.3} {:.3} L {:.3} {:.3} L {:.3} {:.3} L {:.3} {:.3}",
        start_x, start_y, bend_x, start_y, bend_x, end_y, end_x, end_y
    );

    EdgePath {
        key: format!("{}-direct", edge.id),
        d,
        arrow: true,
    }
}

fn anchor_y(node: &PositionedNode, ident: Option<&str>) -> f64 {
    let Some(ident) = ident else {
        return node.y + NODE_HEADER_HEIGHT * 0.5;
    };

    let Some(index) = node
        .field_idents
        .iter()
        .position(|field_ident| field_ident == ident)
    else {
        return node.y + NODE_HEADER_HEIGHT * 0.5;
    };

    let field_y = node.y + NODE_HEADER_HEIGHT + index as f64 * NODE_FIELD_HEIGHT;
    let anchor = field_y + NODE_FIELD_HEIGHT * 0.5;
    let max_anchor = node.y + node.height - NODE_FIELD_HEIGHT * 0.5;

    anchor.min(max_anchor)
}

fn node_height(node: &RelationGraphNode) -> f64 {
    NODE_HEADER_HEIGHT + node.fields.len() as f64 * NODE_FIELD_HEIGHT
}

fn layer_height(layer_ids: &[Uuid], node_heights: &HashMap<Uuid, f64>) -> f64 {
    let nodes_height = layer_ids
        .iter()
        .map(|id| node_heights.get(id).copied().unwrap_or(NODE_HEADER_HEIGHT))
        .sum::<f64>();

    nodes_height + layer_ids.len().saturating_sub(1) as f64 * ROW_GAP
}

fn node_name_cmp(nodes: &[RelationGraphNode], left: &Uuid, right: &Uuid) -> Ordering {
    let left_name = nodes
        .iter()
        .find(|node| node.id == *left)
        .map(|node| node.name.as_str())
        .unwrap_or_default();
    let right_name = nodes
        .iter()
        .find(|node| node.id == *right)
        .map(|node| node.name.as_str())
        .unwrap_or_default();

    left_name.cmp(right_name)
}
