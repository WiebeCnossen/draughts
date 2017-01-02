use std::hash::{Hash, Hasher};

pub trait Position : Eq + Hash {
  fn white_to_move(&self) -> bool;
  fn piece_at(&self, field: usize) -> u64;
  fn go(&self, from: u64, to: u64) -> Self;
  fn take(&self, from: u64, to: u64, via: &[u64]) -> Self;
}

pub struct CodedPosition { upper: u64, lower: u64 }

impl PartialEq for CodedPosition {
  fn eq(&self, other: &CodedPosition) -> bool {
    self.upper == other.upper && self.lower == other.lower
  }
}

impl Eq for CodedPosition {}

impl Hash for CodedPosition {
  fn hash<H: Hasher>(&self, state: &mut H) {
    self.upper.hash(state);
    self.lower.hash(state);
  }
}

const piece_bits : u64 = 12;
const row_fields : u64 = 5;
const side_bit : u64 = 1 << 63;
const one_row : u64 = (1 << piece_bits) - 1;
static field_shift : [u64; 25] = [
  0, 0, 0, 0, 0,
  12, 12, 12, 12, 12,
  24, 24, 24, 24, 24,
  36, 36, 36, 36, 36,
  48, 48, 48, 48, 48
];
static piece_div : [u64; 25] = [
  1, 1, 1, 1, 1,
  5, 5, 5, 5, 5,
  25, 25, 25, 25, 25,
  125, 125, 125, 125, 125,
  625, 625, 625, 625, 625
];

fn piece_at(bits: u64, field: usize) -> u64 {
  let row = (bits >> field_shift[field]) & one_row;
  row / piece_div[field] % 5
}

impl Position for CodedPosition {
  fn white_to_move(&self) -> bool {
    (side_bit & self.lower) != 0
  }

  fn piece_at(&self, field: usize) -> u64 {
    if field < 25 {
      piece_at(self.upper, field)
    }
    else {
      piece_at(self.lower, field - 25)
    }
  }

  fn go(&self, from: u64, to: u64) -> CodedPosition {
    panic!("Not yet implemented");
  }

  fn take(&self, from: u64, to: u64, via: &[u64]) -> CodedPosition {
    panic!("Not yet implemented");
  }
}
