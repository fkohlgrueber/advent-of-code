#[allow(unused_imports)]
use aoc_tools::prelude::*;
use crate::Challenge;

pub struct Day();

type Input = Vec<usize>;

impl Challenge<Input> for Day {
    fn parse(input: String) -> Input {
        input.trim().chars().map(|c| c.to_digit(10).unwrap() as usize).collect()
    }

    fn part_1(input: Input) -> String {
        let mut sum = 0;
        for idx in 0..input.len() {
            sum += (input[idx] == input[(idx+1)%input.len()]) as usize * input[idx] as usize
        }
        sum.to_string()
    }
    
    fn part_2(input: Input) -> String {
        let mut sum = 0;
        for idx in 0..input.len() {
            sum += (input[idx] == input[(idx+(input.len()/2))%input.len()]) as usize * input[idx] as usize
        }
        sum.to_string()
    }
}

impl Day {
    pub fn tests_part_1() -> Vec<(&'static str, &'static str)> {
        vec!(
            ("1122", "3"),
            ("1111", "4"),
            ("1234", "0"),
            ("91212129", "9")
        )
    }

    pub fn tests_part_2() -> Vec<(&'static str, &'static str)> {
        vec!(
            ("1212", "6"),
            ("1221", "0"),
            ("123425", "4"),
            ("123123", "12"),
            ("12131415", "4"),
        )
    }
}
