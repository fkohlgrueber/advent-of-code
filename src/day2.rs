use std::collections::HashMap;

pub fn calc(input: &str) -> (String, String) {
    (part_1(input).to_string(), part_2(input))
}

fn part_1(input: &str) -> i32 {
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

    two * three
}

fn part_2(input: &str) -> String {
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_1() {
        assert_eq!(
            part_1("abcdef\nbababc\nabbcde\nabcccd\naabcdd\nabcdee\nababab"), 
            12
        );
    }

    #[test]
    fn test_part_2() {
        assert_eq!(
            part_2("abcde\nfghij\nklmno\npqrst\nfguij\naxcye\nwvxyz"), 
            "fgij"
        );
    }
}
