#![feature(exclusive_range_pattern)]
extern crate crypto;
#[macro_use]
extern crate lazy_static;
extern crate regex;

mod aoc_2016;

const INPUT_2016_01: &'static str = include_str!("../inputs/2016_01.txt");
const INPUT_2016_02: &'static str = include_str!("../inputs/2016_02.txt");
const INPUT_2016_03: &'static str = include_str!("../inputs/2016_03.txt");
const INPUT_2016_04: &'static str = include_str!("../inputs/2016_04.txt");
const INPUT_2016_05: &'static str = include_str!("../inputs/2016_05.txt");
// const INPUT_2016_06: &'static str = include_str!("../inputs/2016_06.txt");
// const INPUT_2016_07: &'static str = include_str!("../inputs/2016_07.txt");
// const INPUT_2016_08: &'static str = include_str!("../inputs/2016_08.txt");
// const INPUT_2016_09: &'static str = include_str!("../inputs/2016_09.txt");
// const INPUT_2016_10: &'static str = include_str!("../inputs/2016_10.txt");
// const INPUT_2016_11: &'static str = include_str!("../inputs/2016_11.txt");
// const INPUT_2016_12: &'static str = include_str!("../inputs/2016_12.txt");
// const INPUT_2016_13: &'static str = include_str!("../inputs/2016_13.txt");
// const INPUT_2016_14: &'static str = include_str!("../inputs/2016_14.txt");
// const INPUT_2016_15: &'static str = include_str!("../inputs/2016_15.txt");
// const INPUT_2016_16: &'static str = include_str!("../inputs/2016_16.txt");
// const INPUT_2016_17: &'static str = include_str!("../inputs/2016_17.txt");
// const INPUT_2016_18: &'static str = include_str!("../inputs/2016_18.txt");
// const INPUT_2016_19: &'static str = include_str!("../inputs/2016_19.txt");
// const INPUT_2016_20: &'static str = include_str!("../inputs/2016_20.txt");
// const INPUT_2016_21: &'static str = include_str!("../inputs/2016_21.txt");
// const INPUT_2016_22: &'static str = include_str!("../inputs/2016_22.txt");
// const INPUT_2016_23: &'static str = include_str!("../inputs/2016_23.txt");
// const INPUT_2016_24: &'static str = include_str!("../inputs/2016_24.txt");
// const INPUT_2016_25: &'static str = include_str!("../inputs/2016_25.txt");


pub fn main() {
    aoc_2016::day01::report(INPUT_2016_01);
    aoc_2016::day02::report(INPUT_2016_02);
    aoc_2016::day03::report(INPUT_2016_03);
    aoc_2016::day04::report(INPUT_2016_04);
    aoc_2016::day05::report(INPUT_2016_05);
}
