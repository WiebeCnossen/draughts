use algorithm::metric::Metric;
use algorithm::scope::Scope;
use board::position::Game;
use engine::judge::{Eval, MIN_EVAL, Judge};

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
  fn next(&self, quiet: bool) -> Option<DepthScope> {
    if !quiet {
      Some(DepthScope {
        depth: self.depth,
        quiet_moves: 0,
        forced: self.forced + 1,
        unforced: self.unforced
      })
    }
    else if self.quiet_moves == 0 && self.depth > self.unforced {
      Some(DepthScope {
        depth: self.depth,
        quiet_moves: 1,
        forced: self.forced,
        unforced: self.unforced + 1
      })
    }
    else if self.depth > self.forced + self.unforced {
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
}

pub fn makes_cut<TGame, TScope>(judge: &Judge, metric: &mut Metric, position: &TGame, scope: &TScope, cut: Eval) -> Eval where TGame : Game, TScope : Scope {
  if cut <= MIN_EVAL { return MIN_EVAL }

  let moves = judge.moves(position);
  metric.add_nodes(moves.len());
  if moves.len() == 0 { return MIN_EVAL }

  let quiet = judge.quiet_position(position, &moves);
  match scope.next(quiet) {
    None => judge.evaluate(position),
    Some(_) => {
      let mut best = MIN_EVAL;
      for mv in moves {
        let quiet = judge.quiet_move(position, &mv);
        let score =
          match scope.next(quiet) {
            None => judge.evaluate(position),
            Some(next) => {
              -makes_cut(
                judge,
                metric,
                &position.go(&mv),
                &next,
                -cut-1)
            }
          };
        if score > best { best = score; }
        if best >= cut { return best }
      }

      return best
    }
  }
}
