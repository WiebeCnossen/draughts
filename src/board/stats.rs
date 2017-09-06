use std::cmp::{max, min};

use algorithm::judge::Eval;
use board::piece::{WHITE_MAN, BLACK_MAN};
use board::position::Position;

pub struct PositionStats {
    pub piece_count: [Eval; 5],
    pub voffset_white: [Eval; 10],
    pub voffset_black: [Eval; 10],
    pub hoffset_white: [Eval; 10],
    pub hoffset_black: [Eval; 10],
    pub height_white: usize,
    pub height_black: usize,
}

impl PositionStats {
    pub fn for_position(position: &Position) -> PositionStats {
        let mut piece_count = [0; 5];
        let mut voffset_white = [0; 10];
        let mut voffset_black = [0; 10];
        let mut hoffset_white = [0; 10];
        let mut hoffset_black = [0; 10];
        let mut vmin_white = 9;
        let mut vmax_white = 0;
        let mut vmin_black = 9;
        let mut vmax_black = 0;

        for field in 0..50 {
            let piece = position.piece_at(field);
            piece_count[piece as usize] += 1;
            match piece {
                WHITE_MAN => {
                    let x = 1 + 2 * (field % 5) - field / 5 % 2;
                    hoffset_white[x] += 1;
                    let y = 9 - field / 5;
                    voffset_white[y] += 1;
                    vmin_white = min(vmin_white, y);
                    vmax_white = max(vmax_white, y);
                }
                BLACK_MAN => {
                    let x = 8 - 2 * (field % 5) + field / 5 % 2;
                    hoffset_black[x] += 1;
                    let y = field / 5;
                    voffset_black[y] += 1;
                    vmin_black = min(vmin_black, y);
                    vmax_black = max(vmax_black, y);
                }
                _ => (),
            };
        }

        PositionStats {
            piece_count,
            voffset_white,
            voffset_black,
            hoffset_white,
            hoffset_black,
            height_white: if vmax_white > vmin_white {
                vmax_white - vmin_white
            } else {
                0
            },
            height_black: if vmax_black > vmin_black {
                vmax_black - vmin_black
            } else {
                0
            },
        }
    }
}
