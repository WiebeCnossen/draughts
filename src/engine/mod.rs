pub mod randaap;
pub mod slonenok;

use algorithm::search::SearchResult;
use board::position::Position;

pub trait Engine {
    fn suggest(&mut self, position: &Position) -> SearchResult;
    fn display_name(&self) -> &str;
}
