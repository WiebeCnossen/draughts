use algorithm::metric::Metric;
use board::position::Game;
use engine::judge::{Eval, MIN_EVAL, Judge};

pub fn makes_cut<TGame>(judge: &Judge, metric: &mut Metric, position: &TGame, depth: u8, cut: Eval) -> bool where TGame : Game {
  if cut <= MIN_EVAL { return true }

  let moves = judge.moves(position);
  metric.add_nodes(moves.len());
  if moves.len() == 0 { return false }

  let quiet = judge.quiet(position, &moves);
  if quiet && depth == 0 {
    judge.evaluate(position) >= cut
  }
  else {
    moves.iter().any(|mv| {
      !makes_cut(
        judge,
        metric,
        &position.go(mv),
        if quiet { depth - 1} else { depth },
        -(cut + 1))
    })
  }
}
