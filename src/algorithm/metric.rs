use algorithm::scope::Depth;

pub type Nodes = usize;
pub trait Metric {
    fn get_nodes(&self) -> Nodes;
    fn add_nodes(&mut self, increment: Nodes);
    fn get_depth(&self) -> Depth;
    fn put_depth(&mut self, depth: Depth);
}

#[derive(Clone)]
pub struct Meta {
    depth: Depth,
    nodes: Nodes,
}

impl Meta {
    pub fn create() -> Meta {
        Meta { depth: 0, nodes: 0 }
    }
}

impl Metric for Meta {
    fn get_nodes(&self) -> Nodes {
        self.nodes
    }
    fn add_nodes(&mut self, increment: Nodes) {
        self.nodes = self.nodes + increment
    }
    fn get_depth(&self) -> Depth {
        self.depth
    }
    fn put_depth(&mut self, depth: Depth) {
        if self.depth < depth {
            self.depth = depth
        }
    }
}
