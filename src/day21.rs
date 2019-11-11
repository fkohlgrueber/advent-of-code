
#[allow(unused_imports)]
use aoc_tools::prelude::*;


pub fn calc(input: &str) -> (String, String) {
    (part_1(input).to_string(), part_2(input).to_string())
}

fn part_1(_input: &str) -> usize {
    let mut r2 : usize = 0;
    let mut r5 : usize = r2 | 0x10000;
    r2 = 0x49E737;
    loop {
        let mut r4 : usize = r5 & 0xFF;
        r2 = ((r2 + r4) * 0x1016B) & 0xFFFFFF;
        if r5 < 256 {
            break;
        }
        r4 = r5 / 0x100;
        r5 = r4;
    }
    r2
}

fn part_2(_input: &str) -> usize {
    let mut set = HashSet::new();

    let mut r2 : usize = 0;
    let mut r2_prev = 0;
    'outer : loop {
        let mut r5 = r2 | 0x10000;
        r2 = 0x49E737;
        loop {
            let mut r4 = r5 & 0xFF;
            r2 = ((r2 + r4) * 0x1016B) & 0xFFFFFF;
            if r5 < 256 {
                if set.contains(&r2) {
                    break 'outer;
                }
                set.insert(r2);
                r2_prev = r2;
                break;
            }
            r4 = r5 / 0x100;
            r5 = r4;
        }
    }

    r2_prev
}

