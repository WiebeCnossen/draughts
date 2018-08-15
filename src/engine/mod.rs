pub mod randaap;
pub mod sherlock;
pub mod slonenok;

use std::iter::Iterator;

use crate::algorithm::judge::{Eval, ZERO_EVAL};
use crate::algorithm::meta::Meta;
use crate::board::mv::Move;
use crate::board::position::Position;

#[derive(Clone)]
pub struct EngineResult {
    pub mv: Move,
    pub evaluation: Eval,
    pub meta: Meta,
}

impl EngineResult {
    pub fn create(mv: Move, evaluation: Eval, meta: Meta) -> EngineResult {
        EngineResult {
            mv,
            evaluation,
            meta,
        }
    }
    pub fn empty() -> EngineResult {
        EngineResult::create(Move::null(), ZERO_EVAL, Meta::create())
    }
}

pub trait Engine: Iterator<Item = EngineResult> {
    fn display_name(&self) -> &str;
    fn set_position(&mut self, position: &Position);
}
