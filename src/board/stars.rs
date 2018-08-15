use super::coords::{Coords, MinXY};
use super::position::Field;

fn star(mid: Field) -> Option<[Field; 5]> {
    let mid = Coords::from(mid);
    if mid.min_x() == mid.x || mid.max_x() == mid.x || mid.min_y() == mid.y || mid.max_y() == mid.y
    {
        None
    } else {
        Some([
            Field::from(Coords {
                x: mid.x,
                y: mid.y + 1,
            }),
            Field::from(Coords {
                x: mid.x + 1,
                y: mid.y,
            }),
            Field::from(Coords { x: mid.x, y: mid.y }),
            Field::from(Coords {
                x: mid.x - 1,
                y: mid.y,
            }),
            Field::from(Coords {
                x: mid.x,
                y: mid.y - 1,
            }),
        ])
    }
}

#[derive(Clone)]
pub struct Stars {
    pub positions: Vec<Vec<(usize, usize)>>,
    pub stars: Vec<[Field; 5]>,
}

impl Stars {
    pub fn create() -> Stars {
        let stars: Vec<_> = (0..50).filter_map(star).collect();
        let positions = (0..50)
            .map(|field| {
                (0..32)
                    .filter_map(|star| {
                        stars[star]
                            .iter()
                            .position(|&part| field == part)
                            .map(|index| (star, index))
                    }).collect()
            }).collect();
        Stars { positions, stars }
    }
}

#[test]
fn corner() {
    assert_eq!(star(0), None)
}

#[test]
fn top() {
    assert_eq!(star(2), None)
}

#[test]
fn bottom() {
    assert_eq!(star(47), None)
}

#[test]
fn odd() {
    assert_eq!(star(7), Some([1, 2, 7, 11, 12]))
}

#[test]
fn even() {
    assert_eq!(star(12), Some([7, 8, 12, 17, 18]))
}

#[test]
fn star_count() {
    assert_eq!((0..50).filter_map(star).count(), 32)
}
