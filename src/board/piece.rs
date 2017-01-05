pub const EMPTY : u8 = 0u8;
pub const WHITE_MAN : u8 = 1u8;
pub const WHITE_KING : u8 = 2u8;
pub const BLACK_MAN : u8 = 3u8;
pub const BLACK_KING : u8 = 4u8;

pub const TRANSPARENT : usize = 0;
pub const WHITE : usize = 1;
pub const BLACK : usize = 2;

pub fn color(piece : u8) -> usize {
  match piece {
    WHITE_MAN | WHITE_KING => WHITE,
    BLACK_MAN | BLACK_KING => BLACK,
    _ => TRANSPARENT
  }
}
