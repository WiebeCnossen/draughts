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

pub fn makes_cut<TGame, TScope>(judge: &Judge, metric: &mut Metric, position: &TGame, scope: &TScope, cut: Eval) -> bool where TGame : Game, TScope : Scope {
  if cut <= MIN_EVAL { return true }

  let moves = judge.moves(position);
  metric.add_nodes(moves.len());
  if moves.len() == 0 { return false }

  let quiet = judge.quiet_position(position, &moves);
  match scope.next(quiet) {
    None => judge.evaluate(position) >= cut,
    Some(_) => moves.iter().any(|mv| {
      let quiet = judge.quiet_move(position, &mv);
      match scope.next(quiet) {
        None => judge.evaluate(position) >= cut,
        Some(next) =>
          !makes_cut(
            judge,
            metric,
            &position.go(mv),
            &next,
            -(cut + 1))
      }
    })
  }
}
