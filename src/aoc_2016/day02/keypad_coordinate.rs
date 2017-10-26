use aoc_2016::day02::Direction;

#[derive(Copy, Clone, Debug, Hash, PartialEq, Eq)]
pub struct KeyPadCoordinate {
    x: i8,
    y: i8,
}
impl KeyPadCoordinate {
    pub fn new(x: i8, y: i8) -> KeyPadCoordinate {
        KeyPadCoordinate { x: x, y: y }
    }

    pub fn origin() -> KeyPadCoordinate {
        KeyPadCoordinate::new(0, 0)
    }

    pub fn shift(&self, dir: Direction) -> KeyPadCoordinate {
        match dir {
            Direction::Up => KeyPadCoordinate::new(self.x, self.y + 1),
            Direction::Left => KeyPadCoordinate::new(self.x - 1, self.y),
            Direction::Down => KeyPadCoordinate::new(self.x, self.y - 1),
            Direction::Right => KeyPadCoordinate::new(self.x + 1, self.y),
        }
    }
}
