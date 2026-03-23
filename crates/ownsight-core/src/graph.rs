use serde::{Deserialize, Serialize};
use crate::model::{VariableId, FunctionId, ScopeId};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OwnershipGraph {
    pub nodes: Vec<GraphNode>,
    pub edges: Vec<GraphEdge>,
}

impl OwnershipGraph {
    pub fn new() -> Self {
        Self {
            nodes: Vec::new(),
            edges: Vec::new(),
        }
    }
    
    pub fn add_node(&mut self, node: GraphNode) {
        if !self.nodes.contains(&node) {
            self.nodes.push(node);
        }
    }
    
    pub fn add_edge(&mut self, edge: GraphEdge) {
        self.edges.push(edge);
    }
    
    pub fn get_edges_for_variable(&self, var_id: VariableId) -> Vec<&GraphEdge> {
        self.edges.iter()
            .filter(|e| {
                matches!(&e.source, GraphNode::Variable(id) if *id == var_id) ||
                matches!(&e.target, GraphNode::Variable(id) if *id == var_id)
            })
            .collect()
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum GraphNode {
    Variable(VariableId),
    Reference(VariableId),
    Function(FunctionId),
    Scope(ScopeId),
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GraphEdge {
    pub source: GraphNode,
    pub target: GraphNode,
    pub kind: EdgeKind,
    pub label: Option<String>,
}

impl GraphEdge {
    pub fn new(source: GraphNode, target: GraphNode, kind: EdgeKind) -> Self {
        Self {
            source,
            target,
            kind,
            label: None,
        }
    }
    
    pub fn with_label(mut self, label: String) -> Self {
        self.label = Some(label);
        self
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum EdgeKind {
    Owns,
    Borrows,
    MutablyBorrows,
    MovesTo,
    Reborrows,
    DropsAt,
    LivesInScope,
}
