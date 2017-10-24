use aoc_2016::day04::char_stats::CharStats;
use std::str;

#[derive(Clone, Debug, Hash, PartialEq, Eq)]
pub struct Room {
    name: String,
    pub sector_id: u32,
    checksum: String,
}

impl Room {
    pub fn new(name: String, sector_id: u32, checksum: String) -> Room {
        Room {
            name: name,
            sector_id: sector_id,
            checksum: checksum,
        }
    }

    fn occurences(&self, letter: &char) -> usize {
        self.name.chars().filter(|c| c == letter).count()
    }

    // TODO: refactor check and replace loop with `any`-iterator
    pub fn is_valid(&self) -> bool {
        let check_chars: Vec<char> = self.checksum.chars().collect();
        for c_idx in 0..check_chars.iter().len() - 1 {
            if !(self.occurences(&check_chars[c_idx]) > self.occurences(&check_chars[c_idx + 1])
                || (self.occurences(&check_chars[c_idx]) == self.occurences(&check_chars[c_idx + 1])
                    && check_chars[c_idx] <= check_chars[c_idx + 1]))
                || self.occurences(&check_chars[c_idx + 1]) < 1
            {
                return false;
            }
        }
        true
    }

    pub fn is_valid_by_checksum(&self) -> bool {
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

    pub fn decrypt(&self) -> String {
        let shifted_bytes: Vec<u8> = self.name
            .bytes()
            .filter_map(|b| CharStats::shift_byte(b, self.sector_id))
            .collect();
        str::from_utf8(&shifted_bytes).unwrap().to_string()
    }
}
