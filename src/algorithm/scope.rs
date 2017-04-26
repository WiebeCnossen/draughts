use engine::judge::Eval;

pub trait Scope : Sized {
  fn next(&self, quiet: bool, gap: Eval) -> Option<Self>;
  fn depth(&self) -> u8;
}

pub struct DepthScope {
  depth: u8,
  quiet_moves: u8,
  forced: u8,
  unforced: u8
}

impl DepthScope {
  pub fn from_depth(depth: u8) -> DepthScope {
    DepthScope {
      depth: depth,
      quiet_moves: 1,
      forced: 0,
      unforced: 0
    }
  }
}

impl Scope for DepthScope {
  fn next(&self, quiet: bool, gap: Eval) -> Option<DepthScope> {
    if !quiet {
      Some(DepthScope {
        depth: self.depth,
        quiet_moves: 0,
        forced: self.forced + 1,
        unforced: self.unforced
      })
    }
    else if self.quiet_moves == 0 && self.depth > 2 * self.unforced {
      Some(DepthScope {
        depth: self.depth,
        quiet_moves: 1,
        forced: self.forced,
        unforced: self.unforced + 1
      })
    }
    else if gap < 500 * self.depth as Eval && self.depth > self.forced + self.unforced {
      Some(DepthScope {
        depth: self.depth - self.forced - self.unforced - 1,
        quiet_moves: self.quiet_moves + 1,
        forced: 0,
        unforced: 0
      })
    }
    else {
      None
    }
  }
  fn depth(&self) -> u8 { self.depth }
}
