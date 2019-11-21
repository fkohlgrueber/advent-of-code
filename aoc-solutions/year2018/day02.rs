#[allow(unused_imports)]
use aoc_tools::prelude::*;

pub struct Day();

impl Challenge for Day {
    type Input = String;

    fn parse(input: String) -> Self::Input {
        input
    }

    fn part_1(input: Self::Input) -> String {
        let mut two = 0;
        let mut three = 0;
        for line in input.lines(){
            let mut counter = HashMap::new();
            for c in line.chars(){
                *counter.entry(c).or_insert(0) += 1;
            }
            
            two += counter.values().any(|x| *x==2) as i32;
            three += counter.values().any(|x| *x==3) as i32;
        }

        (two * three).to_string()
    }
    
    fn part_2(input: Self::Input) -> String {
        let lines : Vec<_> = input.lines().collect();

        let mut best_chars = String::new();
        for i in 0..lines.len(){
            for j in i+1..lines.len(){
                let char_pairs = lines[i].chars().zip(lines[j].chars());
                let same_chars : String = char_pairs
                        .filter(|(x, y)| x==y)
                        .map(|(x, _)| x)
                        .collect();
                if same_chars.len() > best_chars.len(){
                    best_chars = same_chars;
                }
            }
        }

        best_chars
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_1() {
        Day::test_part_1("abcdef\nbababc\nabbcde\nabcccd\naabcdd\nabcdee\nababab", 12);
    }

    #[test]
    fn test_part_2() {
        Day::test_part_2("abcde\nfghij\nklmno\npqrst\nfguij\naxcye\nwvxyz", "fgij");
    }
}
