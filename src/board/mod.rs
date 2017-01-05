mod array;
pub mod bitboard;
pub mod coded;
pub mod generator;
pub mod piece;
pub mod position;

pub enum Move {
  Shift(usize, usize),
  Take1(usize, usize, usize),
  Take2(usize, usize, usize, usize),
  Take3(usize, usize, usize, usize, usize),
  Take4(usize, usize, usize, usize, usize, usize),
  Take5(usize, usize, usize, usize, usize, usize, usize),
  Take6(usize, usize, usize, usize, usize, usize, usize, usize),
  Take7(usize, usize, usize, usize, usize, usize, usize, usize, usize),
  Take8(usize, usize, usize, usize, usize, usize, usize, usize, usize, usize)
}
