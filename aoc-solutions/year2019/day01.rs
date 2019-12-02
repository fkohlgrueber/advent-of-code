#[allow(unused_imports)]
use aoc_tools::prelude::*;

pub struct Day();

impl Challenge for Day {
    type Input = Vec<isize>;

    fn parse(input: String) -> Self::Input {
        input.lines().map(|x| x.parse().unwrap()).collect()
    }

    fn part_1(input: Self::Input) -> String {
        input.iter().map(|x| x/3-2).sum::<isize>().to_string()
    }
    
    fn part_2(input: Self::Input) -> String {
        input.into_iter().map(calc_fuel).sum::<isize>().to_string()
    }
}

fn calc_fuel(mut n: isize) -> isize {
    let mut sum = 0;
    loop {
        let new_n = n / 3 - 2;
        if new_n <= 0 {
            break;
        }
        sum += new_n;
        n = new_n;
    }
    sum
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_1() {
        Day::test_part_1("", 0);
    }

    #[ignore]
    #[test]
    fn test_part_2() {
        Day::test_part_2("", 0);
    }

    #[test]
    fn test_part_2_2() {
        assert_eq!(calc_fuel(100756), 50346);
    }
}
