#[allow(unused_imports)]
use aoc_tools::prelude::*;

pub struct Day();


impl Challenge for Day {
    type Input = Vec<Vec<usize>>;

    fn parse(input: String) -> Self::Input {
        input.lines().map(|l| l.split("\t").map(|v| v.parse().unwrap()).collect()).collect()
    }

    fn part_1(input: Self::Input) -> String {
        input.iter().map(|l| l.iter().max().unwrap() - l.iter().min().unwrap()).sum::<usize>().to_string()
    }
    
    fn part_2(input: Self::Input) -> String {
        let even_div = |a, b| if max(a, b)%min(a, b) == 0 {
            Some(max(a, b)/min(a, b))
        } else {
            None
        };

        let process_line = |l: &Vec<usize>| {
            for i in 0..l.len() {
                for j in i+1..l.len() {
                    if let Some(n) = even_div(l[i], l[j]) {
                        return n;
                    }
                }
            }
            panic!("nonono")
        };
        input.iter().map(process_line).sum::<usize>().to_string()
    }
}
