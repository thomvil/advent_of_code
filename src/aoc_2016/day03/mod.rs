mod input_parser;
mod triangle;

use aoc_2016::day03::triangle::Triangle;
use aoc_2016::day03::input_parser::*;

pub fn report(input: &str) {
    println!(" -- 2016: Day 3 -- ");
    println!(
        "| {} of the triangles, reading horizontally, are possible.",
        Triangle::count_triangles(parse_instructions_horizontally(input))
    );
    println!(
        "| {} of the triangles, reading vertically, are possible.",
        Triangle::count_triangles(parse_instructions_vertically(input))
    );
    println!(" ------------------");
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST_INPUT: &str =
        "101 301 501\n102 302 502\n103 303 503\n201 401 601\n202 402 602\n203 403 603";

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
