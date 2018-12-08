use regex::Regex;
use std::collections::HashMap;

pub fn calc(input: &str) -> (String, String) {
    (part_1(input).to_string(), part_2(input).to_string())
}

fn part_1(input: &str) -> i32 {
    let patches: Vec<Patch> = input.lines().filter_map(|l| Patch::from_str(l)).collect();
    let hash_map = gen_cell_hashmap(&patches);

    hash_map.values().filter(|x| **x > 1).count() as i32
}

fn part_2(input: &str) -> i32 {
    let patches: Vec<Patch> = input.lines().filter_map(|l| Patch::from_str(l)).collect();
    let hash_map = gen_cell_hashmap(&patches);

    // check patches
    for p in patches {
        if check_patch(&p, &hash_map) {
            return p.id as i32;
        }
    }

    panic!("No non-overlapping patch found")
}

fn check_patch(p: &Patch, hash_map: &HashMap<(u32, u32), i32>) -> bool {
    for i in p.left..p.width + p.left {
        for j in p.top..p.top + p.heigth {
            let key = (i, j);
            let entry = hash_map.get(&key).unwrap();
            if *entry > 1 {
                return false;
            }
        }
    }
    true
}

fn gen_cell_hashmap(patches: &[Patch]) -> HashMap<(u32, u32), i32> {
    let mut hash_map = HashMap::new();

    for p in patches {
        for i in p.left..p.width + p.left {
            for j in p.top..p.top + p.heigth {
                let key = (i, j);
                *hash_map.entry(key).or_insert(0) += 1;
            }
        }
    }

    hash_map
}

#[derive(Debug)]
struct Patch {
    id: u32,
    left: u32,
    top: u32,
    width: u32,
    heigth: u32,
}

impl Patch {
    fn from_str(s: &str) -> Option<Patch> {
        let re = Regex::new(r"^#(\d+) @ (\d+),(\d+): (\d+)x(\d+)$").unwrap();

        if let Some(cap) = re.captures(s) {
            Some(Patch {
                id: cap[1].parse().unwrap(),
                left: cap[2].parse().unwrap(),
                top: cap[3].parse().unwrap(),
                width: cap[4].parse().unwrap(),
                heigth: cap[5].parse().unwrap(),
            })
        } else {
            None
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_1() {
        assert_eq!(part_1("#1 @ 1,3: 4x4\n#2 @ 3,1: 4x4\n#3 @ 5,5: 2x2"), 4);
    }

    #[test]
    fn test_part_2() {
        assert_eq!(part_2("#1 @ 1,3: 4x4\n#2 @ 3,1: 4x4\n#3 @ 5,5: 2x2"), 3);
    }
}
