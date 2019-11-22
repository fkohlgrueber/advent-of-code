#[allow(unused_imports)]
use aoc_tools::prelude::*;

pub struct Day();

impl Challenge for Day {
    type Input = String;

    fn parse(input: String) -> Self::Input {
        input
    }

    fn part_1(input: Self::Input) -> String {
        input.lines().filter_map(|l| {
            let mut s = HashSet::new();
            for e in l.split(" ") {
                s.insert(e);
            }
            if s.len() == l.split(" ").collect::<Vec<_>>().len() {
                Some(s)
            } else {
                None
            }
        }).count().to_string()
    }
    
    fn part_2(input: Self::Input) -> String {
        input.lines().filter_map(|l| {
            let mut s = vec!();
            for e in l.split(" ") {
                let mut word = HashMap::new();
                for c in e.chars() {
                    *word.entry(c).or_insert(0) += 1;
                }
                s.push(word);
            }
            for a in 0..s.len() {
                for b in a+1..s.len() {
                    if s[a] == s[b] {
                        return None;
                    }
                }
            }
            
            Some(s)
            
        }).count().to_string()
    }
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
}
