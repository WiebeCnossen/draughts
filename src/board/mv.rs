#[derive(Clone)]
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

impl Move {
  pub fn from(&self) -> usize {
    match self {
      &Move::Shift(from, ..)
      | &Move::Take1(from, ..)
      | &Move::Take2(from, ..)
      | &Move::Take3(from, ..)
      | &Move::Take4(from, ..)
      | &Move::Take5(from, ..)
      | &Move::Take6(from, ..)
      | &Move::Take7(from, ..)
      | &Move::Take8(from, ..) => from,
    }
  }

  pub fn to(&self) -> usize {
    match self {
      &Move::Shift(_, to)
      | &Move::Take1(_, to, ..)
      | &Move::Take2(_, to, ..)
      | &Move::Take3(_, to, ..)
      | &Move::Take4(_, to, ..)
      | &Move::Take5(_, to, ..)
      | &Move::Take6(_, to, ..)
      | &Move::Take7(_, to, ..)
      | &Move::Take8(_, to, ..) => to,
    }
  }

  pub fn num_taken(&self) -> usize {
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

  pub fn goes_via(&self, via: usize) -> bool {
    match self {
      &Move::Shift(..) =>
        false,
      &Move::Take1(_, _, via0) =>
        via0 == via,
      &Move::Take2(_, _, via0, via1) =>
        via0 == via || via1 == via,
      &Move::Take3(_, _, via0, via1, via2) =>
        via0 == via || via1 == via || via2 == via,
      &Move::Take4(_, _, via0, via1, via2, via3) =>
        via0 == via || via1 == via || via2 == via || via3 == via,
      &Move::Take5(_, _, via0, via1, via2, via3, via4) =>
        via0 == via || via1 == via || via2 == via || via3 == via || via4 == via,
      &Move::Take6(_, _, via0, via1, via2, via3, via4, via5) =>
        via0 == via || via1 == via || via2 == via || via3 == via || via4 == via || via5 == via,
      &Move::Take7(_, _, via0, via1, via2, via3, via4, via5, via6) =>
        via0 == via || via1 == via || via2 == via || via3 == via || via4 == via || via5 == via || via6 == via,
      &Move::Take8(_, _, via0, via1, via2, via3, via4, via5, via6, via7) =>
        via0 == via || via1 == via || via2 == via || via3 == via || via4 == via || via5 == via || via6 == via || via7 == via,
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
