#[allow(unused_imports)]
use aoc_tools::prelude::*;

pub struct Day();

impl Challenge for Day {
    type Input = String;

    fn parse(input: String) -> Self::Input {
        input
    }

    fn part_1(_input: Self::Input) -> String {
        "".to_string()
    }
    
    fn part_2(_input: Self::Input) -> String {
        "".to_string()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[ignore]
    #[test]
    fn test_part_1() {
        Day::test_part_1("", 0);
    }

    #[ignore]
    #[test]
    fn test_part_2() {
        Day::test_part_2("", 0);
    }
}
