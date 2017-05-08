pub trait Metric {
    fn get_nodes(&self) -> usize;
    fn add_nodes(&mut self, increment: usize);
    fn get_depth(&self) -> u8;
    fn put_depth(&mut self, depth: u8);
}

pub struct Meta {
    depth: u8,
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
    fn get_depth(&self) -> u8 {
        self.depth
    }
    fn put_depth(&mut self, depth: u8) {
        if self.depth < depth {
            self.depth = depth
        }
    }
}
