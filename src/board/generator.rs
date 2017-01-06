use board::coords::{Coords,MinXY};
use board::piece::{EMPTY,WHITE_MAN,WHITE_KING,BLACK_MAN,BLACK_KING,BLACK,WHITE,color};
use board::position::Position;
use board::mv::{Move,Take};
use board::mv::Move::{Shift,Take1,Take2,Take3,Take4,Take5,Take6,Take7,Take8};
use board::steps::{white_steps,black_steps,short_jumps};

#[cfg(test)]
use board::position::Game;

#[cfg(test)]
use board::bitboard::BitboardPosition;

fn create_move(from: usize, to: usize, via: &[usize]) -> Move {
  match via.len() {
    0 => Shift(from, to),
    1 => Take1(from, to, via[0]),
    2 => Take2(from, to, via[0], via[1]),
    3 => Take3(from, to, via[0], via[1], via[2]),
    4 => Take4(from, to, via[0], via[1], via[2], via[3]),
    5 => Take5(from, to, via[0], via[1], via[2], via[3], via[4]),
    6 => Take6(from, to, via[0], via[1], via[2], via[3], via[4], via[5]),
    7 => Take7(from, to, via[0], via[1], via[2], via[3], via[4], via[5], via[6]),
    8 => Take8(from, to, via[0], via[1], via[2], via[3], via[4], via[5], via[6], via[7]),
    _ => panic!("Too many captures")
  }
}

fn explode_short_jump(position: &Position, from: usize, to: usize, via: &[usize], min_captures: usize, color_to_capture: usize) -> Vec<Move> {
  fn explode(position: &Position, from: usize, to: usize, via: &[usize], color_to_capture: usize, moves: &mut Vec<Move>) {
    let mut exploded = false;
    for (over, next) in short_jumps(to).into_iter() {
      if color(position.piece_at(over)) == color_to_capture && position.piece_at(next) == EMPTY
         && via.iter().fold(true, |b,&f| b && f != over) {
        exploded = true;
        let mut via_next : Vec<usize> = via.into();
        via_next.push(over);
        explode(position, from, next, &via_next[..], color_to_capture, moves);
      }
    }

    if !exploded {
      moves.push(create_move(from, to, via));
    }
  }

  let mut result = vec![];
  explode(position, from, to, via, color_to_capture, &mut result);
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

pub fn legal_moves<Pos>(position: Pos) -> Vec<Move> where Pos : Position {
  let mut result = Vec::with_capacity(20);
  let mut captures = 0;
  if position.white_to_move() {
    for field in 0..50 {
      match position.piece_at(field) {
        WHITE_MAN => {
          for (via, to) in short_jumps(field).into_iter() {
            if position.piece_at(to) == EMPTY && color(position.piece_at(via)) == BLACK {
              let moves = explode_short_jump(&position, field, to, &vec![via], captures, BLACK);
              match moves.first() {
                Some(ref peek) => {
                  let num = peek.num_taken();
                  if num > captures {
                    result.clear();
                    captures = num;
                  }
                },
                None => ()
              }
              for mv in moves.into_iter() {
                result.push(mv);
              }
            }
          }

          if captures == 0 {
            for step in white_steps(field).into_iter() {
              if position.piece_at(step) == EMPTY {
                result.push(Shift(field, step));
              }
            }
          }
        },
        _ => ()
      }
    }
  }
  else {
    for field in 0..50 {
      match position.piece_at(field) {
        BLACK_MAN => {
          if captures == 0 {
            for step in black_steps(field).into_iter() {
              if position.piece_at(step) == EMPTY {
                result.push(Shift(field, step));
              }
            }
          }
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
  let legal = legal_moves(position);
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
  let legal = legal_moves(position);
  assert_eq!(legal.len(), 0);
}

#[test]
fn one_white_man_center() {
  let position = BitboardPosition::create()
    .put_piece(36, WHITE_MAN);
  let legal = legal_moves(position);
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
  let legal = legal_moves(position);
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
  let legal = legal_moves(position);
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
  let legal = legal_moves(position);
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
  let legal = legal_moves(position);
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
  let legal = legal_moves(position);
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
