use algorithm::metric::Metric;
use algorithm::scope::Scope;
use algorithm::search::SearchResult;
use board::position::Game;
use engine::judge::{Eval, MIN_EVAL, Judge};

pub fn makes_cut<TGame, TScope>(judge: &mut Judge,
                                metric: &mut Metric,
                                position: &TGame,
                                scope: &TScope,
                                cut: Eval)
                                -> SearchResult
    where TGame: Game,
          TScope: Scope
{
    if cut <= MIN_EVAL {
        return SearchResult::evaluation(MIN_EVAL);
    }

    match judge.recall(position, scope.depth()) {
        (evaluation, _) if evaluation >= cut => return SearchResult::evaluation(evaluation),
        (_, evaluation) if evaluation < cut => return SearchResult::evaluation(evaluation),
        _ => (),
    }

    metric.add_nodes(1);

    let moves = judge.moves(position);
    if moves.len() == 0 {
        return SearchResult::evaluation(MIN_EVAL);
    }

    let quiet = judge.quiet_position(position, &moves);
    let current_score = judge.evaluate(position);

    if let Some(_) = scope.next(quiet, cut - current_score) {
        let mut best = MIN_EVAL;
        let mut pending = None;
        for mv in moves {
            let quiet = judge.quiet_move(position, &mv);
            let score = if let Some(next) = scope.next(quiet, cut - current_score) {
                -makes_cut(judge, metric, &position.go(&mv), &next, -cut + 1).evaluation
            } else {
                current_score
            };
            if score > best {
                best = score;
                pending = Some(mv);
                if best >= cut {
                    break;
                }
            }
        }

        if best >= cut {
            if let Some(mv) = pending {
                judge.remember(position, scope.depth(), best, Some(mv.clone()), false);
                SearchResult::with_move(mv, best)
            } else {
                panic!("No move pending");
            }
        } else {
            judge.remember(position, scope.depth(), best, pending, true);
            SearchResult::evaluation(best)
        }
    } else {
        SearchResult::evaluation(current_score)
    }
}
