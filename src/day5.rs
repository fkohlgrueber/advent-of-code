
#[allow(unused_imports)]
use aoc_tools::prelude::*;

pub fn calc(input: &str) -> (String, String) {
    (part_1(input).to_string(), part_2(input).to_string())
}

fn part_1(input: &str) -> i32 {
    let contents : Vec<char> = input.chars().collect();

    react(&contents)
}

fn part_2(input: &str) -> i32 {
    let contents = input.to_owned();

    let contents_chars : Vec<char> = contents.chars().collect();

    let mut all_chars : HashSet<char> = HashSet::new();
    all_chars.extend(contents_chars.iter().map(|x| x.to_ascii_lowercase()));

    all_chars.iter().map(|c| {
        let removed_chars = remove(&contents, *c);
        react(&removed_chars)
    }).min().unwrap() as i32
}

fn same_char(a: char, b: char) -> bool {
    a != b && a.to_ascii_lowercase() == b.to_ascii_lowercase()
}

fn react(contents : &[char]) -> i32{
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

    stack.len() as i32
}

fn remove(s: &str, c: char) -> Vec<char>{
    s.replace(c.to_ascii_lowercase(), "").replace(c.to_ascii_uppercase(), "").chars().collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_1() {
        assert_eq!(part_1("dabAcCaCBAcCcaDA"), 10);
        assert_eq!(part_1("abBAcdDe"), 2);
        assert_eq!(part_1("abBAcCdDe"), 1);
    }

    #[test]
    fn test_part_2() {
        assert_eq!(part_2("dabAcCaCBAcCcaDA"), 4);
    }
}
