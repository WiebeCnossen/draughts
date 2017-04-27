#[derive(Clone)]
#[derive(Copy)]
#[derive(PartialEq)]
#[derive(Eq)]
#[derive(PartialOrd)]
#[derive(Ord)]
pub enum Move {
  Shift(usize, usize),
  Take1(usize, usize, usize),
  Take2(usize, usize, usize, usize),
  Take3(usize, usize, usize, usize, usize),
  Take4(usize, usize, usize, usize, usize, usize),
  Take5(usize, usize, usize, usize, usize, usize, usize),
  Take6(usize, usize, usize, usize, usize, usize, usize, usize),
  Take7(usize, usize, usize, usize, usize, usize, usize, usize, usize),
  Take8(usize, usize, usize, usize, usize, usize, usize, usize, usize, usize),
  Take9(usize, usize, usize, usize, usize, usize, usize, usize, usize, usize, usize),
  Take10(usize, usize, usize, usize, usize, usize, usize, usize, usize, usize, usize, usize),
  Take11(usize, usize, usize, usize, usize, usize, usize, usize, usize, usize, usize, usize, usize),
  Take12(usize, usize, usize, usize, usize, usize, usize, usize, usize, usize, usize, usize, usize, usize)
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
      | &Move::Take8(from, ..)
      | &Move::Take9(from, ..)
      | &Move::Take10(from, ..)
      | &Move::Take11(from, ..)
      | &Move::Take12(from, ..) => from,
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
      | &Move::Take8(_, to, ..)
      | &Move::Take9(_, to, ..)
      | &Move::Take10(_, to, ..)
      | &Move::Take11(_, to, ..)
      | &Move::Take12(_, to, ..) => to,
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
      &Move::Take9(..) => 9,
      &Move::Take10(..) => 10,
      &Move::Take11(..) => 11,
      &Move::Take12(..) => 12,
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
      &Move::Take9(_, _, via0, via1, via2, via3, via4, via5, via6, via7, via8) =>
        via0 == via || via1 == via || via2 == via || via3 == via || via4 == via || via5 == via || via6 == via || via7 == via || via8 == via,
      &Move::Take10(_, _, via0, via1, via2, via3, via4, via5, via6, via7, via8, via9) =>
        via0 == via || via1 == via || via2 == via || via3 == via || via4 == via || via5 == via || via6 == via || via7 == via || via8 == via || via9 == via,
      &Move::Take11(_, _, via0, via1, via2, via3, via4, via5, via6, via7, via8, via9, via10) =>
        via0 == via || via1 == via || via2 == via || via3 == via || via4 == via || via5 == via || via6 == via || via7 == via || via8 == via || via9 == via || via10 == via,
      &Move::Take12(_, _, via0, via1, via2, via3, via4, via5, via6, via7, via8, via9, via10, via11) =>
        via0 == via || via1 == via || via2 == via || via3 == via || via4 == via || via5 == via || via6 == via || via7 == via || via8 == via || via9 == via || via10 == via || via11 == via,
    }
  }

  pub fn as_string(&self) -> String {
    let c = if let &Move::Shift(..) = self { '-' } else { 'x' };
    format!("{}{}{}", self.from() + 1, c, self.to() + 1)
  }

  pub fn as_full_string(&self) -> String {
    match self {
      &Move::Shift(from, to) =>
        format!("{}-{}", from + 1, to + 1),
      &Move::Take1(from, to, via0) =>
        format!("{}x{}x{}", from + 1, to + 1, via0 + 1),
      &Move::Take2(from, to, via0, via1) =>
        format!("{}x{}x{}x{}", from + 1, to + 1, via0 + 1, via1 + 1),
      &Move::Take3(from, to, via0, via1, via2) =>
        format!("{}x{}x{}x{}x{}", from + 1, to + 1, via0 + 1, via1 + 1, via2 + 1),
      &Move::Take4(from, to, via0, via1, via2, via3) =>
        format!("{}x{}x{}x{}x{}x{}", from + 1, to + 1, via0 + 1, via1 + 1, via2 + 1, via3 + 1),
      &Move::Take5(from, to, via0, via1, via2, via3, via4) =>
        format!("{}x{}x{}x{}x{}x{}x{}", from + 1, to + 1, via0 + 1, via1 + 1, via2 + 1, via3 + 1, via4 + 1),
      &Move::Take6(from, to, via0, via1, via2, via3, via4, via5) =>
        format!("{}x{}x{}x{}x{}x{}x{}x{}", from + 1, to + 1, via0 + 1, via1 + 1, via2 + 1, via3 + 1, via4 + 1, via5 + 1),
      &Move::Take7(from, to, via0, via1, via2, via3, via4, via5, via6) =>
        format!("{}x{}x{}x{}x{}x{}x{}x{}x{}", from + 1, to + 1, via0 + 1, via1 + 1, via2 + 1, via3 + 1, via4 + 1, via5 + 1, via6 + 1),
      &Move::Take8(from, to, via0, via1, via2, via3, via4, via5, via6, via7) =>
        format!("{}x{}x{}x{}x{}x{}x{}x{}x{}x{}", from + 1, to + 1, via0 + 1, via1 + 1, via2 + 1, via3 + 1, via4 + 1, via5 + 1, via6 + 1, via7 + 1),
      &Move::Take9(from, to, via0, via1, via2, via3, via4, via5, via6, via7, via8) =>
        format!("{}x{}x{}x{}x{}x{}x{}x{}x{}x{}x{}", from + 1, to + 1, via0 + 1, via1 + 1, via2 + 1, via3 + 1, via4 + 1, via5 + 1, via6 + 1, via7 + 1, via8 + 1),
      &Move::Take10(from, to, via0, via1, via2, via3, via4, via5, via6, via7, via8, via9) =>
        format!("{}x{}x{}x{}x{}x{}x{}x{}x{}x{}x{}x{}", from + 1, to + 1, via0 + 1, via1 + 1, via2 + 1, via3 + 1, via4 + 1, via5 + 1, via6 + 1, via7 + 1, via8 + 1, via9 + 1),
      &Move::Take11(from, to, via0, via1, via2, via3, via4, via5, via6, via7, via8, via9, via10) =>
        format!("{}x{}x{}x{}x{}x{}x{}x{}x{}x{}x{}x{}x{}", from + 1, to + 1, via0 + 1, via1 + 1, via2 + 1, via3 + 1, via4 + 1, via5 + 1, via6 + 1, via7 + 1, via8 + 1, via9 + 1, via10 + 1),
      &Move::Take12(from, to, via0, via1, via2, via3, via4, via5, via6, via7, via8, via9, via10, via11) =>
        format!("{}x{}x{}x{}x{}x{}x{}x{}x{}x{}x{}x{}x{}x{}", from + 1, to + 1, via0 + 1, via1 + 1, via2 + 1, via3 + 1, via4 + 1, via5 + 1, via6 + 1, via7 + 1, via8 + 1, via9 + 1, via10 + 1, via11 + 1),
    }
  }
}

use std::fmt;

impl fmt::Display for Move {
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    write!(f, "{}", self.as_full_string())
  }
}
