use std::collections::HashMap;
use std::sync::{Arc, RwLock};

use algorithm::bns::best_node_search_parallel;
use algorithm::judge::{Eval, Judge, PositionMemory, MAX_EVAL, MIN_EVAL, ZERO_EVAL};
use algorithm::logarithmic::LogarithmicScope;
use algorithm::meta::{Meta, Nodes};
use algorithm::scope::Depth;
use algorithm::search::SearchResult;
use board::generator::Generator;
use board::mv::Move;
use board::piece::{BLACK_KING, BLACK_MAN, WHITE_KING, WHITE_MAN};
use board::piece::Color::White;
use board::position::{Field, Position};
use board::stats::PositionStats;
use board::stars::Stars;
use engine::{Engine, EngineResult};

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
}

#[derive(Clone)]
pub struct SherlockJudge {
    generator: Arc<Generator>,
    stars: Stars,
    evals: [Eval; 243],
    hash: Arc<RwLock<HashMap<Position, HashEval>>>,
    generation: u8,
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
            generator: Arc::new(generator),
            evals,
            stars: Stars::create(),
            hash: Arc::new(RwLock::new(HashMap::new())),
            generation: 0,
        }
    }

    // draw heuristic
    fn drawish(&self, stats: &PositionStats) -> bool {
        let whites = stats.piece_count[WHITE_MAN as usize] + stats.piece_count[WHITE_KING as usize];
        let blacks = stats.piece_count[BLACK_MAN as usize] + stats.piece_count[BLACK_KING as usize];
        stats.piece_count[WHITE_KING as usize] > 0 && stats.piece_count[BLACK_KING as usize] > 0
            && whites <= 3 && blacks <= 3
    }

    pub fn reset(&mut self) {
        let generation = self.generation;
        self.hash
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

const HASH_DEPTH: Depth = 4;

impl Judge for SherlockJudge {
    fn recall(&self, position: &Position, depth: Depth) -> PositionMemory {
        if depth < HASH_DEPTH {
            return PositionMemory::empty();
        }
        match self.hash.read().unwrap().get(position) {
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
        if depth < HASH_DEPTH {
            return;
        }
        let (has_move, from, to) = if let Some(mv) = mv {
            (true, mv.from() as SmallField, mv.to() as SmallField)
        } else {
            (false, 0, 0)
        };

        let mut hash = self.hash.write().unwrap();
        let hash_eval = if let Some(found) = hash.get(position) {
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
                    generation: self.generation,
                }
            } else {
                HashEval {
                    depth,
                    lower: if low { MIN_EVAL } else { evaluation },
                    upper: if low { evaluation } else { MAX_EVAL },
                    from,
                    to,
                    generation: self.generation,
                }
            }
        } else {
            HashEval {
                depth,
                lower: if low { MIN_EVAL } else { evaluation },
                upper: if low { evaluation } else { MAX_EVAL },
                from,
                to,
                generation: self.generation,
            }
        };
        hash.insert(*position, hash_eval);
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

        let score = beans + structure + (32 - men) * (dev_white - dev_black) / 2 + balance_score
            + center_score;
        let scaled = if self.drawish(&stats) {
            score / 10
        } else {
            let min_kings = if stats.piece_count[WHITE_KING as usize]
                < stats.piece_count[BLACK_KING as usize]
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
    sherlock: SherlockJudge,
    previous: EngineResult,
    position: Position,
}

impl Sherlock {
    pub fn create(max_nodes: Nodes) -> Sherlock {
        Sherlock {
            max_nodes,
            sherlock: SherlockJudge::create(Generator::create()),
            previous: EngineResult::create(Move::null(), ZERO_EVAL, Meta::create()),
            position: Position::initial(),
        }
    }
}

impl Iterator for Sherlock {
    type Item = EngineResult;
    fn next(&mut self) -> Option<EngineResult> {
        if self.previous.meta.get_nodes() >= self.max_nodes || self.previous.meta.get_depth() > 27
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
        self.position = *position;
        self.previous = EngineResult::empty();
    }
}
