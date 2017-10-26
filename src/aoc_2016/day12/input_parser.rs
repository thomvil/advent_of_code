lazy_static! {
    static ref INSTR1_RE: Regex = Regex::new(r"(cpy|jnz) ([abcd]|-?\d+) ([abcd]|-?\d+)").unwrap();
    static ref INSTR2_RE: Regex = Regex::new(r"(inc|dec) ([abcd])").unwrap();
}

use aoc_2016::day12::Instruction;
use aoc_2016::day12::Register::*;
use aoc_2016::day12::Target;
use aoc_2016::day12::Target::*;
use regex::Regex;

pub fn parse_instruction(instr: &str) -> Option<Instruction> {
    // INSTR1_RE.captures(instr).map(|cap|
    //     match cap.get(1).unwrap() {
    //         "cpy" =>
    //     }
    // );
    None
}

pub fn parse_target(target: &str) -> Option<Target> {
    match (target, target.parse()) {
        ("a", _) => Some(Register(A)),
        ("b", _) => Some(Register(B)),
        ("c", _) => Some(Register(C)),
        ("d", _) => Some(Register(D)),
        (_, Ok(val)) => Some(Value(val)),
        _ => None,
    }
}
