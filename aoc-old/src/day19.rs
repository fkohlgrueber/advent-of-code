
#[allow(unused_imports)]
use aoc_tools::prelude::*;

pub fn calc(input: &str) -> (String, String) {
    (part_1(input).to_string(), part_2(input).to_string())
}

fn part_1(input: &str) -> usize {
    let ip_reg: usize = input.lines().next().unwrap().trim_start_matches("#ip ").parse().unwrap();
    let insts = Instruction::from_str_multiple(input);

    let mut reg: State = [0; 6];

    while reg[ip_reg] < insts.len() {
        let i = &insts[reg[ip_reg]];
        reg = exec(reg, &i.op_str, i.a, i.b, i.c);
        reg[ip_reg] += 1;
        //println!("{}, {:?}", ip_reg, reg);
    }
    
    reg[0]
}

fn part_2(_input: &str) -> usize {
    let num = 10_551_260;
    let mut sum = 0;
    for i in 1..=num {
        if num % i == 0 {
            sum += i;
        }
    }
    sum
}

type Num = usize;

type State = [Num; 6];


fn exec(before: State, opcode: &str, a: Num, b: Num, c: Num) -> State {
    let mut reg = before;
    reg[c] = match opcode {
        "addr" => reg[a] + reg[b],
        "addi" => reg[a] + b,
        "mulr" => reg[a] * reg[b],
        "muli" => reg[a] * b,
        "banr" => reg[a] & reg[b],
        "bani" => reg[a] & b,
        "borr" => reg[a] | reg[b],
        "bori" => reg[a] | b,
        "setr" => reg[a],
        "seti" => a,
        "gtir" => (a > reg[b]) as usize,
        "gtri" => (reg[a] > b) as usize,
        "gtrr" => (reg[a] > reg[b]) as usize,
        "eqir" => (a == reg[b]) as usize,
        "eqri" => (reg[a] == b) as usize,
        "eqrr" => (reg[a] == reg[b]) as usize,
        s => panic!(format!("invalid opcode: {}", s))
    };
    reg
}


#[parse(r"\n{} {} {} {}")]
#[derive(Debug, PartialEq)]
struct Instruction {
    #[parse = ".+"]
    op_str: String,
    a: usize,
    b: usize,
    c: usize,
}
