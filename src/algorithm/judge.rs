use board::position::Position;
use board::mv::Move;

pub type Eval = i16;
pub const MAX_EVAL: Eval = 15000i16;
pub const ZERO_EVAL: Eval = 0i16;
pub const DRAW_EVAL: Eval = 0i16;
pub const MIN_EVAL: Eval = -15000i16;

pub struct PositionMemory {
    pub depth: u8,
    pub lower: Eval,
    pub upper: Eval,
    pub from: usize,
    pub to: usize,
}

impl PositionMemory {
    pub fn empty() -> PositionMemory {
        PositionMemory {
            depth: 0,
            lower: MIN_EVAL,
            upper: MAX_EVAL,
            from: 0,
            to: 0,
        }
    }

    pub fn create(depth: u8, lower: Eval, upper: Eval, from: usize, to: usize) -> PositionMemory {
        PositionMemory {
            depth,
            lower,
            upper,
            from,
            to,
        }
    }

    pub fn has_move(&self) -> bool {
        self.from != 0 || self.to != 0
    }
}

pub trait Judge {
    fn recall(&self, _: &Position) -> PositionMemory {
        PositionMemory::empty()
    }
    fn remember(&mut self, _: &Position, _: u8, _: Eval, _: Option<Move>, _: bool) {}
    fn evaluate(&self, position: &Position) -> Eval;
    fn moves(&self, position: &Position) -> Vec<Move>;
    fn display_name(&self) -> &str;
    fn quiet_move(&self, position: &Position, mv: &Move) -> bool;
    fn quiet_position(&self, position: &Position, moves: &[Move]) -> bool {
        moves.len() > 1 && self.quiet_move(position, &moves[0])
    }
}
