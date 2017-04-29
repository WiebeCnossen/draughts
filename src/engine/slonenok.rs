use std::cmp::Ordering::{Less, Greater};
use std::collections::HashMap;

use board::generator::Generator;
use board::piece::{EMPTY, WHITE_MAN, WHITE_KING, BLACK_MAN, BLACK_KING};
use board::piece::Color::White;
use board::mv::Move;
use board::position::{Game, Position};
use board::bitboard::BitboardPosition;
use engine::judge::{ZERO_EVAL, MIN_EVAL, MAX_EVAL, Eval, Judge};

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
const KILLERS : usize = 20;

struct HashEval {
  pub lower: Eval,
  pub upper: Eval,
  pub depth: u8
}

pub struct Slonenok {
  generator : Generator,
  hash: HashMap<BitboardPosition, HashEval>,
  white_killer_moves: [Move; KILLERS],
  white_killer_cursor: usize,
  black_killer_moves: [Move; KILLERS],
  black_killer_cursor: usize
}

impl Slonenok {
  pub fn create(generator : Generator) -> Slonenok {
    Slonenok {
      generator: generator,
      hash: HashMap::new(),
      white_killer_moves: [Move::Shift(0, 0); KILLERS],
      white_killer_cursor: 0,
      black_killer_moves: [Move::Shift(0, 0); KILLERS],
      black_killer_cursor: 0
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
    self.hash.clear()
  }
}

impl Judge for Slonenok {
  fn recall(&self, position: &Position, depth: u8) -> (Eval, Eval) {
    let bitboard = BitboardPosition::clone(position);
    match self.hash.get(&bitboard) {
      Some(found) if found.depth >= depth => (found.lower, found.upper),
      _ => (MIN_EVAL, MAX_EVAL)
    }
  }
  fn remember(&mut self, position: &Position, depth: u8, evaluation: Eval, mv: Option<Move>, low: bool) {
    if let Some(mv) = mv {
      if position.side_to_move() == White {
        if !self.white_killer_moves.contains(&mv) {
          self.white_killer_moves[self.white_killer_cursor] = mv;
          self.white_killer_cursor = (self.white_killer_cursor + 1) % KILLERS;
        }
      }
      else {
        if !self.black_killer_moves.contains(&mv) {
          self.black_killer_moves[self.black_killer_cursor] = mv;
          self.black_killer_cursor = (self.black_killer_cursor + 1) % KILLERS;
        }
      }
    }

    let bitboard = BitboardPosition::clone(position);
    let hash_eval =
      if let Some(found) = self.hash.get(&bitboard) {
        if found.depth > depth { return }
        if found.depth == depth {
          if !low && evaluation <= found.lower || low && found.upper >= evaluation { return }
          HashEval {
            depth,
            lower: if low { found.lower } else { evaluation },
            upper: if low { evaluation } else { found.upper }
          }
        }
        else {
          HashEval {
            depth,
            lower: if low { MIN_EVAL } else { evaluation },
            upper: if low { evaluation } else { MAX_EVAL }
          }
        }
      }
      else {
        HashEval {
          depth,
          lower: if low { MIN_EVAL } else { evaluation },
          upper: if low { evaluation } else { MAX_EVAL }
        }
      };
    self.hash.insert(bitboard, hash_eval);
  }
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

    let mut structure = 0;
    for start in 10..14 {
      if position.piece_at(start) == BLACK_MAN &&
         position.piece_at(start + 1) == BLACK_MAN &&
         position.piece_at(start - 5) == BLACK_MAN &&
         position.piece_at(start - 10) == EMPTY &&
         position.piece_at(start - 9) == EMPTY {
        structure += 100;
      }
    }
    for start in 15..19 {
      if position.piece_at(start) == BLACK_MAN &&
         position.piece_at(start + 1) == BLACK_MAN &&
         position.piece_at(start - 4) == BLACK_MAN &&
         position.piece_at(start - 10) == EMPTY &&
         position.piece_at(start - 9) == EMPTY {
        structure += 100;
      }
    }
    for start in 30..34 {
      if position.piece_at(start) == WHITE_MAN &&
         position.piece_at(start + 1) == WHITE_MAN &&
         position.piece_at(start + 6) == WHITE_MAN &&
         position.piece_at(start + 10) == EMPTY &&
         position.piece_at(start + 11) == EMPTY {
        structure -= 100;
      }
    }
    for start in 35..39 {
      if position.piece_at(start) == WHITE_MAN &&
         position.piece_at(start + 1) == WHITE_MAN &&
         position.piece_at(start + 5) == WHITE_MAN &&
         position.piece_at(start + 10) == EMPTY &&
         position.piece_at(start + 11) == EMPTY {
        structure -= 100;
      }
    }

    let score = beans
      + structure
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
    let mut result = self.generator.legal_moves(position);
    if position.side_to_move() == White {
      result.sort_by(|mv1, mv2| {
        match (self.white_killer_moves.contains(&mv1), self.white_killer_moves.contains(&mv2)) {
          (false, true) => Greater,
          (true, false) => Less,
          _ => mv1.to().cmp(&mv2.to())
        }
      })
    }
    else {
      result.sort_by(|mv1, mv2| {
        match (self.black_killer_moves.contains(&mv1), self.black_killer_moves.contains(&mv2)) {
          (false, true) => Greater,
          (true, false) => Less,
          _ => mv2.to().cmp(&mv1.to())
        }
      })
    }
    result
  }

  fn quiet_move(&self, position: &Position, mv: &Move) -> bool {
    mv.num_taken() == 0 &&
    if position.side_to_move() == White {
      mv.to() >= 10 || position.piece_at(mv.from()) != WHITE_MAN
    }
    else {
      mv.to() <= 39 || position.piece_at(mv.from()) != BLACK_MAN
    }
  }

  fn display_name(&self) -> &str { "Slonënok" }
}
