pub mod randaap;
pub mod slonenok;

use algorithm::judge::Eval;
use algorithm::metric::Meta;
use board::mv::Move;
use board::position::Position;

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
}

pub trait Engine {
    fn suggest(&mut self, position: &Position) -> EngineResult;
    fn display_name(&self) -> &str;
}
