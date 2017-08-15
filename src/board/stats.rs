use algorithm::judge::Eval;
use board::piece::{WHITE_MAN, BLACK_MAN};
use board::position::Position;

pub struct PositionStats {
    pub piece_count: [Eval; 5],
    pub voffset_white: [Eval; 10],
    pub voffset_black: [Eval; 10],
    pub hoffset_white: [Eval; 10],
    pub hoffset_black: [Eval; 10],
}

impl PositionStats {
    pub fn for_position(position: &Position) -> PositionStats {
        let mut piece_count = [0; 5];
        let mut voffset_white = [0; 10];
        let mut voffset_black = [0; 10];
        let mut hoffset_white = [0; 10];
        let mut hoffset_black = [0; 10];

        for field in 0..50 {
            let piece = position.piece_at(field);
            piece_count[piece as usize] += 1;
            match piece {
                WHITE_MAN => {
                    let x = 1 + 2 * (field % 5) - field / 5 % 2;
                    hoffset_white[x] += 1;
                    let y = 9 - field / 5;
                    voffset_white[y] += 1;
                }
                BLACK_MAN => {
                    let x = 8 - 2 * (field % 5) + field / 5 % 2;
                    hoffset_black[x] += 1;
                    let y = field / 5;
                    voffset_black[y] += 1;
                }
                _ => (),
            };
        }

        PositionStats {
            piece_count: piece_count,
            voffset_white: voffset_white,
            voffset_black: voffset_black,
            hoffset_white: hoffset_white,
            hoffset_black: hoffset_black,
        }
    }
}
