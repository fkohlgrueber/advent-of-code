use regex::Regex;
use lazy_static::lazy_static;

pub fn calc(input: &str) -> (String, String) {
    (part_1(input).to_string(), part_2(input).to_string())
}

fn part_1(input: &str) -> i32 {
    let samples = Sample::from_str(input);

    // number of samples that behave like >= 3 opcodes
    let mut num_samples = 0;
    for s in samples {
        let mut t = 0;
        for op in OPCODES.iter() {
            if exec(s.before, op, s.a, s.b, s.c) == s.after {
                t += 1;
            }
        }
        if t >= 3 {
            num_samples += 1;
        }
    }
    num_samples
}

fn part_2(input: &str) -> usize {
    let samples = Sample::from_str(input);

    let mut mappings = vec![OPCODES.iter().collect::<Vec<_>>(); 16];

    for s in samples {
        let possible = &mut mappings[s.op_num];
        possible.retain(|op| exec(s.before, op, s.a, s.b, s.c) == s.after);
    }

    let mut final_mappings = vec!["".to_string(); 16];
    while mappings.iter().any(|l| !l.is_empty()) {
        let mut elmt = "";
        for (i, m) in mappings.iter().enumerate(){
            if m.len() == 1 {
                elmt = m[0];
                final_mappings[i] = m[0].to_string();
                break;
            }
        }
        for m in &mut mappings {
            m.retain(|x| x != &&elmt);
        }
    }
    
    let instructions = Inst::from_str(input);
    
    let mut reg : State = [0, 0, 0, 0];
    for i in instructions {
        reg = exec(reg, &final_mappings[i.op_num], i.a, i.b, i.c);
    }
    
    reg[0]
}

type Num = usize;

type State = [Num; 4];

const OPCODES : [&str; 16] = [
    "addr", "addi", "mulr", "muli", "banr", "bani", "borr", "bori", 
    "setr", "seti", "gtir", "gtri", "gtrr", "eqir", "eqri", "eqrr",
];

fn exec(before: State, opcode: &str, a: Num, b: Num, c: Num) -> State {
    let mut reg = before;
    reg[c] = match opcode {
        "addr" => reg[a] + reg[b],
        "addi" => reg[a] + b,
        "mulr" => reg[a] * reg[b],
        "muli" => reg[a] * b,
        "banr" => reg[a] | reg[b],
        "bani" => reg[a] | b,
        "borr" => reg[a] & reg[b],
        "bori" => reg[a] & b,
        "setr" => reg[a],
        "seti" => a,
        "gtir" => (a > reg[b]) as usize,
        "gtri" => (reg[a] > b) as usize,
        "gtrr" => (reg[a] > reg[b]) as usize,
        "eqir" => (a == reg[b]) as usize,
        "eqri" => (reg[a] == b) as usize,
        "eqrr" => (reg[a] == reg[b]) as usize,
        _ => panic!("invalid opcode")
    };
    reg
}


#[derive(Debug, PartialEq)]
struct Sample {
    before: State,
    after: State,
    op_num: Num,
    a: Num,
    b: Num,
    c: Num,
}

impl Sample {
    fn from_str(input: &str) -> Vec<Sample> {
        lazy_static! {
            static ref re : Regex = Regex::new(
                r"Before: \[(\d), (\d), (\d), (\d)\]\n(\d+) (\d+) (\d+) (\d+)\nAfter:  \[(\d), (\d), (\d), (\d)\]").unwrap();
        }
        re.captures_iter(input).map(|cap| {
                Sample { 
                    before: [
                        cap[1].parse().unwrap(), 
                        cap[2].parse().unwrap(), 
                        cap[3].parse().unwrap(), 
                        cap[4].parse().unwrap()
                    ],
                    op_num: cap[5].parse().unwrap(), 
                    a: cap[6].parse().unwrap(), 
                    b: cap[7].parse().unwrap(), 
                    c: cap[8].parse().unwrap(),
                    after: [
                        cap[9].parse().unwrap(), 
                        cap[10].parse().unwrap(), 
                        cap[11].parse().unwrap(), 
                        cap[12].parse().unwrap()
                    ],
                }
            }
        ).collect()
    }
}

#[derive(Debug)]
struct Inst {
    op_num: Num,
    a: Num,
    b: Num,
    c: Num,
}

impl Inst {
    fn from_str(input: &str) -> Vec<Inst> {
        let input = &input[input.rfind("After").unwrap()..];
        lazy_static! {
            static ref re : Regex = Regex::new(
                r"(\d+) (\d+) (\d+) (\d+)").unwrap();
        }
        re.captures_iter(input).map(|cap| {
                Inst { 
                    op_num: cap[1].parse().unwrap(), 
                    a: cap[2].parse().unwrap(), 
                    b: cap[3].parse().unwrap(), 
                    c: cap[4].parse().unwrap(),
                }
            }
        ).collect()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_sample_from_str() {
        assert_eq!(
            Sample::from_str("Before: [2, 1, 1, 0]\n\
                              5 1 0 1\n\
                              After:  [2, 0, 1, 0]\n\
                              \n\
                              Before: [3, 0, 3, 3]\n\
                              13 0 3 1\n\
                              After:  [3, 1, 3, 3]\n\
                              \n\
                              "),
            vec!(
                Sample {
                    before: [2, 1, 1, 0],
                    after: [2, 0, 1, 0],
                    op_num: 5,
                    a: 1,
                    b: 0,
                    c: 1
                },
                Sample {
                    before: [3, 0, 3, 3],
                    after: [3, 1, 3, 3],
                    op_num: 13,
                    a: 0,
                    b: 3,
                    c: 1
                },
                
            )
        )
    }
}
