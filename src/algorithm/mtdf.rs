use algorithm::alphabeta::makes_cut;
use algorithm::judge::{Eval, MIN_EVAL, MAX_EVAL, Judge};
use algorithm::metric::Meta;
use algorithm::scope::Scope;
use board::mv::Move;
use board::position::Game;

struct MtdState {
    lower: Eval,
    guess: Eval,
    upper: Eval,
}

impl MtdState {
    fn initial(guess: Eval) -> MtdState {
        MtdState {
            lower: MIN_EVAL,
            guess: guess,
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

pub fn mtd_f<TGame, TScope>(judge: &mut Judge,
                            position: &TGame,
                            depth: u8,
                            guess: Eval)
                            -> MtdResult
    where TGame: Game,
          TScope: Scope
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
                None => judge.moves(position)[0].clone(),
            };
            return MtdResult::create(mv, state.lower, meta);
        }
    }
}
