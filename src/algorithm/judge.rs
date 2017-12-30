use algorithm::scope::Depth;
use board::mv::Move;
use board::position::{Field, Position};

pub type Eval = i16;
pub const MAX_EVAL: Eval = 15_000;
pub const ZERO_EVAL: Eval = 0;
pub const DRAW_EVAL: Eval = 0;
pub const MIN_EVAL: Eval = -15_000;

pub struct PositionMemory {
    pub depth: Depth,
    pub lower: Eval,
    pub upper: Eval,
    pub from: Field,
    pub to: Field,
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

    pub fn create(
        depth: Depth,
        lower: Eval,
        upper: Eval,
        from: Field,
        to: Field,
    ) -> PositionMemory {
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
    fn recall(&self, _position: &Position, _depth: Depth) -> PositionMemory {
        PositionMemory::empty()
    }
    fn remember(&mut self, _: &Position, _: Depth, _: Eval, _: Option<Move>, _: bool) {}
    fn consolidate(&mut self) {}
    fn evaluate(&self, position: &Position) -> Eval;
    fn moves(&self, position: &Position, depth: Depth) -> Vec<Move>;
    fn display_name(&self) -> &str;
    fn quiet_move(&self, position: &Position, mv: &Move) -> bool;
    fn quiet_position(&self, position: &Position, moves: &[Move]) -> bool {
        moves.len() > 1 && self.quiet_move(position, &moves[0])
    }
}
