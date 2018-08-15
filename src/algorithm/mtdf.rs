use super::alphabeta::{makes_cut, makes_cut_parallel};
use super::judge::{Eval, Judge, MAX_EVAL, MIN_EVAL};
use super::meta::Meta;
use super::scope::{Depth, Scope};
use crate::board::mv::Move;
use crate::board::position::Position;

struct MtdState {
    lower: Eval,
    guess: Eval,
    upper: Eval,
}

impl MtdState {
    fn initial(guess: Eval) -> MtdState {
        MtdState {
            lower: MIN_EVAL,
            guess,
            upper: MAX_EVAL + 1,
        }
    }

    fn next(&self, eval: Eval) -> MtdState {
        if eval < self.guess {
            MtdState {
                lower: self.lower,
                guess: eval,
                upper: eval + 1,
            }
        } else {
            MtdState {
                lower: eval,
                guess: eval + 1,
                upper: self.upper,
            }
        }
    }

    fn finished(&self) -> bool {
        self.guess >= self.upper
    }
}

pub struct MtdResult {
    pub mv: Move,
    pub evaluation: Eval,
    pub meta: Meta,
}

impl MtdResult {
    fn create(mv: Move, evaluation: Eval, meta: Meta) -> MtdResult {
        MtdResult {
            mv,
            evaluation,
            meta,
        }
    }
}

pub fn mtd_f<TScope>(judge: &mut Judge, position: &Position, depth: Depth, guess: Eval) -> MtdResult
where
    TScope: Scope,
{
    let scope = &TScope::from_depth(depth);
    let mut state = MtdState::initial(guess);
    let mut meta = Meta::create();
    let mut mv = None;
    loop {
        let result = makes_cut(judge, &mut meta, position, scope, state.guess);
        state = state.next(result.evaluation);
        if let Some(best_move) = result.mv {
            mv = Some(best_move);
        }
        if state.finished() {
            let mv = match mv {
                Some(mv) => mv,
                None => judge
                    .moves(position, depth)
                    .get(0)
                    .cloned()
                    .unwrap_or_else(Move::null),
            };
            return MtdResult::create(mv, state.lower, meta);
        }
    }
}

pub fn mtd_f_parallel<TJudge, TScope>(
    judges: &mut Vec<TJudge>,
    position: &Position,
    depth: Depth,
    guess: Eval,
) -> MtdResult
where
    TJudge: 'static + Judge + Clone + Send,
    TScope: 'static + Scope + Send,
{
    let scope = &TScope::from_depth(depth);
    let mut state = MtdState::initial(guess);
    let mut meta = Meta::create();
    let mut mv = None;
    loop {
        let result =
            makes_cut_parallel::<TJudge, TScope>(judges, &mut meta, position, scope, state.guess);
        state = state.next(result.evaluation);
        if let Some(best_move) = result.mv {
            mv = Some(best_move);
        }
        if state.finished() {
            let mv = match mv {
                Some(mv) => mv,
                None => judges[0]
                    .moves(position, depth)
                    .get(0)
                    .cloned()
                    .unwrap_or_else(Move::null),
            };
            return MtdResult::create(mv, state.lower, meta);
        }
    }
}
