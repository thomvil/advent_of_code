use input_parser::*;

fn main() {
    println!("{} of the triangles, reading horizontally, are possible.", Triangle::count_triangles(horizontal_instructions()));
    println!("{} of the triangles, reading vertically, are possible.", Triangle::count_triangles(vertical_instructions()));
}

#[derive(Copy, Clone, Debug, Hash, PartialEq, Eq)]
pub struct Triangle { }
impl Triangle {
    fn is_valid(mut sides: Vec<u32>) -> bool {
        if sides.len() != 3 { return false }
        sides.sort();
        sides[0] + sides[1] > sides[2]
    }

    fn count_triangles(list_of_sides: Vec<Vec<u32>>) -> usize {
        list_of_sides.into_iter().map(|sides| Triangle::is_valid(sides))
                                 .filter(|b| *b).count()
    }
}

mod input_parser {
    use std::num::*;

    const INPUT: &str = include_str!("../inputs/day03.txt");

    pub fn horizontal_instructions() -> Vec<Vec<u32>> {
        parse_instructions_horizontally(INPUT)
    }

    pub fn vertical_instructions() -> Vec<Vec<u32>> {
        parse_instructions_vertically(INPUT)
    }

    pub fn parse_instructions_vertically(instruction_str: &str) -> Vec<Vec<u32>> {
        let hor = parse_instructions_horizontally(instruction_str);
        let rows = hor.iter().len();
        if rows % 3 != 0 { return Vec::new() }
        let mut res: Vec<Vec<u32>> = Vec::new();
        for col in 0..3 {
            for row in 0..rows/3 {
                res.push(vec![hor[0 + 3*row][col], hor[1 + 3*row][col], hor[2 + 3*row][col]]);
            }
        }
        res
    }

    pub fn parse_instructions_horizontally(instruction_str: &str) -> Vec<Vec<u32>> {
        instruction_str.lines()
                       .filter_map(|instr_str| parse_instruction(instr_str).ok())
                       .filter(|sides| sides.iter().len() == 3)
                       .collect()
    }

    pub fn parse_instruction(instr: &str) -> Result<Vec<u32>, ParseIntError> {
        instr.split_whitespace().map(|nb_str| nb_str.parse::<u32>()).collect()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST_INPUT: &str = "101 301 501\n102 302 502\n103 303 503\n201 401 601\n202 402 602\n203 403 603";

    #[test]
    fn test_triangle() {
        assert_eq!(false, Triangle::is_valid(vec![5, 10, 25]));
        assert_eq!(true, Triangle::is_valid(vec![19, 10, 25]));
    }

    #[test]
    fn test_vertically() {
        let ver = parse_instructions_vertically(TEST_INPUT);
        assert_eq!(6, Triangle::count_triangles(ver));
    }
}
