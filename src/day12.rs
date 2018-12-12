use regex::Regex;

type Rules = [u8; 32];
type Pots = Vec<u8>;

pub fn calc(input: &str) -> (String, String) {
    (part_1(input).to_string(), part_2(input).to_string())
}

fn part_1(input: &str) -> i64 {
    calc_generations(input, 20)
}

fn part_2(input: &str) -> i64 {
    calc_generations(input, 50_000_000_000)
}

fn calc_generations(input: &str, num_generations: usize) -> i64 {
    
    // return sum of indices of pots that contain a plant
    let (rules, mut pots) = parse_input(input);
    let mut pots_b = pots.clone();

    let mut offset = -4;

    for gen in 0..num_generations {
        let (p1, p2) = if gen % 2 == 0{
            (&mut pots, &mut pots_b)
        } else {
            (&mut pots_b, &mut pots)
        };

        p2.clear();
        // add 4 zeros at the beginning
        for _ in 0..4 {
            p2.push(0);
        }
        let first_idx = p1.iter().position(|x| *x == 1).unwrap();
        let last_idx = p1.iter().rposition(|x| *x == 1).unwrap();
        let skip = first_idx - 4;
        
        // add values that may or may not be zero
        for w in p1.windows(5).skip(skip).take(last_idx + 1 - skip + 4) {
            p2.push(rules[w.iter().fold(0, |a, b| (a<<1) + b) as usize]);
        }

        for _ in 0..4 {
            p2.push(0);
        }

        // detect same pattern moving sidewards
        if p1 == p2 {
            offset += (skip as i64 - 2) * (num_generations - gen) as i64;
            break;
        }
        
        offset += skip as i64 - 2;

        //println!("offset: {}, pots: {}", offset, p2.iter().map(|x| if *x == 1 {'x'} else {'.'}).collect::<String>());
    }

    pots.iter().enumerate().map(|(i, b)| (i as i64 + offset) * i64::from(*b)).sum::<i64>()
}

fn parse_input(input: &str) -> (Rules, Pots) {

    let mut rules : Rules = [0; 32];
    let mut lines = input.lines();

    let re = Regex::new(r"initial state: (.*)").unwrap();
    let state_chars = &re.captures(lines.next().unwrap()).unwrap()[1];

    let mut pots : Pots = vec![0; state_chars.len() + 8];

    for (i, c) in state_chars.chars().enumerate() {
        pots[i + 4] = (c == '#') as u8;
    }

    for line in lines.skip(1) {
        let idx = line.chars().take(5).map(|c| (c == '#') as u8).fold(0, |a, b| (a<<1) + b);
        rules[idx as usize] = line.ends_with('#') as u8;
    }
    
    (rules, pots)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_1() {
        assert_eq!(
            part_1("initial state: #..#.#..##......###...###\n\
                    \n\
                    ...## => #\n\
                    ..#.. => #\n\
                    .#... => #\n\
                    .#.#. => #\n\
                    .#.## => #\n\
                    .##.. => #\n\
                    .#### => #\n\
                    #.#.# => #\n\
                    #.### => #\n\
                    ##.#. => #\n\
                    ##.## => #\n\
                    ###.. => #\n\
                    ###.# => #\n\
                    ####. => #"), 
            325);
    }

    #[test]
    fn test_part_2() {
        assert_eq!(
            part_2("initial state: #..#.#..##......###...###\n\
                    \n\
                    ...## => #\n\
                    ..#.. => #\n\
                    .#... => #\n\
                    .#.#. => #\n\
                    .#.## => #\n\
                    .##.. => #\n\
                    .#### => #\n\
                    #.#.# => #\n\
                    #.### => #\n\
                    ##.#. => #\n\
                    ##.## => #\n\
                    ###.. => #\n\
                    ###.# => #\n\
                    ####. => #"), 
            999_999_999_374);
    }

    #[test]
    fn test_calc_generations() {
        assert_eq!(
            calc_generations("initial state: #..#.#..##......###...###\n\
                    \n\
                    ...## => #\n\
                    ..#.. => #\n\
                    .#... => #\n\
                    .#.#. => #\n\
                    .#.## => #\n\
                    .##.. => #\n\
                    .#### => #\n\
                    #.#.# => #\n\
                    #.### => #\n\
                    ##.#. => #\n\
                    ##.## => #\n\
                    ###.. => #\n\
                    ###.# => #\n\
                    ####. => #", 200), 
            3374);
    }

    #[test]
    fn test_fold() {
        let a : [u8; 5]= [1, 0, 0, 1, 1];
        assert_eq!(a.iter().fold(0, |a, b| (a<<1) + b), 19);
    }
}
