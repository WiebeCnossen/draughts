use algorithm::scope::Scope;
use algorithm::alphabeta::makes_cut;
use algorithm::metric::{Meta, Metric};
use board::position::Game;
use engine::judge::{Eval, MIN_EVAL, MAX_EVAL, Judge};

struct MtdState {
  lower: Eval,
  pub guess: Eval,
  upper: Eval
}

impl MtdState {
  fn initial(guess: Eval) -> MtdState {
    MtdState {
      lower: MIN_EVAL,
      guess: guess,
      upper: MAX_EVAL + 1
    }
  }

  fn next(&self, eval: Eval) -> MtdState {
    if eval < self.guess {
      MtdState {
        lower: self.lower,
        guess: eval,
        upper: eval + 1
      }
    }
    else {
      MtdState {
        lower: eval,
        guess: eval + 1,
        upper: self.upper
      }
    }
  }

  fn finished(&self) -> bool {
    self.guess < self.upper
  }
}

pub fn mtd_f<TGame, TScope>(judge: &mut Judge, position: &TGame, scope: &TScope, guess: Eval) -> Eval where TGame : Game, TScope : Scope {
  let mut state = MtdState::initial(guess);
  let mut meta = Meta::create();
  loop {
    state = state.next(makes_cut(judge, &mut meta, position, scope, state.guess));
    if state.finished() {
      println!("MTD-f {} nodes", meta.get_nodes());
      return state.guess
    }
  }
}

/*
function MTDF(root, f, d)
      g := f
      upperBound := +∞
      lowerBound := -∞
      while lowerBound < upperBound
         β := max(g, lowerBound+1)
         g := AlphaBetaWithMemory(root, β-1, β, d)
         if g < β then
              upperBound := g
         else
              lowerBound := g
     return g
*/
