use algorithm::metric::Metric;
use algorithm::scope::Scope;
use board::position::Game;
use engine::judge::{Eval, MIN_EVAL, Judge};

pub struct DepthScope {
  depth: u8,
  quiet_moves: u8,
  stack: u8
}

impl DepthScope {
  pub fn from_depth(depth: u8) -> DepthScope {
    DepthScope {
      depth: depth,
      quiet_moves: 0,
      stack: 0
    }
  }
}

impl Scope for DepthScope {
  fn next(&self, quiet: bool) -> Option<DepthScope> {
    match (quiet, self.depth <= self.stack, self.quiet_moves > 0) {
      (false, _, _) => Some(DepthScope {
                        depth: self.depth,
                        quiet_moves: 0,
                        stack: self.stack + 1
                      }),
      (_, true, _) => None,
      (_, _, true) => Some(DepthScope {
                        depth: self.depth - self.stack - 1,
                        quiet_moves: self.quiet_moves + 1,
                        stack: 0
                      }),
      _ => Some(DepthScope {
             depth: self.depth,
             quiet_moves: self.quiet_moves + 1,
             stack: self.stack + 1
           })
    }
  }
}

pub fn makes_cut<TGame, TScope>(judge: &Judge, metric: &mut Metric, position: &TGame, scope: &TScope, cut: Eval) -> bool where TGame : Game, TScope : Scope {
  if cut <= MIN_EVAL { return true }

  let moves = judge.moves(position);
  metric.add_nodes(moves.len());
  if moves.len() == 0 { return false }

  let quiet = judge.quiet(position, &moves);
  match scope.next(quiet) {
    None => judge.evaluate(position) >= cut,
    Some(next) => moves.iter().any(|mv| {
                    !makes_cut(
                      judge,
                      metric,
                      &position.go(mv),
                      &next,
                      -(cut + 1))
                  })
  }
}
