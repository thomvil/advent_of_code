#![feature(exclusive_range_pattern)]
#[macro_use]
extern crate lazy_static;
extern crate regex;

use input_parser::*;
use std::collections::HashMap;
use std::str;

fn main() {
    println!(
        "The sum of sector ids for the valid rooms is {:?}",
        sum_of_sector_ids()
    );
    println!(
        "The same sum, by calculating checksums, is   {:?}",
        sum_of_sector_ids2()
    );

    match north_pole_object_storage() {
        Some(sid) => println!("North pole object storage is found at sector {}", sid),
        None => println!("North pole object storage could not be found!"),
    }
}

fn north_pole_object_storage() -> Option<u32> {
    for room in instructions().iter().filter(|room| room.is_valid()) {
        if room.decrypt() == "northpole object storage" {
            return Some(room.sector_id);
        }
    }
    None
}

fn sum_of_sector_ids() -> u32 {
    instructions().iter().filter(|room| room.is_valid()).fold(
        0,
        |acc, room| acc + room.sector_id,
    )
}

fn sum_of_sector_ids2() -> u32 {
    instructions()
        .iter()
        .filter(|room| room.is_valid_by_checksum())
        .fold(0, |acc, room| acc + room.sector_id)
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct CharStats {
    counts: HashMap<char, usize>,
}
impl CharStats {
    fn new() -> CharStats {
        CharStats { counts: HashMap::new() }
    }

    fn log(&mut self, c: char) {
        if c != '-' {
            *self.counts.entry(c).or_insert(0) += 1
        }
    }

    fn log_str(&mut self, word: &str) {
        for c in word.chars() {
            self.log(c)
        }
    }

    fn to_vec(&self) -> Vec<(char, usize)> {
        let mut res: Vec<(char, usize)> = self.counts.iter().map(|(k, v)| (*k, *v)).collect();
        res.sort_by_key(|&(c, _nb)| c);
        res.reverse();
        res.sort_by_key(|&(_c, nb)| nb);
        res.reverse();
        res
    }

    fn shift_byte(b: u8, amount: u32) -> Option<u8> {
        let shift_amount: u8 = (amount % 26u32) as u8;
        match b {
            45 => Some(32),
            97..123 => {
                let tmp = b + shift_amount;
                if tmp < 123 {
                    Some(tmp)
                } else {
                    Some(tmp - 122 + 97 - 1)
                }
            }
            _ => None,
        }
    }
}

#[derive(Clone, Debug, Hash, PartialEq, Eq)]
pub struct Room {
    name: String,
    sector_id: u32,
    checksum: String,
}
impl Room {
    fn new(name: String, sector_id: u32, checksum: String) -> Room {
        Room {
            name: name,
            sector_id: sector_id,
            checksum: checksum,
        }
    }

    fn occurences(&self, letter: &char) -> usize {
        self.name.chars().filter(|c| c == letter).count()
    }

    fn is_valid(&self) -> bool {
        let check_chars: Vec<char> = self.checksum.chars().collect();
        for c_idx in 0..check_chars.iter().len() - 1 {
            if !(self.occurences(&check_chars[c_idx]) > self.occurences(&check_chars[c_idx + 1]) ||
                     (self.occurences(&check_chars[c_idx]) ==
                          self.occurences(&check_chars[c_idx + 1]) &&
                          check_chars[c_idx] <= check_chars[c_idx + 1])) ||
                self.occurences(&check_chars[c_idx + 1]) < 1
            {
                return false;
            }
        }
        true
    }

    fn is_valid_by_checksum(&self) -> bool {
        self.checksum == self.calculate_checksum()
    }

    fn calculate_checksum(&self) -> String {
        let mut cs = CharStats::new();
        cs.log_str(&self.name);
        cs.to_vec()
            .iter()
            .cloned()
            .take(5)
            .map(|(c, _nb)| c)
            .collect()
    }

    fn decrypt(&self) -> String {
        let shifted_bytes: Vec<u8> = self.name
            .bytes()
            .filter_map(|b| CharStats::shift_byte(b, self.sector_id))
            .collect();
        str::from_utf8(&shifted_bytes).unwrap().to_string()
    }
}

mod input_parser {
    use regex::Regex;
    use Room;

    lazy_static! {
        static ref ROOM_RE: Regex = Regex::new(r"([\w-]+)-(\d+)\[(\w+)\]").unwrap();
    }
    const INPUT: &'static str = include_str!("../inputs/day04.txt");

    pub fn parse_room(room_str: &str) -> Option<Room> {
        ROOM_RE.captures(room_str).map(|cap| {
            Room::new(
                cap[1].to_string(),
                cap[2].parse().unwrap(),
                cap[3].to_string(),
            )
        })
    }

    pub fn parse_instructions(instr: &str) -> Vec<Room> {
        instr.lines().filter_map(|line| parse_room(line)).collect()
    }

    pub fn instructions() -> Vec<Room> {
        parse_instructions(INPUT)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_valid_rooms() {
        let valid_rooms_str = "aaaaa-bbb-z-y-x-123[abxyz]\na-b-c-d-e-f-g-h-987[abcde]\nnot-a-real-room-404[oarel]";
        let valid_rooms = &parse_instructions(valid_rooms_str);
        assert_eq!(true, valid_rooms.iter().all(|room| room.is_valid()));
    }

    #[test]
    fn test_decoy_rooms() {
        let invalid_rooms_str = "totally-real-room-200[decoy]\nkdijqrbu-jef-iushuj-sqdto-seqjydw-kiuh-juijydw-218[iqtvx]";
        let invalid_rooms = &parse_instructions(invalid_rooms_str);
        assert_eq!(false, invalid_rooms.iter().all(|room| room.is_valid()));
    }

    #[test]
    fn test_decryption() {
        let room = parse_room("qzmt-zixmtkozy-ivhz-343[abc]");
        assert_eq!("very encrypted name", room.unwrap().decrypt());
    }
}
