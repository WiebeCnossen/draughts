use algorithm::scope::Depth;

pub trait Metric {
    fn get_nodes(&self) -> usize;
    fn add_nodes(&mut self, increment: usize);
    fn get_depth(&self) -> Depth;
    fn put_depth(&mut self, depth: Depth);
}

#[derive(Clone)]
pub struct Meta {
    depth: Depth,
    nodes: usize,
}

impl Meta {
    pub fn create() -> Meta {
        Meta { depth: 0, nodes: 0 }
    }
}

impl Metric for Meta {
    fn get_nodes(&self) -> usize {
        self.nodes
    }
    fn add_nodes(&mut self, increment: usize) {
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
