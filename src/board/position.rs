use std::hash::Hash;

use board::piece::{EMPTY, WHITE_MAN, WHITE_KING, BLACK_MAN, BLACK_KING};

fn promote(field: usize, piece: u8) -> u8 {
  if piece == WHITE_MAN && field < 5 { WHITE_KING }
  else if piece == BLACK_MAN && field >= 45 { BLACK_KING }
  else { piece }
}

pub trait Position : Eq + Hash + Sized {
  fn create() -> Self;
  fn white_to_move(&self) -> bool;
  fn toggle_side(&self) -> Self;
  fn piece_at(&self, field: usize) -> u8;
  fn put_piece(&self, field: usize, piece: u8) -> Self;

  fn initial() -> Self {
    let black = (0..20).fold(
      Self::create(),
      |pos, field| pos.put_piece(field, BLACK_MAN));
    (30..50).fold(
      black,
      |pos, field| pos.put_piece(field, WHITE_MAN))
  }

  fn go(&self, from: usize, to: usize) -> Self {
    self.put_piece(from, EMPTY)
        .put_piece(to, promote(to, self.piece_at(from)))
        .toggle_side()
  }

  fn take(&self, from: usize, to: usize, via: &[usize]) -> Self {
    via.into_iter().fold(
      self.go(from, to),
      |pos, &field| pos.put_piece(field, EMPTY))
  }
}
