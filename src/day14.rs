

pub fn calc(input: &str) -> (String, String) {
    (part_1(input).to_string(), part_2(input).to_string())
}

fn part_1(input: &str) -> String {
    let val: usize = input.parse().unwrap();
    
    let mut scores : Vec<u8> = Vec::with_capacity(val + 10 + 1);
    scores.push(3);
    scores.push(7);

    let mut idx1 = 0;
    let mut idx2 = 1;
    while scores.len() < val + 10 {
        // push new values
        let new_num = scores[idx1] + scores[idx2];
        if new_num >= 10 {
            scores.push(1);
            scores.push(new_num - 10);
        } else {
            scores.push(new_num);
        }
        // update indices
        idx1 = (idx1 + scores[idx1] as usize + 1) % scores.len();
        idx2 = (idx2 + scores[idx2] as usize + 1) % scores.len();
    }

    scores.iter().skip(val).take(10).map(|x| x.to_string()).collect::<String>()
}

fn part_2(input: &str) -> i32 {
    let vals: Vec<u8> = input.chars().map(|x| x.to_digit(10).unwrap() as u8).collect();
    
    let mut scores : Vec<u8> = vec!();
    scores.push(3);
    scores.push(7);

    let mut idx1 = 0;
    let mut idx2 = 1;
    loop {
        // push new values
        let new_num = scores[idx1] + scores[idx2];
        if new_num >= 10 {
            scores.push(1);
            if scores.len() >= vals.len() && vals.iter().zip(scores.iter().skip(scores.len() - vals.len()).take(vals.len())).all(|(a, b)| a == b) {
                return (scores.len() - vals.len()) as i32;
            }
            scores.push(new_num - 10);
            if scores.len() >= vals.len() && vals.iter().zip(scores.iter().skip(scores.len() - vals.len()).take(vals.len())).all(|(a, b)| a == b) {
                return (scores.len() - vals.len()) as i32;
            }
        } else {
            scores.push(new_num);
            if scores.len() >= vals.len() && vals.iter().zip(scores.iter().skip(scores.len() - vals.len()).take(vals.len())).all(|(a, b)| a == b) {
                return (scores.len() - vals.len()) as i32;
            }
        }
        // update indices
        idx1 = (idx1 + scores[idx1] as usize + 1) % scores.len();
        idx2 = (idx2 + scores[idx2] as usize + 1) % scores.len();
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_1() {
        assert_eq!(part_1("9"), "5158916779");
        assert_eq!(part_1("5"), "0124515891");
        assert_eq!(part_1("18"), "9251071085");
        assert_eq!(part_1("2018"), "5941429882");
    }

    #[test]
    fn test_part_2() {
        assert_eq!(part_2("51589"), 9);
        assert_eq!(part_2("01245"), 5);
        assert_eq!(part_2("92510"), 18);
        assert_eq!(part_2("59414"), 2018);
    }
}
