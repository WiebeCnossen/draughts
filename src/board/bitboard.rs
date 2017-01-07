use board::position::{Position, Game};
use board::piece::{EMPTY, WHITE_MAN, WHITE_KING, BLACK_MAN, BLACK_KING, Color};

#[derive(PartialEq)]
#[derive(Eq)]
#[derive(Hash)]
pub struct BitboardPosition {
  empty : u64,
  white_man: u64,
  black_man: u64,
  white_king: u64,
}

const SIDE_BIT : u64 = 1 << 50;
const ALL_BITS : u64 = SIDE_BIT - 1;
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
  fn side_to_move(&self) -> Color {
    if self.empty & SIDE_BIT == 0 { Color::White } else { Color::Black }
  }

  fn piece_at(&self, field: usize) -> u8 {
    if self.empty & BITS[field] != 0 { EMPTY }
    else if self.white_man & BITS[field] != 0 { WHITE_MAN }
    else if self.black_man & BITS[field] != 0 { BLACK_MAN }
    else if self.white_king & BITS[field] != 0 { WHITE_KING }
    else { BLACK_KING }
  }
}

impl Game for BitboardPosition {
  fn create() -> BitboardPosition {
    BitboardPosition {
      empty: ALL_BITS,
      white_man: 0,
      black_man: 0,
      white_king: 0,
    }
  }

  fn toggle_side(&self) -> BitboardPosition {
    BitboardPosition {
      empty: self.empty ^ SIDE_BIT,
      white_man: self.white_man,
      black_man: self.black_man,
      white_king: self.white_king,
    }
  }

  fn put_piece(&self, field: usize, piece: u8) -> BitboardPosition {
    BitboardPosition {
      empty: if piece == EMPTY { set(self.empty, field) } else { clear(self.empty, field) },
      white_man: if piece == WHITE_MAN { set(self.white_man, field) } else { clear(self.white_man, field) },
      black_man: if piece == BLACK_MAN { set(self.black_man, field) } else { clear(self.black_man, field) },
      white_king: if piece == WHITE_KING { set(self.white_king, field) } else { clear(self.white_king, field) },
    }
  }
}

#[test]
fn create() {
  let empty = BitboardPosition::create();
  assert_eq!(empty.side_to_move(), Color::White);
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
  assert_eq!(position.side_to_move(), Color::White);
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
    .put_piece(32, BLACK_MAN)
    .put_piece(33, BLACK_KING);
  assert_eq!(position.side_to_move(), Color::White);
  assert_eq!(position.piece_at(25), EMPTY);
  assert_eq!(position.piece_at(30), EMPTY);
  assert_eq!(position.piece_at(31), WHITE_MAN);
  assert_eq!(position.piece_at(32), BLACK_MAN);
  assert_eq!(position.piece_at(33), BLACK_KING);
  assert_eq!(position.piece_at(35), EMPTY);
}

#[test]
fn put_pieces_in_distinct_rows() {
  let position = BitboardPosition::create()
    .put_piece(6, WHITE_MAN)
    .put_piece(16, BLACK_MAN)
    .put_piece(11, BLACK_MAN);
  assert_eq!(position.side_to_move(), Color::White);
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
  assert_eq!(initial.side_to_move(), Color::White);
  assert_eq!(initial.piece_at(0), BLACK_MAN);
  assert_eq!(initial.piece_at(19), BLACK_MAN);
  assert_eq!(initial.piece_at(23), EMPTY);
  assert_eq!(initial.piece_at(30), WHITE_MAN);
  assert_eq!(initial.piece_at(49), WHITE_MAN);
}

#[test]
fn toggle_side() {
  let black = BitboardPosition::create().toggle_side();
  assert_eq!(black.side_to_move(), Color::Black);
}
