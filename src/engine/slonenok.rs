use board::generator::Generator;
use board::piece::{WHITE_MAN, WHITE_KING, BLACK_MAN, BLACK_KING};
use board::piece::Color::White;
use board::mv::Move;
use board::position::Position;
use engine::judge::{ZERO_EVAL, Eval, Judge};

pub struct Slonenok {
  generator : Generator
}

const PIECES : [i16; 5] = [0, 1000, 3000, -1000, -3000];
const OFFSET : [i16; 10] = [0, 1, 3, 7, 15, 15, 7, 3, 1, 0];

impl Slonenok {
  pub fn create(generator : Generator) -> Slonenok {
    Slonenok { generator: generator }
  }

  fn evaluate(piece: u8, field: usize) -> Eval {
    match piece {
      WHITE_MAN | WHITE_KING => PIECES[piece as usize] + OFFSET[field / 5] + OFFSET[2 * (field % 5) + 1 - field / 5 % 2],
      BLACK_MAN | BLACK_KING => PIECES[piece as usize] - OFFSET[field / 5] - OFFSET[2 * (field % 5) + 1 - field / 5 % 2],
      _ => ZERO_EVAL
    }

  }
}

impl Judge for Slonenok {
  fn evaluate(&self, position: &Position) -> Eval {
    let eval = (0usize..50usize)
      .map(|i| Slonenok::evaluate(position.piece_at(i), i))
      .sum();
    if position.side_to_move() == White { eval } else { -eval }
  }

  fn moves(&self, position: &Position) -> Vec<Move> {
    self.generator.legal_moves(position)
  }

  fn quiet(&self, _: &Position, moves: &[Move]) -> bool {
    moves.len() > 1 && moves[0].num_taken() == 0
  }
}
