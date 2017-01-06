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

use std::fmt;

impl fmt::Display for Move {
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    match self {
      &Move::Shift(from, to) => write!(f, "{}-{}", from , to),
      &Move::Take1(from, to, via0) => write!(f, "{}x{}x{}", from , to, via0),
      &Move::Take2(from, to, ..)
      | &Move::Take3(from, to, ..)
      | &Move::Take4(from, to, ..)
      | &Move::Take5(from, to, ..)
      | &Move::Take6(from, to, ..)
      | &Move::Take7(from, to, ..)
      | &Move::Take8(from, to, ..)
       => write!(f, "{}xx{}", from , to),
    }
  }
}
