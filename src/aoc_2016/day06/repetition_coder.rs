use std::cmp;
use std::collections::HashMap;

#[derive(Debug)]
pub struct RepetitionCoder {
    stats: HashMap<(usize, char), u32>,
}

impl RepetitionCoder {
    pub fn new() -> RepetitionCoder {
        RepetitionCoder {
            stats: HashMap::new(),
        }
    }

    pub fn recover_msg_v1(&self) -> String {
        (0..self.last_column() + 1)
            .map(|col_idx| self.most_likely_char_for_column(col_idx))
            .collect()
    }

    pub fn recover_msg_v2(&self) -> String {
        (0..self.last_column() + 1)
            .map(|col_idx| self.least_likely_char_for_column(col_idx))
            .collect()
    }

    fn last_column(&self) -> usize {
        self.stats
            .keys()
            .fold(0usize, |acc, &(col_idx, _c)| cmp::max(acc, col_idx))
    }

    fn most_likely_char_for_column(&self, col_idx: usize) -> char {
        self.stats
            .iter()
            .filter(|&(&(col, _c), _count)| col_idx == col)
            .max_by_key(|&(&(_col, _c), count)| count)
            .map(|(&(_col, c), _count)| c)
            .unwrap_or(' ')
    }

    fn least_likely_char_for_column(&self, col_idx: usize) -> char {
        self.stats
            .iter()
            .filter(|&(&(col, _c), _count)| col_idx == col)
            .min_by_key(|&(&(_col, _c), count)| count)
            .map(|(&(_col, c), _count)| c)
            .unwrap_or(' ')
    }

    pub fn parse(&mut self, input_str: &str) {
        input_str.lines().for_each(|line| self.parse_line(line));
    }

    fn parse_line(&mut self, input_line: &str) {
        input_line
            .chars()
            .enumerate()
            .for_each(|(i, c)| *self.stats.entry((i, c)).or_insert(0) += 1);
    }
}
