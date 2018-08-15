use std::cmp::Ordering;
use std::collections::HashMap;
use std::iter;
use std::sync::{Arc, RwLock};

use super::{Engine, EngineResult};
use crate::algorithm::bns::best_node_search_parallel;
use crate::algorithm::judge::{Eval, Judge, PositionMemory, MAX_EVAL, MIN_EVAL, ZERO_EVAL};
use crate::algorithm::logarithmic::LogarithmicScope;
use crate::algorithm::meta::{Meta, Nodes};
use crate::algorithm::scope::Depth;
use crate::algorithm::search::SearchResult;
use crate::board::generator::Generator;
use crate::board::mv::Move;
use crate::board::piece::Color::White;
use crate::board::piece::{BLACK_KING, BLACK_MAN, WHITE_KING, WHITE_MAN};
use crate::board::position::{Field, Position};
use crate::board::stars::Stars;
use crate::board::stats::PositionStats;

const PIECES: [Eval; 5] = [ZERO_EVAL, 500, 1475, -500, -1475];
const BALANCE: [Eval; 10] = [-54, -52, -48, -42, -10, 10, 42, 48, 52, 54];
const CENTER: [Eval; 10] = [-16, -8, 6, 8, 10, 10, 8, 6, -8, -16];
const THREES: [usize; 5] = [1, 3, 9, 27, 81];
const LOCKED: Eval = -300;
const SEMI_LOCKED: Eval = -49;
const HANGING: Eval = -25;
const ISOLATED: Eval = -50;
const SEMI_HANGING: Eval = -5;
const BIRDY: Eval = 19;
const TAIL: Eval = 23;
const EXTRA: Eval = 8;
const TL: usize = 0;
const TR: usize = 1;
const MM: usize = 2;
const BL: usize = 3;
const BR: usize = 4;

type SmallField = u8;

struct HashEval {
    lower: Eval,
    upper: Eval,
    depth: Depth,
    from: SmallField,
    to: SmallField,
    generation: u8,
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

    fn update(&mut self, other: &HashEval) {
        match self.depth.cmp(&other.depth) {
            Ordering::Greater => (),
            Ordering::Equal => {
                self.lower = self.lower.max(other.lower);
                self.upper = self.upper.min(other.upper);
                self.generation = other.generation;
                if self.from == 0 && self.to == 0 {
                    self.from = other.from;
                    self.to = other.to;
                }
            }
            Ordering::Less => {
                self.depth = other.depth;
                self.lower = other.lower;
                self.upper = other.upper;
                self.from = other.from;
                self.to = other.to;
                self.generation = other.generation;
            }
        }
    }
}

pub struct SherlockJudge {
    generator: Generator,
    stars: Stars,
    evals: [Eval; 243],
    private_hash: HashMap<Position, HashEval>,
    shared_hash: Arc<RwLock<HashMap<Position, HashEval>>>,
    generation: u8,
}

impl Clone for SherlockJudge {
    fn clone(&self) -> Self {
        Self {
            generator: self.generator.clone(),
            stars: self.stars.clone(),
            evals: self.evals,
            private_hash: HashMap::new(),
            shared_hash: Arc::clone(&self.shared_hash),
            generation: self.generation,
        }
    }
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
                            evals[star] = sign * match (supporters, blockers, lockers) {
                                (2, _, 2) => LOCKED,
                                (_, _, 1) => SEMI_LOCKED,
                                (0, 2, 0) => HANGING,
                                (0, 0, _) => ISOLATED,
                                (1, 2, 0) => SEMI_HANGING,
                                (2, 0, 0) => BIRDY,
                                (2, b, 0) => TAIL + EXTRA * b,
                                (s, b, 0) => EXTRA * (s + b),
                                _ => 0,
                            };
                        }
                    }
                }
            }
        }

        SherlockJudge {
            generator,
            evals,
            stars: Stars::create(),
            private_hash: HashMap::new(),
            shared_hash: Arc::new(RwLock::new(HashMap::new())),
            generation: 0,
        }
    }

    // draw heuristic
    fn drawish(&self, stats: &PositionStats) -> bool {
        let whites = stats.piece_count[WHITE_MAN as usize] + stats.piece_count[WHITE_KING as usize];
        let blacks = stats.piece_count[BLACK_MAN as usize] + stats.piece_count[BLACK_KING as usize];
        stats.piece_count[WHITE_KING as usize] > 0
            && stats.piece_count[BLACK_KING as usize] > 0
            && whites <= 3
            && blacks <= 3
    }

    pub fn reset(&mut self) {
        let generation = self.generation;
        self.shared_hash
            .write()
            .unwrap()
            .retain(|_, value| value.generation == generation);
        self.generation += 1;
    }

    fn balance(&self, hoffset: &[Eval]) -> Eval {
        -hoffset
            .iter()
            .enumerate()
            .map(|(i, &offset)| BALANCE[i] * offset)
            .sum::<Eval>()
            .abs()
    }

    fn center(&self, hoffset: &[Eval]) -> Eval {
        hoffset
            .iter()
            .enumerate()
            .map(|(i, &offset)| CENTER[i] * offset)
            .sum()
    }
}

const HASH_DEPTH: Depth = 2;

impl Judge for SherlockJudge {
    fn recall(&self, position: &Position, depth: Depth) -> PositionMemory {
        if depth < HASH_DEPTH {
            return PositionMemory::empty();
        }
        match self.private_hash.get(position) {
            Some(found) => found.as_memory(),
            None if depth < HASH_DEPTH => PositionMemory::empty(),
            _ => match self.shared_hash.read().unwrap().get(position) {
                Some(found) => found.as_memory(),
                _ => PositionMemory::empty(),
            },
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
        // if depth < HASH_DEPTH {
        //     return;
        // }
        let mv = mv.unwrap_or_else(Move::null);
        let hash_eval = HashEval {
            depth,
            lower: if low { MIN_EVAL } else { evaluation },
            upper: if low { evaluation } else { MAX_EVAL },
            from: mv.from() as SmallField,
            to: mv.to() as SmallField,
            generation: self.generation,
        };

        self.private_hash
            .entry(*position)
            .and_modify(|found| found.update(&hash_eval))
            .or_insert(hash_eval);
    }
    fn consolidate(&mut self) {
        let mut hash = self.shared_hash.write().unwrap();
        for (position, hash_eval) in self.private_hash.drain() {
            hash.entry(position)
                .and_modify(|found| found.update(&hash_eval))
                .or_insert(hash_eval);
        }
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

        let balance_score = self.balance(&stats.hoffset_white) - self.balance(&stats.hoffset_black);
        let center_score = self.center(&stats.hoffset_white) - self.center(&stats.hoffset_black);

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

        let score = beans
            + structure
            + (32 - men) * (dev_white - dev_black) / 2
            + balance_score
            + center_score;
        let scaled = if self.drawish(&stats) {
            score / 100
        } else {
            let min_kings = if stats.piece_count[WHITE_KING as usize]
                < stats.piece_count[BLACK_KING as usize]
            {
                stats.piece_count[WHITE_KING as usize]
            } else {
                stats.piece_count[BLACK_KING as usize]
            };
            score >> (2 * min_kings)
        };
        if position.side_to_move() == White {
            scaled
        } else {
            -scaled
        }
    }

    fn moves(&self, position: &Position, depth: Depth) -> Vec<Move> {
        let mut moves = self.generator.legal_moves(position);
        let memory = self.recall(position, depth);
        if memory.has_move() {
            if let Some(found) = moves
                .iter()
                .position(|mv| mv.from() == memory.from && mv.to() == memory.to)
            {
                if found > 0 {
                    moves.swap(0, found);
                }
            }
        }
        moves
    }

    fn quiet_move(&self, position: &Position, mv: &Move) -> bool {
        mv.num_taken() == 0 && if position.side_to_move() == White {
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
    sherlocks: Vec<SherlockJudge>,
    previous: EngineResult,
    position: Position,
}

impl Sherlock {
    pub fn create(max_nodes: Nodes) -> Sherlock {
        Sherlock {
            max_nodes,
            sherlocks: iter::repeat(SherlockJudge::create(Generator::create()))
                .take(8)
                .collect(),
            previous: EngineResult::create(Move::null(), ZERO_EVAL, Meta::create()),
            position: Position::initial(),
        }
    }
}

impl Iterator for Sherlock {
    type Item = EngineResult;
    fn next(&mut self) -> Option<EngineResult> {
        if self.previous.meta.get_nodes() >= self.max_nodes
            || self.previous.meta.get_depth() > 27
            || self.previous.evaluation == MIN_EVAL
            || self.previous.evaluation == MAX_EVAL
        {
            return None;
        }

        let mut meta = self.previous.meta.clone();
        let search_result = if meta.get_depth() == 0 {
            SearchResult::evaluation(self.previous.evaluation)
        } else {
            SearchResult::with_move(self.previous.mv, self.previous.evaluation)
        };
        let depth = meta.get_depth() + 1;
        meta.put_depth(depth);
        let bns = best_node_search_parallel::<SherlockJudge, LogarithmicScope>(
            &mut self.sherlocks,
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
        self.sherlocks[0].reset();
        self.sherlocks[0].consolidate();
        let generation = self.sherlocks[0].generation;
        for sherlock in &mut self.sherlocks {
            sherlock.generation = generation;
        }
        self.position = *position;
        self.previous = EngineResult::empty();
    }
}
