use board::generator::Generator;
use board::piece::{WHITE_MAN, WHITE_KING, BLACK_MAN, BLACK_KING};
use board::piece::Color::White;
use board::mv::Move;
use board::position::Position;
use engine::judge::{ZERO_EVAL, Eval, Judge};

pub struct Slonenok {
  generator : Generator
}

struct PositionStats {
  pub piece_count: [Eval; 5],
  pub voffset_white: [Eval; 10],
  pub voffset_black: [Eval; 10],
  pub hoffset_white: [Eval; 10],
  pub hoffset_black: [Eval; 10]
}

impl PositionStats {
  pub fn for_position(position: &Position) -> PositionStats {
    let mut piece_count = [0; 5];
    let mut voffset_white = [0; 10];
    let mut voffset_black = [0; 10];
    let mut hoffset_white = [0; 10];
    let mut hoffset_black = [0; 10];

    for field in 0..50 {
      let piece = position.piece_at(field);
      piece_count[piece as usize] = piece_count[piece as usize] + 1;
      match piece {
        WHITE_MAN => {
          let x = 1 + 2 * (field % 5) - field / 5 % 2;
          hoffset_white[x] = hoffset_white[x] + 1;
          let y = 9 - field / 5;
          voffset_white[y] = voffset_white[y] + 1;
        },
        BLACK_MAN => {
          let x = 8 - 2 * (field % 5) + field / 5 % 2;
          hoffset_black[x] = hoffset_white[x] + 1;
          let y = field / 5;
          voffset_black[y] = voffset_black[y] + 1;
        },
        _ => ()
      };
    }

    PositionStats {
      piece_count: piece_count,
      voffset_white: voffset_white,
      voffset_black: voffset_black,
      hoffset_white: hoffset_white,
      hoffset_black: hoffset_black
    }
  }
}

const PIECES : [Eval; 5] = [ZERO_EVAL, 500, 1500, -500, -1500];
const HOFFSET : [Eval; 10] = [0, 1, 3, 7, 15, 15, 7, 3, 1, 0];
const VOFFSET_FULL : [Eval; 10] = [8, 7, 5, 1, -7, -23, -7, 1, 5, 7];
const VOFFSET_EMPTY : [Eval; 10] = [-15, -23, -7, 1, 5, 7, 8, 9, 10, 11];
const BALANCE : [Eval; 10] = [-6, -5, -4, -3, -2, 2, 3, 4, 5, 6];

impl Slonenok {
  pub fn create(generator : Generator) -> Slonenok {
    Slonenok {
      generator: generator
    }
  }

  // draw heuristic
  fn drawish(&self, stats: &PositionStats) -> bool {
    let whites = stats.piece_count[WHITE_MAN as usize] + stats.piece_count[WHITE_KING as usize];
    let blacks = stats.piece_count[BLACK_MAN as usize] + stats.piece_count[BLACK_KING as usize];
    stats.piece_count[WHITE_KING as usize] > 0 && stats.piece_count[BLACK_KING as usize] > 0
    && whites <= 3 && blacks <= 3
  }
}

impl Judge for Slonenok {
  fn evaluate(&self, position: &Position) -> Eval {
    let stats = PositionStats::for_position(position);

    let beans = (0..5).fold(0, |b,i| b + PIECES[i] * stats.piece_count[i]);
    let men = stats.piece_count[WHITE_MAN as usize] + stats.piece_count[BLACK_MAN as usize];
    let hoffset_white = (0..10).fold(0, |b,i| b + HOFFSET[i] * stats.hoffset_white[i]);
    let hoffset_black = (0..10).fold(0, |b,i| b + HOFFSET[i] * stats.hoffset_black[i]);
    let voffset_white_full = (0..10).fold(0, |b,i| b + VOFFSET_FULL[i] * stats.voffset_white[i]);
    let voffset_white_empty = (0..10).fold(0, |b,i| b + VOFFSET_EMPTY[i] * stats.voffset_white[i]);
    let voffset_black_full = (0..10).fold(0, |b,i| b + VOFFSET_FULL[i] * stats.voffset_black[i]);
    let voffset_black_empty = (0..10).fold(0, |b,i| b + VOFFSET_EMPTY[i] * stats.voffset_black[i]);
    let voffset_white =
      if men >= 30 { voffset_white_full }
      else if men <= 10 { voffset_white_empty }
      else { ((men - 30) * voffset_white_full + (30 - men) * voffset_white_empty) / 20 };
    let voffset_black =
      if men >= 30 { voffset_black_full }
      else if men <= 10 { voffset_black_empty }
      else { ((men - 30) * voffset_black_full + (30 - men) * voffset_black_empty) / 20 };
    let balance_white = (0..10).fold(0, |b,i| b + BALANCE[i] * stats.hoffset_white[i]);
    let balance_black = (0..10).fold(0, |b,i| b + BALANCE[i] * stats.hoffset_black[i]);

    let score = beans
      + (hoffset_white - hoffset_black)
      + (voffset_white - voffset_black)
      - 2 * (balance_white.abs() - balance_black.abs());
    let scaled =
      if self.drawish(&stats) { score / 10 } else {
        let min_kings =
          if stats.piece_count[WHITE_KING as usize] < stats.piece_count[BLACK_KING as usize] {
            stats.piece_count[WHITE_KING as usize]
          }
          else {
            stats.piece_count[BLACK_KING as usize]
          };
        score >> min_kings
      };
    if position.side_to_move() == White { scaled } else { -scaled }
  }

  fn moves(&self, position: &Position) -> Vec<Move> {
    self.generator.legal_moves(position)
  }

  fn quiet_move(&self, position: &Position, mv: &Move) -> bool {
    mv.num_taken() == 0 &&
    if position.side_to_move() == White {
      mv.to() >= 14 || position.piece_at(mv.from()) != WHITE_MAN
    }
    else {
      mv.to() <= 35 || position.piece_at(mv.from()) != BLACK_MAN
    }
  }

  fn display_name(&self) -> &str { "Slonënok" }
}
