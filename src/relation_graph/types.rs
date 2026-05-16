use uuid::Uuid;

/// Single graph node rendered as a card in the relation graph.
#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub struct RelationGraphNode {
    pub id: Uuid,
    pub icon: String,
    pub name: String,
    pub description: String,
    pub loaded: bool,
    pub fields: Vec<RelationGraphNodeField>,
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
            fields: Vec::new(),
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

    /// Sets structured rows shown below the node header.
    #[must_use]
    pub fn fields(mut self, fields: Vec<RelationGraphNodeField>) -> Self {
        self.fields = fields;
        self
    }
}

/// Structured field row rendered inside a relation graph node.
#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub struct RelationGraphNodeField {
    pub ident: String,
    pub name: String,
    pub typ: String,
    pub highlighted: bool,
}

impl RelationGraphNodeField {
    /// Creates a new graph node field.
    pub fn new(ident: impl Into<String>, name: impl Into<String>, typ: impl Into<String>) -> Self {
        Self {
            ident: ident.into(),
            name: name.into(),
            typ: typ.into(),
            highlighted: false,
        }
    }

    /// Marks the field as visually highlighted.
    #[must_use]
    pub fn highlighted(mut self, highlighted: bool) -> Self {
        self.highlighted = highlighted;
        self
    }
}

/// Directed relation between a source and target node.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct RelationGraphEdge {
    pub id: Uuid,
    pub source: Uuid,
    pub source_ident: Option<String>,
    pub target: Uuid,
    pub target_ident: Option<String>,
    pub name: String,
}

impl RelationGraphEdge {
    /// Creates a new directed relation edge.
    pub fn new(id: Uuid, source: Uuid, target: Uuid, name: impl Into<String>) -> Self {
        Self {
            id,
            source,
            source_ident: None,
            target,
            target_ident: None,
            name: name.into(),
        }
    }

    /// Anchors the source side of the edge to the given source field.
    #[must_use]
    pub fn source_ident(mut self, ident: impl Into<String>) -> Self {
        self.source_ident = Some(ident.into());
        self
    }

    /// Anchors the target side of the edge to the given target field.
    #[must_use]
    pub fn target_ident(mut self, ident: impl Into<String>) -> Self {
        self.target_ident = Some(ident.into());
        self
    }
}
