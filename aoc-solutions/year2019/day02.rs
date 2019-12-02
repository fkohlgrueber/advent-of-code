#[allow(unused_imports)]
use aoc_tools::prelude::*;

pub struct Day();

impl Challenge for Day {
    type Input = Vec<usize>;

    fn parse(input: String) -> Self::Input {
        input.trim().split(",").map(|x| x.parse().unwrap()).collect()
    }

    fn part_1(mut input: Self::Input) -> String {
        input[1] = 12;
        input[2] = 2;
        run_program(&mut input);
        input[0].to_string()
    }
    
    fn part_2(input: Self::Input) -> String {
        for noun in 0..100 {
            for verb in 0..100 {
                let mut i = input.clone();
                i[1] = noun;
                i[2] = verb;
                run_program(&mut i);
                if i[0] == 19690720 {
                    return (100 * noun + verb).to_string();
                }
            }
        }
        panic!("unreachable!")
    }
}

fn run_program(input: &mut Vec<usize>) {
    let mut ip = 0;
    loop {
        match input[ip] {
            99 => break,
            1 => {
                let target = input[ip+3];
                input[target] = input[input[ip+1]] + input[input[ip+2]];
            },
            2 => {
                let target = input[ip+3];
                input[target] = input[input[ip+1]] * input[input[ip+2]];
            },
            _ => panic!("invalid opcode!")
        }
        ip += 4;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_1() {
        Day::test_part_1("1,9,10,3,2,3,11,0,99,30,40,50", 3500);
    }

    #[ignore]
    #[test]
    fn test_part_2() {
        Day::test_part_2("", 0);
    }
}
