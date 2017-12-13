use std::hash::Hash;

use board::mv::Move;
use board::piece::{Color, Piece, BLACK_KING, BLACK_MAN, EMPTY, WHITE_KING, WHITE_MAN};

#[cfg(test)]
use board::bitboard::BitboardPosition;

pub type Field = usize;
fn promote(field: Field, piece: Piece) -> Piece {
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

pub trait Position {
    fn side_to_move(&self) -> Color;
    fn piece_at(&self, field: Field) -> Piece;

    fn fen(&self) -> String {
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

    fn sfen(&self) -> String {
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

    fn ascii(&self) -> String {
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
}

use std::fmt;

impl fmt::Display for Position {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.sfen())
    }
}

pub trait Game: Position + Hash + Sized {
    fn create() -> Self;
    fn toggle_side(&self) -> Self;
    fn put_piece(&self, field: Field, piece: Piece) -> Self;

    fn initial() -> Self {
        let black = (0..20).fold(Self::create(), |pos, field| pos.put_piece(field, BLACK_MAN));
        (30..50).fold(black, |pos, field| pos.put_piece(field, WHITE_MAN))
    }

    fn clone(source: &Position) -> Self {
        (0..50).fold(
            if source.side_to_move() == Color::White {
                Self::create()
            } else {
                Self::create().toggle_side()
            },
            |pos, field| pos.put_piece(field, source.piece_at(field)),
        )
    }

    fn parse(fen: &str) -> Result<Self, String> {
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

    fn go(&self, mv: &Move) -> Self {
        match *mv {
            Move::Shift(from, to) => self.put_piece(from, EMPTY)
                .put_piece(to, promote(to, self.piece_at(from)))
                .toggle_side(),
            Move::Take1(from, to, via0) => self.put_piece(from, EMPTY)
                .put_piece(to, promote(to, self.piece_at(from)))
                .put_piece(via0, EMPTY)
                .toggle_side(),
            Move::Take2(from, to, via0, via1) => self.put_piece(from, EMPTY)
                .put_piece(to, promote(to, self.piece_at(from)))
                .put_piece(via0, EMPTY)
                .put_piece(via1, EMPTY)
                .toggle_side(),
            Move::Take3(from, to, via0, via1, via2) => self.put_piece(from, EMPTY)
                .put_piece(to, promote(to, self.piece_at(from)))
                .put_piece(via0, EMPTY)
                .put_piece(via1, EMPTY)
                .put_piece(via2, EMPTY)
                .toggle_side(),
            Move::Take4(from, to, via0, via1, via2, via3) => self.put_piece(from, EMPTY)
                .put_piece(to, promote(to, self.piece_at(from)))
                .put_piece(via0, EMPTY)
                .put_piece(via1, EMPTY)
                .put_piece(via2, EMPTY)
                .put_piece(via3, EMPTY)
                .toggle_side(),
            Move::Take5(from, to, via0, via1, via2, via3, via4) => self.put_piece(from, EMPTY)
                .put_piece(to, promote(to, self.piece_at(from)))
                .put_piece(via0, EMPTY)
                .put_piece(via1, EMPTY)
                .put_piece(via2, EMPTY)
                .put_piece(via3, EMPTY)
                .put_piece(via4, EMPTY)
                .toggle_side(),
            Move::Take6(from, to, via0, via1, via2, via3, via4, via5) => {
                self.put_piece(from, EMPTY)
                    .put_piece(to, promote(to, self.piece_at(from)))
                    .put_piece(via0, EMPTY)
                    .put_piece(via1, EMPTY)
                    .put_piece(via2, EMPTY)
                    .put_piece(via3, EMPTY)
                    .put_piece(via4, EMPTY)
                    .put_piece(via5, EMPTY)
                    .toggle_side()
            }
            Move::Take7(from, to, via0, via1, via2, via3, via4, via5, via6) => {
                self.put_piece(from, EMPTY)
                    .put_piece(to, promote(to, self.piece_at(from)))
                    .put_piece(via0, EMPTY)
                    .put_piece(via1, EMPTY)
                    .put_piece(via2, EMPTY)
                    .put_piece(via3, EMPTY)
                    .put_piece(via4, EMPTY)
                    .put_piece(via5, EMPTY)
                    .put_piece(via6, EMPTY)
                    .toggle_side()
            }
            Move::Take8(from, to, via0, via1, via2, via3, via4, via5, via6, via7) => {
                self.put_piece(from, EMPTY)
                    .put_piece(to, promote(to, self.piece_at(from)))
                    .put_piece(via0, EMPTY)
                    .put_piece(via1, EMPTY)
                    .put_piece(via2, EMPTY)
                    .put_piece(via3, EMPTY)
                    .put_piece(via4, EMPTY)
                    .put_piece(via5, EMPTY)
                    .put_piece(via6, EMPTY)
                    .put_piece(via7, EMPTY)
                    .toggle_side()
            }
            Move::Take9(from, to, via0, via1, via2, via3, via4, via5, via6, via7, via8) => {
                self.put_piece(from, EMPTY)
                    .put_piece(to, promote(to, self.piece_at(from)))
                    .put_piece(via0, EMPTY)
                    .put_piece(via1, EMPTY)
                    .put_piece(via2, EMPTY)
                    .put_piece(via3, EMPTY)
                    .put_piece(via4, EMPTY)
                    .put_piece(via5, EMPTY)
                    .put_piece(via6, EMPTY)
                    .put_piece(via7, EMPTY)
                    .put_piece(via8, EMPTY)
                    .toggle_side()
            }
            Move::Take10(from, to, via0, via1, via2, via3, via4, via5, via6, via7, via8, via9) => {
                self.put_piece(from, EMPTY)
                    .put_piece(to, promote(to, self.piece_at(from)))
                    .put_piece(via0, EMPTY)
                    .put_piece(via1, EMPTY)
                    .put_piece(via2, EMPTY)
                    .put_piece(via3, EMPTY)
                    .put_piece(via4, EMPTY)
                    .put_piece(via5, EMPTY)
                    .put_piece(via6, EMPTY)
                    .put_piece(via7, EMPTY)
                    .put_piece(via8, EMPTY)
                    .put_piece(via9, EMPTY)
                    .toggle_side()
            }
            Move::Take11(
                from,
                to,
                via0,
                via1,
                via2,
                via3,
                via4,
                via5,
                via6,
                via7,
                via8,
                via9,
                via10,
            ) => self.put_piece(from, EMPTY)
                .put_piece(to, promote(to, self.piece_at(from)))
                .put_piece(via0, EMPTY)
                .put_piece(via1, EMPTY)
                .put_piece(via2, EMPTY)
                .put_piece(via3, EMPTY)
                .put_piece(via4, EMPTY)
                .put_piece(via5, EMPTY)
                .put_piece(via6, EMPTY)
                .put_piece(via7, EMPTY)
                .put_piece(via8, EMPTY)
                .put_piece(via9, EMPTY)
                .put_piece(via10, EMPTY)
                .toggle_side(),
            Move::Take12(
                from,
                to,
                via0,
                via1,
                via2,
                via3,
                via4,
                via5,
                via6,
                via7,
                via8,
                via9,
                via10,
                via11,
            ) => self.put_piece(from, EMPTY)
                .put_piece(to, promote(to, self.piece_at(from)))
                .put_piece(via0, EMPTY)
                .put_piece(via1, EMPTY)
                .put_piece(via2, EMPTY)
                .put_piece(via3, EMPTY)
                .put_piece(via4, EMPTY)
                .put_piece(via5, EMPTY)
                .put_piece(via6, EMPTY)
                .put_piece(via7, EMPTY)
                .put_piece(via8, EMPTY)
                .put_piece(via9, EMPTY)
                .put_piece(via10, EMPTY)
                .put_piece(via11, EMPTY)
                .toggle_side(),
        }
    }
}

#[test]
fn promotion() {
    let position = BitboardPosition::create()
        .put_piece(1, BLACK_MAN)
        .put_piece(5, WHITE_MAN)
        .go(&Move::Shift(5, 0));
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
        }
        Ok(parsed) => assert!(constructed == parsed),
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
    assert_eq!(
        "bebeeeweeeeeBeeeWeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeee",
        constructed.fen()
    );
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
        }
        Ok(parsed) => assert!(constructed == parsed),
    }
}

#[test]
fn from_ufen() {
    let constructed = BitboardPosition::create()
        .put_piece(1, BLACK_MAN)
        .put_piece(5, WHITE_MAN)
        .put_piece(6, WHITE_MAN)
        .put_piece(7, WHITE_MAN)
        .put_piece(11, BLACK_KING)
        .put_piece(15, WHITE_KING)
        .toggle_side();
    match BitboardPosition::parse("beb3i21B3W4555555") {
        Err(msg) => {
            println!("{}", msg);
            assert!(false);
        }
        Ok(parsed) => assert!(constructed == parsed),
    }
}

#[test]
fn goerres_bayar() {
    let spaced = match BitboardPosition::parse("w ce/bea/k/a2/2b2/5/r/r/et/eie") {
        Err(msg) => {
            println!("{}", msg);
            assert!(false);
            None
        }
        Ok(parsed) => Some(parsed),
    };
    let small = match BitboardPosition::parse("wcebeaka22b25rreteie") {
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
    match BitboardPosition::parse("w 5/4b/b4/5/2w2/bewwb/2w2/ewebe/3be/ew3") {
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
