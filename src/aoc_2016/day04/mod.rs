mod char_stats;
mod input_parser;
mod room;

use aoc_2016::day04::input_parser::*;
use aoc_2016::day04::room::Room;

pub fn report(instr: &str) {
    let list = parse_instructions(instr);
    println!(" -- 2016: Day 4 -- ");
    println!(
        "| The sum of sector ids for the valid rooms is {:?}",
        sum_of_sector_ids(&list)
    );
    println!(
        "| The same sum, by calculating checksums, is   {:?}",
        sum_of_sector_ids2(&list)
    );
    match north_pole_object_storage(&list) {
        Some(sid) => println!("| North pole object storage is found at sector {}", sid),
        None => println!("| North pole object storage could not be found!"),
    }
    println!(" ------------------");
}

fn north_pole_object_storage(rooms: &[Room]) -> Option<u32> {
    for room in rooms.iter().filter(|room| room.is_valid()) {
        if room.decrypt() == "northpole object storage" {
            return Some(room.sector_id);
        }
    }
    None
}

fn sum_of_sector_ids(rooms: &[Room]) -> u32 {
    rooms
        .iter()
        .filter(|room| room.is_valid())
        .fold(0, |acc, room| acc + room.sector_id)
}

fn sum_of_sector_ids2(rooms: &[Room]) -> u32 {
    rooms
        .iter()
        .filter(|room| room.is_valid_by_checksum())
        .fold(0, |acc, room| acc + room.sector_id)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_valid_rooms() {
        let valid_rooms_str =
            "aaaaa-bbb-z-y-x-123[abxyz]\na-b-c-d-e-f-g-h-987[abcde]\nnot-a-real-room-404[oarel]";
        let valid_rooms = &parse_instructions(valid_rooms_str);
        assert_eq!(true, valid_rooms.iter().all(|room| room.is_valid()));
    }

    #[test]
    fn test_decoy_rooms() {
        let invalid_rooms_str =
            "totally-real-room-200[decoy]\nkdijqrbu-jef-iushuj-sqdto-seqjydw-kiuh-juijydw-2[iqtvx]";
        let invalid_rooms = &parse_instructions(invalid_rooms_str);
        assert_eq!(false, invalid_rooms.iter().all(|room| room.is_valid()));
    }

    #[test]
    fn test_decryption() {
        let room = parse_room("qzmt-zixmtkozy-ivhz-343[abc]");
        assert_eq!("very encrypted name", room.unwrap().decrypt());
    }
}
