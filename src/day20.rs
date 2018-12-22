use std::collections::HashMap;

pub fn calc(input: &str) -> (String, String) {
    (part_1(input).to_string(), part_2(input).to_string())
}

fn part_1(input: &str) -> i32 {
    let dists = calc_dists(input);
    *dists.values().max().unwrap()
}

fn part_2(input: &str) -> i32 {
    let dists = calc_dists(input);
    dists.values().filter(|x| **x >= 1000).count() as i32
}

fn calc_dists(input: &str) -> HashMap<(i32, i32), i32> {
    let chars: Vec<char> = input.chars().skip(1).collect();
    
    let mut stack = vec!();
    let mut dists = HashMap::new();
    
    let mut pos = (0, 0, 0);
    
    for c in chars {
        match c {
            '(' => stack.push(pos),
            ')' => pos = stack.pop().unwrap(),
            '|' => pos = *stack.last().unwrap(),
            'N' => pos = (pos.0, pos.1 - 1, pos.2 + 1),
            'S' => pos = (pos.0, pos.1 + 1, pos.2 + 1),
            'W' => pos = (pos.0 - 1, pos.1, pos.2 + 1),
            'E' => pos = (pos.0 + 1, pos.1, pos.2 + 1),
            _ => (),
        }
        if let Some(d) = dists.get(&(pos.0, pos.1)) {
            if pos.2 < *d {
                dists.insert((pos.0, pos.1), pos.2);
            }
        } else {
            dists.insert((pos.0, pos.1), pos.2);
        }
    }
    dists
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_1() {
        assert_eq!(part_1("^ESSWWN(E|NNENN(EESS(WNSE|)SSS|WWWSSSSE(SW|NNNE)))$"), 23);
        assert_eq!(part_1("^WNE$"), 3);
        assert_eq!(part_1("^ENWWW(NEEE|SSE(EE|N))$"), 10);
        assert_eq!(part_1("^ENNWSWW(NEWS|)SSSEEN(WNSE|)EE(SWEN|)NNN$"), 18);
        assert_eq!(part_1("^WSSEESWWWNW(S|NENNEEEENN(ESSSSW(NWSW|SSEN)|WSWWN(E|WWS(E|SS))))$"), 31);
    }
}
