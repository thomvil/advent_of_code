lazy_static! {
    static ref RECT_RE: Regex = Regex::new(r"^rect (\d+)x(\d+)$").unwrap();
    static ref ROTATE_ROW_RE: Regex = Regex::new(r"^rotate row y=(\d+) by (\d+)$").unwrap();
    static ref ROTATE_COL_RE: Regex = Regex::new(r"^rotate column x=(\d+) by (\d+)$").unwrap();
}

use aoc_2016::day08::Instruction;
use aoc_2016::day08::Instruction::*;
use regex::Regex;

fn parse_rect(instr: &str) -> Option<Instruction> {
    RECT_RE
        .captures(instr)
        .map(|cap| Rect(cap[1].parse().unwrap(), cap[2].parse().unwrap()))
}

fn parse_rotate_row(instr: &str) -> Option<Instruction> {
    ROTATE_ROW_RE.captures(instr).map(|cap| {
        RotateRow(cap[1].parse().unwrap(), cap[2].parse().unwrap())
    })
}

fn parse_rotate_column(instr: &str) -> Option<Instruction> {
    ROTATE_COL_RE.captures(instr).map(|cap| {
        RotateColumn(cap[1].parse().unwrap(), cap[2].parse().unwrap())
    })
}

pub fn parse_instruction(instr: &str) -> Option<Instruction> {
    parse_rect(instr)
        .or_else(|| parse_rotate_row(instr))
        .or_else(|| parse_rotate_column(instr))
}

pub fn parse_instructions(instr: &str) -> Vec<Instruction> {
    instr
        .lines()
        .filter_map(|line| parse_instruction(line))
        .collect()
}
