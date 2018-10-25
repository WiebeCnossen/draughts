use std::cmp::{max, min};
use std::sync::mpsc;
use std::thread;

use super::judge::{Eval, Judge, MAX_EVAL, MIN_EVAL};
use super::meta::Meta;
use super::scope::{Depth, Scope};
use super::search::SearchResult;
use crate::board::position::Position;

pub fn makes_cut<TScope>(
    judge: &mut Judge,
    meta: &mut Meta,
    position: &Position,
    scope: &TScope,
    cut: Eval,
) -> SearchResult
where
    TScope: Scope,
{
    if cut <= MIN_EVAL {
        return SearchResult::evaluation(MIN_EVAL);
    }

    if cut > MAX_EVAL {
        return SearchResult::evaluation(MAX_EVAL);
    }

    let memory = judge.recall(position, scope.depth());
    if memory.depth >= scope.depth() {
        if memory.lower >= cut {
            return SearchResult::evaluation(memory.lower);
        }
        if memory.upper < cut {
            return SearchResult::evaluation(memory.upper);
        }
    }

    meta.add_nodes(1);

    let mut moves = judge.moves(position, scope.depth());
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
        let single = moves.len() == 1;
        for mv in moves {
            let quiet = !single && judge.quiet_move(position, &mv);
            let score = if let Some(next) = scope.next(len, quiet, cut - current_score) {
                -makes_cut(judge, meta, &position.go(&mv), &next, -cut + 1).evaluation
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

const MIN_DEPTH: Depth = 6;
const MAX_DEPTH: Depth = 9;
const MIN_THREADS: usize = 6;

pub fn makes_cut_parallel<TJudge, TScope>(
    judges: &mut Vec<TJudge>,
    meta: &mut Meta,
    position: &Position,
    scope: &TScope,
    cut: Eval,
) -> SearchResult
where
    TJudge: 'static + Judge + Clone + Send,
    TScope: 'static + Scope + Send,
{
    if cut <= MIN_EVAL {
        return SearchResult::evaluation(MIN_EVAL);
    }

    if cut > MAX_EVAL {
        return SearchResult::evaluation(MAX_EVAL);
    }

    let memory = judges[0].recall(position, scope.depth());
    if memory.depth >= scope.depth() {
        if memory.lower >= cut {
            return SearchResult::evaluation(memory.lower);
        }
        if memory.upper < cut {
            return SearchResult::evaluation(memory.upper);
        }
    }

    meta.add_nodes(1);

    let mut moves = judges[0].moves(position, scope.depth());
    if moves.is_empty() {
        return SearchResult::evaluation(MIN_EVAL);
    }

    let quiet = judges[0].quiet_position(position, &moves);
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

    let current_score = min(
        max(judges[0].evaluate(position), memory.lower),
        memory.upper,
    );

    if scope.next(len, quiet, cut - current_score).is_some() {
        let mut best = MIN_EVAL;
        let mut pending = None;
        if moves.len() >= MIN_THREADS && scope.depth() >= MIN_DEPTH && scope.depth() <= MAX_DEPTH {
            judges[0].consolidate();
            let (tx, rx) = mpsc::channel();
            let mut open = 0;
            let mut mi = 0;
            while mi < moves.len() || open > 0 {
                // Start worker(s)
                while mi < moves.len() && !judges.is_empty() {
                    let mut judge = judges.pop().unwrap();
                    let mv = moves[mi];
                    let tx = tx.clone();
                    let position = *position;
                    let scope: TScope = scope.clone();

                    thread::spawn(move || {
                        let quiet = judge.quiet_move(&position, &mv);
                        let mut meta = Meta::create();
                        let score = if let Some(next) = scope.next(len, quiet, cut - current_score)
                        {
                            -makes_cut(&mut judge, &mut meta, &position.go(&mv), &next, -cut + 1)
                                .evaluation
                        } else {
                            current_score
                        };
                        tx.send((score, mv, meta, judge))
                            .expect("Bummer: send failed");
                    });

                    mi += 1;
                    open += 1;
                }

                if open > 0 {
                    let (score, mv, thread_meta, mut judge) =
                        rx.recv().expect("Bummer: recv failed");
                    meta.put_depth(thread_meta.get_depth());
                    meta.add_nodes(thread_meta.get_nodes());
                    judge.consolidate();
                    judges.push(judge);
                    open -= 1;
                    if score > best {
                        best = score;
                        pending = Some(mv);
                        if best >= cut {
                            break;
                        }
                    }
                }
            }

            while open > 0 {
                let (score, mv, thread_meta, mut judge) = rx.recv().unwrap();
                meta.put_depth(thread_meta.get_depth());
                meta.add_nodes(thread_meta.get_nodes());
                judge.consolidate();
                judges.push(judge);
                open -= 1;
                if score > best {
                    best = score;
                    pending = Some(mv);
                }
            }
        } else {
            let single = moves.len() == 1;
            for mv in moves {
                let quiet = !single && judges[0].quiet_move(position, &mv);
                let score = if let Some(next) = scope.next(len, quiet, cut - current_score) {
                    -makes_cut_parallel::<TJudge, TScope>(
                        judges,
                        meta,
                        &position.go(&mv),
                        &next,
                        -cut + 1,
                    )
                    .evaluation
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
        }

        if best >= cut {
            if let Some(mv) = pending {
                judges[0].remember(position, scope.depth(), best, Some(mv), false);
                SearchResult::with_move(mv, best)
            } else {
                panic!("No move pending");
            }
        } else {
            judges[0].remember(position, scope.depth(), best, pending, true);
            SearchResult::evaluation(best)
        }
    } else {
        SearchResult::evaluation(current_score)
    }
}
