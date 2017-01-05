use board::coords::{Coords,MinXY};
use board::piece::{EMPTY,WHITE_MAN,WHITE_KING,BLACK_MAN,BLACK_KING,BLACK,WHITE,color};
use board::position::Position;
use board::Move;
use board::Move::{Shift,Take1,Take2,Take3,Take4,Take5,Take6,Take7,Take8};

#[cfg(test)]
use board::position::Game;

#[cfg(test)]
use board::bitboard::BitboardPosition;

fn white_steps(field: usize) -> Vec<usize> {
  let mut result = vec![];
  let coords = Coords::from(field);
  if coords.max_x() > coords.x {
    result.push(usize::from(Coords { x: coords.x + 1, y: coords.y }));
  }
  if coords.max_y() > coords.y {
    result.push(usize::from(Coords { x: coords.x, y: coords.y + 1 }));
  }
  result
}

#[test]
fn white_steps_side() {
  let steps = white_steps(35);
  assert_eq!(steps.len(), 1);
  for step in steps.into_iter() {
    assert!(
      match step {
        30 => true,
        _ => false
      });
  }
}

#[test]
fn white_steps_center() {
  let steps = white_steps(36);
  assert_eq!(steps.len(), 2);
  for step in steps.into_iter() {
    assert!(
      match step {
        30 | 31 => true,
        _ => false
      });
  }
}

fn black_steps(field: usize) -> Vec<usize> {
  let mut result = vec![];
  let coords = Coords::from(field);
  if coords.min_x() < coords.x {
    result.push(usize::from(Coords { x: coords.x - 1, y: coords.y }));
  }
  if coords.min_y() < coords.y {
    result.push(usize::from(Coords { x: coords.x, y: coords.y - 1 }));
  }
  result
}

#[test]
fn black_steps_side() {
  let steps = black_steps(35);
  assert_eq!(steps.len(), 1);
  for step in steps.into_iter() {
    assert!(
      match step {
        40 => true,
        _ => false
      });
  }
}

#[test]
fn black_steps_center() {
  let steps = black_steps(36);
  assert_eq!(steps.len(), 2);
  for step in steps.into_iter() {
    assert!(
      match step {
        40 | 41 => true,
        _ => false
      });
  }
}

fn num_taken(mv: &Move) -> usize {
  match *mv {
    Shift(..) => 0,
    Take1(..) => 1,
    Take2(..) => 2,
    Take3(..) => 3,
    Take4(..) => 4,
    Take5(..) => 5,
    Take6(..) => 6,
    Take7(..) => 7,
    Take8(..) => 8,
  }
}

fn short_jumps(field: usize) -> Vec<(usize, usize)> {
  let mut result = vec![];
  let coords = Coords::from(field);
  if coords.max_x() > coords.x + 1 {
    result.push((
      usize::from(Coords { x: coords.x + 1, y: coords.y }),
      usize::from(Coords { x: coords.x + 2, y: coords.y })
    ));
  }
  if coords.min_x() < coords.x - 1 {
    result.push((
      usize::from(Coords { x: coords.x - 1, y: coords.y }),
      usize::from(Coords { x: coords.x - 2, y: coords.y })
    ));
  }
  if coords.max_y() > coords.y + 1 {
    result.push((
      usize::from(Coords { x: coords.x, y: coords.y + 1 }),
      usize::from(Coords { x: coords.x, y: coords.y + 2 })
    ));
  }
  if coords.min_y() < coords.y - 1 {
    result.push((
      usize::from(Coords { x: coords.x, y: coords.y - 1 }),
      usize::from(Coords { x: coords.x, y: coords.y - 2 })
    ));
  }
  result
}

#[test]
fn short_jumps_side() {
  let steps = short_jumps(30);
  assert_eq!(steps.len(), 2);
  for step in steps.into_iter() {
    assert!(
      match step {
        (26, 21) | (36, 41) => true,
        _ => false
      });
  }
}

#[test]
fn short_jumps_center() {
  let steps = short_jumps(31);
  assert_eq!(steps.len(), 4);
  for step in steps.into_iter() {
    assert!(
      match step {
        (26, 20) | (27, 22) | (36, 40) | (37, 42) => true,
        _ => false
      });
  }
}

fn king_roads(field: usize, min_size: i8) -> Vec<usize> {
  let mut result = vec![];
  let coords = Coords::from(field);
  if coords.max_x() > coords.x + min_size - 1 {
    for x in coords.x + min_size .. coords.max_x() + 1 {
      result.push(usize::from(Coords { x: x, y: coords.y }));
    }
  }
  if coords.min_x() < coords.x - min_size + 1 {
    for x in coords.min_x() .. coords.x - min_size + 1 {
      result.push(usize::from(Coords { x: x, y: coords.y }));
    }
  }
  if coords.max_y() > coords.y + min_size - 1 {
    for y in coords.y + min_size .. coords.max_y() + 1 {
      result.push(usize::from(Coords { x: coords.x, y: y }));
    }
  }
  if coords.min_y() < coords.y - min_size + 1 {
    for y in coords.min_y() .. coords.y - min_size + 1 {
      result.push(usize::from(Coords { x: coords.x, y: y }));
    }
  }
  result
}

fn long_steps(field: usize) -> Vec<usize> {
  return king_roads(field, 1)
}

#[test]
fn long_steps_side() {
  let steps = long_steps(30);
  assert_eq!(steps.len(), 11);
  for step in steps.into_iter() {
    assert!(
      match step {
        25 | 26 | 21 | 17 | 12 | 8 | 3 | 35 | 36 | 41 | 47 => true,
        _ => false
      });
  }
}

#[test]
fn long_steps_center() {
  let steps = long_steps(31);
  assert_eq!(steps.len(), 15);
  for step in steps.into_iter() {
    assert!(
      match step {
        26 | 20 | 15 | 27 | 22 | 18 | 13 | 9 | 4 | 36 | 40 | 45 | 37 | 42 | 48 => true,
        _ => false
      });
  }
}

fn long_jumps(field: usize) -> Vec<usize> {
  return king_roads(field, 2)
}

#[test]
fn long_jumps_side() {
  let steps = long_jumps(30);
  assert_eq!(steps.len(), 7);
  for step in steps.into_iter() {
    assert!(
      match step {
        21 | 17 | 12 | 8 | 3 | 41 | 47 => true,
        _ => false
      });
  }
}

#[test]
fn long_jumps_center() {
  let steps = long_jumps(31);
  assert_eq!(steps.len(), 11);
  for step in steps.into_iter() {
    assert!(
      match step {
        20 | 15 | 22 | 18 | 13 | 9 | 4 | 40 | 45 | 42 | 48 => true,
        _ => false
      });
  }
}

fn explode_short_jump(from: usize, to: usize, via: &[usize], min_captures: usize) -> Vec<Move> {
  vec![Take1(from, to, via[0])]
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
              let moves = explode_short_jump(field, to, &vec![via], captures);
              match moves.first() {
                Some(ref peek) => {
                  let num = num_taken(peek);
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
        _ => false
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
        _ => false
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
        _ => false
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
        _ => false
      });
  }
}
