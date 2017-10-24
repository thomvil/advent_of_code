mod magnetic_strip;
mod input_parser;

use aoc_2016::day08::Instruction::*;
use aoc_2016::day08::input_parser::*;
use aoc_2016::day08::magnetic_strip::MagneticStrip;

#[derive(Debug)]
pub enum Instruction {
    Rect(usize, usize),
    RotateRow(usize, u32),
    RotateColumn(usize, u32),
}

pub fn report(instr: &str) {
    println!(" -- 2016: Day 8 -- ");
    let mut ms = MagneticStrip::new(6, 50);
    ms.run(&parse_instructions(instr));
    println!("| The display has {} lights lit.", ms.brightness());
    println!("| The message reads:");
    ms.print();
    println!(" ------------------");
}
