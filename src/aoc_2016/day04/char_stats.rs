use std::collections::HashMap;

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct CharStats {
    counts: HashMap<char, usize>,
}

impl CharStats {
    pub fn new() -> CharStats {
        CharStats {
            counts: HashMap::new(),
        }
    }

    fn log(&mut self, c: char) {
        if c != '-' {
            *self.counts.entry(c).or_insert(0) += 1
        }
    }

    pub fn log_str(&mut self, word: &str) {
        word.chars().for_each(|c| self.log(c));
    }

    pub fn to_vec(&self) -> Vec<(char, usize)> {
        let mut res: Vec<(char, usize)> = self.counts.iter().map(|(k, v)| (*k, *v)).collect();
        res.sort_by_key(|&(c, _nb)| c);
        res.reverse();
        res.sort_by_key(|&(_c, nb)| nb);
        res.reverse();
        res
    }

    pub fn shift_byte(b: u8, amount: u32) -> Option<u8> {
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
