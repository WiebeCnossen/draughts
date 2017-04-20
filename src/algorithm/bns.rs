use algorithm::scope::Scope;
use algorithm::alphabeta::makes_cut;
use algorithm::metric::Meta;
use board::mv::Move;
use board::position::Game;
use engine::judge::{Eval, MIN_EVAL, MAX_EVAL, Judge};

pub struct BnsResult {
  pub cut: Eval,
  pub meta: Meta,
  pub mv: Move
}

struct BnsState {
  pub lower: Eval,
  pub upper: Eval,
  pub cut: Eval,
  pub step: Eval,
  pub up: bool
}

impl BnsState {
  fn initial(cut: Eval) -> BnsState {
    BnsState {
      lower: MIN_EVAL,
      upper: MAX_EVAL,
      cut: cut,
      step: 0,
      up: false
    }
  }

  fn next(&self, better_count: u8) -> BnsState {
    let up = better_count > 0;
    let lower = if up { self.cut } else { self.lower };
    let upper = if up { self.upper } else { self.cut };
    let step =
      if self.step == 0 { 4 }
      else {
        let mid = (lower + upper) / 2 - lower;
        let temp = if self.up == up { self.step * 2 } else { self.step / 2 };
        if mid < temp { mid } else { temp }
      };
    let cut = if up { lower + step } else { upper - step };
    BnsState {
      lower: lower,
      upper: upper,
      cut: cut,
      step: step,
      up: up
    }
  }
}

pub fn best_node_search<TGame, TScope>(judge: &Judge, position: &TGame, scope: &TScope, initial_cut: Eval, precision: Eval) -> BnsResult where TGame : Game, TScope : Scope {
  let moves = judge.moves(position);
  let mut best_move = None;
  let mut state = BnsState::initial(initial_cut);
  let mut meta = Meta::create();
  loop {
    /*
    if state.lower >= state.upper { panic!("Lower must be smaller than upper") }
    if state.lower > state.cut { panic!("Cut must be at least lower") }
    if state.upper <= state.cut { panic!("Cut must be smaller than upper") }
    */
    let mut better_count = 0u8;
    for mv in &moves[..] {
      if !makes_cut(judge, &mut meta, &position.go(mv), scope, -state.cut - 1) {
        best_move = Some(mv);
        better_count = better_count + 1;
        if state.lower + precision >= state.upper  || better_count > 1 {
          break;
        }
      }
    }

    let next = state.next(better_count);
    match (better_count, next.lower + precision >= next.upper, best_move) {
      (1, _, Some(mv)) |
      (_, true, Some(mv)) => {
        return BnsResult {
          cut: state.cut,
          meta: meta,
          mv: mv.clone()
        }
      },
      _ => state = next
    }
  }
}


/*

function BNS(node, α, β)
  subtreeCount := number of children of node
  do
    test := NextGuess(α, β, subtreeCount)
    betterCount := 0
    foreach child of node
      bestVal := -AlphaBeta(child, -test, -(test - 1))
      if bestVal ≥ test
        betterCount := betterCount + 1
        bestNode := child
    //update number of sub-trees that exceeds separation test value
    //update alpha-beta range
  while not((β - α < 2) or (betterCount = 1))
  return bestNode

 */
