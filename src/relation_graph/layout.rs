use std::cmp::Ordering;
use std::collections::{HashMap, HashSet, VecDeque};

use uuid::Uuid;

use super::internal::{
    EdgeLayout, EdgePath, GRAPH_PADDING_X, GRAPH_PADDING_Y, GRAPH_TRAILING_PADDING, LAYER_GAP,
    NODE_HEIGHT, NODE_WIDTH, PositionedNode, ROW_GAP, RelationGraphLayout, NodeLayout,
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
            !edge.sources.is_empty()
                && !edge.targets.is_empty()
                && edge.sources.iter().all(|id| valid_nodes.contains_key(id))
                && edge.targets.iter().all(|id| valid_nodes.contains_key(id))
        })
        .collect::<Vec<_>>();

    let layers = assign_layers(&nodes, &edges);
    let ordering = order_layers(&nodes, &edges, &layers);
    let max_layer = layers.values().copied().max().unwrap_or_default();
    let lane_count = ordering.iter().map(Vec::len).max().unwrap_or(1);
    let height = ((lane_count as f64 * NODE_HEIGHT)
        + (lane_count.saturating_sub(1) as f64 * ROW_GAP)
        + (GRAPH_PADDING_Y * 2.0))
        .max((NODE_HEIGHT + (GRAPH_PADDING_Y * 2.0)).ceil());
    let width = ((max_layer as f64 * (NODE_WIDTH + LAYER_GAP))
        + NODE_WIDTH
        + GRAPH_PADDING_X * 2.0
        + GRAPH_TRAILING_PADDING)
        .max((NODE_WIDTH + GRAPH_PADDING_X * 2.0 + GRAPH_TRAILING_PADDING).ceil());

    let mut positioned = HashMap::<Uuid, PositionedNode>::new();

    for (layer_index, layer_ids) in ordering.iter().enumerate() {
        let layer_height = layer_ids.len() as f64 * NODE_HEIGHT
            + layer_ids.len().saturating_sub(1) as f64 * ROW_GAP;
        let base_y = GRAPH_PADDING_Y + ((height - (GRAPH_PADDING_Y * 2.0) - layer_height) * 0.5);

        for (row_index, id) in layer_ids.iter().enumerate() {
            let x = GRAPH_PADDING_X + layer_index as f64 * (NODE_WIDTH + LAYER_GAP);
            let y = base_y + row_index as f64 * (NODE_HEIGHT + ROW_GAP);

            positioned.insert(
                *id,
                PositionedNode {
                    id: *id,
                    x,
                    y,
                    layer: layer_index,
                    row: row_index,
                },
            );
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

fn assign_layers(
    nodes: &[RelationGraphNode],
    edges: &[RelationGraphEdge],
) -> HashMap<Uuid, usize> {
    let mut incoming = HashMap::<Uuid, HashSet<Uuid>>::new();
    let mut outgoing = HashMap::<Uuid, HashSet<Uuid>>::new();
    let mut indegree = HashMap::<Uuid, usize>::new();

    for node in nodes {
        incoming.insert(node.id, HashSet::new());
        outgoing.insert(node.id, HashSet::new());
        indegree.insert(node.id, 0);
    }

    for edge in edges {
        for source in &edge.sources {
            for target in &edge.targets {
                if source == target {
                    continue;
                }
                if outgoing.entry(*source).or_default().insert(*target) {
                    incoming.entry(*target).or_default().insert(*source);
                    *indegree.entry(*target).or_default() += 1;
                }
            }
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
        for source in &edge.sources {
            for target in &edge.targets {
                if source == target {
                    continue;
                }
                outgoing.entry(*source).or_default().push(*target);
                incoming.entry(*target).or_default().push(*source);
            }
        }
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
        .flat_map(|layer| layer.iter().copied().enumerate().map(|(index, id)| (id, index)))
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
    let mut sources = edge
        .sources
        .iter()
        .filter_map(|id| positioned.get(id))
        .cloned()
        .collect::<Vec<_>>();
    let mut targets = edge
        .targets
        .iter()
        .filter_map(|id| positioned.get(id))
        .cloned()
        .collect::<Vec<_>>();

    if sources.is_empty() || targets.is_empty() {
        return None;
    }

    sources.sort_by(|left, right| left.y.partial_cmp(&right.y).unwrap_or(Ordering::Equal));
    targets.sort_by(|left, right| left.y.partial_cmp(&right.y).unwrap_or(Ordering::Equal));

    let paths = if sources.len() == 1 && targets.len() == 1 {
        vec![build_single_path(&edge.id, &sources[0], &targets[0])]
    } else {
        build_grouped_paths(&edge.id, &sources, &targets)
    };

    Some(EdgeLayout { edge, paths })
}

fn build_single_path(edge_id: &Uuid, source: &PositionedNode, target: &PositionedNode) -> EdgePath {
    let start_x = source.x + NODE_WIDTH;
    let start_y = source.y + NODE_HEIGHT * 0.5;
    let end_x = target.x;
    let end_y = target.y + NODE_HEIGHT * 0.5;
    let control = ((end_x - start_x) * 0.42).max(52.0);
    let d = format!(
        "M {:.3} {:.3} C {:.3} {:.3}, {:.3} {:.3}, {:.3} {:.3}",
        start_x,
        start_y,
        start_x + control,
        start_y,
        end_x - control,
        end_y,
        end_x,
        end_y
    );

    EdgePath {
        key: format!("{edge_id}-direct"),
        d,
        arrow: true,
    }
}

fn build_grouped_paths(
    edge_id: &Uuid,
    sources: &[PositionedNode],
    targets: &[PositionedNode],
) -> Vec<EdgePath> {
    let source_exit_x = sources
        .iter()
        .map(|node| node.x + NODE_WIDTH)
        .fold(0.0_f64, f64::max);
    let target_entry_x = targets
        .iter()
        .map(|node| node.x)
        .fold(f64::INFINITY, f64::min);
    let span = (target_entry_x - source_exit_x).max(120.0);
    let bundle_x = source_exit_x + (span * 0.5);

    let mut paths = Vec::new();
    let mut y_values = Vec::new();

    for source in sources {
        let y = source.y + NODE_HEIGHT * 0.5;
        y_values.push(y);
        paths.push(EdgePath {
            key: format!("{edge_id}-source-{}", source.id),
            d: format!(
                "M {:.3} {:.3} C {:.3} {:.3}, {:.3} {:.3}, {:.3} {:.3}",
                source.x + NODE_WIDTH,
                y,
                source.x + NODE_WIDTH + 32.0,
                y,
                bundle_x - 18.0,
                y,
                bundle_x,
                y
            ),
            arrow: false,
        });
    }

    for target in targets {
        let y = target.y + NODE_HEIGHT * 0.5;
        y_values.push(y);
        paths.push(EdgePath {
            key: format!("{edge_id}-target-{}", target.id),
            d: format!(
                "M {:.3} {:.3} C {:.3} {:.3}, {:.3} {:.3}, {:.3} {:.3}",
                bundle_x,
                y,
                bundle_x + 18.0,
                y,
                target.x - 32.0,
                y,
                target.x,
                y
            ),
            arrow: true,
        });
    }

    if let (Some(min_y), Some(max_y)) = (
        y_values.iter().copied().reduce(f64::min),
        y_values.iter().copied().reduce(f64::max),
    ) {
        if (max_y - min_y).abs() > 1.0 {
            paths.push(EdgePath {
                key: format!("{edge_id}-bundle"),
                d: format!("M {:.3} {:.3} L {:.3} {:.3}", bundle_x, min_y, bundle_x, max_y),
                arrow: false,
            });
        }
    }

    paths
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
