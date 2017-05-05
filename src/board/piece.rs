pub const EMPTY: u8 = 0u8;
pub const WHITE_MAN: u8 = 1u8;
pub const WHITE_KING: u8 = 2u8;
pub const BLACK_MAN: u8 = 3u8;
pub const BLACK_KING: u8 = 4u8;

#[derive(Debug)]
#[derive(PartialEq)]
#[derive(Eq)]
#[derive(Clone)]
pub enum Color {
    White,
    Black,
}

pub fn color(piece: u8) -> Option<Color> {
    match piece {
        WHITE_MAN | WHITE_KING => Some(Color::White),
        BLACK_MAN | BLACK_KING => Some(Color::Black),
        _ => None,
    }
}

pub fn piece_own(piece: u8, c: Color) -> Option<bool> {
    color(piece).map(|p| p == c)
}

pub fn piece_is(piece: u8, c: Color) -> bool {
    piece_own(piece, c).unwrap_or_default()
}
