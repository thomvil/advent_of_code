use std::num::*;

pub fn parse_instructions_vertically(instruction_str: &str) -> Vec<Vec<u32>> {
    let hor = parse_instructions_horizontally(instruction_str);
    let rows = hor.iter().len();
    if rows % 3 != 0 {
        return Vec::new();
    }
    let mut res: Vec<Vec<u32>> = Vec::new();
    for col in 0..3 {
        for row in 0..rows / 3 {
            res.push(vec![
                hor[0 + 3 * row][col],
                hor[1 + 3 * row][col],
                hor[2 + 3 * row][col],
            ]);
        }
    }
    res
}

pub fn parse_instructions_horizontally(instruction_str: &str) -> Vec<Vec<u32>> {
    instruction_str
        .lines()
        .filter_map(|instr_str| parse_instruction(instr_str).ok())
        .filter(|sides| sides.iter().len() == 3)
        .collect()
}

pub fn parse_instruction(instr: &str) -> Result<Vec<u32>, ParseIntError> {
    instr
        .split_whitespace()
        .map(|nb_str| nb_str.parse::<u32>())
        .collect()
}
