use std::cmp::Ordering::{Less, Greater};
use std::collections::HashMap;

use algorithm::bns::best_node_search;
use algorithm::judge::{ZERO_EVAL, MIN_EVAL, MAX_EVAL, Eval, Judge, PositionMemory};
use algorithm::logarithmic::LogarithmicScope;
use algorithm::metric::{Nodes, Meta, Metric};
use algorithm::scope::Depth;
use algorithm::search::SearchResult;
use board::bitboard::BitboardPosition;
use board::generator::Generator;
use board::mv::Move;
use board::piece::{EMPTY, WHITE_MAN, WHITE_KING, BLACK_MAN, BLACK_KING};
use board::piece::Color::White;
use board::position::{Field, Game, Position};
use board::stats::PositionStats;
use engine::{Engine, EngineResult};

const PIECES: [Eval; 5] = [ZERO_EVAL, 500, 1500, -500, -1500];
const HOFFSET: [Eval; 10] = [0, 1, 3, 7, 15, 15, 7, 3, 1, 0];
const VOFFSET_FULL: [Eval; 10] = [8, 7, 5, 1, -7, -23, -7, 1, 5, 7];
const VOFFSET_EMPTY: [Eval; 10] = [-15, -23, -7, 1, 5, 7, 8, 9, 10, 11];
const BALANCE: [Eval; 10] = [-6, -5, -4, -3, -2, 2, 3, 4, 5, 6];
const KILLERS: usize = 20;

type SmallField = u8;
struct HashEval {
    lower: Eval,
    upper: Eval,
    depth: Depth,
    from: SmallField,
    to: SmallField,
}

impl HashEval {
    fn as_memory(&self) -> PositionMemory {
        PositionMemory::create(
            self.depth,
            self.lower,
            self.upper,
            self.from as Field,
            self.to as Field,
        )
    }
}

pub struct SlonenokJudge {
    generator: Generator,
    hash: HashMap<BitboardPosition, HashEval>,
    white_killer_moves: [Move; KILLERS],
    white_killer_cursor: usize,
    black_killer_moves: [Move; KILLERS],
    black_killer_cursor: usize,
}

impl SlonenokJudge {
    pub fn create(generator: Generator) -> SlonenokJudge {
        SlonenokJudge {
            generator: generator,
            hash: HashMap::new(),
            white_killer_moves: [Move::Shift(0, 0); KILLERS],
            white_killer_cursor: 0,
            black_killer_moves: [Move::Shift(0, 0); KILLERS],
            black_killer_cursor: 0,
        }
    }

    // draw heuristic
    fn drawish(&self, stats: &PositionStats) -> bool {
        let whites = stats.piece_count[WHITE_MAN as usize] + stats.piece_count[WHITE_KING as usize];
        let blacks = stats.piece_count[BLACK_MAN as usize] + stats.piece_count[BLACK_KING as usize];
        stats.piece_count[WHITE_KING as usize] > 0 &&
            stats.piece_count[BLACK_KING as usize] > 0 && whites <= 3 && blacks <= 3
    }

    pub fn reset(&mut self) {
        self.hash.clear()
    }

    fn evaluate_structure(&self, position: &Position) -> Eval {
        let mut structure = 0;

        // hanging piece penalty
        for start in 10..14 {
            if position.piece_at(start) == BLACK_MAN && position.piece_at(start + 1) == BLACK_MAN &&
                position.piece_at(start - 5) == BLACK_MAN &&
                position.piece_at(start - 10) == EMPTY
            {
                if position.piece_at(start - 9) == EMPTY {
                    structure += 100;
                } else if start == 13 {
                    structure += if position.piece_at(24) == WHITE_MAN {
                        100
                    } else {
                        20
                    };
                }
            }
        }
        for start in 15..19 {
            if position.piece_at(start) == BLACK_MAN && position.piece_at(start + 1) == BLACK_MAN &&
                position.piece_at(start - 4) == BLACK_MAN &&
                position.piece_at(start - 10) == EMPTY &&
                position.piece_at(start - 9) == EMPTY
            {
                structure += 100;
            }
        }
        for start in 30..34 {
            if position.piece_at(start) == WHITE_MAN && position.piece_at(start + 1) == WHITE_MAN &&
                position.piece_at(start + 6) == WHITE_MAN &&
                position.piece_at(start + 10) == EMPTY &&
                position.piece_at(start + 11) == EMPTY
            {
                structure -= 100;
            }
        }
        for start in 35..39 {
            if position.piece_at(start) == WHITE_MAN && position.piece_at(start + 1) == WHITE_MAN &&
                position.piece_at(start + 5) == WHITE_MAN &&
                position.piece_at(start + 10) == EMPTY
            {
                if position.piece_at(start + 11) == EMPTY {
                    structure -= 100;
                } else if start == 35 {
                    structure -= if position.piece_at(25) == BLACK_MAN {
                        100
                    } else {
                        20
                    };
                }
            }
        }

        // corner penalty
        if position.piece_at(4) == BLACK_MAN {
            structure += 15;
        }
        if position.piece_at(45) == WHITE_MAN {
            structure -= 15;
        }

        // fork locks
        for row in 1..4 {
            for start in 10 * row - 5..10 * row - 1 {
                if position.piece_at(start) == BLACK_MAN &&
                    position.piece_at(start + 1) == BLACK_MAN &&
                    position.piece_at(start + 10) == WHITE_MAN &&
                    position.piece_at(start + 11) == WHITE_MAN
                {
                    match position.piece_at(start + 5) {
                        WHITE_MAN => structure -= 100,
                        BLACK_MAN => structure += 100,
                        _ => (),
                    };
                }
            }
            for start in 10 * row..10 * row + 4 {
                if position.piece_at(start) == BLACK_MAN &&
                    position.piece_at(start + 1) == BLACK_MAN &&
                    position.piece_at(start + 10) == WHITE_MAN &&
                    position.piece_at(start + 11) == WHITE_MAN
                {
                    match position.piece_at(start + 6) {
                        WHITE_MAN => structure -= 100,
                        BLACK_MAN => structure += 100,
                        _ => (),
                    };
                }
            }
        }

        structure
    }
}

impl Judge for SlonenokJudge {
    fn recall(&self, position: &Position) -> PositionMemory {
        let bitboard = BitboardPosition::clone(position);
        match self.hash.get(&bitboard) {
            Some(found) => found.as_memory(),
            _ => PositionMemory::empty(),
        }
    }
    fn remember(
        &mut self,
        position: &Position,
        depth: Depth,
        evaluation: Eval,
        mv: Option<Move>,
        low: bool,
    ) {
        let (has_move, from, to) = if let Some(mv) = mv {
            if position.side_to_move() == White {
                if !self.white_killer_moves.contains(&mv) {
                    self.white_killer_moves[self.white_killer_cursor] = mv;
                    self.white_killer_cursor = (self.white_killer_cursor + 1) % KILLERS;
                }
            } else if !self.black_killer_moves.contains(&mv) {
                self.black_killer_moves[self.black_killer_cursor] = mv;
                self.black_killer_cursor = (self.black_killer_cursor + 1) % KILLERS;
            }
            (true, mv.from() as SmallField, mv.to() as SmallField)
        } else {
            (false, 0, 0)
        };

        let bitboard = BitboardPosition::clone(position);
        let hash_eval = if let Some(found) = self.hash.get(&bitboard) {
            if found.depth > depth {
                return;
            }
            if found.depth == depth {
                if !low && evaluation <= found.lower || low && found.upper >= evaluation {
                    return;
                }
                HashEval {
                    depth,
                    lower: if low { found.lower } else { evaluation },
                    upper: if low { evaluation } else { found.upper },
                    from: if has_move { from } else { found.from },
                    to: if has_move { to } else { found.to },
                }
            } else {
                HashEval {
                    depth,
                    lower: if low { MIN_EVAL } else { evaluation },
                    upper: if low { evaluation } else { MAX_EVAL },
                    from,
                    to,
                }
            }
        } else {
            HashEval {
                depth,
                lower: if low { MIN_EVAL } else { evaluation },
                upper: if low { evaluation } else { MAX_EVAL },
                from,
                to,
            }
        };
        self.hash.insert(bitboard, hash_eval);
    }

    fn evaluate(&self, position: &Position) -> Eval {
        let stats = PositionStats::for_position(position);

        let beans = (0..5).fold(0, |b, i| b + PIECES[i] * stats.piece_count[i]);
        let men = stats.piece_count[WHITE_MAN as usize] + stats.piece_count[BLACK_MAN as usize];
        let hoffset_white = (0..10).fold(0, |b, i| b + HOFFSET[i] * stats.hoffset_white[i]);
        let hoffset_black = (0..10).fold(0, |b, i| b + HOFFSET[i] * stats.hoffset_black[i]);
        let voffset_white_full =
            (0..10).fold(0, |b, i| b + VOFFSET_FULL[i] * stats.voffset_white[i]);
        let voffset_white_empty =
            (0..10).fold(0, |b, i| b + VOFFSET_EMPTY[i] * stats.voffset_white[i]);
        let voffset_black_full =
            (0..10).fold(0, |b, i| b + VOFFSET_FULL[i] * stats.voffset_black[i]);
        let voffset_black_empty =
            (0..10).fold(0, |b, i| b + VOFFSET_EMPTY[i] * stats.voffset_black[i]);
        let voffset_white = if men >= 30 {
            voffset_white_full
        } else if men <= 10 {
            voffset_white_empty
        } else {
            ((men - 30) * voffset_white_full + (30 - men) * voffset_white_empty) / 20
        };
        let voffset_black = if men >= 30 {
            voffset_black_full
        } else if men <= 10 {
            voffset_black_empty
        } else {
            ((men - 30) * voffset_black_full + (30 - men) * voffset_black_empty) / 20
        };
        let balance_white = (0..10).fold(0, |b, i| b + BALANCE[i] * stats.hoffset_white[i]);
        let balance_black = (0..10).fold(0, |b, i| b + BALANCE[i] * stats.hoffset_black[i]);

        let structure = self.evaluate_structure(position);

        let score = beans + structure + (hoffset_white - hoffset_black) +
            (voffset_white - voffset_black) -
            2 * (balance_white.abs() - balance_black.abs());
        let scaled = if self.drawish(&stats) {
            score / 10
        } else {
            let min_kings = if stats.piece_count[WHITE_KING as usize] <
                stats.piece_count[BLACK_KING as usize]
            {
                stats.piece_count[WHITE_KING as usize]
            } else {
                stats.piece_count[BLACK_KING as usize]
            };
            score >> min_kings
        };
        if position.side_to_move() == White {
            scaled
        } else {
            -scaled
        }
    }

    fn moves(&self, position: &Position) -> Vec<Move> {
        let mut result = self.generator.legal_moves(position);
        if position.side_to_move() == White {
            result.sort_by(|mv1, mv2| match (
                self.white_killer_moves.contains(mv1),
                self.white_killer_moves.contains(mv2),
            ) {
                (false, true) => Greater,
                (true, false) => Less,
                _ => mv1.to().cmp(&mv2.to()),
            })
        } else {
            result.sort_by(|mv1, mv2| match (
                self.black_killer_moves.contains(mv1),
                self.black_killer_moves.contains(mv2),
            ) {
                (false, true) => Greater,
                (true, false) => Less,
                _ => mv2.to().cmp(&mv1.to()),
            })
        }
        result
    }

    fn quiet_move(&self, _: &Position, _: &Move) -> bool {
        false
    }

    fn quiet_position(&self, _: &Position, _: &[Move]) -> bool {
        false
    }

    fn display_name(&self) -> &str {
        "Slonënok"
    }
}

pub struct Slonenok {
    max_nodes: Nodes,
    slonenok: SlonenokJudge,
    previous: EngineResult,
    position: BitboardPosition,
}

impl Slonenok {
    pub fn create(max_nodes: Nodes) -> Slonenok {
        Slonenok {
            max_nodes,
            slonenok: SlonenokJudge::create(Generator::create()),
            previous: EngineResult::create(Move::Shift(0, 0), ZERO_EVAL, Meta::create()),
            position: BitboardPosition::initial(),
        }
    }
}

impl Iterator for Slonenok {
    type Item = EngineResult;
    fn next(&mut self) -> Option<EngineResult> {
        if self.previous.meta.get_nodes() >= self.max_nodes ||
            self.previous.meta.get_depth() > 63 ||
            self.previous.evaluation == MIN_EVAL ||
            self.previous.evaluation == MAX_EVAL
        {
            return None;
        }

        let mut meta = self.previous.meta.clone();
        let search_result = if meta.get_depth() == 0 {
            SearchResult::evaluation(self.previous.evaluation)
        } else {
            SearchResult::with_move(self.previous.mv, self.previous.evaluation)
        };
        let depth = if meta.get_nodes() == 0 {
            0
        } else {
            meta.get_depth() + 1
        };
        meta.put_depth(depth);
        meta.put_depth(depth);
        let bns = best_node_search::<BitboardPosition, LogarithmicScope>(
            &mut self.slonenok,
            &self.position,
            depth,
            &search_result,
        );
        meta.add_nodes(bns.meta.get_nodes());
        self.previous = EngineResult::create(bns.mv, bns.lower, meta);
        Some(self.previous.clone())
    }
}

impl Engine for Slonenok {
    fn display_name(&self) -> &str {
        "Slonënok"
    }
    fn set_position(&mut self, position: &Position) {
        self.slonenok.reset();
        self.position = BitboardPosition::clone(position);
        self.previous = EngineResult::empty();
    }
}
