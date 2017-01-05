use std::hash::{Hash, Hasher};

use board::position::{Position, Game};
use board::piece::{EMPTY, WHITE_MAN, BLACK_MAN};

pub struct ArrayPosition { white_to_move: bool, pieces: [u8; 50] }

impl PartialEq for ArrayPosition {
  fn eq(&self, other: &ArrayPosition) -> bool {
    self.white_to_move == other.white_to_move &&
    (0..50).fold(true, |a,i| a && self.pieces[i] == other.pieces[i])
  }
}

impl Eq for ArrayPosition {}

impl Hash for ArrayPosition {
  fn hash<H: Hasher>(&self, state: &mut H) {
    self.white_to_move.hash(state);
    self.pieces.hash(state);
  }
}

fn clone(pieces: [u8; 50], field: usize, piece: u8) -> [u8; 50] {
  [
    if field == 0 { piece } else { pieces[0] },
    if field == 1 { piece } else { pieces[1] },
    if field == 2 { piece } else { pieces[2] },
    if field == 3 { piece } else { pieces[3] },
    if field == 4 { piece } else { pieces[4] },
    if field == 5 { piece } else { pieces[5] },
    if field == 6 { piece } else { pieces[6] },
    if field == 7 { piece } else { pieces[7] },
    if field == 8 { piece } else { pieces[8] },
    if field == 9 { piece } else { pieces[9] },
    if field == 10 { piece } else { pieces[10] },
    if field == 11 { piece } else { pieces[11] },
    if field == 12 { piece } else { pieces[12] },
    if field == 13 { piece } else { pieces[13] },
    if field == 14 { piece } else { pieces[14] },
    if field == 15 { piece } else { pieces[15] },
    if field == 16 { piece } else { pieces[16] },
    if field == 17 { piece } else { pieces[17] },
    if field == 18 { piece } else { pieces[18] },
    if field == 19 { piece } else { pieces[19] },
    if field == 20 { piece } else { pieces[20] },
    if field == 21 { piece } else { pieces[21] },
    if field == 22 { piece } else { pieces[22] },
    if field == 23 { piece } else { pieces[23] },
    if field == 24 { piece } else { pieces[24] },
    if field == 25 { piece } else { pieces[25] },
    if field == 26 { piece } else { pieces[26] },
    if field == 27 { piece } else { pieces[27] },
    if field == 28 { piece } else { pieces[28] },
    if field == 29 { piece } else { pieces[29] },
    if field == 30 { piece } else { pieces[30] },
    if field == 31 { piece } else { pieces[31] },
    if field == 32 { piece } else { pieces[32] },
    if field == 33 { piece } else { pieces[33] },
    if field == 34 { piece } else { pieces[34] },
    if field == 35 { piece } else { pieces[35] },
    if field == 36 { piece } else { pieces[36] },
    if field == 37 { piece } else { pieces[37] },
    if field == 38 { piece } else { pieces[38] },
    if field == 39 { piece } else { pieces[39] },
    if field == 40 { piece } else { pieces[40] },
    if field == 41 { piece } else { pieces[41] },
    if field == 42 { piece } else { pieces[42] },
    if field == 43 { piece } else { pieces[43] },
    if field == 44 { piece } else { pieces[44] },
    if field == 45 { piece } else { pieces[45] },
    if field == 46 { piece } else { pieces[46] },
    if field == 47 { piece } else { pieces[47] },
    if field == 48 { piece } else { pieces[48] },
    if field == 49 { piece } else { pieces[49] },
  ]
}

impl Position for ArrayPosition {
  fn white_to_move(&self) -> bool {
    self.white_to_move
  }

  fn piece_at(&self, field: usize) -> u8 {
    self.pieces[field]
  }
}

impl Game for ArrayPosition {
  fn create() -> ArrayPosition {
    ArrayPosition { white_to_move: true, pieces: [EMPTY; 50] }
  }

  fn toggle_side(&self) -> ArrayPosition {
    ArrayPosition { white_to_move: !self.white_to_move, pieces: self.pieces }
  }

  fn put_piece(&self, field: usize, piece: u8) -> ArrayPosition {
    ArrayPosition {
      white_to_move: self.white_to_move,
      pieces: clone(self.pieces, field, piece)
    }
  }
}

#[test]
fn create() {
  let empty = ArrayPosition::create();
  assert!(empty.white_to_move());
  assert_eq!(empty.piece_at(0), EMPTY);
  assert_eq!(empty.piece_at(19), EMPTY);
  assert_eq!(empty.piece_at(23), EMPTY);
  assert_eq!(empty.piece_at(30), EMPTY);
  assert_eq!(empty.piece_at(49), EMPTY);
}

#[test]
fn put_one_piece() {
  let position = ArrayPosition::create()
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
  let position = ArrayPosition::create()
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
  let position = ArrayPosition::create()
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
  let initial = ArrayPosition::initial();
  assert!(initial.white_to_move());
  assert_eq!(initial.piece_at(0), BLACK_MAN);
  assert_eq!(initial.piece_at(19), BLACK_MAN);
  assert_eq!(initial.piece_at(23), EMPTY);
  assert_eq!(initial.piece_at(30), WHITE_MAN);
  assert_eq!(initial.piece_at(49), WHITE_MAN);
}

#[test]
fn toggle_side() {
  let black = ArrayPosition::create().toggle_side();
  assert!(!black.white_to_move());
}
