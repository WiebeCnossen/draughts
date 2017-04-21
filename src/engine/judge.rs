use board::position::Position;
use board::mv::Move;

pub type Eval = i16;
pub const MAX_EVAL : Eval = 15000i16;
pub const ZERO_EVAL : Eval = 0i16;
pub const DRAW_EVAL : Eval = 0i16;
pub const MIN_EVAL : Eval = -15000i16;

pub trait Judge {
  fn evaluate(&self, position: &Position) -> Eval;
  fn moves(&self, position: &Position) -> Vec<Move>;
  fn display_name(&self) -> &str;
  fn quiet_move(&self, position: &Position, mv: &Move) -> bool;
  fn quiet_position(&self, position: &Position, moves: &[Move]) -> bool {
    moves.len() > 1 && !self.quiet_move(position, &moves[0])
  }
}
