mod input_parser;

use aoc_2016::day12::input_parser::*;

#[derive(Debug)]
pub enum Register {
    A,
    B,
    C,
    D,
}

#[derive(Debug)]
pub enum Target {
    Register(Register),
    Value(i32),
}

#[derive(Debug)]
pub enum Instruction {
    Cpy(Target, Target),
    JumpIfNotZero(Target, Target),
    Increment(Target),
    Decrement(Target),
}

pub fn report(instr: &str) {
    println!(" -- 2016: Day 12 -- ");
    println!("| ");
    println!("{:?}", parse_target("a"));
    println!("{:?}", parse_target("-26"));
    println!("{:?}", parse_target("_"));
    println!(" ------------------");
}
