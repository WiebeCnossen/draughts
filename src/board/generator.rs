use board::coords::{Coords,MinXY};
use board::piece::{EMPTY,WHITE_MAN,WHITE_KING,BLACK_MAN,BLACK_KING,TRANSPARENT,BLACK,WHITE,color};
use board::position::Position;
use board::mv::{Move,Mover};
use board::mv::Move::{Shift,Take1,Take2,Take3,Take4,Take5,Take6,Take7,Take8};
use board::steps::{white_steps,black_steps,short_jumps};

#[cfg(test)]
use board::position::Game;

#[cfg(test)]
use board::bitboard::BitboardPosition;

fn take_more(mv: &Move, via: usize, to: usize) -> Move {
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
    _ => panic!("Too many captures")
  }
}

fn explode_short_jump(position: &Position, mv: Move, min_captures: usize, color_to_capture: usize) -> Vec<Move> {
  fn explode(position: &Position, mv: Move, color_to_capture: usize, moves: &mut Vec<Move>) {
    let mut exploded = false;
    for (via, to) in short_jumps(mv.to()).into_iter() {
      if color(position.piece_at(via)) == color_to_capture
         && position.piece_at(to) == EMPTY
         && !mv.goes_via(via) {
        exploded = true;
        explode(position, take_more(&mv, via, to), color_to_capture, moves);
      }
    }

    if !exploded {
      moves.push(mv);
    }
  }

  let mut result = vec![];
  explode(position, mv, color_to_capture, &mut result);
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

fn add_short_jumps(position: &Position, field: usize, result: &mut Vec<Move>, captures: &mut usize, color_to_capture: usize) {
  for (via, to) in short_jumps(field).into_iter() {
    if position.piece_at(to) == EMPTY && color(position.piece_at(via)) == color_to_capture {
      let mut moves = explode_short_jump(position, Take1(field, to, via), *captures, color_to_capture);
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

fn add_king_moves(position: &Position, field: usize, result: &mut Vec<Move>, captures: &mut usize, color_to_capture: usize) {
  let coords = Coords::from(field);
  if coords.x < coords.max_x() {
    let mut via : Option<usize> = None;
    let mut x = coords.x + 1;
    while x <= coords.max_x() {
      let to = usize::from(Coords { x: x, y: coords.y });
      match color(position.piece_at(to)) {
        color if color == TRANSPARENT => {
          if *captures == 0 {
            result.push(Shift(field, to));
          }
        },
        color if color == color_to_capture => {
          via = Some(to);
          break;
        },
        _ => break
      }
      x += 1;
    }
  }
}

pub fn legal_moves(position: &Position) -> Vec<Move> {
  let mut result = Vec::with_capacity(20);
  let mut captures = 0;
  if position.white_to_move() {
    for field in 0..50 {
      match position.piece_at(field) {
        WHITE_MAN => {
          add_short_jumps(position, field, &mut result, &mut captures, BLACK);

          if captures == 0 {
            for step in white_steps(field).into_iter() {
              if position.piece_at(step) == EMPTY {
                result.push(Shift(field, step));
              }
            }
          }
        },
        WHITE_KING => {
          add_king_moves(position, field, &mut result, &mut captures, BLACK);
        },
        _ => ()
      }
    }
  }
  else {
    for field in 0..50 {
      match position.piece_at(field) {
        BLACK_MAN => {
          add_short_jumps(position, field, &mut result, &mut captures, WHITE);

          if captures == 0 {
            for step in black_steps(field).into_iter() {
              if position.piece_at(step) == EMPTY {
                result.push(Shift(field, step));
              }
            }
          }
        },
        BLACK_KING => {
          add_king_moves(position, field, &mut result, &mut captures, WHITE);
        },
        _ => ()
      }
    }
  }
  result
}

#[cfg(test)]
fn fail(mv: Move) -> bool {
  println!("{}", mv);
  false
}

#[test]
fn one_white_man_side() {
  let position = BitboardPosition::create()
    .put_piece(35, WHITE_MAN);
  let legal = legal_moves(&position);
  assert_eq!(legal.len(), 1);
  for mv in legal.into_iter() {
    assert!(
      match mv {
        Shift(35, 30) => true,
        _ => fail(mv)
      });
  }
}

#[test]
fn one_white_man_blocked() {
  let position = BitboardPosition::create()
    .put_piece(35, WHITE_MAN)
    .put_piece(30, BLACK_MAN)
    .put_piece(26, BLACK_MAN);
  let legal = legal_moves(&position);
  assert_eq!(legal.len(), 0);
}

#[test]
fn one_white_man_center() {
  let position = BitboardPosition::create()
    .put_piece(36, WHITE_MAN);
  let legal = legal_moves(&position);
  assert_eq!(legal.len(), 2);
  for mv in legal.into_iter() {
    assert!(
      match mv {
        Shift(36, 30)
        | Shift(36, 31) => true,
        _ => fail(mv)
      });
  }
}

#[test]
fn one_black_man_side() {
  let position = BitboardPosition::create()
    .put_piece(35, BLACK_MAN)
    .toggle_side();
  let legal = legal_moves(&position);
  assert_eq!(legal.len(), 1);
  for mv in legal.into_iter() {
    assert!(
      match mv {
        Shift(35, 40) => true,
        _ => fail(mv)
      });
  }
}

#[test]
fn one_single_capture_white_man() {
  let position = BitboardPosition::create()
    .put_piece(15, WHITE_MAN)
    .put_piece(40, BLACK_MAN)
    .put_piece(45, WHITE_MAN);
  let legal = legal_moves(&position);
  assert_eq!(legal.len(), 1);
  for mv in legal.into_iter() {
    assert!(
      match mv {
        Take1(45, 36, 40) => true,
        _ => fail(mv)
      });
  }
}

#[test]
fn one_double_capture_white_man() {
  let position = BitboardPosition::create()
    .put_piece(15, WHITE_MAN)
    .put_piece(31, BLACK_MAN)
    .put_piece(40, BLACK_MAN)
    .put_piece(45, WHITE_MAN);
  let legal = legal_moves(&position);
  assert_eq!(legal.len(), 1);
  for mv in legal.into_iter() {
    assert!(
      match mv {
        Take2(45, 27, 40, 31) => true,
        _ => fail(mv)
      });
  }
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
  let legal = legal_moves(&position);
  assert_eq!(legal.len(), 1);
  for mv in legal.into_iter() {
    assert!(
      match mv {
        Take3(45, 38, 40, 41, 42) => true,
        _ => fail(mv)
      });
  }
}

#[test]
fn two_captures_white_man() {
  let position = BitboardPosition::create()
    .put_piece(40, BLACK_MAN)
    .put_piece(41, BLACK_MAN)
    .put_piece(46, WHITE_MAN);
  let legal = legal_moves(&position);
  assert_eq!(legal.len(), 2);
  for mv in legal.into_iter() {
    assert!(
      match mv {
        Take1(46, 35, 40)
        | Take1(46, 37, 41) => true,
        _ => fail(mv)
      });
  }
}

#[test]
fn two_captures_black_man() {
  let position = BitboardPosition::create()
    .put_piece(30, WHITE_MAN)
    .put_piece(31, WHITE_MAN)
    .put_piece(36, BLACK_MAN)
    .toggle_side();
  let legal = legal_moves(&position);
  assert_eq!(legal.len(), 2);
  for mv in legal.into_iter() {
    assert!(
      match mv {
        Take1(36, 25, 30)
        | Take1(36, 27, 31) => true,
        _ => fail(mv)
      });
  }
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
  let legal = legal_moves(&position);
  assert_eq!(legal.len(), 4);
  for mv in legal.into_iter() {
    assert!(
      match mv {
        Shift(43, 34)
        | Shift(43, 39)
        | Shift(43, 48)
        | Shift(43, 49) => true,
        _ => fail(mv)
      });
  }
}
