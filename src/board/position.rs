use std::hash::Hash;

use board::piece::{EMPTY, WHITE_MAN, WHITE_KING, BLACK_MAN, BLACK_KING, Color};

#[cfg(test)]
use board::bitboard::BitboardPosition;

fn promote(field: usize, piece: u8) -> u8 {
  if piece == WHITE_MAN && field < 5 { WHITE_KING }
  else if piece == BLACK_MAN && field >= 45 { BLACK_KING }
  else { piece }
}

const FEN_CHARS : [char; 5] = ['e', 'w', 'W', 'b', 'B'];
const ASCII_CHARS : [char; 5] = ['.', 'w', 'W', 'b', 'B'];

pub trait Position {
  fn side_to_move(&self) -> Color;
  fn piece_at(&self, field: usize) -> u8;

  fn fen(&self) -> String {
    let mut fen = String::from(if self.side_to_move() == Color::White { "w" } else { "b" });
    for i in 1..51 {
      fen.push(FEN_CHARS[self.piece_at(i - 1) as usize]);
    }
    fen
  }

  fn sfen(&self) -> String {
    let mut fen = String::from(if self.side_to_move() == Color::White { "w" } else { "b" });
    let mut num_empty = 0;
    fn flush(fen: &mut String, num_empty: usize) -> usize {
      match num_empty {
        0 => (),
        1 => fen.push('e'),
        n => fen.push_str(format!("{}", n).as_str())
      }
      0
    };
    for i in 1..51 {
      match self.piece_at(i - 1) as usize {
        0 => num_empty += 1,
        n => {
          num_empty = flush(&mut fen, num_empty);
          fen.push(FEN_CHARS[n]);
        }
      }
      if i % 5 == 0 {
        num_empty = flush(&mut fen, num_empty);
      }
    }
    fen
  }

  fn ascii(&self) -> String {
    let mut ascii = String::new();
    for field in 0..100 {
      let c = if (field + (field / 10)) % 2 == 0 { ' ' } else { ASCII_CHARS[self.piece_at(field / 2) as usize] };
      ascii.push(c); ascii.push(c);
      if field % 10 == 9 { ascii.push('\r'); ascii.push('\n'); } else { ascii.push(' '); }
    }
    ascii
  }
}

use std::fmt;

impl fmt::Display for Position {
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    write!(f, "{}", self.sfen())
  }
}

pub trait Game : Position + Hash + Sized {
  fn create() -> Self;
  fn toggle_side(&self) -> Self;
  fn put_piece(&self, field: usize, piece: u8) -> Self;

  fn initial() -> Self {
    let black = (0..20).fold(
      Self::create(),
      |pos, field| pos.put_piece(field, BLACK_MAN));
    (30..50).fold(
      black,
      |pos, field| pos.put_piece(field, WHITE_MAN))
  }

  fn parse<'a>(fen: &'a str) -> Result<Self, String> {
    if fen.len() < 11 {
      return Err("Invalid length".into())
    }
    let mut position = Self::create();
    let mut i = 0;
    let mut field = 0;

    for c in fen.chars() {
      if i == 0 {
        match c {
          'w' => (),
          'b' => position = position.toggle_side(),
          _ => return Err("Invalid side to move".into())
        }
      }
      else {
        let pieces = match c {
          'e' | '1' => Some((EMPTY, 1)),
          '2' => Some((EMPTY, 2)),
          '3' => Some((EMPTY, 3)),
          '4' => Some((EMPTY, 4)),
          '5' => Some((EMPTY, 5)),
          'w' => Some((WHITE_MAN, 1)),
          'b' => Some((BLACK_MAN, 1)),
          'W' => Some((WHITE_KING, 1)),
          'B' => Some((BLACK_KING, 1)),
          _ => None
        };
        match pieces {
          Some((piece, count)) =>
            for _ in 0..count {
              position = position.put_piece(field, piece);
              field += 1;
            },
          None => return Err(format!("Invalid piece at {}", i))
        }
      }
      i += 1;
    }
    if field != 50 {
      return Err("Invalid number of fields".into())
    }
    Ok(position)
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

#[test]
fn promotion() {
  let position = BitboardPosition::create()
    .put_piece(1, BLACK_MAN)
    .put_piece(5, WHITE_MAN)
    .go(5, 0);
  assert_eq!(position.side_to_move(), Color::Black);
  assert_eq!(position.piece_at(0), WHITE_KING);
  assert_eq!(position.piece_at(1), BLACK_MAN);
  assert_eq!(position.piece_at(5), EMPTY);
}

#[test]
fn from_fen() {
  let constructed = BitboardPosition::create()
    .put_piece(1, BLACK_MAN)
    .put_piece(5, WHITE_MAN)
    .put_piece(11, BLACK_KING)
    .put_piece(15, WHITE_KING)
    .toggle_side();
  match BitboardPosition::parse("bebeeeweeeeeBeeeWeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeee") {
    Err(msg) => {
      println!("{}", msg);
      assert!(false);
    },
    Ok(parsed) => assert!(constructed == parsed)
  }
}

#[test]
fn as_fen() {
  let constructed = BitboardPosition::create()
    .put_piece(1, BLACK_MAN)
    .put_piece(5, WHITE_MAN)
    .put_piece(11, BLACK_KING)
    .put_piece(15, WHITE_KING)
    .toggle_side();
  assert_eq!("bebeeeweeeeeBeeeWeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeee", constructed.fen());
}

#[test]
fn from_sfen() {
  let constructed = BitboardPosition::create()
    .put_piece(1, BLACK_MAN)
    .put_piece(5, WHITE_MAN)
    .put_piece(11, BLACK_KING)
    .put_piece(15, WHITE_KING)
    .toggle_side();
  match BitboardPosition::parse("beb3w41B3W4555555") {
    Err(msg) => {
      println!("{}", msg);
      assert!(false);
    },
    Ok(parsed) => assert!(constructed == parsed)
  }
}

#[test]
fn as_sfen() {
  let constructed = BitboardPosition::create()
    .put_piece(1, BLACK_MAN)
    .put_piece(5, WHITE_MAN)
    .put_piece(11, BLACK_KING)
    .put_piece(15, WHITE_KING)
    .toggle_side();
  assert_eq!("beb3w4eB3W4555555", constructed.sfen());
}

#[test]
fn as_ascii() {
  match BitboardPosition::parse("w54bb452w2bewwb2w2ewebe3beew3") {
    Ok(position) => {
      let ascii = position.ascii();
      println!("\r\n{}\r\n", ascii);
      assert_eq!(ascii.len(), 310);
    },
    Err(msg) => {
      println!("{}", msg);
      assert!(false);
    }
  }
}
