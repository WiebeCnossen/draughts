use std::cmp::min;
use std::cmp::Ordering::{Equal, Greater, Less};

use algorithm::alphabeta::{makes_cut, makes_cut_parallel};
use algorithm::judge::{Eval, Judge, MAX_EVAL, MIN_EVAL};
use algorithm::meta::Meta;
use algorithm::mtdf::{mtd_f, mtd_f_parallel};
use algorithm::scope::{Depth, Scope};
use algorithm::search::SearchResult;
use board::mv::Move;
use board::position::Position;

pub struct BnsResult {
    pub lower: Eval,
    pub meta: Meta,
    pub mv: Move,
}

type MoveCount = u8;
struct BnsState {
    pub lower: Eval,
    pub upper: Eval,
    pub cut: Eval,
    pub mv: Move,
    pub count: MoveCount,
}

impl BnsState {
    fn initial(cut: Eval, mv: Move) -> BnsState {
        BnsState {
            lower: MIN_EVAL,
            upper: MAX_EVAL + 1,
            cut,
            mv,
            count: 2,
        }
    }

    fn next(&self, better_count: MoveCount, search_result: &SearchResult) -> BnsState {
        let up = better_count > 0;
        let lower =
            if up { self.cut } else { self.lower };
        let upper = if up {
            self.upper
        } else {
            min(self.cut, search_result.evaluation + 1)
        };
        let cut = upper - 1;
        let mv = if up {
            search_result.mv.unwrap()
        } else {
            self.mv
        };
        let count = if upper - lower <= 1 || better_count == 1 {
            0
        } else if upper - cut <= 1 {
            1
        } else {
            2
        };
        BnsState {
            lower,
            upper,
            cut,
            mv,
            count,
        }
    }
}

#[test]
fn up_zero() {
    let initial = BnsState::initial(0, Move::shift(0, 5));
    let next = initial.next(
        2,
        &SearchResult::with_move(Move::shift(1, 5), initial.upper - 1),
    );
    assert_eq!(next.lower, initial.cut);
    assert_eq!(next.upper, initial.upper);
    assert!(next.mv == Move::shift(1, 5));
    assert_eq!(next.count, 1);
}

#[test]
fn up_one() {
    let initial = BnsState::initial(0, Move::shift(0, 5));
    let next = initial.next(
        2,
        &SearchResult::with_move(Move::shift(1, 5), initial.upper - 2),
    );
    assert_eq!(next.lower, initial.cut);
    assert_eq!(next.upper, initial.upper);
    assert!(next.mv == Move::shift(1, 5));
    assert_eq!(next.count, 1);
    assert!(next.cut < next.upper);
}

#[test]
fn up_two() {
    let initial = BnsState::initial(0, Move::shift(0, 5));
    let next = initial.next(2, &SearchResult::with_move(Move::shift(1, 5), 5));
    assert_eq!(next.lower, initial.cut);
    assert_eq!(next.upper, initial.upper);
    assert!(next.mv == Move::shift(1, 5));
    assert_eq!(next.count, 1);
    assert!(next.lower < next.cut);
    assert!(next.cut < next.upper);
}

#[test]
fn down() {
    let initial = BnsState::initial(0, Move::shift(0, 5));
    let next = initial.next(0, &SearchResult::with_move(Move::shift(1, 5), -2));
    assert_eq!(next.lower, initial.lower);
    assert_eq!(next.upper, -1);
    assert!(next.mv == Move::shift(0, 5));
    assert!(next.count > 0);
    assert!(next.lower < next.cut);
    assert!(next.cut < next.upper);
}

pub fn best_node_search<TScope>(
    judge: &mut Judge,
    position: &Position,
    depth: Depth,
    initial: &SearchResult,
) -> BnsResult
where
    TScope: Scope,
{
    let mut moves = judge.moves(position, depth);
    let mut meta = Meta::create();
    let mut state = match initial.mv {
        Some(mv) if depth > 1 => {
            let mtd = mtd_f::<TScope>(judge, &position.go(&mv), depth - 1, -initial.evaluation);
            meta.add_nodes(mtd.meta.get_nodes() + 1);
            moves.sort_by(|&mv1, &mv2| match (mv1 == mv, mv2 == mv) {
                (true, false) => Less,
                (false, true) => Greater,
                _ => Equal,
            });
            BnsState::initial(-mtd.evaluation, mv)
        }
        _ => BnsState::initial(initial.evaluation, moves[0]),
    };
    loop {
        let scope = &TScope::from_depth(depth);
        let mut better_count = 0;
        let mut best = SearchResult::evaluation(MIN_EVAL - 1);
        let mut beta = state.cut - 1;
        for mv in &moves[..] {
            let score = -makes_cut(judge, &mut meta, &position.go(mv), scope, -beta).evaluation;
            if score > best.evaluation {
                best = SearchResult::with_move(*mv, score);
            }

            if score > beta {
                better_count += 1;
                if better_count >= state.count {
                    break;
                }
                beta = state.cut;
            }
        }

        let next = state.next(better_count, &best);
        if next.count == 0 {
            return BnsResult {
                lower: next.lower,
                meta,
                mv: next.mv,
            };
        }
        state = next;
    }
}

pub fn best_node_search_parallel<TJudge, TScope>(
    judge: &mut TJudge,
    position: &Position,
    depth: Depth,
    initial: &SearchResult,
) -> BnsResult
where
    TJudge: 'static + Judge + Clone + Send,
    TScope: 'static + Scope + Send,
{
    let mut moves = judge.moves(position, depth);
    let mut meta = Meta::create();
    let mut state = match initial.mv {
        Some(mv) if depth > 1 => {
            let mtd = mtd_f_parallel::<TJudge, TScope>(
                judge,
                &position.go(&mv),
                depth - 1,
                -initial.evaluation,
            );
            meta.add_nodes(mtd.meta.get_nodes() + 1);
            moves.sort_by(|&mv1, &mv2| match (mv1 == mv, mv2 == mv) {
                (true, false) => Less,
                (false, true) => Greater,
                _ => Equal,
            });
            BnsState::initial(-mtd.evaluation, mv)
        }
        _ => BnsState::initial(initial.evaluation, moves[0]),
    };
    loop {
        let scope = &TScope::from_depth(depth);
        let mut better_count = 0;
        let mut best = SearchResult::evaluation(MIN_EVAL - 1);
        let mut beta = state.cut - 1;
        for mv in &moves[..] {
            let score = -makes_cut_parallel::<TJudge, TScope>(
                judge,
                &mut meta,
                &position.go(mv),
                scope,
                -beta,
            ).evaluation;
            if score > best.evaluation {
                best = SearchResult::with_move(*mv, score);
            }

            if score > beta {
                better_count += 1;
                if better_count >= state.count {
                    break;
                }
                beta = state.cut;
            }
        }

        let next = state.next(better_count, &best);
        if next.count == 0 {
            return BnsResult {
                lower: next.lower,
                meta,
                mv: next.mv,
            };
        }
        state = next;
    }
}
