use super::scope::Depth;

pub type Nodes = usize;

#[derive(Clone)]
pub struct Meta {
    depth: Depth,
    nodes: Nodes,
}

impl Meta {
    pub fn create() -> Meta {
        Meta { depth: 0, nodes: 0 }
    }
    pub fn get_nodes(&self) -> Nodes {
        self.nodes
    }
    pub fn add_nodes(&mut self, increment: Nodes) {
        self.nodes += increment
    }
    pub fn get_depth(&self) -> Depth {
        self.depth
    }
    pub fn put_depth(&mut self, depth: Depth) {
        if self.depth < depth {
            self.depth = depth
        }
    }
}
