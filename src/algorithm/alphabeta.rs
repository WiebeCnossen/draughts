use std::cmp::{min, max};

use algorithm::judge::{Eval, MIN_EVAL, MAX_EVAL, Judge};
use algorithm::metric::Metric;
use algorithm::scope::Scope;
use algorithm::search::SearchResult;
use board::position::Game;

pub fn makes_cut<TGame, TScope>(
    judge: &mut Judge,
    metric: &mut Metric,
    position: &TGame,
    scope: &TScope,
    cut: Eval,
) -> SearchResult
where
    TGame: Game,
    TScope: Scope,
{
    if cut <= MIN_EVAL {
        return SearchResult::evaluation(MIN_EVAL);
    }

    if cut > MAX_EVAL {
        return SearchResult::evaluation(MAX_EVAL);
    }

    let memory = judge.recall(position);
    if memory.depth >= scope.depth() {
        if memory.lower >= cut {
            return SearchResult::evaluation(memory.lower);
        }
        if memory.upper < cut {
            return SearchResult::evaluation(memory.upper);
        }
    }

    metric.add_nodes(1);

    let mut moves = judge.moves(position);
    if moves.is_empty() {
        return SearchResult::evaluation(MIN_EVAL);
    }

    let quiet = judge.quiet_position(position, &moves);
    let len = moves.len();
    if !quiet && len > 1 && memory.has_move() {
        for i in 0..len {
            if moves[i].from() == memory.from && moves[i].to() == memory.to {
                if i > 0 {
                    let mv = moves[i];
                    moves.remove(i);
                    moves.insert(0, mv);
                }
                break;
            }
        }
    }

    let current_score = min(max(judge.evaluate(position), memory.lower), memory.upper);

    if scope.next(len, quiet, cut - current_score).is_some() {
        let mut best = MIN_EVAL;
        let mut pending = None;
        for mv in moves {
            let quiet = judge.quiet_move(position, &mv);
            let score = if let Some(next) = scope.next(len, quiet, cut - current_score) {
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
                judge.remember(position, scope.depth(), best, Some(mv), false);
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
