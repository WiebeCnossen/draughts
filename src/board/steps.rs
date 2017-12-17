use std::ops::Range;
use board::coords::{Coords, MinXY};
use board::position::Field;

fn all<F, G>(generator: F) -> Vec<G>
where
    F: Fn(Field) -> G,
{
    let mut result = Vec::with_capacity(50);
    for field in 0..50 {
        result.push(generator(field));
    }
    result
}

fn path<F>(len: i8, generator: F) -> Vec<Field>
where
    F: Fn(i8) -> Coords,
{
    let mut result = vec![];
    for d in 1..len + 1 {
        result.push(Field::from(generator(d)));
    }
    result
}

fn paths(field: Field) -> [Vec<Field>; 4] {
    let coords = Coords::from(field);
    [
        path(coords.max_x() - coords.x, |d| {
            Coords {
                x: coords.x + d,
                y: coords.y,
            }
        }),
        path(coords.max_y() - coords.y, |d| {
            Coords {
                x: coords.x,
                y: coords.y + d,
            }
        }),
        path(coords.x - coords.min_x(), |d| {
            Coords {
                x: coords.x - d,
                y: coords.y,
            }
        }),
        path(coords.y - coords.min_y(), |d| {
            Coords {
                x: coords.x,
                y: coords.y - d,
            }
        }),
    ]
}

fn steps(field: Field, range: Range<Field>) -> Vec<Field> {
    let mut result = vec![];
    let paths = paths(field);
    for i in range {
        if !paths[i].is_empty() {
            result.push(paths[i][0]);
        }
    }
    result
}

fn white_steps(field: Field) -> Vec<Field> {
    steps(field, 0..2)
}

#[test]
fn white_steps_side() {
    let steps = white_steps(35);
    assert_eq!(steps.len(), 1);
    for step in steps.into_iter() {
        assert!(match step {
            30 => true,
            _ => false,
        });
    }
}

#[test]
fn white_steps_center() {
    let steps = white_steps(36);
    assert_eq!(steps.len(), 2);
    for step in steps.into_iter() {
        assert!(match step {
            30 | 31 => true,
            _ => false,
        });
    }
}

fn black_steps(field: Field) -> Vec<Field> {
    steps(field, 2..4)
}

#[test]
fn black_steps_side() {
    let steps = black_steps(35);
    assert_eq!(steps.len(), 1);
    for step in steps.into_iter() {
        assert!(match step {
            40 => true,
            _ => false,
        });
    }
}

#[test]
fn black_steps_center() {
    let steps = black_steps(36);
    assert_eq!(steps.len(), 2);
    for step in steps.into_iter() {
        assert!(match step {
            40 | 41 => true,
            _ => false,
        });
    }
}

fn short_jumps(field: Field) -> Vec<(Field, Field)> {
    let mut result = vec![];
    let paths = paths(field);
    for path in &paths {
        if path.len() > 1 {
            result.push((path[0], path[1]));
        }
    }
    result
}

#[test]
fn short_jumps_side() {
    let steps = short_jumps(30);
    assert_eq!(steps.len(), 2);
    for step in steps.into_iter() {
        assert!(match step {
            (26, 21) | (36, 41) => true,
            _ => false,
        });
    }
}

#[test]
fn short_jumps_center() {
    let steps = short_jumps(31);
    assert_eq!(steps.len(), 4);
    for step in steps.into_iter() {
        assert!(match step {
            (26, 20) | (27, 22) | (36, 40) | (37, 42) => true,
            _ => false,
        });
    }
}

#[cfg(test)]
fn long_steps(field: Field) -> Vec<Field> {
    let mut v = vec![];
    let paths = paths(field);
    for i in 0..4 {
        for j in 0..paths[i].len() {
            v.push(paths[i][j]);
        }
    }
    v
}

#[test]
fn long_steps_side() {
    let steps = long_steps(30);
    assert_eq!(steps.len(), 11);
    for step in steps.into_iter() {
        assert!(match step {
            25 | 26 | 21 | 17 | 12 | 8 | 3 | 35 | 36 | 41 | 47 => true,
            _ => false,
        });
    }
}

#[test]
fn long_steps_center() {
    let steps = long_steps(31);
    assert_eq!(steps.len(), 15);
    for step in steps.into_iter() {
        assert!(match step {
            26 | 20 | 15 | 27 | 22 | 18 | 13 | 9 | 4 | 36 | 40 | 45 | 37 | 42 | 48 => true,
            _ => false,
        });
    }
}

pub struct Steps {
    all_white_steps: Vec<Vec<Field>>,
    all_black_steps: Vec<Vec<Field>>,
    all_short_jumps: Vec<Vec<(Field, Field)>>,
    all_paths: Vec<[Vec<Field>; 4]>,
}

impl Steps {
    pub fn create() -> Steps {
        Steps {
            all_white_steps: all(white_steps),
            all_black_steps: all(black_steps),
            all_short_jumps: all(short_jumps),
            all_paths: all(paths),
        }
    }

    pub fn white_steps(&self, field: Field) -> &[Field] {
        &self.all_white_steps[field]
    }
    pub fn black_steps(&self, field: Field) -> &[Field] {
        &self.all_black_steps[field]
    }
    pub fn short_jumps(&self, field: Field) -> &[(Field, Field)] {
        &self.all_short_jumps[field]
    }
    pub fn paths(&self, field: Field) -> [&[Field]; 4] {
        let vecs = &self.all_paths[field];
        [&vecs[0], &vecs[1], &vecs[2], &vecs[3]]
    }
}
