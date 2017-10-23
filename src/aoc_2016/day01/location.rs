use aoc_2016::day01::coordinate::Coordinate;
use aoc_2016::day01::Direction;
use aoc_2016::day01::Direction::*;
use aoc_2016::day01::Turn;
use aoc_2016::day01::Turn::*;

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

    pub fn turn(&mut self, turn: &Turn) {
        match (turn, &self.direction) {
            (&Left, &South) | (&Right, &North) => self.direction = East,
            (&Left, &West) | (&Right, &East) => self.direction = South,
            (&Left, &North) | (&Right, &South) => self.direction = West,
            (&Left, &East) | (&Right, &West) => self.direction = North,
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
