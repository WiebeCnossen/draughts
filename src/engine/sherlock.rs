use std::cmp::Ordering::{Less, Greater};
use std::collections::HashMap;

use algorithm::adaptive::AdaptiveScope;
use algorithm::bns::best_node_search;
use algorithm::judge::{ZERO_EVAL, MIN_EVAL, MAX_EVAL, Eval, Judge, PositionMemory};
use algorithm::metric::{Nodes, Meta, Metric};
use algorithm::scope::Depth;
use algorithm::search::SearchResult;
use board::bitboard::BitboardPosition;
use board::generator::Generator;
use board::mv::Move;
use board::piece::{WHITE_MAN, WHITE_KING, BLACK_MAN, BLACK_KING};
use board::piece::Color::White;
use board::position::{Field, Game, Position};
use board::stats::PositionStats;
use board::stars::Stars;
use engine::{Engine, EngineResult};

const PIECES: [Eval; 5] = [ZERO_EVAL, 500, 1500, -500, -1500];
const BALANCE: [Eval; 10] = [-27, -26, -24, -21, -15, 15, 21, 24, 26, 27];
const HOLE: [Eval; 10] = [0, 0, -35, -60, -100, -100, -100, -100, -100, -100];
const THREES: [usize; 5] = [1, 3, 9, 27, 81];
const TL: usize = 0;
const TR: usize = 1;
const MM: usize = 2;
const BL: usize = 3;
const BR: usize = 4;
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

pub struct SherlockJudge {
    generator: Generator,
    stars: Stars,
    evals: [Eval; 243],
    hash: HashMap<BitboardPosition, HashEval>,
    white_killer_moves: [Move; KILLERS],
    white_killer_cursor: usize,
    black_killer_moves: [Move; KILLERS],
    black_killer_cursor: usize,
}

impl SherlockJudge {
    pub fn create(generator: Generator) -> SherlockJudge {
        let mut evals = [0; 243];
        for tl in 0..3 {
            let star = tl * THREES[TL];
            for tr in 0..3 {
                let star = star + tr * THREES[TR];
                for mm in 1..3 {
                    // 0 is not interesting
                    let star = star + mm * THREES[MM];
                    let (sign, op) = if mm == 1 { (1, 2) } else { (-1, 1) };
                    for bl in 0..3 {
                        let star = star + bl * THREES[BL];
                        for br in 0..3 {
                            let star = star + br * THREES[BR];
                            let supporters = if mm == 1 {
                                (if mm == bl { 1 } else { 0 }) + (if mm == br { 1 } else { 0 })
                            } else {
                                (if mm == tl { 1 } else { 0 }) + (if mm == tr { 1 } else { 0 })
                            };
                            let blockers = if mm == 1 {
                                (if mm == tl { 1 } else { 0 }) + (if mm == tr { 1 } else { 0 })
                            } else {
                                (if mm == bl { 1 } else { 0 }) + (if mm == br { 1 } else { 0 })
                            };
                            let lockers = if mm == 1 {
                                (if op == tl { 1 } else { 0 }) + (if op == tr { 1 } else { 0 })
                            } else {
                                (if op == bl { 1 } else { 0 }) + (if op == br { 1 } else { 0 })
                            };
                            evals[star] = sign *
                                match (supporters, blockers, lockers) {
                                    (2, _, 2) => -150,        // locked
                                    (_, _, 2) => -100,        // ?!
                                    (_, _, 1) => -50,         // semi-locked
                                    (0, 2, 0) => -25,         // hanging
                                    (0, 0, _) => -20,         // isolated
                                    (1, 2, 0) => -5,          // semi -hanging
                                    (2, 0, 0) => 19,
                                    (2, b, 0) => 23 + 8 * b,
                                    (s, b, 0) => 8 * (s + b),
                                    _ => 0,
                                };
                        }
                    }
                }
            }
        }

        SherlockJudge {
            generator: generator,
            evals,
            stars: Stars::create(),
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
}

impl Judge for SherlockJudge {
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

        let dev_white: Eval = (1..10)
            .map(|i| i as Eval * stats.voffset_white[i] as Eval)
            .sum();
        let dev_black: Eval = (1..10)
            .map(|i| i as Eval * stats.voffset_black[i] as Eval)
            .sum();

        let balance_white = (0..10).fold(0, |b, i| b + BALANCE[i] * stats.hoffset_white[i]);
        let balance_black = (0..10).fold(0, |b, i| b + BALANCE[i] * stats.hoffset_black[i]);

        let hole_white = HOLE[(1..9)
                                  .scan(0, |&mut hole, i| {
            Some(if stats.hoffset_white[i] == 0 {
                hole + 1
            } else {
                0
            })
        })
                                  .max()
                                  .unwrap()];
        let hole_black = HOLE[(1..9)
                                  .scan(0, |&mut hole, i| {
            Some(if stats.hoffset_black[i] == 0 {
                hole + 1
            } else {
                0
            })
        })
                                  .max()
                                  .unwrap()];

        let structure = if men < 8 {
            0
        } else {
            let mut stars = [0; 32];
            for field in 0..50 {
                let value = match position.piece_at(field) {
                    WHITE_MAN => 1,
                    BLACK_MAN => 2,
                    _ => continue,
                };
                for &(star, pos) in &self.stars.positions[field][..] {
                    stars[star] += THREES[pos] * value;
                }
            }
            stars.iter().map(|&star| self.evals[star]).sum()
        };

        let score = beans + structure + (28 - men) * (dev_white - dev_black) +
            (hole_white - hole_black) -
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

    fn quiet_move(&self, position: &Position, mv: &Move) -> bool {
        mv.num_taken() == 0 &&
            if position.side_to_move() == White {
                mv.to() >= 10 || position.piece_at(mv.from()) != WHITE_MAN
            } else {
                mv.to() <= 39 || position.piece_at(mv.from()) != BLACK_MAN
            }
    }

    fn display_name(&self) -> &str {
        "Sherlock"
    }
}

pub struct Sherlock {
    max_nodes: Nodes,
    sherlock: SherlockJudge,
    previous: EngineResult,
    position: BitboardPosition,
}

impl Sherlock {
    pub fn create(max_nodes: Nodes) -> Sherlock {
        Sherlock {
            max_nodes,
            sherlock: SherlockJudge::create(Generator::create()),
            previous: EngineResult::create(Move::Shift(0, 0), ZERO_EVAL, Meta::create()),
            position: BitboardPosition::initial(),
        }
    }
}

impl Iterator for Sherlock {
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
        let bns = best_node_search::<BitboardPosition, AdaptiveScope>(
            &mut self.sherlock,
            &self.position,
            depth,
            &search_result,
        );
        meta.add_nodes(bns.meta.get_nodes());
        self.previous = EngineResult::create(bns.mv, bns.lower, meta);
        Some(self.previous.clone())
    }
}

impl Engine for Sherlock {
    fn display_name(&self) -> &str {
        "Sherlock"
    }
    fn set_position(&mut self, position: &Position) {
        self.sherlock.reset();
        self.position = BitboardPosition::clone(position);
        self.previous = EngineResult::empty();
    }
}