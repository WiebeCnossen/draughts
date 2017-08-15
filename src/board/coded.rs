use std;

use board::piece::{Color, Piece};
use board::position::{Field, Position, Game};

#[cfg(test)]
use board::piece::{EMPTY, WHITE_MAN, BLACK_MAN};

#[derive(PartialEq)]
#[derive(Eq)]
#[derive(Hash)]
pub struct CodedPosition {
    upper: u64,
    lower: u64,
}

type Index = usize;
const PIECE_BITS: [Index; 6] = [0, 12, 24, 36, 48, 60];
const ROW_FIELDS: Index = 5;
const COL_POW: [u64; ROW_FIELDS + 1] = [1, 5, 25, 125, 625, 3125];
const SIDE_BIT: u64 = 1 << 63;
const BEFORE_MASK: [u64; ROW_FIELDS] = [
    (1 << PIECE_BITS[0]) - 1,
    (1 << PIECE_BITS[1]) - 1,
    (1 << PIECE_BITS[2]) - 1,
    (1 << PIECE_BITS[3]) - 1,
    (1 << PIECE_BITS[4]) - 1,
];
const AFTER_MASK: [u64; ROW_FIELDS] = [
    std::u64::MAX - (1 << PIECE_BITS[1]) + 1,
    std::u64::MAX - (1 << PIECE_BITS[2]) + 1,
    std::u64::MAX - (1 << PIECE_BITS[3]) + 1,
    std::u64::MAX - (1 << PIECE_BITS[4]) + 1,
    std::u64::MAX - (1 << PIECE_BITS[5]) + 1,
];

fn piece_at(bits: u64, field: Field) -> Piece {
    let row = (bits >> PIECE_BITS[field / ROW_FIELDS]) & BEFORE_MASK[1];
    (row / COL_POW[field % ROW_FIELDS] % ROW_FIELDS as u64) as Piece
}

fn put_piece(bits: u64, field: Field, piece: Piece) -> u64 {
    let row = field / ROW_FIELDS;
    let other = bits & (BEFORE_MASK[row] | AFTER_MASK[row]);
    let cr = bits >> PIECE_BITS[row] & BEFORE_MASK[1];
    let col = field % ROW_FIELDS;
    let cb = cr % COL_POW[col];
    let ca = cr / COL_POW[col + 1] * COL_POW[col + 1];
    let nr = cb + piece as u64 * COL_POW[col] + ca;
    other | (nr << PIECE_BITS[row])
}

impl Position for CodedPosition {
    fn side_to_move(&self) -> Color {
        if SIDE_BIT & self.lower != 0 {
            Color::White
        } else {
            Color::Black
        }
    }

    fn piece_at(&self, field: Field) -> Piece {
        if field < 25 {
            piece_at(self.upper, field)
        } else {
            piece_at(self.lower, field - 25)
        }
    }
}

impl Game for CodedPosition {
    fn create() -> CodedPosition {
        CodedPosition {
            upper: 0,
            lower: SIDE_BIT,
        }
    }

    fn toggle_side(&self) -> CodedPosition {
        CodedPosition {
            upper: self.upper,
            lower: SIDE_BIT ^ self.lower,
        }
    }

    fn put_piece(&self, field: Field, piece: Piece) -> CodedPosition {
        if field < 25 {
            CodedPosition {
                upper: put_piece(self.upper, field, piece),
                lower: self.lower,
            }
        } else {
            CodedPosition {
                upper: self.upper,
                lower: put_piece(self.lower, field - 25, piece),
            }
        }
    }
}

#[test]
fn create() {
    let empty = CodedPosition::create();
    assert_eq!(empty.side_to_move(), Color::White);
    assert_eq!(empty.piece_at(0), EMPTY);
    assert_eq!(empty.piece_at(19), EMPTY);
    assert_eq!(empty.piece_at(23), EMPTY);
    assert_eq!(empty.piece_at(30), EMPTY);
    assert_eq!(empty.piece_at(49), EMPTY);
}

#[test]
fn put_one_piece() {
    let position = CodedPosition::create().put_piece(31, WHITE_MAN);
    assert_eq!(position.side_to_move(), Color::White);
    assert_eq!(position.piece_at(25), EMPTY);
    assert_eq!(position.piece_at(30), EMPTY);
    assert_eq!(position.piece_at(31), WHITE_MAN);
    assert_eq!(position.piece_at(32), EMPTY);
    assert_eq!(position.piece_at(35), EMPTY);
}

#[test]
fn put_pieces_in_same_row() {
    let position = CodedPosition::create().put_piece(31, WHITE_MAN).put_piece(
        32,
        BLACK_MAN,
    );
    assert_eq!(position.side_to_move(), Color::White);
    assert_eq!(position.piece_at(25), EMPTY);
    assert_eq!(position.piece_at(30), EMPTY);
    assert_eq!(position.piece_at(31), WHITE_MAN);
    assert_eq!(position.piece_at(32), BLACK_MAN);
    assert_eq!(position.piece_at(33), EMPTY);
    assert_eq!(position.piece_at(35), EMPTY);
}

#[test]
fn put_pieces_in_distinct_rows() {
    let position = CodedPosition::create()
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
    let initial = CodedPosition::initial();
    assert_eq!(initial.side_to_move(), Color::White);
    assert_eq!(initial.piece_at(0), BLACK_MAN);
    assert_eq!(initial.piece_at(19), BLACK_MAN);
    assert_eq!(initial.piece_at(23), EMPTY);
    assert_eq!(initial.piece_at(30), WHITE_MAN);
    assert_eq!(initial.piece_at(49), WHITE_MAN);
}

#[test]
fn toggle_side() {
    let black = CodedPosition::create().toggle_side();
    assert_eq!(black.side_to_move(), Color::Black);
}
