lazy_static! {
    static ref INSTR_RE: Regex = Regex::new(r"(L|R)(\d+)").unwrap();
    static ref INPUT: &'static str = include_str!("../../../inputs/day01.txt");
}

use Instruction;
use Turn;
use Turn::*;
use regex::Regex;

pub fn instructions() -> Vec<Instruction> {
    INPUT
        .split(", ")
        .filter_map(|instr_str| parse_instruction(instr_str))
        .collect()
}

pub fn parse_instructions(instructions: &str) -> Vec<Instruction> {
    instructions
        .split(", ")
        .filter_map(|instr_str| parse_instruction(instr_str))
        .collect()
}

pub fn parse_instruction(instr_str: &str) -> Option<Instruction> {
    INSTR_RE.captures(instr_str).map(|cap| {
        let turn = parse_turn(&cap[1]).unwrap();
        let offset = cap[2].parse().unwrap();
        (turn, offset)
    })
}

pub fn parse_turn(turn_str: &str) -> Option<Turn> {
    match turn_str {
        "R" => Some(Right),
        "L" => Some(Left),
        _ => None,
    }
}
