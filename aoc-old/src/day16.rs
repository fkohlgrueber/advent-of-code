
#[allow(unused_imports)]
use aoc_tools::prelude::*;

pub fn calc(input: &str) -> (String, String) {
    (part_1(input).to_string(), part_2(input).to_string())
}

fn part_1(input: &str) -> i32 {
    let samples = Sample::from_str_multiple(input);

    // number of samples that behave like >= 3 opcodes
    let mut num_samples = 0;
    for s in samples {
        let mut t = 0;
        for op in OPCODES.iter() {
            if exec(s.before, op, s.inst.clone()) == s.after {
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
    let input = InputData::from_str(input).unwrap();

    let mut mappings = vec![OPCODES.iter().collect::<Vec<_>>(); 16];

    for s in input.samples {
        let possible = &mut mappings[s.inst.op_num];
        possible.retain(|op| exec(s.before, op, s.inst) == s.after);
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
    
    let mut reg = State(0, 0, 0, 0);
    for i in input.instructions {
        reg = exec(reg, &final_mappings[i.op_num], i);
    }
    
    reg[0]
}

const OPCODES: [&str; 16] = [
    "addr", "addi", "mulr", "muli", "banr", "bani", "borr", "bori", 
    "setr", "seti", "gtir", "gtri", "gtrr", "eqir", "eqri", "eqrr",
];

fn exec(before: State, opcode: &str, inst: Inst) -> State {
    let Inst { op_num: _, a, b, c } = inst;
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

parse_multiple!{
    #[parse(r"\[{}, {}, {}, {}\]")]
    #[derive(PartialEq, Clone, Copy, Debug)]
    struct State(usize, usize, usize, usize);
    
    #[parse("{} {} {} {}")]
    #[derive(Clone, Copy, Debug)]
    struct Inst {
        op_num: usize,
        a: usize,
        b: usize,
        c: usize,
    }

    #[parse(r"Before: {}\n{}\nAfter: *{}")]
    #[derive(Debug)]
    struct Sample {
        before: State,
        inst: Inst,
        after: State,
    }

    #[parse(r"{}\n\n\n+{}")]
    #[derive(Debug)]
    struct InputData {
        #[parse = "(?s:.*)" ]
        samples: Vec<Sample>,
        #[parse = "(?s:.*)" ]
        instructions: Vec<Inst>,
    }
}


impl std::ops::Index<usize> for State {
    type Output = usize;

    fn index(&self, idx: usize) -> &Self::Output {
        match idx {
            0 => &self.0,
            1 => &self.1,
            2 => &self.2,
            3 => &self.3,
            _ => panic!("Invalid index!"),
        }
    }
}

impl std::ops::IndexMut<usize> for State {
    fn index_mut(&mut self, idx: usize) -> &mut Self::Output {
        match idx {
            0 => &mut self.0,
            1 => &mut self.1,
            2 => &mut self.2,
            3 => &mut self.3,
            _ => panic!("Invalid index!"),
        }
    }
}

