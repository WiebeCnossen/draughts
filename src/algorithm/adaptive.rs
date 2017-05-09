use algorithm::judge::Eval;
use algorithm::scope::Scope;

pub struct AdaptiveScope {
    depth: u8,
    forcing: bool,
    forced: u8,
    unforced: u8,
}

impl Scope for AdaptiveScope {
    fn from_depth(depth: u8) -> AdaptiveScope {
        AdaptiveScope {
            depth: depth,
            forcing: false,
            forced: 0,
            unforced: 0,
        }
    }

    fn next(&self, quiet: bool, gap: Eval) -> Option<AdaptiveScope> {
        if !quiet {
            Some(AdaptiveScope {
                     depth: self.depth,
                     forcing: true,
                     forced: self.forced + 1,
                     unforced: self.unforced,
                 })
        } else if self.forcing && self.depth > 2 * self.unforced {
            Some(AdaptiveScope {
                     depth: self.depth,
                     forcing: false,
                     forced: self.forced,
                     unforced: self.unforced + 1,
                 })
        } else if self.depth > self.forced + self.unforced && gap < 500 {
            Some(AdaptiveScope::from_depth(self.depth - self.forced - self.unforced - 1))
        } else {
            None
        }
    }

    fn depth(&self) -> u8 {
        self.depth
    }
}
