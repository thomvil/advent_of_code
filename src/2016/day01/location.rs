use coordinate::Coordinate;
use Direction;
use Direction::*;
use Turn;
use Turn::*;

#[derive(Debug)]
pub struct Location {
    direction: Direction,
    pub coordinate: Coordinate,
}
impl Location {
    pub fn new() -> Location {
        Location {
            direction: North,
            coordinate: Coordinate::origin(),
        }
    }

    pub fn turn(&mut self, turn: Turn) {
        match (turn, &self.direction) {
            (Left, &North) => self.direction = West,
            (Left, &East) => self.direction = North,
            (Left, &South) => self.direction = East,
            (Left, &West) => self.direction = South,
            (Right, &North) => self.direction = East,
            (Right, &East) => self.direction = South,
            (Right, &South) => self.direction = West,
            (Right, &West) => self.direction = North,
        }
    }

    pub fn increment(&mut self) {
        match self.direction {
            North => self.coordinate.offset_y(1),
            East => self.coordinate.offset_x(1),
            South => self.coordinate.offset_y(-1),
            West => self.coordinate.offset_x(-1),
        }
    }

    pub fn distance_to_origin(&self) -> u32 {
        self.coordinate.norm1()
    }
}
