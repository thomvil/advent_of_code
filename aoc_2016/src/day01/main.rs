// #[macro_use]
// extern crate lazy_static;
// extern crate regex;
//
// const INPUT: &'static str = include_str!("../../inputs/day01.txt");
//
// mod day01;
// // mod input_parser;
//
// use Direction::*;
//
// use std::collections::HashSet;
// use std::fmt;
// use Turn::*;
//
// use day01::coordinate::Coordinate;
// use day01::input_parser::instructions;
//
// fn main() {
//     let mut sleigh = Sleigh::new();
//     sleigh.run(instructions());
//     println!(
//         "The sleigh ends {:?} blocks from the starting point.",
//         sleigh.distance_to_origin()
//     );
//     match sleigh.bunny_hq() {
//         Some(c) => {
//             println!(
//                 "Bunny HQ is {:?} blocks from the starting point.",
//                 c.norm1()
//             )
//         }
//         None => println!("Bunny HQ is not found :("),
//     }
// }
//
// // type Instruction = (Turn, i32);
//
// // #[derive(Debug)]
// // enum Direction {
// //     North,
// //     East,
// //     South,
// //     West,
// // }
// //
// // #[derive(Debug)]
// // pub enum Turn {
// //     Left,
// //     Right,
// // }
//
// // #[derive(Debug)]
// // struct Location {
// //     direction: Direction,
// //     coordinate: Coordinate,
// // }
// // impl Location {
// //     fn new() -> Location {
// //         Location {
// //             direction: North,
// //             coordinate: Coordinate::origin(),
// //         }
// //     }
// //
// //     fn turn_left(&mut self) {
// //         match self.direction {
// //             North => self.direction = West,
// //             East => self.direction = North,
// //             South => self.direction = East,
// //             West => self.direction = South,
// //         }
// //     }
// //
// //     fn turn_right(&mut self) {
// //         match self.direction {
// //             North => self.direction = East,
// //             East => self.direction = South,
// //             South => self.direction = West,
// //             West => self.direction = North,
// //         }
// //     }
// //
// //     fn turn(&mut self, turn: Turn) {
// //         match turn {
// //             Left => self.turn_left(),
// //             Right => self.turn_right(),
// //         }
// //     }
// //
// //     fn advance(&mut self, offset: i32) {
// //         match self.direction {
// //             North => self.coordinate.offset_y(offset),
// //             East => self.coordinate.offset_x(offset),
// //             South => self.coordinate.offset_y(-offset),
// //             West => self.coordinate.offset_x(-offset),
// //         }
// //     }
// //
// //     fn increment(&mut self) {
// //         self.advance(1)
// //     }
// // }
// // impl fmt::Display for Location {
// //     fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
// //         write!(f, "{:?}:{}", self.direction, self.coordinate)
// //     }
// // }
//
// // #[derive(Debug)]
// // pub struct Sleigh {
// //     location: Location,
// //     location_log: HashSet<Coordinate>,
// //     first_recurrence: Option<Coordinate>,
// // }
// //
// // impl Sleigh {
// //     fn new() -> Sleigh {
// //         let mut log: HashSet<Coordinate> = HashSet::new();
// //         log.insert(Coordinate::origin());
// //         Sleigh {
// //             location: Location::new(),
// //             location_log: log,
// //             first_recurrence: None,
// //         }
// //     }
// //
// //     fn execute(&mut self, (turn, offset): Instruction) {
// //         self.location.turn(turn);
// //         self.location.advance(offset);
// //     }
// //
// //     fn execute_and_log(&mut self, (turn, offset): Instruction) {
// //         self.location.turn(turn);
// //         for _i in 0..offset {
// //             self.location.increment();
// //             let coord = self.location.coordinate;
// //             if self.first_recurrence.is_none() && self.location_log.contains(&coord) {
// //                 self.first_recurrence = Some(coord);
// //             }
// //             self.location_log.insert(self.location.coordinate);
// //         }
// //     }
// //
// //     fn run(&mut self, list: Vec<Instruction>) {
// //         list.into_iter().for_each(
// //             |instr| self.execute_and_log(instr),
// //         )
// //     }
// //
// //     fn distance_to_origin(&self) -> i32 {
// //         self.location.coordinate.norm1()
// //     }
// //
// //     fn bunny_hq(&self) -> Option<Coordinate> {
// //         self.first_recurrence
// //     }
// // }
// // impl fmt::Display for Sleigh {
// //     fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
// //         match self.first_recurrence {
// //             Some(house) => {
// //                 write!(
// //                     f,
// //                     "Sleigh at {}, first house visited twice: {}",
// //                     self.location,
// //                     house
// //                 )
// //             }
// //             None => {
// //                 write!(
// //                     f,
// //                     "Sleigh at {}, no house visited twice (yet)",
// //                     self.location
// //                 )
// //             }
// //         }
// //     }
// // }
//
//
//
// #[cfg(test)]
// mod tests {
//     use super::*;
//
//     fn sleigh_after_run(list: Vec<Instruction>) -> Sleigh {
//         let mut sleigh = Sleigh::new();
//         sleigh.run(list);
//         sleigh
//     }
//
//     #[test]
//     fn test_distance_after_run1() {
//         let list = vec![(Right, 2), (Left, 3)];
//         assert_eq!(5, sleigh_after_run(list).distance_to_origin());
//     }
//
//     #[test]
//     fn test_distance_after_run2() {
//         let list = vec![(Right, 2), (Right, 2), (Right, 2)];
//         assert_eq!(2, sleigh_after_run(list).distance_to_origin());
//     }
//
//     #[test]
//     fn test_distance_after_run3() {
//         let list = vec![(Right, 5), (Left, 5), (Right, 5), (Right, 3)];
//         assert_eq!(12, sleigh_after_run(list).distance_to_origin());
//     }
//
//     #[test]
//     fn test_first_recurrence() {
//         let list = vec![(Right, 8), (Right, 4), (Right, 4), (Right, 8)];
//         let coord = Coordinate::new(4, 0);
//         assert_eq!(Some(coord), sleigh_after_run(list).first_recurrence);
//     }
// }
