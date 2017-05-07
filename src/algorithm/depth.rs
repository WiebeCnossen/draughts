use algorithm::judge::Eval;
use algorithm::scope::Scope;

pub struct DepthScope {
    depth: u8,
}

impl Scope for DepthScope {
    fn from_depth(depth: u8) -> DepthScope {
        DepthScope { depth: depth }
    }

    fn next(&self, quiet: bool, _: Eval) -> Option<DepthScope> {
        match (quiet, self.depth > 0) {
            (false, false) => Some(DepthScope::from_depth(0)),
            (_, true) => Some(DepthScope::from_depth(self.depth - 1)),
            _ => None,
        }
    }

    fn depth(&self) -> u8 {
        self.depth
    }
}
