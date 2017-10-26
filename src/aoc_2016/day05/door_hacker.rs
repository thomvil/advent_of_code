use crypto::digest::Digest;
use crypto::md5::Md5;

pub struct DoorHacker<'a> {
    door_id: &'a str,
    hasher: Md5,
}
<<<<<<< HEAD:aoc_2016/src/day05.rs

pub struct DoorHacker<'a> {
    door_id: &'a str,
    hasher: Md5,
}
impl<'a> DoorHacker<'a> {
    fn new(id: &'a str) -> DoorHacker<'a> {
=======
impl<'a> DoorHacker<'a> {
    pub fn new(id: &'a str) -> DoorHacker<'a> {
>>>>>>> fcb99f31369d337b5068c9e32ecf728b9ddf9ce8:src/aoc_2016/day05/door_hacker.rs
        DoorHacker {
            door_id: id,
            hasher: Md5::new(),
        }
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
            _ => None,
        }
    }

    fn code_digit_for_index_v2(&mut self, idx: u32) -> Option<(usize, char)> {
        let hash_output = self.hash(idx);
        match &hash_output[0..5] {
            "00000" => {
                let location_char = hash_output.chars().nth(5).unwrap();
                let location_digit: usize =
                    location_char.to_string().parse().ok().or(Some(8)).unwrap();
                if location_digit < 8 {
                    let code_char = hash_output.chars().nth(6).unwrap();
                    Some((location_digit, code_char))
                } else {
                    None
                }
            }
            _ => None,
        }
    }

    pub fn door_code_v2(&mut self) -> String {
        let mut res = vec![None; 8];
        let mut idx = 1;
        while res.iter().any(|digits| digits.is_none()) {
<<<<<<< HEAD:aoc_2016/src/day05.rs
            self.code_digit_for_index_v2(idx).map(|(i, c)| {
                res[i] = res[i].or(Some(c))
            });
=======
            self.code_digit_for_index_v2(idx)
                .map(|(i, c)| res[i] = res[i].or_else(|| Some(c)));
>>>>>>> fcb99f31369d337b5068c9e32ecf728b9ddf9ce8:src/aoc_2016/day05/door_hacker.rs
            idx += 1;
        }
        res.iter().map(|opt| opt.unwrap()).collect()
    }

    pub fn door_code(&mut self) -> String {
        let mut idx = 1;
        let mut res = String::new();
        while res.len() < 8 {
            self.code_digit_for_index(idx).map(|c| res.push(c));
            idx += 1;
        }
        res
    }
}
