use aoc_2016::day01::coordinate::Coordinate;
use aoc_2016::day01::Instruction;
use aoc_2016::day01::location::Location;
use std::collections::HashSet;

#[derive(Debug)]
pub struct Sleigh {
    location: Location,
    location_log: HashSet<Coordinate>,
    first_recurrence: Option<Coordinate>,
}

impl Sleigh {
    pub fn new() -> Sleigh {
        let mut log: HashSet<Coordinate> = HashSet::new();
        log.insert(Coordinate::origin());
        Sleigh {
            location: Location::new(),
            location_log: log,
            first_recurrence: None,
        }
    }

    fn execute_and_log(&mut self, instr: &Instruction) {
        let (ref turn, offset) = *instr;
        self.location.turn(turn);
        for _i in 0..offset {
            self.location.increment();
            let coord = self.location.coordinate;
            if !self.location_log.insert(coord) {
                self.first_recurrence = self.first_recurrence.or_else(|| Some(coord));
            }
        }
    }

    pub fn run(&mut self, list: &Vec<Instruction>) {
        list.iter().for_each(|instr| self.execute_and_log(instr))
    }

    pub fn distance_to_origin(&self) -> u32 {
        self.location.distance_to_origin()
    }

    pub fn bunny_hq(&self) -> Option<Coordinate> {
        self.first_recurrence
    }
}
