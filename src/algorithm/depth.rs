use algorithm::judge::Eval;
use algorithm::scope::{Depth, Scope};

pub struct DepthScope {
    depth: Depth,
}

impl Scope for DepthScope {
    fn from_depth(depth: Depth) -> DepthScope {
        DepthScope { depth: depth }
    }

    fn next(&self, quiet: bool, _: Eval) -> Option<DepthScope> {
        match (quiet, self.depth > 0) {
            (false, false) => Some(DepthScope::from_depth(0)),
            (_, true) => Some(DepthScope::from_depth(self.depth - 1)),
            _ => None,
        }
    }

    fn depth(&self) -> Depth {
        self.depth
    }
}
