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
  pub mv: Move,
  pub count: usize
}

impl BnsState {
  fn initial(cut: Eval, mv: Move) -> BnsState {
    BnsState {
      lower: MIN_EVAL,
      upper: MAX_EVAL + 1,
      cut,
      mv,
      count: 2
    }
  }

  fn next(&self, better_count: usize, search_result: SearchResult) -> BnsState {
    let up = better_count > 0;
    let lower = if up { max(search_result.evaluation, self.lower + 1) } else { self.lower };
    let upper = if up { self.upper } else { min(self.cut, search_result.evaluation + 1) };
    let cut = if up { lower + 1 } else { upper - 1 };
    let mv = if up { search_result.mv.unwrap() } else { self.mv.clone() };
    let count =
      if upper - lower <= 1 { 0 }
      else if upper - cut <= 1 { 1 }
      else { 2 };
    BnsState { lower, upper, cut, mv, count }
  }
}

#[test]
fn up_zero() {
  let initial = BnsState::initial(0, Move::Shift(0, 5));
  let next = initial.next(2, SearchResult::with_move(Move::Shift(1, 5), initial.upper - 1));
  assert_eq!(next.lower, initial.upper - 1);
  assert_eq!(next.upper, initial.upper);
  assert!(next.mv == Move::Shift(1, 5));
  assert_eq!(next.count, 0);
}

#[test]
fn up_one() {
  let initial = BnsState::initial(0, Move::Shift(0, 5));
  let next = initial.next(2, SearchResult::with_move(Move::Shift(1, 5), initial.upper - 2));
  assert_eq!(next.lower, initial.upper - 2);
  assert_eq!(next.upper, initial.upper);
  assert!(next.mv == Move::Shift(1, 5));
  assert_eq!(next.count, 1);
  assert!(next.cut < next.upper);
}

#[test]
fn up_two() {
  let initial = BnsState::initial(0, Move::Shift(0, 5));
  let next = initial.next(2, SearchResult::with_move(Move::Shift(1, 5), 5));
  assert_eq!(next.lower, 5);
  assert_eq!(next.upper, initial.upper);
  assert!(next.mv == Move::Shift(1, 5));
  assert_eq!(next.count, 2);
  assert!(next.lower < next.cut);
  assert!(next.cut < next.upper);
}

#[test]
fn down() {
  let initial = BnsState::initial(0, Move::Shift(0, 5));
  let next = initial.next(0, SearchResult::with_move(Move::Shift(1, 5), -2));
  assert_eq!(next.lower, initial.lower);
  assert_eq!(next.upper, -1);
  assert!(next.mv == Move::Shift(0, 5));
  assert_eq!(next.count, 1);
  assert!(next.lower < next.cut);
  assert!(next.cut < next.upper);
}

pub fn best_node_search<TGame, TScope>(judge: &mut Judge, position: &TGame, scope: &TScope, initial_cut: Eval) -> BnsResult where TGame : Game, TScope : Scope {
  let mut moves = judge.moves(position);
  let mut state = BnsState::initial(initial_cut, moves[0]);
  let mut meta = Meta::create();
  loop {
    let mut better_count = 0;
    let mut best = SearchResult::evaluation(MIN_EVAL);
    let beta = state.cut + 1;
    let mut dropped = 0;
    for i in 0..moves.len() {
      let score;
      let single_best;
      {
        let mv = &moves[i - dropped];
        score = -makes_cut(judge, &mut meta, &position.go(mv), scope, -beta).evaluation;
        single_best = score > best.evaluation;
        if score >= best.evaluation { best = SearchResult::with_move(mv.clone(), score); }
      }

      if score > beta {
        let from = if single_best { 0 } else { better_count };
        dropped += i - from;
        for _ in from..i { moves.remove(better_count); }
        better_count = better_count + 1;
        if better_count >= state.count { break; }
      }
    }

    let next = state.next(better_count, best);
    if next.count == 0 { return BnsResult { cut: state.cut, meta, mv: next.mv } }
    state = next;
  }
}
