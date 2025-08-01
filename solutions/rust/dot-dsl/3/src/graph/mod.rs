use std::collections::HashMap;

pub mod graph_items {
    pub mod edge;
    pub mod node;
}

#[derive(Default)]
pub struct Graph {
    pub nodes: Vec<graph_items::node::Node>,
    pub edges: Vec<graph_items::edge::Edge>,
    pub attrs: HashMap<String, String>,
}

impl Graph {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn with_nodes(mut self, nodes: &[graph_items::node::Node]) -> Self {
        self.nodes = nodes.to_vec();
        self
    }

    pub fn with_edges(mut self, edges: &[graph_items::edge::Edge]) -> Self {
        self.edges = edges.to_vec();
        self
    }

    pub fn with_attrs(mut self, attrs: &[(&str, &str)]) -> Self {
        self.attrs = attrs
            .iter()
            .map(|(k, v)| (k.to_string(), v.to_string()))
            .collect();
        self
    }

    pub fn node(&self, name: &str) -> Option<&graph_items::node::Node> {
        self.nodes.iter().find(|node| node.id == name)
    }
}