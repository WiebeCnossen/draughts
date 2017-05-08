use algorithm::judge::{ZERO_EVAL, MAX_EVAL, Eval, Judge};
use algorithm::metric::{Meta, Metric};
use algorithm::mtdf::mtd_f;
use algorithm::depth::DepthScope;
use board::bitboard::BitboardPosition;
use board::generator::Generator;
use board::piece::{WHITE_MAN, WHITE_KING, BLACK_MAN, BLACK_KING};
use board::piece::Color::White;
use board::mv::Move;
use board::position::{Game, Position};
use engine::{Engine, EngineResult};

struct RandAapJudge {
    generator: Generator,
}

const PIECES: [i16; 5] = [0, 500, 1500, -500, -1500];
const FIELDS: [i16; 50] = [0, 0, 0, 0, 0, 1, 0, 0, 0, 1, 1, 0, 0, 0, 1, 1, 0, 0, 0, 1, 1, 0, 0, 0,
                           20, 20, 0, 0, 0, 30, 10, 0, 0, 0, 15, 1, 0, 10, 0, 1, 1, 20, 20, 0, 1,
                           0, 30, 50, 30, 0];

impl RandAapJudge {
    pub fn create() -> RandAapJudge {
        RandAapJudge { generator: Generator::create() }
    }

    fn evaluate(&self, piece: u8, field: usize) -> Eval {
        PIECES[piece as usize] +
        match piece {
            WHITE_MAN => FIELDS[field],
            BLACK_MAN => -FIELDS[49 - field],
            _ => ZERO_EVAL,
        }
    }
}

impl Judge for RandAapJudge {
    fn evaluate(&self, position: &Position) -> Eval {
        let eval = (0usize..50usize).fold((0, 0, 0), |(white, black, score), i| {
            let piece = position.piece_at(i);
            (match piece {
                 WHITE_MAN | WHITE_KING => white + 1,
                 _ => white,
             },
             match piece {
                 BLACK_MAN | BLACK_KING => black + 1,
                 _ => black,
             },
             score + self.evaluate(piece, i))
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
    max_nodes: usize,
    judge: RandAapJudge,
}

impl RandAap {
    pub fn create(max_nodes: usize) -> RandAap {
        RandAap {
            max_nodes,
            judge: RandAapJudge::create(),
        }
    }
}

impl Engine for RandAap {
    fn suggest(&mut self, position: &Position) -> EngineResult {
        let position = &BitboardPosition::clone(position);
        let mut depth = 0u8;
        let mut cut = 0;
        let mut meta = Meta::create();
        loop {
            meta.put_depth(depth);
            let mtd = mtd_f::<BitboardPosition, DepthScope>(&mut self.judge, position, depth, cut);
            cut = mtd.evaluation;
            depth = depth + 1;
            meta.add_nodes(mtd.meta.get_nodes());
            if depth > 63 || mtd.evaluation >= MAX_EVAL || meta.get_nodes() >= self.max_nodes {
                return EngineResult::create(mtd.mv, mtd.evaluation, meta);
            }
        }
    }

    fn display_name(&self) -> &str {
        self.judge.display_name()
    }
}
