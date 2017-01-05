mod array;
pub mod bitboard;
pub mod coded;
mod coords;
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

pub trait Take {
  fn num_taken(&self) -> usize;
}

impl Take for Move {
  fn num_taken(&self) -> usize {
    match self {
      &Move::Shift(..) => 0,
      &Move::Take1(..) => 1,
      &Move::Take2(..) => 2,
      &Move::Take3(..) => 3,
      &Move::Take4(..) => 4,
      &Move::Take5(..) => 5,
      &Move::Take6(..) => 6,
      &Move::Take7(..) => 7,
      &Move::Take8(..) => 8,
    }
  }
}
