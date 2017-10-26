mod repetition_coder;

use aoc_2016::day06::repetition_coder::RepetitionCoder;

pub fn report(instr: &str) {
    println!(" -- 2016: Day 6 -- ");
    let mut rc = RepetitionCoder::new();
    rc.parse(instr);
    println!(
        "| Recoverd message: {:?} (most common filter)",
        rc.recover_msg_v1()
    );
    println!(
        "| Recoverd message: {:?} (least common filter)",
        rc.recover_msg_v2()
    );
    println!(" ------------------");
}

#[cfg(test)]
mod tests {
    use super::*;
    const TEST_INPUT1: &str = "eedadn\ndrvtee\neandsr\nraavrd\natevrs\ntsrnev\nsdttsa\nrasrtv";
    const TEST_INPUT2: &str = "nssdts\nntnada\nsvetve\ntesnvt\nvntsnd\nvrdear\ndvrsen\nenarar";

    #[test]
    fn test_recover_v1() {
        let mut rc = RepetitionCoder::new();
        rc.parse(TEST_INPUT1);
        rc.parse(TEST_INPUT2);
        assert_eq!("easter", rc.recover_msg_v1());
    }

    #[test]
    fn test_recover_v2() {
        let mut rc = RepetitionCoder::new();
        rc.parse(TEST_INPUT1);
        rc.parse(TEST_INPUT2);
        assert_eq!("advent", rc.recover_msg_v2());
    }
}
