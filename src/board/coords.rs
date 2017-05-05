#[derive(Debug)]
pub struct Coords {
    pub x: i8,
    pub y: i8,
}

pub trait MinXY {
    fn min_x(&self) -> i8;
    fn max_x(&self) -> i8;
    fn min_y(&self) -> i8;
    fn max_y(&self) -> i8;
}

fn min_x(y: i8) -> i8 {
    y.abs()
}
fn max_x(y: i8) -> i8 {
    9 - y.abs()
}
fn min_y(x: i8) -> i8 {
    -max_y(x)
}
fn max_y(x: i8) -> i8 {
    if x > 4 { 9 - x } else { x }
}

impl MinXY for Coords {
    fn min_x(&self) -> i8 {
        min_x(self.y)
    }
    fn max_x(&self) -> i8 {
        max_x(self.y)
    }
    fn min_y(&self) -> i8 {
        min_y(self.x)
    }
    fn max_y(&self) -> i8 {
        max_y(self.x)
    }
}

impl PartialEq for Coords {
    fn eq(&self, other: &Coords) -> bool {
        self.x == other.x && self.y == other.y
    }
}
impl Eq for Coords {}

#[test]
fn partial_eq() {
    assert_eq!(Coords { x: 1, y: 2 }, Coords { x: 1, y: 2 });
    assert!(Coords { x: 1, y: 2 } == Coords { x: 1, y: 2 });
    assert!(Coords { x: 1, y: 2 } != Coords { x: 2, y: 1 });
}

impl From<Coords> for usize {
    fn from(c: Coords) -> usize {
        (45 - (5 * (c.x + c.y)) - ((c.y - c.x) / 2)) as usize
    }
}

#[test]
fn into_usize() {
    assert_eq!(usize::from(Coords { x: 0, y: 0 }), 45);
    assert_eq!(usize::from(Coords { x: 1, y: 0 }), 40);
    assert_eq!(usize::from(Coords { x: 1, y: 1 }), 35);
    assert_eq!(usize::from(Coords { x: 5, y: 4 }), 0);
    assert_eq!(usize::from(Coords { x: 4, y: -4 }), 49);
    assert_eq!(usize::from(Coords { x: 9, y: 0 }), 4);
}

impl From<usize> for Coords {
    fn from(n: usize) -> Coords {
        let n = n as i8;
        let ny = (49 - n) / 5; // rows from bottom
        let nx = (ny % 2) + (2 * (n % 5)); // columns from left
        Coords {
            x: (nx + ny) / 2,
            y: (ny - nx) / 2,
        }
    }
}

#[test]
fn from_usize() {
    assert_eq!(Coords::from(45usize), Coords { x: 0, y: 0 });
    assert_eq!(Coords::from(40usize), Coords { x: 1, y: 0 });
    assert_eq!(Coords::from(35usize), Coords { x: 1, y: 1 });
    assert_eq!(Coords::from(30usize), Coords { x: 2, y: 1 });
    assert_eq!(Coords::from(0usize), Coords { x: 5, y: 4 });
    assert_eq!(Coords::from(49usize), Coords { x: 4, y: -4 });
    assert_eq!(Coords::from(4usize), Coords { x: 9, y: 0 });
}

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
