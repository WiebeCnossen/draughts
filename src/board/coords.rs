extern crate core;

use std::convert::From;

#[derive(Debug)]
pub struct Coords { x: i8, y: i8 }

impl PartialEq for Coords {
    fn eq(&self, other: &Coords) -> bool {
        self.x == other.x && self.y == other.y
    }
}
impl Eq for Coords {}

#[test]
fn partial_eq() {
    assert_eq!(Coords{ x: 1, y: 2 }, Coords{ x: 1, y: 2 });
    assert!(Coords{ x: 1, y: 2 } == Coords{ x: 1, y: 2 });
    assert!(Coords{ x: 1, y: 2 } != Coords{ x: 2, y: 1 });
}

impl From<Coords> for i8 {
    fn from(c: Coords) -> i8 {
        46 - (5 * (c.x + c.y)) - ((c.y - c.x) / 2)
    }
}

#[test]
fn into_i8() {
    assert_eq!(i8::from(Coords { x: 0, y: 0 }), 46);
    assert_eq!(i8::from(Coords { x: 1, y: 0 }), 41);
    assert_eq!(i8::from(Coords { x: 1, y: 1 }), 36);
    assert_eq!(i8::from(Coords { x: 5, y: 4 }), 1);
    assert_eq!(i8::from(Coords { x: 4, y: -4 }), 50);
    assert_eq!(i8::from(Coords { x: 9, y: 0 }), 5);
}

impl From<i8> for Coords {
    fn from(n: i8) -> Coords {
        let ny = (50 - n) / 5; // rows from bottom
        let nx = (ny % 2) + (2 * ((n - 1) % 5)); // columns from left
        Coords { x: (nx + ny) / 2, y: (ny - nx) / 2 }
    }
}

#[test]
fn from_i8() {
    assert_eq!(Coords::from(46i8), Coords { x: 0, y: 0 });
    assert_eq!(Coords::from(41i8), Coords { x: 1, y: 0 });
    assert_eq!(Coords::from(36i8), Coords { x: 1, y: 1 });
    assert_eq!(Coords::from(31i8), Coords { x: 2, y: 1 });
    assert_eq!(Coords::from(1i8), Coords { x: 5, y: 4 });
    assert_eq!(Coords::from(50i8), Coords { x: 4, y: -4 });
    assert_eq!(Coords::from(5i8), Coords { x: 9, y: 0 });
}

fn min_x(y: i8) -> i8 { y.abs() }

fn max_x(y: i8) -> i8 { 9 - y.abs() }

fn min_y(x: i8) -> i8 { -max_y(x) }

fn max_y(x: i8) -> i8 { if x > 4 { 9 - x } else { x } }

#[test]
fn min_max() {
    assert_eq!(min_x(-4), 4);
    assert_eq!(min_x(0), 0);
    assert_eq!(min_x(1), 1);
    assert_eq!(max_x(-4), 5);
    assert_eq!(max_x(0), 9);
    assert_eq!(max_x(1), 8);
    assert_eq!(min_y(0), 0);
    assert_eq!(min_y(1), -1);
    assert_eq!(min_y(5), -4);
    assert_eq!(max_y(0), 0);
    assert_eq!(max_y(1), 1);
    assert_eq!(max_y(5), 4);
}
