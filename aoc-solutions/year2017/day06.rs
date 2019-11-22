#[allow(unused_imports)]
use aoc_tools::prelude::*;

pub struct Day();

impl Challenge for Day {
    type Input = Vec<usize>;

    fn parse(input: String) -> Self::Input {
        input.trim().split("\t").map(|x| x.parse().unwrap()).collect()
    }

    fn part_1(mut input: Self::Input) -> String {
        let mut steps = 0;
        let mut prev_steps = HashSet::new();
        loop {
            let max_ = input.iter().max().unwrap();
            let mut idx = 0;
            for i in 0..input.len() {
                if input[i] == *max_ {
                    idx = i;
                    break;
                }
            }

            let n = input[idx];
            input[idx] = 0;
            for i in 0..n {
                let j = (idx+i+1)%input.len();
                input[j] += 1;
            }

            steps += 1;

            if prev_steps.contains(&input) {
                break;
            }
            prev_steps.insert(input.clone());
        }
    
        steps.to_string()
    }
    
    fn part_2(mut input: Self::Input) -> String {
        let mut steps = 0;
        let mut prev_steps = HashSet::new();
        let mut double_state = None;
        loop {
            let max_ = input.iter().max().unwrap();
            let mut idx = 0;
            for i in 0..input.len() {
                if input[i] == *max_ {
                    idx = i;
                    break;
                }
            }

            let n = input[idx];
            input[idx] = 0;
            for i in 0..n {
                let j = (idx+i+1)%input.len();
                input[j] += 1;
            }

            steps += 1;

            if prev_steps.contains(&input) && double_state.is_none() {
                double_state = Some(input.clone());
                steps = 0;
            } else if Some(input.clone()) == double_state {
                break;
            }
            
            prev_steps.insert(input.clone());
        }
    
        steps.to_string()
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
