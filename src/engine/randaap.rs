use board::generator::Generator;
use board::piece::{WHITE_MAN, BLACK_MAN};
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

  fn evaluate(piece: u8, field: usize) -> Eval {
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
    let eval = (0usize..50usize)
      .map(|i| RandAap::evaluate(position.piece_at(i), i))
      .sum();
    if position.side_to_move() == White { eval } else { -eval }
  }

  fn moves(&self, position: &Position) -> Vec<Move> {
    self.generator.legal_moves(position)
  }

  fn quiet(&self, _: &Position, moves: &[Move]) -> bool {
    moves.len() > 1 && moves[0].num_taken() == 0
  }

  fn display_name(&self) -> &str { "RandAap" }
}
