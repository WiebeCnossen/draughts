use algorithm::alphabeta::makes_cut;
use algorithm::metric::{Metric, Meta};
use board::mv::Move;
use board::position::Game;
use engine::judge::{Eval, MIN_EVAL, MAX_EVAL, Judge};

pub struct BnsResult {
  pub cut: Eval,
  pub meta: Meta,
  pub mv: Move
}

fn next_guess(cut: Eval, lower: Eval, upper: Eval, step: Eval) -> Eval {
  let part = (upper - lower) / 2;
  let offset = if step < part { step } else { part };
  if lower == cut { cut + offset } else { cut - offset }
}

pub fn best_node_search<TGame>(judge: &Judge, position: &TGame, depth: u8, initial_cut: Eval, precision: Eval) -> BnsResult where TGame : Game {
  let moves = judge.moves(position);
  let mut meta = Meta::create();
  let mut lower = MIN_EVAL;
  let mut upper = MAX_EVAL;
  let mut best_move = None;
  let mut cut = initial_cut;
  loop {
    println!("{} -> {} -> {} ({} nodes)", lower, cut, upper, meta.get_nodes());
    if lower == upper { panic!() }
    let mut better_count = 0;
    for mv in &moves[..] {
      if !makes_cut(judge, &mut meta, &position.go(mv), depth, -cut - 1) {
        best_move = Some(mv);
        better_count = better_count + 1;
        if lower + precision >= upper  || better_count > 1 {
          break;
        }
      }
    }

    match (better_count, lower + precision >= upper, best_move) {
      (1, _, Some(mv)) |
      (_, true, Some(mv)) => {
        return BnsResult {
          cut: cut,
          meta: meta,
          mv: mv.clone()
        }
      },
      (0, _, _) => upper = cut,
      (_, _, _) => lower = cut
    }

    cut = next_guess(cut, lower, upper, 200);
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
