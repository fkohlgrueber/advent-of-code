#[allow(unused_imports)]
use aoc_tools::prelude::*;

pub struct Day();

impl Challenge for Day {
    type Input = Vec<char>;

    fn parse(input: String) -> Self::Input {
        input.trim().chars().collect()
    }

    fn part_1(input: Self::Input) -> String {
        react(&input).to_string()
    }
    
    fn part_2(input: Self::Input) -> String {
        let contents = input.clone();
        let mut all_chars : HashSet<char> = HashSet::new();
        all_chars.extend(input.iter().map(|x| x.to_ascii_lowercase()));

        all_chars.iter().map(|c| {
            let removed_chars = remove(&contents, *c);
            react(&removed_chars)
        }).min().unwrap().to_string()
    }
}

fn same_char(a: char, b: char) -> bool {
    a != b && a.to_ascii_lowercase() == b.to_ascii_lowercase()
}

fn react(contents: &[char]) -> usize {
    let mut stack = Vec::new();
    stack.reserve(contents.len());

    let mut i = 0;
    let mut j = 1;
    while i < contents.len() {
        if j < contents.len() && same_char(contents[i], contents[j]) {
            match stack.pop() {
                None => {i = j+1; j += 2;},
                Some(x) => {i = x; j += 1;},
            }
        } else {
            stack.push(i);
            i = j;
            j += 1;
        }
    }

    stack.len()
}

fn remove(s: &[char], c: char) -> Vec<char> {
    s.into_iter().cloned().filter(|e| *e != c.to_ascii_lowercase() && *e != c.to_ascii_uppercase()).collect()
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_1() {
        Day::test_part_1("dabAcCaCBAcCcaDA", 10);
        Day::test_part_1("abBAcdDe", 2);
        Day::test_part_1("abBAcCdDe", 1);
    }

    #[test]
    fn test_part_2() {
        Day::test_part_2("dabAcCaCBAcCcaDA", 4);
    }
}
