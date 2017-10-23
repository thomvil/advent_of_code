lazy_static! {
    static ref INSTR_RE: Regex = Regex::new(r"(L|R)(\d+)").unwrap();
}

use Instruction;
use regex::Regex;
use Turn;

pub fn parse_instructions(instructions: &str) -> Vec<Instruction> {
    instructions
        .split(", ")
        .filter_map(|instr_str| parse_instruction(instr_str))
        .collect()
}

pub fn parse_instruction(instr_str: &str) -> Option<Instruction> {
    INSTR_RE.captures(instr_str).map(|cap| {
        let turn = parse_turn(&cap[1]).unwrap();
        let offset: i32 = cap[2].parse().unwrap();
        (turn, offset)
    })
}

pub fn parse_turn(turn_str: &str) -> Option<Turn> {
    match turn_str {
        "R" => Some(Turn::Right),
        "L" => Some(Turn::Left),
        _ => None,
    }
}
