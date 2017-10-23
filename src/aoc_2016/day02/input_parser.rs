use aoc_2016::day02::Direction;
use aoc_2016::day02::Direction::*;

pub fn parse_instructions(instruction_str: &str) -> Vec<Vec<Direction>> {
    instruction_str.lines().map(|instr_str| parse_instruction(instr_str)).collect()
}

fn parse_instruction(instr: &str) -> Vec<Direction> {
    instr.split("").filter_map(|dir_str| parse_direction(dir_str)).collect()
}

fn parse_direction(dir_str: &str) -> Option<Direction> {
    match dir_str {
        "U" => Some(Up),
        "L" => Some(Left),
        "D" => Some(Down),
        "R" => Some(Right),
        _   => None
    }
}
