use crate::code_example::CodeExample;
use birei::{Card, RelationGraph, RelationGraphEdge, RelationGraphNode};
use leptos::prelude::*;
use uuid::Uuid;

#[component]
pub fn RelationGraphPage() -> impl IntoView {
    let tenant = id("11111111-1111-1111-1111-111111111111");
    let account = id("22222222-2222-2222-2222-222222222222");
    let workspace = id("33333333-3333-3333-3333-333333333333");
    let invoice = id("44444444-4444-4444-4444-444444444444");
    let contract = id("55555555-5555-5555-5555-555555555555");
    let owner = id("66666666-6666-6666-6666-666666666666");
    let billing = id("77777777-7777-7777-7777-777777777777");
    let payment = id("88888888-8888-8888-8888-888888888888");
    let reminder = id("99999999-9999-9999-9999-999999999999");
    let role = id("aaaaaaaa-aaaa-aaaa-aaaa-aaaaaaaaaaaa");
    let audit = id("bbbbbbbb-bbbb-bbbb-bbbb-bbbbbbbbbbbb");

    let nodes = RwSignal::new(vec![
        RelationGraphNode::new(tenant, "building-2", "Tenant")
            .description("Root workspace container for the whole customer installation.")
            .loaded(true),
        RelationGraphNode::new(account, "badge-euro", "Account")
            .description("Financial account that owns contracts and billing records."),
        RelationGraphNode::new(workspace, "layout-panel-left", "Workspace")
            .description("Application area that groups users, roles, and audit trails."),
        RelationGraphNode::new(invoice, "receipt-text", "Invoice")
            .description("Billing document that can fan out into payments and reminders."),
    ]);
    let edges = RwSignal::new(vec![
        RelationGraphEdge::new(
            id("c1111111-1111-1111-1111-111111111111"),
            vec![tenant],
            vec![account],
            "owns account",
        ),
        RelationGraphEdge::new(
            id("c2222222-2222-2222-2222-222222222222"),
            vec![tenant],
            vec![workspace],
            "hosts workspace",
        ),
        RelationGraphEdge::new(
            id("c3333333-3333-3333-3333-333333333333"),
            vec![tenant],
            vec![invoice],
            "issues invoice",
        ),
    ]);
    let opened_node = RwSignal::new(String::from("Nothing opened yet."));

    let on_load_node = Callback::new(move |node_id: Uuid| {
        if node_id == account {
            nodes.update(|items| {
                mark_loaded(items, account);
                push_node(
                    items,
                    RelationGraphNode::new(contract, "file-signature", "Contract")
                        .description("Commercial agreement attached to the financial account.")
                        .loaded(true),
                );
                push_node(
                    items,
                    RelationGraphNode::new(billing, "map-pinned", "Billing Address")
                        .description("Postal record used for tax and invoice delivery.")
                        .loaded(true),
                );
            });
            edges.update(|items| {
                push_edge(
                    items,
                    RelationGraphEdge::new(
                        id("c4444444-4444-4444-4444-444444444444"),
                        vec![account],
                        vec![contract],
                        "governs contract",
                    ),
                );
                push_edge(
                    items,
                    RelationGraphEdge::new(
                        id("c5555555-5555-5555-5555-555555555555"),
                        vec![account],
                        vec![billing],
                        "uses billing address",
                    ),
                );
            });
        } else if node_id == invoice {
            nodes.update(|items| {
                mark_loaded(items, invoice);
                push_node(
                    items,
                    RelationGraphNode::new(payment, "wallet-cards", "Payment")
                        .description("Captured transaction linked back to the invoice.")
                        .loaded(true),
                );
                push_node(
                    items,
                    RelationGraphNode::new(reminder, "bell-ring", "Reminder")
                        .description("Escalation notice sent when the invoice remains open.")
                        .loaded(true),
                );
            });
            edges.update(|items| {
                push_edge(
                    items,
                    RelationGraphEdge::new(
                        id("c6666666-6666-6666-6666-666666666666"),
                        vec![invoice],
                        vec![payment, reminder],
                        "creates follow-up records",
                    ),
                );
            });
        } else if node_id == workspace {
            nodes.update(|items| {
                mark_loaded(items, workspace);
                push_node(
                    items,
                    RelationGraphNode::new(owner, "user-round", "Owner")
                        .description("Human account with the highest administrative privileges.")
                        .loaded(true),
                );
                push_node(
                    items,
                    RelationGraphNode::new(role, "shield-check", "Role")
                        .description("Permission profile applied across users inside the workspace.")
                        .loaded(true),
                );
                push_node(
                    items,
                    RelationGraphNode::new(audit, "scroll-text", "Audit Trail")
                        .description("Immutable activity stream for security-sensitive changes.")
                        .loaded(true),
                );
            });
            edges.update(|items| {
                push_edge(
                    items,
                    RelationGraphEdge::new(
                        id("c7777777-7777-7777-7777-777777777777"),
                        vec![workspace],
                        vec![owner, role, audit],
                        "contains workspace data",
                    ),
                );
                push_edge(
                    items,
                    RelationGraphEdge::new(
                        id("c8888888-8888-8888-8888-888888888888"),
                        vec![contract, role],
                        vec![workspace],
                        "grants workspace access",
                    ),
                );
            });
        }
    });
    let on_open_node = Callback::new(move |node_id: Uuid| {
        let label = nodes
            .get_untracked()
            .into_iter()
            .find(|node| node.id == node_id)
            .map(|node| node.name)
            .unwrap_or_else(|| String::from("Unknown"));
        opened_node.set(format!("Opened node: {label}"));
    });

    view! {
        <section class="page-header">
            <div class="page-header__eyebrow">"Component"</div>
            <h2>"Relation Graph"</h2>
            <p class="page-header__lede">
                "Left-to-right dependency graph with bundled directed edges, edge hover details, pan and zoom, and lazy node expansion."
            </p>
        </section>

        <section class="doc-grid">
            <Card header="Lazy relation loading" class="doc-card">
                <span class="doc-card__kicker">"Interactive"</span>
                <div class="doc-card__preview doc-card__preview--stack">
                    <p>{move || opened_node.get()}</p>
                    <RelationGraph
                        aria_label="Tenant relation graph"
                        nodes=nodes
                        edges=edges
                        on_load_node=on_load_node
                        on_open_node=on_open_node
                    />
                </div>
                <CodeExample code={r#"let nodes = vec![
    RelationGraphNode::new(tenant_id, "building-2", "Tenant").loaded(true),
    RelationGraphNode::new(account_id, "badge-euro", "Account"),
];
let edges = vec![
    RelationGraphEdge::new(edge_id, vec![tenant_id], vec![account_id], "owns account"),
];

view! {
    <RelationGraph
        nodes=nodes
        edges=edges
        on_load_node=Callback::new(move |node_id| {
            // Fetch more relations for `node_id` and then pass updated node and edge lists back in.
        })
        on_open_node=Callback::new(move |node_id| {
            // Open the already loaded node in your surrounding app.
        })
    />
}"#}/>
            </Card>
        </section>
    }
}

fn id(value: &str) -> Uuid {
    Uuid::parse_str(value).expect("static relation graph ids should be valid UUIDs")
}

fn mark_loaded(nodes: &mut [RelationGraphNode], node_id: Uuid) {
    if let Some(node) = nodes.iter_mut().find(|node| node.id == node_id) {
        node.loaded = true;
    }
}

fn push_node(nodes: &mut Vec<RelationGraphNode>, node: RelationGraphNode) {
    if nodes.iter().all(|existing| existing.id != node.id) {
        nodes.push(node);
    }
}

fn push_edge(edges: &mut Vec<RelationGraphEdge>, edge: RelationGraphEdge) {
    if edges.iter().all(|existing| existing.id != edge.id) {
        edges.push(edge);
    }
}
