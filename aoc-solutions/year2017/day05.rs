#[allow(unused_imports)]
use aoc_tools::prelude::*;

pub struct Day();

impl Challenge for Day {
    type Input = Vec<i32>;

    fn parse(input: String) -> Self::Input {
        input.lines().map(|x| x.parse().unwrap()).collect()
    }

    fn part_1(mut input: Self::Input) -> String {
        let mut ip = 0;
        let mut steps = 0;

        while ip >= 0 && ip < input.len() as i32 {
            let offset = input[ip as usize];
            input[ip as usize] += 1;
            ip += offset;
            steps += 1;
        }

        steps.to_string()
    }
    
    fn part_2(mut input: Self::Input) -> String {
        let mut ip = 0;
        let mut steps = 0;

        while ip >= 0 && ip < input.len() as i32 {
            let offset = input[ip as usize];
            input[ip as usize] += if input[ip as usize] >= 3 { -1 } else { 1 };
            ip += offset;
            steps += 1;
        }

        steps.to_string()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_1() {
        Day::test_part_1("0\n3\n0\n1\n-3", 5);
    }

    #[test]
    fn test_part_2() {
        Day::test_part_2("0\n3\n0\n1\n-3", 10);
    }
}
