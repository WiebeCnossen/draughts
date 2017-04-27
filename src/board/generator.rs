use board::bitboard::BitboardPosition;
use board::piece::{EMPTY,WHITE_MAN,WHITE_KING,BLACK_MAN,BLACK_KING,piece_own,piece_is,Color};
use board::piece::Color::{White, Black};
use board::position::{Game,Position};
use board::mv::Move;
use board::mv::Move::{Shift,Take1,Take2,Take3,Take4,Take5,Take6,Take7,Take8,Take9,Take10,Take11,Take12};
use board::steps::Steps;

fn take_more(mv: &Move, via: usize, to: usize, position: &Position) -> Move {
  match mv {
    &Shift(..) =>
      panic!("Taking more after Shift is prohibited"),
    &Take1(from, _, via0) =>
      Take2(from, to, via0, via),
    &Take2(from, _, via0, via1) =>
      Take3(from, to, via0, via1, via),
    &Take3(from, _, via0, via1, via2) =>
      Take4(from, to, via0, via1, via2, via),
    &Take4(from, _, via0, via1, via2, via3) =>
      Take5(from, to, via0, via1, via2, via3, via),
    &Take5(from, _, via0, via1, via2, via3, via4) =>
      Take6(from, to, via0, via1, via2, via3, via4, via),
    &Take6(from, _, via0, via1, via2, via3, via4, via5) =>
      Take7(from, to, via0, via1, via2, via3, via4, via5, via),
    &Take7(from, _, via0, via1, via2, via3, via4, via5, via6) =>
      Take8(from, to, via0, via1, via2, via3, via4, via5, via6, via),
    &Take8(from, _, via0, via1, via2, via3, via4, via5, via6, via7) =>
      Take9(from, to, via0, via1, via2, via3, via4, via5, via6, via7, via),
    &Take9(from, _, via0, via1, via2, via3, via4, via5, via6, via7, via8) =>
      Take10(from, to, via0, via1, via2, via3, via4, via5, via6, via7, via8, via),
    &Take10(from, _, via0, via1, via2, via3, via4, via5, via6, via7, via8, via9) =>
      Take11(from, to, via0, via1, via2, via3, via4, via5, via6, via7, via8, via9, via),
    &Take11(from, _, via0, via1, via2, via3, via4, via5, via6, via7, via8, via9, via10) =>
      Take12(from, to, via0, via1, via2, via3, via4, via5, via6, via7, via8, via9, via10, via),
    _ => panic!("Too many captures at \r\n{}", position.ascii())
  }
}

pub struct Generator {
  steps: Steps
}

impl Generator {
  pub fn create() -> Generator {
    Generator { steps: Steps::create() }
  }

  fn merge_moves(mut result : &mut Vec<Move>, moves : &mut Vec<Move>) {
    while let Some(mv) = moves.pop() {
      if !result.contains(&mv) {
        result.push(mv)
      }
    }
  }

  fn trim_result(mut result : Vec<Move>, min_captures : usize) -> Vec<Move> {
    if result.len() == 0 {
      return result;
    }

    let max = result.iter().fold(0, |mx, mv| { let nt = mv.num_taken(); if mx > nt { mx } else { nt }});
    if max < min_captures {
      result.clear();
      return result;
    }

    let mut i = 0;
    while i < result.len() {
      if result[i].num_taken() < max {
        result.swap_remove(i);
      }
      else {
        i += 1;
      }
    }

    result
  }

  fn explode_short_jump(&self, position: &Position, mv: Move, color_to_capture: Color, moves: &mut Vec<Move>) {
    let mut exploded = false;
    for &(via, to) in self.steps.short_jumps(mv.to()).into_iter() {
      if piece_is(position.piece_at(via), color_to_capture.clone())
        && position.piece_at(to) == EMPTY
        && !mv.goes_via(via) {
        exploded = true;
        self.explode_short_jump(position, take_more(&mv, via, to, position), color_to_capture.clone(), moves);
      }
    }

    if !exploded {
      moves.push(mv);
    }
  }

  fn explode_short_jumps(&self, position: &Position, mv: Move, min_captures: usize, color_to_capture: Color) -> Vec<Move> {
    let mut result = vec![];
    self.explode_short_jump(position, mv, color_to_capture, &mut result);
    Generator::trim_result(result, min_captures)
  }

  fn add_short_jumps(&self, position: &Position, field: usize, result: &mut Vec<Move>, captures: &mut usize, color_to_capture: Color) {
    for &(via, to) in self.steps.short_jumps(field).into_iter() {
      if position.piece_at(to) == EMPTY && piece_is(position.piece_at(via), color_to_capture.clone()) {
        let mut moves = self.explode_short_jumps(position, Take1(field, to, via), *captures, color_to_capture.clone());
        match moves.first() {
          Some(ref peek) => {
            let num = peek.num_taken();
            if num > *captures {
              result.clear();
              *captures = num;
            }
          },
          None => ()
        }
        result.append(&mut moves);
      }
    }
  }

  fn explode_long_jump(&self, position: &Position, mv: Move, color_to_capture: Color, moves: &mut Vec<Move>) {
    let mut exploded = false;
    let paths = self.steps.paths(mv.to());
    for dir in 0..4 {
      let mut via : Option<usize> = None;
      for &to in paths[dir] {
        match (piece_own(position.piece_at(to), color_to_capture.clone()), via) {
          (Some(false), _)
          | (Some(true), Some(_)) => break,
          (Some(true), None) => via = Some(to),
          (None, Some(via)) => {
            if mv.goes_via(via) {
              break;
            }
            else {
              exploded = true;
              self.explode_long_jump(position, take_more(&mv, via, to, position), color_to_capture.clone(), moves);
            }
          },
          (None, None) => ()
        }
      }
    }

    if !exploded {
      moves.push(mv);
    }
  }

  fn explode_long_jumps(&self, position: &Position, mv: Move, min_captures: usize, color_to_capture: Color) -> Vec<Move> {
    let mut result = vec![];
    self.explode_long_jump(position, mv, color_to_capture, &mut result);
    Generator::trim_result(result, min_captures)
  }

  fn add_king_moves(&self, position: &Position, field: usize, mut result: &mut Vec<Move>, captures: &mut usize, color_to_capture: Color) {
    let paths = self.steps.paths(field);
    let without_king = &BitboardPosition::clone(position).put_piece(field, EMPTY);
    for dir in 0..4 {
      let mut via : Option<usize> = None;
      for &to in paths[dir] {
        match (piece_own(position.piece_at(to), color_to_capture.clone()), via) {
          (Some(false), _)
          | (Some(true), Some(_)) => break,
          (Some(true), None) => via = Some(to),
          (None, Some(via)) => {
            let mut moves = self.explode_long_jumps(without_king, Take1(field, to, via), *captures, color_to_capture.clone());
            if let Some(ref peek) = moves.first() {
              let num = peek.num_taken();
              if num > *captures {
                result.clear();
                *captures = num;
              }
            }
            Generator::merge_moves(&mut result, &mut moves);
          },
          (None, None) => {
            if *captures == 0 {
              result.push(Shift(field, to));
            }
          }
        }
      }
    }
  }

  pub fn legal_moves(&self, position: &Position) -> Vec<Move> {
    let mut result = Vec::with_capacity(20);
    let mut captures = 0;
    if position.side_to_move() == White {
      for field in 0..50 {
        match position.piece_at(field) {
          WHITE_MAN => {
            self.add_short_jumps(position, field, &mut result, &mut captures, Black);

            if captures == 0 {
              for &step in self.steps.white_steps(field) {
                if position.piece_at(step) == EMPTY {
                  result.push(Shift(field, step));
                }
              }
            }
          },
          WHITE_KING => {
            self.add_king_moves(position, field, &mut result, &mut captures, Black);
          },
          _ => ()
        }
      }
    }
    else {
      for field in 0..50 {
        match position.piece_at(field) {
          BLACK_MAN => {
            self.add_short_jumps(position, field, &mut result, &mut captures, White);

            if captures == 0 {
              for &step in self.steps.black_steps(field).into_iter() {
                if position.piece_at(step) == EMPTY {
                  result.push(Shift(field, step));
                }
              }
            }
          },
          BLACK_KING => {
            self.add_king_moves(position, field, &mut result, &mut captures, White);
          },
          _ => ()
        }
      }
    }
    result
  }
}

#[cfg(test)]
fn verify(position: &Position, moves: &[Move]) {
  let legal = Generator::create().legal_moves(position);
  let count = legal.len();
  assert!(count >= moves.len());
  assert!(
    legal.into_iter().fold(
      true,
      |ok, mv| {
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
  let position = BitboardPosition::create()
    .put_piece(35, WHITE_MAN);
  verify(&position, &vec![Shift(35, 30)][..]);
}

#[test]
fn one_white_man_blocked() {
  let position = BitboardPosition::create()
    .put_piece(35, WHITE_MAN)
    .put_piece(30, BLACK_MAN)
    .put_piece(26, BLACK_MAN);
  verify(&position, &vec![][..]);
}

#[test]
fn one_white_man_center() {
  let position = BitboardPosition::create()
    .put_piece(36, WHITE_MAN);
  verify(&position, &vec![Shift(36, 30), Shift(36, 31)][..]);
}

#[test]
fn one_black_man_side() {
  let position = BitboardPosition::create()
    .put_piece(35, BLACK_MAN)
    .toggle_side();
  verify(&position, &vec![Shift(35, 40)][..]);
}

#[test]
fn one_single_capture_white_man() {
  let position = BitboardPosition::create()
    .put_piece(15, WHITE_MAN)
    .put_piece(40, BLACK_MAN)
    .put_piece(45, WHITE_MAN);
  verify(&position, &vec![Take1(45, 36, 40)][..]);
}

#[test]
fn one_double_capture_white_man() {
  let position = BitboardPosition::create()
    .put_piece(15, WHITE_MAN)
    .put_piece(31, BLACK_MAN)
    .put_piece(40, BLACK_MAN)
    .put_piece(45, WHITE_MAN);
  verify(&position, &vec![Take2(45, 27, 40, 31)][..]);
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
  verify(&position, &vec![Take3(45, 38, 40, 41, 42)][..]);
}

#[test]
fn two_captures_white_man() {
  let position = BitboardPosition::create()
    .put_piece(40, BLACK_MAN)
    .put_piece(41, BLACK_MAN)
    .put_piece(46, WHITE_MAN);
  verify(&position, &vec![Take1(46, 35, 40), Take1(46, 37, 41)][..]);
}

#[test]
fn two_captures_black_man() {
  let position = BitboardPosition::create()
    .put_piece(30, WHITE_MAN)
    .put_piece(31, WHITE_MAN)
    .put_piece(36, BLACK_MAN)
    .toggle_side();
  verify(&position, &vec![Take1(36, 25, 30), Take1(36, 27, 31)][..]);
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
  verify(&position, &vec![Shift(43, 34), Shift(43, 39), Shift(43, 48), Shift(43, 49)][..]);
}

#[test]
fn black_king_moves() {
  let position = BitboardPosition::create()
    .put_piece(0, BLACK_KING)
    .put_piece(11, WHITE_MAN)
    .put_piece(17, WHITE_KING)
    .toggle_side();
  verify(&position, &vec![Shift(0, 5), Shift(0, 6)][..]);
}

#[test]
fn study1() {
  let position =
    BitboardPosition::parse("w 5/3be/5/3be/web2/wewbe/ew3/3bb/5/3ww")
      .ok()
      .unwrap()
      .go(&Shift(48,43));
  verify(&position, &vec![Take1(39, 48, 43)][..]);
}

#[test]
fn study2() {
  let position =
    BitboardPosition::parse("w 5/3be/5/3be/web2/wewbe/ew3/3bb/5/3ww")
      .ok()
      .unwrap()
      .go(&Shift(48, 43))
      .go(&Take1(39, 48, 43))
      .go(&Shift(49, 43));
  verify(&position, &vec![Take2(48, 15, 31, 20)][..]);
}

#[test]
fn study3() {
  let position =
    BitboardPosition::parse("w 5/3be/5/3be/web2/wewbe/ew3/3bb/5/3ww")
      .ok()
      .unwrap()
      .go(&Shift(48, 43))
      .go(&Take1(39, 48, 43))
      .go(&Shift(49, 43))
      .go(&Take2(48, 15, 31, 20))
      .go(&Take4(43, 38, 28, 18, 8, 3));
  verify(&position, &vec![Take1(22, 31, 27)][..]);
}

#[test]
fn study4() {
  let position =
    BitboardPosition::parse("w 5/3be/5/3be/web2/wewbe/ew3/3bb/5/3ww")
      .ok()
      .unwrap()
      .go(&Shift(48, 43))
      .go(&Take1(39, 48, 43))
      .go(&Shift(49, 43))
      .go(&Take2(48, 15, 31, 20))
      .go(&Take4(43, 38, 28, 18, 8, 3))
      .go(&Take1(22, 31, 27))
      .go(&Shift(25, 20));
  verify(&position, &vec![Take1(15, 26, 20)][..]);
}

#[test]
fn multi_long_capture() {
  let position =
    BitboardPosition::parse("w 5/5/3b1/5/5/5/5/1b3/5/W4").ok().unwrap();
  verify(&position, &vec![Take2(45, 4, 36, 13), Take2(45, 9, 36, 13)][..]);
}

#[test]
fn coup_turc() {
  let position =
    BitboardPosition::parse("b 5/el2/5/Bebew/2w2/5/eh2/3we/ew3/5").ok().unwrap();
  verify(&position, &vec![Take4(15, 27, 31, 38, 19, 22)][..]);
}

#[test]
fn to_start_field() {
  let position =
    BitboardPosition::parse("b 2b2/b4/3bb/5/wewww/3we/4B/ww2w/eww2/5").ok().unwrap();
  verify(&position, &vec![Take4(34, 29, 39, 42, 22, 23),Take4(34, 34, 39, 42, 22, 23),Take4(34, 34, 23, 22, 42, 39)][..]);
}
