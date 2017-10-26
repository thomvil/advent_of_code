mod door_hacker;

use aoc_2016::day05::door_hacker::DoorHacker;

pub fn report(instr: &str) {
    println!(" -- 2016: Day 5 -- ");
    let mut dh = DoorHacker::new(instr.trim());
    println!("| Door code is:          {:?}", dh.door_code());
    println!("| Improved door code is: {:?}", dh.door_code_v2());
    println!(" ------------------");
}

#[cfg(test)]
mod tests {
    use super::*;

    // #[test]
    fn test_doorcode() {
        let mut dh = DoorHacker::new("abc");
        assert_eq!("18f47a30", dh.door_code());
    }

    // #[test]
    fn test_doorcode_v2() {
        let mut dh = DoorHacker::new("abc");
        assert_eq!("05ace8e3", dh.door_code_v2());
    }
}
