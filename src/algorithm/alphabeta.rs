use algorithm::metric::Metric;
use algorithm::scope::Scope;
use board::position::Game;
use engine::judge::{Eval, MIN_EVAL, Judge};

pub fn makes_cut<TGame, TScope>(judge: &mut Judge, metric: &mut Metric, position: &TGame, scope: &TScope, cut: Eval) -> Eval where TGame : Game, TScope : Scope {
  if cut <= MIN_EVAL { return MIN_EVAL }

  if let Some(eval) = judge.recall(position, scope.depth()) {
    if eval >= cut { return eval }
  }

  let moves = judge.moves(position);
  metric.add_nodes(moves.len());
  if moves.len() == 0 { return MIN_EVAL }

  let quiet = judge.quiet_position(position, &moves);
  let current_score = judge.evaluate(position);

  match scope.next(quiet, cut - current_score) {
    None => current_score,
    Some(_) => {
      let mut best = MIN_EVAL;
      for mv in moves {
        let quiet = judge.quiet_move(position, &mv);
        let score =
          match scope.next(quiet, cut - current_score) {
            None => current_score,
            Some(next) => {
              -makes_cut(
                judge,
                metric,
                &position.go(&mv),
                &next,
                -cut+1)
            }
          };
        if score > best {
          best = score;
          if best >= cut { break; }
        }
      }

      if best >= cut {
        judge.remember(position, scope.depth(), best);
      }
      best
    }
  }
}
