use aoc_2016::day02::Direction;
use aoc_2016::day02::KeyPadCoordinate;
use std::collections::HashMap;

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct KeyPad {
    buttons: HashMap<KeyPadCoordinate, char>,
    cursor: KeyPadCoordinate,
    pub code: String,
}

impl KeyPad {
    pub fn cube() -> KeyPad {
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

    pub fn star() -> KeyPad {
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
        let key = &self.buttons[&self.cursor];
        self.code.push(*key);
    }

    fn move_cursor(&mut self, dir: Direction) {
        let ncursor = self.cursor.shift(dir);
        if self.buttons.contains_key(&ncursor) {
            self.cursor = ncursor;
        }
    }

    pub fn execute(&mut self, list: &[Direction]) {
        for dir in list {
            self.move_cursor(*dir)
        }
        self.save_key();
    }

    pub fn run(&mut self, instructions: &[Vec<Direction>]) {
        for list in instructions {
            self.execute(list);
        }
    }
}
