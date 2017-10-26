use regex::Regex;
use aoc_2016::day04::room::Room;

lazy_static! {
    static ref ROOM_RE: Regex = Regex::new(r"([\w-]+)-(\d+)\[(\w+)\]").unwrap();
}

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
