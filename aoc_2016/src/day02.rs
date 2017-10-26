use Direction::*;
use std::collections::HashMap;
use input_parser::*;

fn main() {
    let mut cube_pad = KeyPad::cube();
    cube_pad.run(instructions());
    println!(
        "The bathroom code for the cube keypad is: {}",
        cube_pad.code.as_str()
    );
    let mut star_pad = KeyPad::star();
    star_pad.run(instructions());
    println!(
        "The bathroom code for the star keypad is: {}",
        star_pad.code.as_str()
    );
}

#[derive(Copy, Clone, Debug, Hash, PartialEq, Eq)]
pub enum Direction {
    Up,
    Left,
    Down,
    Right,
}

#[derive(Copy, Clone, Debug, Hash, PartialEq, Eq)]
struct KeyPadCoordinate {
    x: i8,
    y: i8,
}
impl KeyPadCoordinate {
    fn new(x: i8, y: i8) -> KeyPadCoordinate {
        KeyPadCoordinate { x: x, y: y }
    }

    fn origin() -> KeyPadCoordinate {
        KeyPadCoordinate { x: 0, y: 0 }
    }

    fn shift(&self, dir: Direction) -> KeyPadCoordinate {
        match dir {
            Up => KeyPadCoordinate::new(self.x, self.y + 1),
            Left => KeyPadCoordinate::new(self.x - 1, self.y),
            Down => KeyPadCoordinate::new(self.x, self.y - 1),
            Right => KeyPadCoordinate::new(self.x + 1, self.y),
        }
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
struct KeyPad {
    buttons: HashMap<KeyPadCoordinate, char>,
    cursor: KeyPadCoordinate,
    code: String,
}
impl KeyPad {
    fn cube() -> KeyPad {
        let mut buttons: HashMap<KeyPadCoordinate, char> = HashMap::new();
        buttons.insert(KeyPadCoordinate::new(-1, 1), '1');
        buttons.insert(KeyPadCoordinate::new(0, 1), '2');
        buttons.insert(KeyPadCoordinate::new(1, 1), '3');
        buttons.insert(KeyPadCoordinate::new(-1, 0), '4');
        buttons.insert(KeyPadCoordinate::new(0, 0), '5');
        buttons.insert(KeyPadCoordinate::new(1, 0), '6');
        buttons.insert(KeyPadCoordinate::new(-1, -1), '7');
        buttons.insert(KeyPadCoordinate::new(0, -1), '8');
        buttons.insert(KeyPadCoordinate::new(1, -1), '9');
        KeyPad {
            buttons: buttons,
            cursor: KeyPadCoordinate::origin(),
            code: String::new(),
        }
    }

    fn star() -> KeyPad {
        let mut buttons: HashMap<KeyPadCoordinate, char> = HashMap::new();
        buttons.insert(KeyPadCoordinate::new(2, 2), '1');
        buttons.insert(KeyPadCoordinate::new(1, 1), '2');
        buttons.insert(KeyPadCoordinate::new(2, 1), '3');
        buttons.insert(KeyPadCoordinate::new(3, 1), '4');
        buttons.insert(KeyPadCoordinate::new(0, 0), '5');
        buttons.insert(KeyPadCoordinate::new(1, 0), '6');
        buttons.insert(KeyPadCoordinate::new(2, 0), '7');
        buttons.insert(KeyPadCoordinate::new(3, 0), '8');
        buttons.insert(KeyPadCoordinate::new(4, 0), '9');
        buttons.insert(KeyPadCoordinate::new(1, -1), 'A');
        buttons.insert(KeyPadCoordinate::new(2, -1), 'B');
        buttons.insert(KeyPadCoordinate::new(3, -1), 'C');
        buttons.insert(KeyPadCoordinate::new(2, -2), 'D');
        KeyPad {
            buttons: buttons,
            cursor: KeyPadCoordinate::origin(),
            code: String::new(),
        }
    }

    fn save_key(&mut self) {
        let key = *self.buttons.get(&self.cursor).unwrap();
        self.code.push(key);
    }

    fn move_cursor(&mut self, dir: Direction) {
        let ncursor = self.cursor.shift(dir);
        if self.buttons.contains_key(&ncursor) {
            self.cursor = ncursor;
        }
    }

    fn execute(&mut self, list: Vec<Direction>) {
        for dir in list {
            self.move_cursor(dir)
        }
        self.save_key();
    }

    fn run(&mut self, instructions: Vec<Vec<Direction>>) {
        for list in instructions {
            self.execute(list);
        }
    }
}

mod input_parser {
    use Direction;
    use Direction::*;

    const INPUT: &str = include_str!("../inputs/day02.txt");

    pub fn instructions() -> Vec<Vec<Direction>> {
        INPUT
            .lines()
            .map(|instr_str| parse_instruction(instr_str))
            .collect()
    }

    pub fn parse_instructions(instruction_str: &str) -> Vec<Vec<Direction>> {
        instruction_str
            .lines()
            .map(|instr_str| parse_instruction(instr_str))
            .collect()
    }

    fn parse_instruction(instr: &str) -> Vec<Direction> {
        instr
            .split("")
            .filter_map(|dir_str| parse_direction(dir_str))
            .collect()
    }

    fn parse_direction(dir_str: &str) -> Option<Direction> {
        match dir_str {
            "U" => Some(Up),
            "L" => Some(Left),
            "D" => Some(Down),
            "R" => Some(Right),
            _ => None,
        }
    }
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
