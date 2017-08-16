use std::hash::{Hash, Hasher};

use board::position::{Field, Position, Game};
use board::piece::{EMPTY, Color, Piece};

#[cfg(test)]
use board::piece::{WHITE_MAN, BLACK_MAN};

pub struct ArrayPosition {
    white_to_move: bool,
    pieces: [Piece; 50],
}

impl PartialEq for ArrayPosition {
    fn eq(&self, other: &ArrayPosition) -> bool {
        self.white_to_move == other.white_to_move &&
            (0..50).fold(true, |a, i| a && self.pieces[i] == other.pieces[i])
    }
}

impl Eq for ArrayPosition {}

impl Hash for ArrayPosition {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.white_to_move.hash(state);
        for i in 0..50 {
            self.pieces[i].hash(state);
        }
    }
}

fn clone(pieces: [Piece; 50], field: Field, piece: Piece) -> [Piece; 50] {
    let read_or_set = |i| if field == i { piece } else { pieces[i] };
    [
        read_or_set(0),
        read_or_set(1),
        read_or_set(2),
        read_or_set(3),
        read_or_set(4),
        read_or_set(5),
        read_or_set(6),
        read_or_set(7),
        read_or_set(8),
        read_or_set(9),
        read_or_set(10),
        read_or_set(11),
        read_or_set(12),
        read_or_set(13),
        read_or_set(14),
        read_or_set(15),
        read_or_set(16),
        read_or_set(17),
        read_or_set(18),
        read_or_set(19),
        read_or_set(20),
        read_or_set(21),
        read_or_set(22),
        read_or_set(23),
        read_or_set(24),
        read_or_set(25),
        read_or_set(26),
        read_or_set(27),
        read_or_set(28),
        read_or_set(29),
        read_or_set(30),
        read_or_set(31),
        read_or_set(32),
        read_or_set(33),
        read_or_set(34),
        read_or_set(35),
        read_or_set(36),
        read_or_set(37),
        read_or_set(38),
        read_or_set(39),
        read_or_set(40),
        read_or_set(41),
        read_or_set(42),
        read_or_set(43),
        read_or_set(44),
        read_or_set(45),
        read_or_set(46),
        read_or_set(47),
        read_or_set(48),
        read_or_set(49),
    ]
}

impl Position for ArrayPosition {
    fn side_to_move(&self) -> Color {
        if self.white_to_move {
            Color::White
        } else {
            Color::Black
        }
    }

    fn piece_at(&self, field: Field) -> Piece {
        self.pieces[field]
    }
}

impl Game for ArrayPosition {
    fn create() -> ArrayPosition {
        ArrayPosition {
            white_to_move: true,
            pieces: [EMPTY; 50],
        }
    }

    fn toggle_side(&self) -> ArrayPosition {
        ArrayPosition {
            white_to_move: !self.white_to_move,
            pieces: self.pieces,
        }
    }

    fn put_piece(&self, field: Field, piece: Piece) -> ArrayPosition {
        ArrayPosition {
            white_to_move: self.white_to_move,
            pieces: clone(self.pieces, field, piece),
        }
    }
}

#[test]
fn create() {
    let empty = ArrayPosition::create();
    assert_eq!(empty.side_to_move(), Color::White);
    assert_eq!(empty.piece_at(0), EMPTY);
    assert_eq!(empty.piece_at(19), EMPTY);
    assert_eq!(empty.piece_at(23), EMPTY);
    assert_eq!(empty.piece_at(30), EMPTY);
    assert_eq!(empty.piece_at(49), EMPTY);
}

#[test]
fn put_one_piece() {
    let position = ArrayPosition::create().put_piece(31, WHITE_MAN);
    assert_eq!(position.side_to_move(), Color::White);
    assert_eq!(position.piece_at(25), EMPTY);
    assert_eq!(position.piece_at(30), EMPTY);
    assert_eq!(position.piece_at(31), WHITE_MAN);
    assert_eq!(position.piece_at(32), EMPTY);
    assert_eq!(position.piece_at(35), EMPTY);
}

#[test]
fn put_pieces_in_same_row() {
    let position = ArrayPosition::create().put_piece(31, WHITE_MAN).put_piece(
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
    let position = ArrayPosition::create()
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
    let initial = ArrayPosition::initial();
    assert_eq!(initial.side_to_move(), Color::White);
    assert_eq!(initial.piece_at(0), BLACK_MAN);
    assert_eq!(initial.piece_at(19), BLACK_MAN);
    assert_eq!(initial.piece_at(23), EMPTY);
    assert_eq!(initial.piece_at(30), WHITE_MAN);
    assert_eq!(initial.piece_at(49), WHITE_MAN);
}

#[test]
fn toggle_side() {
    let black = ArrayPosition::create().toggle_side();
    assert_eq!(black.side_to_move(), Color::Black);
}
