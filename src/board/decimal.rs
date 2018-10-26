use super::piece::{Color, BLACK_KING, BLACK_MAN, EMPTY, WHITE_KING, WHITE_MAN};
use super::position::{Field, Position};

pub type DecimalData = [u8; 16];

const TOP_PLACES: [u16; 5] = [0, 0, 1, 2, 3];
const CENTER_PLACES: [u16; 5] = [0, 1, 2, 3, 4];
const BOTTOM_PLACES: [u16; 5] = [0, 1, 2, 0, 3];
const TOP_PIECES: [u8; 5] = [EMPTY, WHITE_KING, BLACK_MAN, BLACK_KING, EMPTY];
const CENTER_PIECES: [u8; 5] = [EMPTY, WHITE_MAN, WHITE_KING, BLACK_MAN, BLACK_KING];
const BOTTOM_PIECES: [u8; 5] = [EMPTY, WHITE_MAN, WHITE_KING, BLACK_KING, EMPTY];
const BIT_COUNT: [usize; 10] = [10, 12, 12, 12, 12, 12, 12, 12, 12, 10];
const BIT_START: [usize; 10] = [2, 20, 32, 44, 56, 68, 80, 92, 104, 116];
const BIT_MASK: [u8; 9] = [0x0, 0x1, 0x3, 0x7, 0xf, 0x1f, 0x3f, 0x7f, 0xff];

pub fn to_decimal(position: &Position) -> DecimalData {
    let sign = if position.side_to_move() == Color::White {
        0
    } else {
        0x80
    };
    let mut data: DecimalData = [sign, 0, 0x40, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0x01];

    let rows = (0..50)
        .step_by(5)
        .map(|start| to_decimal_row(position, start))
        .enumerate();
    for (i, row) in rows {
        let bit_start = BIT_START[i];
        let higher_start = bit_start % 8;
        let lower_bits = BIT_COUNT[i] + higher_start - 8;
        data[bit_start / 8] |= (row >> lower_bits) as u8;
        data[bit_start / 8 + 1] |= (row << (8 - lower_bits)) as u8;
    }

    data
}

fn to_decimal_row(position: &Position, start: Field) -> u16 {
    let (p, pieces) = match start {
        0 => (4, TOP_PLACES),
        5...40 if start % 5 == 0 => (5, CENTER_PLACES),
        45 => (4, BOTTOM_PLACES),
        _ => unreachable!(),
    };
    (start..start + 5).fold(0u16, |a, field| {
        p * a + pieces[position.piece_at(field) as usize]
    })
}

pub fn to_position(decimal: &DecimalData) -> Position {
    let mut position = Position::create();
    if decimal[0] >= 0x80 {
        position = position.toggle_side();
    }

    for i in 0..10 {
        let (p, pieces) = match i {
            0 => (4, TOP_PIECES),
            1...8 => (5, CENTER_PIECES),
            9 => (4, BOTTOM_PIECES),
            _ => unreachable!(),
        };
        let mut row = from_decimal_row(decimal, i);
        for field in (0..5).map(|j| 5 * i + j).rev() {
            position = position.put_piece(field, pieces[row % p]);
            row /= p;
        }
    }

    position
}

fn from_decimal_row(data: &DecimalData, i: usize) -> usize {
    let bit_start = BIT_START[i];
    let higher_start = bit_start % 8;
    let higher_bits = 8 - higher_start;
    let lower_bits = BIT_COUNT[i] - higher_bits;
    let lower_left = 8 - lower_bits;

    let higher = ((data[bit_start / 8] & BIT_MASK[higher_bits]) as usize) << lower_bits;
    let lower = (data[bit_start / 8 + 1] >> lower_left) as usize;
    higher | lower
}

#[test]
fn to_decimal_test() {
    let data = to_decimal(&Position::initial());
    assert_eq!(0x2au8, data[0]);
    assert_eq!(0xa0u8, data[1]);
    assert_eq!(0x1, data[15] & 0x1);
}

#[cfg(test)]
mod test {
    use super::super::generator::Generator;
    use super::super::mv::Move;
    use super::*;
    use decimal::d128;

    #[test]
    fn initial_roundtrip_test() {
        let initial = Position::initial();
        let d = to_decimal(&initial);
        let p = to_position(&d);
        assert!(initial == p);
    }

    fn moving_roundtrip_test(pick: &Fn(&[Move]) -> &Move) {
        let generator = Generator::create();
        let mut position = Position::initial();
        let mut i = 0;
        loop {
            let d = unsafe { d128::from_raw_bytes(to_decimal(&position)) };
            assert!(!d.is_nan());
            i = match i {
                100 => break,
                _ => i + 1,
            };

            let moves = generator.legal_moves(&position);
            if moves.is_empty() {
                break;
            }

            position = position.go(pick(&moves));
            let d = to_decimal(&position);
            let p = to_position(&d);
            assert!(position == p);
        }
    }

    #[test]
    fn moving_roundtrip_first_test() {
        moving_roundtrip_test(&|moves: &[Move]| &moves[0])
    }

    #[test]
    fn moving_roundtrip_last_test() {
        moving_roundtrip_test(&|moves: &[Move]| &moves[moves.len() - 1])
    }
}
