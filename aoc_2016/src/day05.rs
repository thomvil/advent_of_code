extern crate crypto;

use crypto::digest::Digest;
use crypto::md5::Md5;

const INPUT: &str = include_str!("../inputs/day05.txt");

fn main() {
    // let mut dh = DoorHacker::new(INPUT.trim());
    // println!("Door code is: {:?}", dh.door_code());
    // println!("Door code is: {:?}", dh.door_code_v2());

    let mut dh = DoorHacker::new("abc");
    let code = dh.door_code();
    // println!("Door code is: {:?}", dh.door_code_v2());
    // let mut pair = dh.code_digit_for_index_v2(3231929);
    // println!("{:?}", pair);
    //
    // let mut res = vec![None; 8];
    // pair.map(|(i, c)| res[i] = res[i].or(Some(c)));
    // println!("{:?}", res);
    // pair = Some((1, 'a'));
    // pair.map(|(i, c)| res[i] = res[i].or(Some(c)));
    // println!("{:?}", res);
}

pub struct DoorHacker<'a> { door_id: &'a str, hasher: Md5 }
impl<'a> DoorHacker<'a> {
    fn new(id: &'a str) -> DoorHacker<'a> {
        DoorHacker { door_id: id, hasher: Md5::new() }
    }

    fn hash(&mut self, idx: u32) -> String {
        let input = format!("{}{}", self.door_id, idx);
        self.hasher.input_str(&input);
        let res = self.hasher.result_str();
        self.hasher.reset();
        res
    }

    fn code_digit_for_index(&mut self, idx: u32) -> Option<char> {
        let hash_output = self.hash(idx);
        match &hash_output[0..5] {
            "00000" => Some(hash_output.chars().nth(5).unwrap()),
            _       => None
        }
    }

    fn code_digit_for_index_v2(&mut self, idx: u32) -> Option<(usize, char)> {
        let hash_output = self.hash(idx);
        match &hash_output[0..5] {
            "00000" => {
                let location_char = hash_output.chars().nth(5).unwrap();
                let location_digit: usize = location_char.to_string().parse().ok().or(Some(8)).unwrap();
                if location_digit < 8 {
                    let code_char = hash_output.chars().nth(6).unwrap();
                    Some((location_digit, code_char))
                } else {
                    None
                }
            },
            _       => None
        }
    }

    fn door_code_v2(&mut self) -> String {
        let mut res = vec![None; 8];
        let mut idx = 1;
        while res.iter().any(|digits| digits.is_none()) {
            self.code_digit_for_index_v2(idx).map(|(i, c)| res[i] = res[i].or(Some(c)));
        }
        res.iter().map(|opt| opt.unwrap()).collect()
    }

    fn door_code(&mut self) -> String {
        let mut idx = 1;
        let mut res = String::new();
        while res.len() < 8 {
            self.code_digit_for_index(idx).map(|c| res.push(c));
            idx += 1;
        }
        res
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    // #[test]
    fn test_doorcode() {
        let mut dh = DoorHacker::new("abc");
        assert_eq!("18f47a30", dh.door_code());
    }
}
