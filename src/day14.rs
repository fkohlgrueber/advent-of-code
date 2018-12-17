

pub fn calc(input: &str) -> (String, String) {
    let val: usize = input.parse().unwrap();
    let vals: Vec<u8> = input.chars().map(|x| x.to_digit(10).unwrap() as u8).collect();
    
    let mut scores : Vec<u8> = vec!();
    scores.push(3);
    scores.push(7);

    let mut idx1 = 0;
    let mut idx2 = 1;
    let mut res_part_2 = 0;
    while res_part_2 == 0 || scores.len() < val + 10 {
        // push new values
        let new_num = scores[idx1] + scores[idx2];
        if new_num >= 10 {
            scores.push(1);
            if scores.len() >= vals.len() && vals.iter().zip(scores.iter().skip(scores.len() - vals.len()).take(vals.len())).all(|(a, b)| a == b) {
                if res_part_2 == 0 {
                    res_part_2 = scores.len() - vals.len();
                }
                if scores.len() >= val + 10 { break }
            }
            scores.push(new_num - 10);
            if scores.len() >= vals.len() && vals.iter().zip(scores.iter().skip(scores.len() - vals.len()).take(vals.len())).all(|(a, b)| a == b) {
                if res_part_2 == 0 {
                    res_part_2 = scores.len() - vals.len();
                }
                if scores.len() >= val + 10 { break }
            }
        } else {
            scores.push(new_num);
            if scores.len() >= vals.len() && vals.iter().zip(scores.iter().skip(scores.len() - vals.len()).take(vals.len())).all(|(a, b)| a == b) {
                if res_part_2 == 0 {
                    res_part_2 = scores.len() - vals.len();
                }
                if scores.len() >= val + 10 { break }
            }
        }
        // update indices
        idx1 = idx1 + scores[idx1] as usize + 1;
        idx2 = idx2 + scores[idx2] as usize + 1;
        idx1 -= (idx1 >= scores.len()) as usize * scores.len();
        idx2 -= (idx2 >= scores.len()) as usize * scores.len();
        idx1 -= (idx1 >= scores.len()) as usize * scores.len();
        idx2 -= (idx2 >= scores.len()) as usize * scores.len();
    };
    (scores.iter().skip(val).take(10).map(|x| x.to_string()).collect::<String>(), res_part_2.to_string())
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_1() {
        assert_eq!(calc("9").0, "5158916779");
        assert_eq!(calc("5").0, "0124515891");
        assert_eq!(calc("18").0, "9251071085");
        assert_eq!(calc("2018").0, "5941429882");
    }

    #[test]
    fn test_part_2() {
        assert_eq!(calc("51589").1, "9");
        assert_eq!(calc("01245").1, "5");
        assert_eq!(calc("92510").1, "18");
        assert_eq!(calc("59414").1, "2018");
    }
}
