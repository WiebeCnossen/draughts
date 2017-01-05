use std::hash::{Hash, Hasher};

use board::position::{Position, Game};
use board::piece::{EMPTY, WHITE_MAN, WHITE_KING, BLACK_MAN, BLACK_KING};

pub struct BitboardPosition {
  white_to_move: bool,
  pieces: [u64; 4]
}

impl PartialEq for BitboardPosition {
  fn eq(&self, other: &BitboardPosition) -> bool {
    self.white_to_move == other.white_to_move &&
    self.pieces == other.pieces
  }
}

impl Eq for BitboardPosition {}

impl Hash for BitboardPosition {
  fn hash<H: Hasher>(&self, state: &mut H) {
    self.white_to_move.hash(state);
    self.pieces.hash(state);
  }
}

const BITS : [u64; 50] = [
  1 << 0, 1 << 1, 1 << 2, 1 << 3, 1 << 4,
  1 << 5, 1 << 6, 1 << 7, 1 << 8, 1 << 9,
  1 << 10, 1 << 11, 1 << 12, 1 << 13, 1 << 14,
  1 << 15, 1 << 16, 1 << 17, 1 << 18, 1 << 19,
  1 << 20, 1 << 21, 1 << 22, 1 << 23, 1 << 24,
  1 << 25, 1 << 26, 1 << 27, 1 << 28, 1 << 29,
  1 << 30, 1 << 31, 1 << 32, 1 << 33, 1 << 34,
  1 << 35, 1 << 36, 1 << 37, 1 << 38, 1 << 39,
  1 << 40, 1 << 41, 1 << 42, 1 << 43, 1 << 44,
  1 << 45, 1 << 46, 1 << 47, 1 << 48, 1 << 49,
];

fn set(mask: u64, bit: usize) -> u64 {
  mask | BITS[bit]
}

fn clear(mask: u64, bit: usize) -> u64 {
  mask ^ (mask & BITS[bit])
}

impl Position for BitboardPosition {
  fn white_to_move(&self) -> bool {
    self.white_to_move
  }

  fn piece_at(&self, field: usize) -> u8 {
    for pos in 0..4 {
      if self.pieces[pos] & BITS[field] != 0 { return (pos + 1) as u8 }
    }

    EMPTY
  }
}

impl Game for BitboardPosition {
  fn create() -> BitboardPosition {
    BitboardPosition { white_to_move: true, pieces: [0; 4] }
  }

  fn toggle_side(&self) -> BitboardPosition {
    BitboardPosition { white_to_move: !self.white_to_move, pieces: self.pieces }
  }

  fn put_piece(&self, field: usize, piece: u8) -> BitboardPosition {
    BitboardPosition {
      white_to_move: self.white_to_move,
      pieces: [
        if piece == WHITE_MAN { set(self.pieces[0], field) } else { clear(self.pieces[0], field) },
        if piece == WHITE_KING { set(self.pieces[1], field) } else { clear(self.pieces[1], field) },
        if piece == BLACK_MAN { set(self.pieces[2], field) } else { clear(self.pieces[2], field) },
        if piece == BLACK_KING { set(self.pieces[3], field) } else { clear(self.pieces[3], field) },
      ]
    }
  }
}

#[test]
fn create() {
  let empty = BitboardPosition::create();
  assert!(empty.white_to_move());
  assert_eq!(empty.piece_at(0), EMPTY);
  assert_eq!(empty.piece_at(19), EMPTY);
  assert_eq!(empty.piece_at(23), EMPTY);
  assert_eq!(empty.piece_at(30), EMPTY);
  assert_eq!(empty.piece_at(49), EMPTY);
}

#[test]
fn put_one_piece() {
  let position = BitboardPosition::create()
    .put_piece(31, WHITE_MAN);
  assert!(position.white_to_move());
  assert_eq!(position.piece_at(25), EMPTY);
  assert_eq!(position.piece_at(30), EMPTY);
  assert_eq!(position.piece_at(31), WHITE_MAN);
  assert_eq!(position.piece_at(32), EMPTY);
  assert_eq!(position.piece_at(35), EMPTY);
}

#[test]
fn put_pieces_in_same_row() {
  let position = BitboardPosition::create()
    .put_piece(31, WHITE_MAN)
    .put_piece(32, BLACK_MAN);
  assert!(position.white_to_move());
  assert_eq!(position.piece_at(25), EMPTY);
  assert_eq!(position.piece_at(30), EMPTY);
  assert_eq!(position.piece_at(31), WHITE_MAN);
  assert_eq!(position.piece_at(32), BLACK_MAN);
  assert_eq!(position.piece_at(33), EMPTY);
  assert_eq!(position.piece_at(35), EMPTY);
}

#[test]
fn put_pieces_in_distinct_rows() {
  let position = BitboardPosition::create()
    .put_piece(6, WHITE_MAN)
    .put_piece(16, BLACK_MAN)
    .put_piece(11, BLACK_MAN);
  assert!(position.white_to_move());
  assert_eq!(position.piece_at(0), EMPTY);
  assert_eq!(position.piece_at(5), EMPTY);
  assert_eq!(position.piece_at(6), WHITE_MAN);
  assert_eq!(position.piece_at(7), EMPTY);
  assert_eq!(position.piece_at(10), EMPTY);
  assert_eq!(position.piece_at(11), BLACK_MAN);
  assert_eq!(position.piece_at(12), EMPTY);
  assert_eq!(position.piece_at(15), EMPTY);
  assert_eq!(position.piece_at(16), BLACK_MAN);
  assert_eq!(position.piece_at(17), EMPTY);
  assert_eq!(position.piece_at(20), EMPTY);
}

#[test]
fn initial() {
  let initial = BitboardPosition::initial();
  assert!(initial.white_to_move());
  assert_eq!(initial.piece_at(0), BLACK_MAN);
  assert_eq!(initial.piece_at(19), BLACK_MAN);
  assert_eq!(initial.piece_at(23), EMPTY);
  assert_eq!(initial.piece_at(30), WHITE_MAN);
  assert_eq!(initial.piece_at(49), WHITE_MAN);
}

#[test]
fn toggle_side() {
  let black = BitboardPosition::create().toggle_side();
  assert!(!black.white_to_move());
}
