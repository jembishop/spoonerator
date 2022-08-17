use serde::{Deserialize, Serialize};

#[derive(PartialEq, PartialOrd, Eq, Copy, Clone, Hash, Serialize, Deserialize)]
pub struct GraphNode {
    pub from: u16,
    pub to: u16,
    pub cut1: u8,
    pub cut2: u8
}

#[derive(Serialize, Deserialize)]
pub struct Graph {
    pub words: Vec<String>,
    pub nodes: Vec<GraphNode>
}