use std::collections::HashSet;

pub fn calc(input: &str) -> (String, String) {
    (part_1(input).to_string(), part_2(input).to_string())
}

fn part_1(input: &str) -> i32 {
    input.lines().map(|x| x.parse::<i32>().unwrap()).sum()
}

fn part_2(input: &str) -> i32 {
    let num_iter = input.lines().map(|x| x.parse::<i32>().unwrap());

    let mut frequencies = HashSet::new();
    frequencies.insert(0);

    let mut current_freq = 0;
    for num in num_iter.cycle() {
        current_freq += num;
        if frequencies.contains(&current_freq) {
            return current_freq;
        } else {
            frequencies.insert(current_freq);
        }
    }

    panic!("The sequence does not contain a double frequency.")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_1() {
        assert_eq!(part_1("+1\n-2\n+3\n+1"), 3);
    }

    #[test]
    fn test_part_2() {
        assert_eq!(part_2("+1\n-2\n+3\n+1\n+1\n-2"), 2);
        assert_eq!(part_2("+1\n-1"), 0);
    }
}
