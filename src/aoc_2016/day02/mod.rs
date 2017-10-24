mod input_parser;
mod keypad;
mod keypad_coordinate;

use aoc_2016::day02::keypad_coordinate::KeyPadCoordinate;
use aoc_2016::day02::keypad::KeyPad;


#[derive(Copy, Clone, Debug, Hash, PartialEq, Eq)]
pub enum Direction {
    Up,
    Left,
    Down,
    Right,
}

pub fn report(instructions: &str) {
    let instr = input_parser::parse_instructions(instructions);
    println!(" -- 2016: Day 2 -- ");
    let mut cube_pad = KeyPad::cube();
    cube_pad.run(&instr);
    let mut star_pad = KeyPad::star();
    star_pad.run(&instr);
    println!(
        "| The bathroom code for the cube keypad is: {}",
        cube_pad.code.as_str()
    );
    println!(
        "| The bathroom code for the star keypad is: {}",
        star_pad.code.as_str()
    );
    println!(" ------------------");
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST_INPUT: &str = "ULL\nRRDDD\nLURDL\nUUUUD";

    #[test]
    fn test_one_key_on_cube() {
        let list = vec![Up, Left, Left];
        let mut pad = KeyPad::cube();
        pad.execute(list);
        assert_eq!("1", pad.code.as_str());
    }

    #[test]
    fn test_input_on_cube() {
        let mut pad = KeyPad::cube();
        pad.run(test_input());
        assert_eq!("1985", pad.code.as_str());
    }

    #[test]
    fn test_input_on_star() {
        let mut pad = KeyPad::star();
        pad.run(test_input());
        assert_eq!("5DB3", pad.code.as_str());
    }

    fn test_input() -> Vec<Vec<Direction>> {
        parse_instructions(TEST_INPUT)
    }
}
