use uuid::Uuid;

/// Single graph node rendered as a card in the relation graph.
#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub struct RelationGraphNode {
    pub id: Uuid,
    pub icon: String,
    pub name: String,
    pub description: String,
    pub loaded: bool,
}

impl RelationGraphNode {
    /// Creates a new graph node with the required visible fields.
    pub fn new(id: Uuid, icon: impl Into<String>, name: impl Into<String>) -> Self {
        Self {
            id,
            icon: icon.into(),
            name: name.into(),
            description: String::new(),
            loaded: false,
        }
    }

    /// Sets the longer descriptive text exposed in the node hover popup.
    #[must_use]
    pub fn description(mut self, description: impl Into<String>) -> Self {
        self.description = description.into();
        self
    }

    /// Marks whether this node has already loaded its adjacent relations.
    #[must_use]
    pub fn loaded(mut self, loaded: bool) -> Self {
        self.loaded = loaded;
        self
    }
}

/// Directed relation between one or more source and target nodes.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct RelationGraphEdge {
    pub id: Uuid,
    pub sources: Vec<Uuid>,
    pub targets: Vec<Uuid>,
    pub name: String,
}

impl RelationGraphEdge {
    /// Creates a new directed relation edge.
    pub fn new(
        id: Uuid,
        sources: Vec<Uuid>,
        targets: Vec<Uuid>,
        name: impl Into<String>,
    ) -> Self {
        Self {
            id,
            sources,
            targets,
            name: name.into(),
        }
    }
}
