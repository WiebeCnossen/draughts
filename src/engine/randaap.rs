use algorithm::depth::DepthScope;
use algorithm::judge::{Eval, Judge, MAX_EVAL, MIN_EVAL, ZERO_EVAL};
use algorithm::metric::{Meta, Metric, Nodes};
use algorithm::mtdf::mtd_f;
use board::generator::Generator;
use board::piece::{Piece, BLACK_KING, BLACK_MAN, WHITE_KING, WHITE_MAN};
use board::piece::Color::White;
use board::mv::Move;
use board::position::{Field, Position};
use engine::{Engine, EngineResult};

struct RandAapJudge {
    generator: Generator,
}

const PIECES: [Eval; 5] = [0, 500, 1500, -500, -1500];
const FIELDS: [Eval; 50] = [
    0, 0, 0, 0, 0, 1, 0, 0, 0, 1, 1, 0, 0, 0, 1, 1, 0, 0, 0, 1, 1, 0, 0, 0, 20, 20, 0, 0, 0, 30,
    10, 0, 0, 0, 15, 1, 0, 10, 0, 1, 1, 20, 20, 0, 1, 0, 30, 50, 30, 0,
];

impl RandAapJudge {
    pub fn create() -> RandAapJudge {
        RandAapJudge {
            generator: Generator::create(),
        }
    }

    fn evaluate(&self, piece: Piece, field: Field) -> Eval {
        PIECES[piece as usize] + match piece {
            WHITE_MAN => FIELDS[field],
            BLACK_MAN => -FIELDS[49 - field],
            _ => ZERO_EVAL,
        }
    }
}

impl Judge for RandAapJudge {
    fn evaluate(&self, position: &Position) -> Eval {
        let eval = (0..50).fold((0, 0, 0), |(white, black, score), i| {
            let piece = position.piece_at(i);
            (
                match piece {
                    WHITE_MAN | WHITE_KING => white + 1,
                    _ => white,
                },
                match piece {
                    BLACK_MAN | BLACK_KING => black + 1,
                    _ => black,
                },
                score + self.evaluate(piece, i),
            )
        });
        let score = if eval.0 <= 3 && eval.1 <= 3 {
            eval.2 / 10
        } else {
            eval.2
        };
        if position.side_to_move() == White {
            score
        } else {
            -score
        }
    }

    fn moves(&self, position: &Position) -> Vec<Move> {
        self.generator.legal_moves(position)
    }

    fn quiet_move(&self, _: &Position, mv: &Move) -> bool {
        mv.num_taken() == 0
    }

    fn display_name(&self) -> &str {
        "RandAap"
    }
}

pub struct RandAap {
    max_nodes: Nodes,
    judge: RandAapJudge,
    previous: EngineResult,
    position: Position,
}

impl RandAap {
    pub fn create(max_nodes: Nodes) -> RandAap {
        RandAap {
            max_nodes,
            judge: RandAapJudge::create(),
            previous: EngineResult::create(Move::null(), ZERO_EVAL, Meta::create()),
            position: Position::initial(),
        }
    }
}

impl Iterator for RandAap {
    type Item = EngineResult;
    fn next(&mut self) -> Option<EngineResult> {
        if self.previous.meta.get_nodes() >= self.max_nodes || self.previous.meta.get_depth() > 63
            || self.previous.evaluation == MIN_EVAL
            || self.previous.evaluation == MAX_EVAL
        {
            return None;
        }

        let mut meta = self.previous.meta.clone();
        let depth = if meta.get_nodes() == 0 {
            0
        } else {
            meta.get_depth() + 1
        };
        meta.put_depth(depth);
        let mtd = mtd_f::<Position, DepthScope>(
            &mut self.judge,
            &self.position,
            depth,
            self.previous.evaluation,
        );
        meta.add_nodes(mtd.meta.get_nodes());
        self.previous = EngineResult::create(mtd.mv, mtd.evaluation, meta);
        Some(self.previous.clone())
    }
}

impl Engine for RandAap {
    fn set_position(&mut self, position: &Position) {
        self.position = *position;
        self.previous = EngineResult::empty();
    }

    fn display_name(&self) -> &str {
        self.judge.display_name()
    }
}
