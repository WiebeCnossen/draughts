use board::position::Position;
use board::mv::Move;

pub type Eval = i16;
pub const MAX_EVAL : Eval = 30000i16;
pub const ZERO_EVAL : Eval = 0i16;
pub const DRAW_EVAL : Eval = 0i16;
pub const MIN_EVAL : Eval = -30000i16;

pub trait Judge {
  fn evaluate(&self, position: &Position) -> Eval;
  fn quiet(&self, position: &Position, moves: &[Move]) -> bool;
  fn moves(&self, position: &Position) -> Vec<Move>;
}
