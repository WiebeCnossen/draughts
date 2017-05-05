use engine::judge::Eval;

pub trait Scope: Sized {
    fn next(&self, quiet: bool, gap: Eval) -> Option<Self>;
    fn depth(&self) -> u8;
}

pub struct DepthScope {
    depth: u8,
    forcing: bool,
    forced: u8,
    unforced: u8,
}

impl DepthScope {
    pub fn from_depth(depth: u8) -> DepthScope {
        DepthScope {
            depth: depth,
            forcing: false,
            forced: 0,
            unforced: 0,
        }
    }
}

impl Scope for DepthScope {
    fn next(&self, quiet: bool, gap: Eval) -> Option<DepthScope> {
        if !quiet {
            Some(DepthScope {
                     depth: self.depth,
                     forcing: true,
                     forced: self.forced + 1,
                     unforced: self.unforced,
                 })
        } else if self.forcing && self.depth > 2 * self.unforced {
            Some(DepthScope {
                     depth: self.depth,
                     forcing: false,
                     forced: self.forced,
                     unforced: self.unforced + 1,
                 })
        } else if self.depth > self.forced + self.unforced &&
                  gap < 100 + 500 * ((self.depth - self.forced - self.unforced - 1) / 5) as Eval {
            Some(DepthScope {
                     depth: self.depth - self.forced - self.unforced - 1,
                     forcing: false,
                     forced: 0,
                     unforced: 0,
                 })
        } else {
            None
        }
    }
    fn depth(&self) -> u8 {
        self.depth
    }
}
