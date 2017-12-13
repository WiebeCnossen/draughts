pub type Piece = u8;
pub const EMPTY: Piece = 0;
pub const WHITE_MAN: Piece = 1;
pub const WHITE_KING: Piece = 2;
pub const BLACK_MAN: Piece = 3;
pub const BLACK_KING: Piece = 4;

#[derive(Debug, PartialEq, Eq)]
pub enum Color {
    White,
    Black,
}

pub fn color(piece: Piece) -> Option<Color> {
    match piece {
        WHITE_MAN | WHITE_KING => Some(Color::White),
        BLACK_MAN | BLACK_KING => Some(Color::Black),
        _ => None,
    }
}

pub fn piece_own(piece: Piece, c: &Color) -> Option<bool> {
    color(piece).map(|p| p == *c)
}

pub fn piece_is(piece: Piece, c: &Color) -> bool {
    piece_own(piece, c).unwrap_or_default()
}
