use std::fmt;

#[derive(Copy, Clone, Debug, Hash, PartialEq, Eq)]
pub struct Coordinate {
    x: i32,
    y: i32,
}
impl Coordinate {
    pub fn new(x: i32, y: i32) -> Coordinate {
        Coordinate { x: x, y: y }
    }

    pub fn origin() -> Coordinate {
        Coordinate { x: 0, y: 0 }
    }

    pub fn offset_x(&mut self, offset: i32) {
        self.x += offset;
    }

    pub fn offset_y(&mut self, offset: i32) {
        self.y += offset;
    }

    pub fn norm1(&self) -> i32 {
        self.x.abs() + self.y.abs()
    }
}
impl fmt::Display for Coordinate {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "({},{})", self.x, self.y)
    }
}
