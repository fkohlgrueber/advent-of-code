#[allow(unused_imports)]
use aoc_tools::prelude::*;

pub struct Day();

impl Challenge for Day {
    type Input = Vec<i32>;

    fn parse(input: String) -> Self::Input {
        input.lines().map(|x| x.parse().unwrap()).collect()
    }

    fn part_1(input: Self::Input) -> String {
        input.iter().sum::<i32>().to_string()
    }

    fn part_2(input: Self::Input) -> String {
        let mut frequencies = HashSet::new();
        frequencies.insert(0);

        let mut current_freq = 0;
        for num in input.iter().cycle() {
            current_freq += num;
            if frequencies.contains(&current_freq) {
                return current_freq.to_string();
            } else {
                frequencies.insert(current_freq);
            }
        }

        panic!("The sequence does not contain a double frequency.")   
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_1() {
        Day::test_part_1("+1\n-2\n+3\n+1", "3");
    }

    #[test]
    fn test_part_2() {
        Day::test_part_1("+1\n-2\n+3\n+1\n+1\n-2", "2");
        Day::test_part_1("+1\n-1", "0");
    }
}
