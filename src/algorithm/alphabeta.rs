use algorithm::metric::Metric;
use algorithm::scope::Scope;
use algorithm::search::SearchResult;
use board::position::Game;
use engine::judge::{Eval, MIN_EVAL, Judge};

pub fn makes_cut<TGame, TScope>(judge: &mut Judge, metric: &mut Metric, position: &TGame, scope: &TScope, cut: Eval) -> SearchResult where TGame : Game, TScope : Scope {
  if cut <= MIN_EVAL { return SearchResult::evaluation(MIN_EVAL) }

  if let Some(eval) = judge.recall(position, scope.depth()) {
    if eval >= cut { return SearchResult::evaluation(eval) }
  }

  let moves = judge.moves(position);
  metric.add_nodes(moves.len());
  if moves.len() == 0 { return SearchResult::evaluation(MIN_EVAL) }

  let quiet = judge.quiet_position(position, &moves);
  let current_score = judge.evaluate(position);

  match scope.next(quiet, cut - current_score) {
    None => SearchResult::evaluation(current_score),
    Some(_) => {
      let mut best = MIN_EVAL;
      let mut pending = None;
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
                -cut+1).evaluation
            }
          };
        if score > best {
          best = score;
          pending = Some(mv);
          if best >= cut { break; }
        }
      }

      if best >= cut {
        if let Some(mv) = pending {
          judge.remember(position, scope.depth(), best, mv, false);
          SearchResult::with_move(mv, best)
        }
        else {
          panic!("No move pending");
        }
      }
      else {
        if let Some(mv) = pending {
          judge.remember(position, scope.depth(), best, mv, true);
        }
        SearchResult::evaluation(best)
      }
    }
  }
}
