use std::cmp::{min,max};

use algorithm::scope::Scope;
use algorithm::search::SearchResult;
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
  pub mv: Option<Move>,
  pub count: u8
}

impl BnsState {
  fn initial(cut: Eval, mv: Move) -> BnsState {
    BnsState {
      lower: MIN_EVAL,
      upper: MAX_EVAL + 1,
      cut,
      mv: Some(mv),
      count: 2
    }
  }

  fn next(&self, better_count: u8, search_result: SearchResult) -> BnsState {
    let up = better_count > 0;
    let lower = if up { max(search_result.evaluation, self.lower + 1) } else { self.lower };
    let upper = if up { self.upper } else { min(self.cut, search_result.evaluation + 1) };
    let cut = if up { lower + 1 } else { upper - 1 };
    let mv = if up { search_result.mv } else { self.mv.clone() };
    let count =
      if upper - lower <= 1 && mv.is_some() { 0 }
      else if upper - cut <= 1 { 1 }
      else { 2 };
    BnsState { lower, upper, cut, mv, count }
  }
}

pub fn best_node_search<TGame, TScope>(judge: &mut Judge, position: &TGame, scope: &TScope, initial_cut: Eval) -> BnsResult where TGame : Game, TScope : Scope {
  let moves = judge.moves(position);
  let mut state = BnsState::initial(initial_cut, moves[0]);
  let mut meta = Meta::create();
  loop {
    let mut better_count = 0u8;
    let mut best = SearchResult::evaluation(MIN_EVAL);
    let beta = state.cut + 1;
    for mv in &moves[..] {
      let score = -makes_cut(judge, &mut meta, &position.go(mv), scope, -beta).evaluation;
      if score >= best.evaluation { best = SearchResult::with_move(mv.clone(), score); }

      if score > beta {
        better_count = better_count + 1;
        if better_count >= state.count { break; }
      }
    }

    let next = state.next(better_count, best);
    if next.count == 0 { return BnsResult { cut: state.cut, meta, mv: next.mv.unwrap() } }
    state = next;
  }
}
