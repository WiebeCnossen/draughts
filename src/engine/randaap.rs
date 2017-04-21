use board::generator::Generator;
use board::piece::{WHITE_MAN, WHITE_KING, BLACK_MAN, BLACK_KING};
use board::piece::Color::White;
use board::mv::Move;
use board::position::Position;
use engine::judge::{ZERO_EVAL, Eval, Judge};

pub struct RandAap {
  generator : Generator
}

const PIECES : [i16; 5] = [0, 500, 1500, -500, -1500];
const FIELDS : [i16; 50] = [
  0, 0, 0, 0, 0,
  1, 0, 0, 0, 1,
  1, 0, 0, 0, 1,
  1, 0, 0, 0, 1,
  1, 0, 0, 0, 20,
  20, 0, 0, 0, 30,
  10, 0, 0, 0, 15,
  1, 0, 10, 0, 1,
  1, 20, 20, 0, 1,
  0, 30, 50, 30, 0,
];

impl RandAap {
  pub fn create(generator : Generator) -> RandAap {
    RandAap { generator: generator }
  }

  fn evaluate(&self, piece: u8, field: usize) -> Eval {
    PIECES[piece as usize] +
    match piece {
      WHITE_MAN => FIELDS[field],
      BLACK_MAN => -FIELDS[49 - field],
      _ => ZERO_EVAL
    }
  }
}

impl Judge for RandAap {
  fn evaluate(&self, position: &Position) -> Eval {
    let eval = (0usize..50usize).fold(
      (0, 0, 0),
      |(white, black, score), i| {
        let piece = position.piece_at(i);
        (
          match piece { WHITE_MAN | WHITE_KING => white + 1, _ => white },
          match piece { BLACK_MAN | BLACK_KING => black + 1, _ => black },
          score + self.evaluate(piece, i)
        )
      });
    let score = if eval.0 <= 3 && eval.1 <= 3 { eval.2 / 10 } else { eval.2 };
    if position.side_to_move() == White { score } else { -score }
  }

  fn moves(&self, position: &Position) -> Vec<Move> {
    self.generator.legal_moves(position)
  }

  fn quiet_move(&self, _: &Position, mv: &Move) -> bool {
    mv.num_taken() == 0
  }

  fn display_name(&self) -> &str { "RandAap" }
}
