use std::hash::{Hash, Hasher};

use board::position::{Field, Game, Position};
use board::piece::{Color, Piece, EMPTY};

#[cfg(test)]
use board::piece::{BLACK_MAN, WHITE_MAN};

pub struct ArrayPosition {
    white_to_move: bool,
    pieces: [Piece; 50],
}

impl PartialEq for ArrayPosition {
    fn eq(&self, other: &ArrayPosition) -> bool {
        self.white_to_move == other.white_to_move
            && (0..50).fold(true, |a, i| a && self.pieces[i] == other.pieces[i])
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
    let mut clone = pieces;
    clone[field] = piece;
    clone
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
    let position = ArrayPosition::create()
        .put_piece(31, WHITE_MAN)
        .put_piece(32, BLACK_MAN);
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
