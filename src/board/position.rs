use super::mv::Move;
use super::piece::{Color, Piece, BLACK_KING, BLACK_MAN, EMPTY, WHITE_KING, WHITE_MAN};

pub type Field = usize;
pub fn promote(field: Field, piece: Piece) -> Piece {
    if piece == WHITE_MAN && field < 5 {
        WHITE_KING
    } else if piece == BLACK_MAN && field >= 45 {
        BLACK_KING
    } else {
        piece
    }
}

const FEN_CHARS: [char; 5] = ['e', 'w', 'W', 'b', 'B'];
const ASCII_CHARS: [char; 5] = ['.', 'w', 'W', 'b', 'B'];

#[derive(PartialEq, Eq, Hash, Clone, Copy)]
pub struct Position {
    empty: u64,
    white_man: u64,
    black_man: u64,
    white_king: u64,
}

type Index = usize;
const SIDE_BIT: u64 = 1 << 50;
const ALL_BITS: u64 = SIDE_BIT - 1;
const BITS: [u64; 50] = [
    1,
    1 << 1,
    1 << 2,
    1 << 3,
    1 << 4,
    1 << 5,
    1 << 6,
    1 << 7,
    1 << 8,
    1 << 9,
    1 << 10,
    1 << 11,
    1 << 12,
    1 << 13,
    1 << 14,
    1 << 15,
    1 << 16,
    1 << 17,
    1 << 18,
    1 << 19,
    1 << 20,
    1 << 21,
    1 << 22,
    1 << 23,
    1 << 24,
    1 << 25,
    1 << 26,
    1 << 27,
    1 << 28,
    1 << 29,
    1 << 30,
    1 << 31,
    1 << 32,
    1 << 33,
    1 << 34,
    1 << 35,
    1 << 36,
    1 << 37,
    1 << 38,
    1 << 39,
    1 << 40,
    1 << 41,
    1 << 42,
    1 << 43,
    1 << 44,
    1 << 45,
    1 << 46,
    1 << 47,
    1 << 48,
    1 << 49,
];

fn set(mask: u64, bit: Index) -> u64 {
    mask | BITS[bit]
}

fn clear(mask: u64, bit: Index) -> u64 {
    mask ^ (mask & BITS[bit])
}

impl Position {
    pub fn side_to_move(&self) -> Color {
        if self.empty & SIDE_BIT == 0 {
            Color::White
        } else {
            Color::Black
        }
    }

    pub fn piece_at(&self, field: Field) -> Piece {
        if self.empty & BITS[field] != 0 {
            EMPTY
        } else if self.white_man & BITS[field] != 0 {
            WHITE_MAN
        } else if self.black_man & BITS[field] != 0 {
            BLACK_MAN
        } else if self.white_king & BITS[field] != 0 {
            WHITE_KING
        } else {
            BLACK_KING
        }
    }

    pub fn create() -> Position {
        Position {
            empty: ALL_BITS,
            white_man: 0,
            black_man: 0,
            white_king: 0,
        }
    }

    pub fn toggle_side(&self) -> Position {
        Position {
            empty: self.empty ^ SIDE_BIT,
            white_man: self.white_man,
            black_man: self.black_man,
            white_king: self.white_king,
        }
    }

    pub fn put_piece(&self, field: Field, piece: Piece) -> Position {
        Position {
            empty: if piece == EMPTY {
                set(self.empty, field)
            } else {
                clear(self.empty, field)
            },
            white_man: if piece == WHITE_MAN {
                set(self.white_man, field)
            } else {
                clear(self.white_man, field)
            },
            black_man: if piece == BLACK_MAN {
                set(self.black_man, field)
            } else {
                clear(self.black_man, field)
            },
            white_king: if piece == WHITE_KING {
                set(self.white_king, field)
            } else {
                clear(self.white_king, field)
            },
        }
    }

    pub fn go(&self, mv: &Move) -> Self {
        let from = mv.from();
        let to = mv.to();
        let mut empty = self.empty ^ SIDE_BIT;
        empty = set(empty, from);
        empty = clear(empty, to);

        let mut white_man = self.white_man;
        let mut black_man = self.black_man;
        let mut white_king = self.white_king;
        for &taken in mv.taken() {
            match self.piece_at(taken) {
                WHITE_MAN => white_man = clear(white_man, taken),
                BLACK_MAN => black_man = clear(black_man, taken),
                WHITE_KING => white_king = clear(white_king, taken),
                _ => (),
            }
            empty = set(empty, taken);
        }

        let from_piece = self.piece_at(from);
        match from_piece {
            WHITE_MAN => white_man = clear(white_man, from),
            BLACK_MAN => black_man = clear(black_man, from),
            WHITE_KING => white_king = clear(white_king, from),
            _ => (),
        }
        match promote(to, from_piece) {
            WHITE_MAN => white_man = set(white_man, to),
            BLACK_MAN => black_man = set(black_man, to),
            WHITE_KING => white_king = set(white_king, to),
            _ => (),
        }

        Position {
            empty,
            white_man,
            black_man,
            white_king,
        }
    }

    pub fn fen(&self) -> String {
        let mut fen = String::from(if self.side_to_move() == Color::White {
            "w"
        } else {
            "b"
        });
        for c in (0..50).map(|i| FEN_CHARS[self.piece_at(i) as Field]) {
            fen.push(c);
        }
        fen
    }

    pub fn hfen(&self) -> String {
        let mut fen = String::from(if self.side_to_move() == Color::White {
            "W"
        } else {
            "B"
        });
        for c in (0..50).map(|i| FEN_CHARS[self.piece_at(i) as Field]) {
            fen.push(c);
        }
        fen
    }

    pub fn sfen(&self) -> String {
        let mut fen = String::from(if self.side_to_move() == Color::White {
            "w"
        } else {
            "b"
        });
        let mut num_empty = 0;
        fn flush(fen: &mut String, num_empty: Field) -> Field {
            match num_empty {
                0 => (),
                1 => fen.push('e'),
                n => fen.push_str(format!("{}", n).as_str()),
            }
            0
        };
        for i in 1..51 {
            match self.piece_at(i - 1) as Field {
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

    fn ascii_char(&self, field: usize) -> char {
        if (field + (field / 10)) % 2 == 0 {
            ' '
        } else {
            ASCII_CHARS[self.piece_at(field / 2) as usize]
        }
    }

    pub fn ascii(&self) -> String {
        let mut ascii = String::new();
        for field in 0..100 {
            let c = self.ascii_char(field);
            ascii.push(c);
            ascii.push(c);
            if (field == 9 && self.side_to_move() == Color::Black)
                || (field == 99 && self.side_to_move() == Color::White)
            {
                ascii.push_str("  *")
            }
            if field % 10 == 9 {
                ascii.push('\r');
                ascii.push('\n');
            } else {
                ascii.push(' ');
            }
        }
        ascii
    }

    pub fn initial() -> Self {
        let black = (0..20).fold(Self::create(), |pos, field| pos.put_piece(field, BLACK_MAN));
        (30..50).fold(black, |pos, field| pos.put_piece(field, WHITE_MAN))
    }

    pub fn parse(fen: &str) -> Result<Self, String> {
        if fen.len() < 11 {
            return Err("Invalid length".into());
        }
        let mut position = Self::create();
        let mut i = 0;
        let mut field = 0;

        for c in fen.chars() {
            if i == 0 {
                match c {
                    'w' => (),
                    'b' => position = position.toggle_side(),
                    _ => return Err("Invalid side to move".into()),
                }
            } else {
                let pieces = match c {
                    '|' | ' ' | '/' => Some((EMPTY, 0)),
                    'e' | '1' => Some((EMPTY, 1)),
                    '2' => Some((EMPTY, 2)),
                    '3' => Some((EMPTY, 3)),
                    '4' => Some((EMPTY, 4)),
                    '5' => Some((EMPTY, 5)),
                    'w' => Some((WHITE_MAN, 1)),
                    'h' => Some((WHITE_MAN, 2)),
                    'i' => Some((WHITE_MAN, 3)),
                    't' => Some((WHITE_MAN, 4)),
                    'r' => Some((WHITE_MAN, 5)),
                    'b' => Some((BLACK_MAN, 1)),
                    'l' => Some((BLACK_MAN, 2)),
                    'a' => Some((BLACK_MAN, 3)),
                    'c' => Some((BLACK_MAN, 4)),
                    'k' => Some((BLACK_MAN, 5)),
                    'W' => Some((WHITE_KING, 1)),
                    'H' => Some((WHITE_KING, 2)),
                    'I' => Some((WHITE_KING, 3)),
                    'T' => Some((WHITE_KING, 4)),
                    'E' => Some((WHITE_KING, 5)),
                    'B' => Some((BLACK_KING, 1)),
                    'L' => Some((BLACK_KING, 2)),
                    'A' => Some((BLACK_KING, 3)),
                    'C' => Some((BLACK_KING, 4)),
                    'K' => Some((BLACK_KING, 5)),
                    _ => None,
                };
                match pieces {
                    Some((piece, count)) => for _ in 0..count {
                        if field == 50 {
                            return Err(String::from("Too many fields"));
                        }
                        position = position.put_piece(field, piece);
                        field += 1;
                    },
                    None => return Err(format!("Invalid piece at {}", i)),
                }
            }
            i += 1;
        }
        if field != 50 {
            return Err(format!("Insufficient number of fields: {}", field));
        }
        Ok(position)
    }
}

use std::fmt;

impl fmt::Display for Position {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.sfen())
    }
}

#[test]
fn create() {
    let empty = Position::create();
    assert_eq!(empty.side_to_move(), Color::White);
    assert_eq!(empty.piece_at(0), EMPTY);
    assert_eq!(empty.piece_at(19), EMPTY);
    assert_eq!(empty.piece_at(23), EMPTY);
    assert_eq!(empty.piece_at(30), EMPTY);
    assert_eq!(empty.piece_at(49), EMPTY);
}

#[test]
fn put_one_piece() {
    let position = Position::create().put_piece(31, WHITE_MAN);
    assert_eq!(position.side_to_move(), Color::White);
    assert_eq!(position.piece_at(25), EMPTY);
    assert_eq!(position.piece_at(30), EMPTY);
    assert_eq!(position.piece_at(31), WHITE_MAN);
    assert_eq!(position.piece_at(32), EMPTY);
    assert_eq!(position.piece_at(35), EMPTY);
}

#[test]
fn put_pieces_in_same_row() {
    let position = Position::create()
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
    let position = Position::create()
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
    let initial = Position::initial();
    assert_eq!(initial.side_to_move(), Color::White);
    assert_eq!(initial.piece_at(0), BLACK_MAN);
    assert_eq!(initial.piece_at(19), BLACK_MAN);
    assert_eq!(initial.piece_at(23), EMPTY);
    assert_eq!(initial.piece_at(30), WHITE_MAN);
    assert_eq!(initial.piece_at(49), WHITE_MAN);
}

#[test]
fn toggle_side() {
    let black = Position::create().toggle_side();
    assert_eq!(black.side_to_move(), Color::Black);
}

#[test]
fn promotion() {
    let position = Position::create()
        .put_piece(1, BLACK_MAN)
        .put_piece(5, WHITE_MAN)
        .go(&Move::shift(5, 0));
    assert_eq!(position.side_to_move(), Color::Black);
    assert_eq!(position.piece_at(0), WHITE_KING);
    assert_eq!(position.piece_at(1), BLACK_MAN);
    assert_eq!(position.piece_at(5), EMPTY);
}

#[test]
fn from_fen() {
    let constructed = Position::create()
        .put_piece(1, BLACK_MAN)
        .put_piece(5, WHITE_MAN)
        .put_piece(11, BLACK_KING)
        .put_piece(15, WHITE_KING)
        .toggle_side();
    match Position::parse("bebeeeweeeeeBeeeWeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeee") {
        Err(msg) => {
            println!("{}", msg);
            assert!(false);
        }
        Ok(parsed) => assert!(constructed == parsed),
    }
}

#[test]
fn as_fen() {
    let constructed = Position::create()
        .put_piece(1, BLACK_MAN)
        .put_piece(5, WHITE_MAN)
        .put_piece(11, BLACK_KING)
        .put_piece(15, WHITE_KING)
        .toggle_side();
    assert_eq!(
        "bebeeeweeeeeBeeeWeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeee",
        constructed.fen()
    );
}

#[test]
fn from_sfen() {
    let constructed = Position::create()
        .put_piece(1, BLACK_MAN)
        .put_piece(5, WHITE_MAN)
        .put_piece(11, BLACK_KING)
        .put_piece(15, WHITE_KING)
        .toggle_side();
    match Position::parse("beb3w41B3W4555555") {
        Err(msg) => {
            println!("{}", msg);
            assert!(false);
        }
        Ok(parsed) => assert!(constructed == parsed),
    }
}

#[test]
fn from_ufen() {
    let constructed = Position::create()
        .put_piece(1, BLACK_MAN)
        .put_piece(5, WHITE_MAN)
        .put_piece(6, WHITE_MAN)
        .put_piece(7, WHITE_MAN)
        .put_piece(11, BLACK_KING)
        .put_piece(15, WHITE_KING)
        .toggle_side();
    match Position::parse("beb3i21B3W4555555") {
        Err(msg) => {
            println!("{}", msg);
            assert!(false);
        }
        Ok(parsed) => assert!(constructed == parsed),
    }
}

#[test]
fn goerres_bayar() {
    let spaced = match Position::parse("w ce/bea/k/a2/2b2/5/r/r/et/eie") {
        Err(msg) => {
            println!("{}", msg);
            assert!(false);
            None
        }
        Ok(parsed) => Some(parsed),
    };
    let small = match Position::parse("wcebeaka22b25rreteie") {
        Err(msg) => {
            println!("{}", msg);
            assert!(false);
            None
        }
        Ok(parsed) => Some(parsed),
    };
    assert!(spaced == small);
}

#[test]
fn as_sfen() {
    let constructed = Position::create()
        .put_piece(1, BLACK_MAN)
        .put_piece(5, WHITE_MAN)
        .put_piece(11, BLACK_KING)
        .put_piece(15, WHITE_KING)
        .toggle_side();
    assert_eq!("beb3w4eB3W4555555", constructed.sfen());
}

#[test]
fn as_ascii() {
    match Position::parse("w 5/4b/b4/5/2w2/bewwb/2w2/ewebe/3be/ew3") {
        Ok(position) => {
            let ascii = position.ascii();
            println!("\r\n{}\r\n", ascii);
            assert_eq!(ascii.len(), 313);
        }
        Err(msg) => {
            println!("{}", msg);
            assert!(false);
        }
    }
}
