#[derive(Copy, Clone, Debug, Hash, PartialEq, Eq)]
pub struct Coordinate {
    pub x: i32,
    pub y: i32,
}
impl Coordinate {
    pub fn new(x: i32, y: i32) -> Coordinate {
        Coordinate { x: x, y: y }
    }

    pub fn origin() -> Coordinate {
        Coordinate::new(0, 0)
    }

    pub fn offset_x(&mut self, offset: i32) {
        self.x += offset;
    }

    pub fn offset_y(&mut self, offset: i32) {
        self.y += offset;
    }

    pub fn norm1(&self) -> u32 {
        (self.x.abs() + self.y.abs()) as u32
    }
}
