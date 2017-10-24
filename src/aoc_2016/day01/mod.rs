mod input_parser;
mod coordinate;
mod location;
mod sleigh;

use aoc_2016::day01::sleigh::Sleigh;

type Instruction = (Turn, i32);

#[derive(Debug)]
enum Direction {
    North,
    East,
    South,
    West,
}

#[derive(Debug)]
pub enum Turn {
    Left,
    Right,
}

pub fn report(instructions: &str) {
    println!(" -- 2016: Day 1 -- ");
    let mut sleigh = Sleigh::new();
    sleigh.run(&input_parser::parse_instructions(instructions));
    println!(
        "|  The sleigh ends {:?} blocks from the starting point.",
        sleigh.distance_to_origin()
    );
    match sleigh.bunny_hq() {
        Some(c) => println!(
            "|  Bunny HQ is {:?} blocks from the starting point.",
            c.norm1()
        ),
        None => println!("|  Bunny HQ is not found :("),
    }
    println!(" ------------------");
}

#[cfg(test)]
mod tests {
    use super::*;
    use super::coordinate::*;
    use Turn::*;

    fn sleigh_after_run(list: Vec<Instruction>) -> Sleigh {
        let mut sleigh = Sleigh::new();
        sleigh.run(list);
        sleigh
    }

    #[test]
    fn test_distance_after_run1() {
        let list = vec![(Right, 2), (Left, 3)];
        assert_eq!(5, sleigh_after_run(list).distance_to_origin());
    }

    #[test]
    fn test_distance_after_run2() {
        let list = vec![(Right, 2), (Right, 2), (Right, 2)];
        assert_eq!(2, sleigh_after_run(list).distance_to_origin());
    }

    #[test]
    fn test_distance_after_run3() {
        let list = vec![(Right, 5), (Left, 5), (Right, 5), (Right, 3)];
        assert_eq!(12, sleigh_after_run(list).distance_to_origin());
    }

    #[test]
    fn test_first_recurrence() {
        let list = vec![(Right, 8), (Right, 4), (Right, 4), (Right, 8)];
        let coord = Coordinate::new(4, 0);
        assert_eq!(Some(coord), sleigh_after_run(list).bunny_hq());
    }
}
