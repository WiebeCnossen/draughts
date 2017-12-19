use board::bitboard::BitboardPosition;
use board::piece::{piece_is, piece_own, Color, BLACK_KING, BLACK_MAN, EMPTY, WHITE_KING, WHITE_MAN};
use board::piece::Color::{Black, White};
use board::position::{Field, Game, Position};
use board::mv::Move;
use board::steps::Steps;

pub struct Generator {
    steps: Steps,
}

impl Generator {
    pub fn create() -> Generator {
        Generator {
            steps: Steps::create(),
        }
    }

    fn trim_list(list: &mut Vec<Move>) {
        if list.is_empty() {
            return;
        }

        let num_taken = list.iter().map(|mv| mv.num_taken()).max().unwrap();
        if num_taken == 0 {
            return;
        }

        let mut i = 0;
        while i < list.len() {
            if list[i].num_taken() < num_taken {
                list.swap_remove(i);
            } else {
                i += 1;
            }
        }
        if num_taken < 2 {
            return;
        }

        list.sort_unstable_by_key(|mv| (mv.from(), mv.to()));
        list.dedup();
    }

    fn explode_jump(
        &self,
        position: &Position,
        mv: Move,
        color_to_capture: &Color,
        list: &mut Vec<Move>,
    ) {
        let mut exploded = false;
        for &(via, to) in self.steps.short_jumps(mv.to()) {
            if piece_is(position.piece_at(via), color_to_capture) && position.piece_at(to) == EMPTY
                && !mv.goes_via(via)
            {
                exploded = true;
                self.explode_jump(position, mv.take_more(via, to), color_to_capture, list);
            }
        }

        if !exploded {
            list.push(mv);
        }
    }

    fn add_short_jumps(
        &self,
        position: &Position,
        field: Field,
        list: &mut Vec<Move>,
        color_to_capture: &Color,
    ) -> bool {
        let mut captures = false;
        for &(via, to) in self.steps.short_jumps(field) {
            if position.piece_at(to) == EMPTY && piece_is(position.piece_at(via), color_to_capture)
            {
                captures = true;
                self.explode_jump(
                    position,
                    Move::take_one(field, to, via),
                    color_to_capture,
                    list,
                );
            }
        }
        captures
    }

    fn explode_long_jump(
        &self,
        position: &Position,
        mv: Move,
        color_to_capture: &Color,
        moves: &mut Vec<Move>,
    ) {
        let mut exploded = false;
        for path in self.steps.paths(mv.to()) {
            let mut via: Option<Field> = None;
            for &to in path.iter() {
                match (piece_own(position.piece_at(to), color_to_capture), via) {
                    (Some(false), _) | (Some(true), Some(_)) => break,
                    (Some(true), None) => via = Some(to),
                    (None, Some(via)) => {
                        if mv.goes_via(via) {
                            break;
                        } else {
                            exploded = true;
                            self.explode_long_jump(
                                position,
                                mv.take_more(via, to),
                                color_to_capture,
                                moves,
                            );
                        }
                    }
                    (None, None) => (),
                }
            }
        }

        if !exploded {
            moves.push(mv);
        }
    }

    fn add_king_moves(
        &self,
        position: &Position,
        field: Field,
        list: &mut Vec<Move>,
        captures: &mut bool,
        color_to_capture: &Color,
    ) {
        let without_king = &BitboardPosition::clone(position).put_piece(field, EMPTY);
        for path in self.steps.paths(field) {
            let mut via: Option<Field> = None;
            for &to in path.iter() {
                match (piece_own(position.piece_at(to), color_to_capture), via) {
                    (Some(false), _) | (Some(true), Some(_)) => break,
                    (Some(true), None) => via = Some(to),
                    (None, Some(via)) => {
                        self.explode_long_jump(
                            without_king,
                            Move::take_one(field, to, via),
                            color_to_capture,
                            list,
                        );
                        *captures = true;
                    }
                    (None, None) => {
                        if !*captures {
                            list.push(Move::shift(field, to));
                        }
                    }
                }
            }
        }
    }

    pub fn legal_moves(&self, position: &Position) -> Vec<Move> {
        let mut list = Vec::with_capacity(20);
        let mut captures = false;
        if position.side_to_move() == White {
            for field in 0..50 {
                match position.piece_at(field) {
                    WHITE_MAN => {
                        if self.add_short_jumps(position, field, &mut list, &Black) {
                            captures = true;
                        }

                        if !captures {
                            for &step in self.steps.white_steps(field) {
                                if position.piece_at(step) == EMPTY {
                                    list.push(Move::shift(field, step));
                                }
                            }
                        }
                    }
                    WHITE_KING => {
                        self.add_king_moves(position, field, &mut list, &mut captures, &Black);
                    }
                    _ => (),
                }
            }
        } else {
            for field in 0..50 {
                match position.piece_at(field) {
                    BLACK_MAN => {
                        if self.add_short_jumps(position, field, &mut list, &White) {
                            captures = true;
                        }

                        if !captures {
                            for &step in self.steps.black_steps(field) {
                                if position.piece_at(step) == EMPTY {
                                    list.push(Move::shift(field, step));
                                }
                            }
                        }
                    }
                    BLACK_KING => {
                        self.add_king_moves(position, field, &mut list, &mut captures, &White);
                    }
                    _ => (),
                }
            }
        }

        Generator::trim_list(&mut list);
        list
    }
}

#[cfg(test)]
fn verify(position: &Position, moves: &[Move]) {
    let legal = Generator::create().legal_moves(position);
    let count = legal.len();
    assert!(count >= moves.len());
    assert!(legal.into_iter().fold(true, |ok, mv| {
        let expected = moves.iter().fold(false, |v, m| v || mv == *m);
        if !expected {
            println!("Unexpected move {}", mv);
        }
        expected && ok
    }));
    assert_eq!(count, moves.len());
}

#[test]
fn one_white_man_side() {
    let position = BitboardPosition::create().put_piece(35, WHITE_MAN);
    verify(&position, &vec![Move::shift(35, 30)]);
}

#[test]
fn one_white_man_blocked() {
    let position = BitboardPosition::create()
        .put_piece(35, WHITE_MAN)
        .put_piece(30, BLACK_MAN)
        .put_piece(26, BLACK_MAN);
    verify(&position, &vec![]);
}

#[test]
fn one_white_man_center() {
    let position = BitboardPosition::create().put_piece(36, WHITE_MAN);
    verify(&position, &vec![Move::shift(36, 30), Move::shift(36, 31)]);
}

#[test]
fn one_black_man_side() {
    let position = BitboardPosition::create()
        .put_piece(35, BLACK_MAN)
        .toggle_side();
    verify(&position, &vec![Move::shift(35, 40)]);
}

#[test]
fn one_single_capture_white_man() {
    let position = BitboardPosition::create()
        .put_piece(15, WHITE_MAN)
        .put_piece(40, BLACK_MAN)
        .put_piece(45, WHITE_MAN);
    verify(&position, &vec![Move::take_one(45, 36, 40)]);
}

#[test]
fn one_double_capture_white_man() {
    let position = BitboardPosition::create()
        .put_piece(15, WHITE_MAN)
        .put_piece(31, BLACK_MAN)
        .put_piece(40, BLACK_MAN)
        .put_piece(45, WHITE_MAN);
    verify(&position, &vec![Move::take(45, 27, &[40, 31])]);
}

#[test]
fn double_and_triple_capture_white_man() {
    let position = BitboardPosition::create()
        .put_piece(15, WHITE_MAN)
        .put_piece(31, BLACK_MAN)
        .put_piece(40, BLACK_MAN)
        .put_piece(41, BLACK_MAN)
        .put_piece(42, BLACK_MAN)
        .put_piece(45, WHITE_MAN);
    verify(&position, &vec![Move::take(45, 38, &[40, 41, 42])]);
}

#[test]
fn two_captures_white_man() {
    let position = BitboardPosition::create()
        .put_piece(40, BLACK_MAN)
        .put_piece(41, BLACK_MAN)
        .put_piece(46, WHITE_MAN);
    verify(
        &position,
        &vec![Move::take_one(46, 35, 40), Move::take_one(46, 37, 41)],
    );
}

#[test]
fn two_captures_black_man() {
    let position = BitboardPosition::create()
        .put_piece(30, WHITE_MAN)
        .put_piece(31, WHITE_MAN)
        .put_piece(36, BLACK_MAN)
        .toggle_side();
    verify(
        &position,
        &vec![Move::take_one(36, 25, 30), Move::take_one(36, 27, 31)],
    );
}

#[test]
fn white_king_moves() {
    let position = BitboardPosition::create()
        .put_piece(27, BLACK_MAN)
        .put_piece(29, BLACK_MAN)
        .put_piece(32, BLACK_MAN)
        .put_piece(33, BLACK_MAN)
        .put_piece(38, WHITE_MAN)
        .put_piece(43, WHITE_KING);
    verify(
        &position,
        &vec![
            Move::shift(43, 34),
            Move::shift(43, 39),
            Move::shift(43, 48),
            Move::shift(43, 49),
        ],
    );
}

#[test]
fn black_king_moves() {
    let position = BitboardPosition::create()
        .put_piece(0, BLACK_KING)
        .put_piece(11, WHITE_MAN)
        .put_piece(17, WHITE_KING)
        .toggle_side();
    verify(&position, &vec![Move::shift(0, 5), Move::shift(0, 6)]);
}

#[test]
fn study1() {
    let position = BitboardPosition::parse("w 5/3be/5/3be/web2/wewbe/ew3/3bb/5/3ww")
        .ok()
        .unwrap()
        .go(&Move::shift(48, 43));
    verify(&position, &vec![Move::take_one(39, 48, 43)]);
}

#[test]
fn study2() {
    let position = BitboardPosition::parse("w 5/3be/5/3be/web2/wewbe/ew3/3bb/5/3ww")
        .ok()
        .unwrap()
        .go(&Move::shift(48, 43))
        .go(&Move::take_one(39, 48, 43))
        .go(&Move::shift(49, 43));
    verify(&position, &vec![Move::take(48, 15, &[31, 20])]);
}

#[test]
fn study3() {
    let position = BitboardPosition::parse("w 5/3be/5/3be/web2/wewbe/ew3/3bb/5/3ww")
        .ok()
        .unwrap()
        .go(&Move::shift(48, 43))
        .go(&Move::take_one(39, 48, 43))
        .go(&Move::shift(49, 43))
        .go(&Move::take(48, 15, &[31, 20]))
        .go(&Move::take(43, 38, &[28, 18, 8, 3]));
    verify(&position, &vec![Move::take_one(22, 31, 27)]);
}

#[test]
fn study4() {
    let position = BitboardPosition::parse("w 5/3be/5/3be/web2/wewbe/ew3/3bb/5/3ww")
        .ok()
        .unwrap()
        .go(&Move::shift(48, 43))
        .go(&Move::take_one(39, 48, 43))
        .go(&Move::shift(49, 43))
        .go(&Move::take(48, 15, &[31, 20]))
        .go(&Move::take(43, 38, &[28, 18, 8, 3]))
        .go(&Move::take_one(22, 31, 27))
        .go(&Move::shift(25, 20));
    verify(&position, &vec![Move::take_one(15, 26, 20)]);
}

#[test]
fn multi_long_capture() {
    let position = BitboardPosition::parse("w 5/5/3b1/5/5/5/5/1b3/5/W4")
        .ok()
        .unwrap();
    verify(
        &position,
        &vec![Move::take(45, 4, &[36, 13]), Move::take(45, 9, &[36, 13])],
    );
}

#[test]
fn coup_turc() {
    let position = BitboardPosition::parse("b 5/el2/5/Bebew/2w2/5/eh2/3we/ew3/5")
        .ok()
        .unwrap();
    verify(&position, &vec![Move::take(15, 27, &[31, 38, 19, 22])]);
}

#[test]
fn coup_tour() {
    let position = BitboardPosition::parse("w 3We/5/5/5/l3/5/l3/ew3/b4/5")
        .ok()
        .unwrap();
    verify(&position, &vec![Move::take(37, 37, &[21, 22, 31, 32])]);
}

#[test]
fn to_start_field() {
    let position = BitboardPosition::parse("b 2b2/b4/3bb/5/wewww/3we/4B/ww2w/eww2/5")
        .ok()
        .unwrap();
    verify(
        &position,
        &vec![
            Move::take(34, 29, &[39, 42, 22, 23]),
            Move::take(34, 34, &[39, 42, 22, 23]),
        ],
    );
}
