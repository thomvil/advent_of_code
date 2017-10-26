mod marker_processor;

use aoc_2016::day09::marker_processor::*;
use colored::*;

pub fn report(instr: &str) {
    println!(" -- 2016: Day 9 -- ");
    println!(
        "| Decompressed length of file is:        {}",
        decompressed_length(instr)
    );
    println!(
        "| Fullly decompressed length of file is: {}",
        "not yet implemented!".red()
    );
    println!(" ------------------");
}
