use algorithm::judge::Eval;
use board::mv::Move;

pub struct SearchResult {
    pub mv: Option<Move>,
    pub evaluation: Eval,
}

impl SearchResult {
    pub fn with_move(mv: Move, evaluation: Eval) -> SearchResult {
        SearchResult {
            mv: Some(mv),
            evaluation,
        }
    }

    pub fn evaluation(evaluation: Eval) -> SearchResult {
        SearchResult {
            mv: None,
            evaluation,
        }
    }
}
