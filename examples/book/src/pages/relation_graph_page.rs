use crate::code_example::CodeExample;
use birei::{Card, RelationGraph, RelationGraphEdge, RelationGraphNode, RelationGraphNodeField};
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
            tenant,
            account,
            "owns account",
        ),
        RelationGraphEdge::new(
            id("c2222222-2222-2222-2222-222222222222"),
            tenant,
            workspace,
            "hosts workspace",
        ),
        RelationGraphEdge::new(
            id("c3333333-3333-3333-3333-333333333333"),
            tenant,
            invoice,
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
                        account,
                        contract,
                        "governs contract",
                    ),
                );
                push_edge(
                    items,
                    RelationGraphEdge::new(
                        id("c5555555-5555-5555-5555-555555555555"),
                        account,
                        billing,
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
                        invoice,
                        payment,
                        "creates payment",
                    ),
                );
                push_edge(
                    items,
                    RelationGraphEdge::new(
                        id("c6666667-6666-6666-6666-666666666667"),
                        invoice,
                        reminder,
                        "creates reminder",
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
                        .description(
                            "Permission profile applied across users inside the workspace.",
                        )
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
                        workspace,
                        owner,
                        "contains owner",
                    ),
                );
                push_edge(
                    items,
                    RelationGraphEdge::new(
                        id("c7777778-7777-7777-7777-777777777778"),
                        workspace,
                        role,
                        "contains role",
                    ),
                );
                push_edge(
                    items,
                    RelationGraphEdge::new(
                        id("c7777779-7777-7777-7777-777777777779"),
                        workspace,
                        audit,
                        "contains audit trail",
                    ),
                );
                push_edge(
                    items,
                    RelationGraphEdge::new(
                        id("c8888888-8888-8888-8888-888888888888"),
                        contract,
                        workspace,
                        "grants workspace access",
                    ),
                );
                push_edge(
                    items,
                    RelationGraphEdge::new(
                        id("c8888889-8888-8888-8888-888888888889"),
                        role,
                        workspace,
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

    let uml_nodes = vec![
        RelationGraphNode::new(tenant, "building-2", "Tenant")
            .description("Root workspace container for the whole customer installation.")
            .fields(vec![
                RelationGraphNodeField::new("ident", "Ident", "String"),
                RelationGraphNodeField::new("account", "Account", "Reference").highlighted(true),
                RelationGraphNodeField::new("workspace", "Workspace", "Reference")
                    .highlighted(true),
                RelationGraphNodeField::new("invoice", "Invoice", "Reference").highlighted(true),
            ])
            .loaded(true),
        RelationGraphNode::new(account, "badge-euro", "Account")
            .description("Financial account that owns contracts and billing records.")
            .fields(vec![
                RelationGraphNodeField::new("name", "Name", "String"),
                RelationGraphNodeField::new("contract", "Contract", "Reference")
                    .highlighted(true),
                RelationGraphNodeField::new("billing_address", "Billing Address", "Reference")
                    .highlighted(true),
            ])
            .loaded(true),
        RelationGraphNode::new(workspace, "layout-panel-left", "Workspace")
            .description("Application area that groups users, roles, and audit trails.")
            .fields(vec![
                RelationGraphNodeField::new("owner", "Owner", "Reference").highlighted(true),
                RelationGraphNodeField::new("role", "Role", "List").highlighted(true),
                RelationGraphNodeField::new("audit", "Audit Trail", "Table").highlighted(true),
            ])
            .loaded(true),
        RelationGraphNode::new(invoice, "receipt-text", "Invoice")
            .description("Billing document that can fan out into payments and reminders.")
            .fields(vec![
                RelationGraphNodeField::new("number", "Number", "String"),
                RelationGraphNodeField::new("payment", "Payment", "Reference").highlighted(true),
                RelationGraphNodeField::new("reminder", "Reminder", "Reference").highlighted(true),
            ])
            .loaded(true),
        RelationGraphNode::new(contract, "file-signature", "Contract")
            .description("Commercial agreement attached to the financial account.")
            .fields(vec![
                RelationGraphNodeField::new("workspace", "Workspace", "Reference")
                    .highlighted(true),
                RelationGraphNodeField::new("valid_until", "Valid Until", "DateTime"),
            ])
            .loaded(true),
        RelationGraphNode::new(billing, "map-pinned", "Billing Address")
            .description("Postal record used for tax and invoice delivery.")
            .fields(vec![
                RelationGraphNodeField::new("street", "Street", "String"),
                RelationGraphNodeField::new("country", "Country", "Location"),
            ])
            .loaded(true),
        RelationGraphNode::new(payment, "wallet-cards", "Payment")
            .description("Captured transaction linked back to the invoice.")
            .fields(vec![
                RelationGraphNodeField::new("amount", "Amount", "Currency"),
                RelationGraphNodeField::new("captured_at", "Captured At", "DateTime"),
            ])
            .loaded(true),
        RelationGraphNode::new(reminder, "bell-ring", "Reminder")
            .description("Escalation notice sent when the invoice remains open.")
            .fields(vec![
                RelationGraphNodeField::new("level", "Level", "Integer"),
                RelationGraphNodeField::new("sent_at", "Sent At", "DateTime"),
            ])
            .loaded(true),
        RelationGraphNode::new(owner, "user-round", "Owner")
            .description("Human account with the highest administrative privileges.")
            .fields(vec![
                RelationGraphNodeField::new("email", "Email", "String"),
                RelationGraphNodeField::new("language", "Language", "Selection"),
            ])
            .loaded(true),
        RelationGraphNode::new(role, "shield-check", "Role")
            .description("Permission profile applied across users inside the workspace.")
            .fields(vec![
                RelationGraphNodeField::new("name", "Name", "String"),
                RelationGraphNodeField::new("workspace", "Workspace", "Reference")
                    .highlighted(true),
            ])
            .loaded(true),
        RelationGraphNode::new(audit, "scroll-text", "Audit Trail")
            .description("Immutable activity stream for security-sensitive changes.")
            .fields(vec![
                RelationGraphNodeField::new("actor", "Actor", "Reference").highlighted(true),
                RelationGraphNodeField::new("changed_at", "Changed At", "DateTime"),
            ])
            .loaded(true),
    ];
    let uml_edges = vec![
        RelationGraphEdge::new(
            id("d1111111-1111-1111-1111-111111111111"),
            tenant,
            account,
            "Account",
        )
        .source_ident("account"),
        RelationGraphEdge::new(
            id("d2222222-2222-2222-2222-222222222222"),
            tenant,
            workspace,
            "Workspace",
        )
        .source_ident("workspace"),
        RelationGraphEdge::new(
            id("d3333333-3333-3333-3333-333333333333"),
            tenant,
            invoice,
            "Invoice",
        )
        .source_ident("invoice"),
        RelationGraphEdge::new(
            id("d4444444-4444-4444-4444-444444444444"),
            account,
            contract,
            "Contract",
        )
        .source_ident("contract"),
        RelationGraphEdge::new(
            id("d5555555-5555-5555-5555-555555555555"),
            account,
            billing,
            "Billing Address",
        )
        .source_ident("billing_address"),
        RelationGraphEdge::new(
            id("d6666666-6666-6666-6666-666666666666"),
            invoice,
            payment,
            "Payment",
        )
        .source_ident("payment"),
        RelationGraphEdge::new(
            id("d6666667-6666-6666-6666-666666666667"),
            invoice,
            reminder,
            "Reminder",
        )
        .source_ident("reminder"),
        RelationGraphEdge::new(
            id("d7777777-7777-7777-7777-777777777777"),
            workspace,
            owner,
            "Owner",
        )
        .source_ident("owner"),
        RelationGraphEdge::new(
            id("d7777778-7777-7777-7777-777777777778"),
            workspace,
            role,
            "Role (List)",
        )
        .source_ident("role"),
        RelationGraphEdge::new(
            id("d7777779-7777-7777-7777-777777777779"),
            workspace,
            audit,
            "Audit Trail (Table)",
        )
        .source_ident("audit"),
        RelationGraphEdge::new(
            id("d8888888-8888-8888-8888-888888888888"),
            contract,
            workspace,
            "Workspace",
        )
        .source_ident("workspace"),
        RelationGraphEdge::new(
            id("d8888889-8888-8888-8888-888888888889"),
            role,
            workspace,
            "Workspace",
        )
        .source_ident("workspace"),
        RelationGraphEdge::new(
            id("d9999999-9999-9999-9999-999999999999"),
            audit,
            owner,
            "Actor",
        )
        .source_ident("actor"),
    ];

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
    RelationGraphEdge::new(edge_id, tenant_id, account_id, "owns account"),
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
            <Card header="Field anchored graph" class="doc-card">
                <span class="doc-card__kicker">"UML style"</span>
                <div class="doc-card__preview doc-card__preview--stack">
                    <RelationGraph
                        aria_label="UML style relation graph"
                        nodes=uml_nodes
                        edges=uml_edges
                        height=String::from("34rem")
                    />
                </div>
                <CodeExample code={r#"let nodes = vec![
    RelationGraphNode::new(tenant_id, "building-2", "Tenant")
        .fields(vec![
            RelationGraphNodeField::new("account", "Account", "Reference").highlighted(true),
            RelationGraphNodeField::new("invoice", "Invoice", "Reference").highlighted(true),
        ])
        .loaded(true),
    RelationGraphNode::new(account_id, "badge-euro", "Account").loaded(true),
];
let edges = vec![
    RelationGraphEdge::new(edge_id, tenant_id, account_id, "Account")
        .source_ident("account"),
];

view! {
    <RelationGraph nodes=nodes edges=edges />
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
