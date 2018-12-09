use regex::Regex;
use std::cmp::{max, min};
use lazy_static::lazy_static;

pub fn calc(input: &str) -> (String, String) {
    (part_1(input).to_string(), part_2(input).to_string())
}

fn part_1(input: &str) -> i32 {
    let patches: Vec<Patch> = input.lines().filter_map(|l| Patch::from_str(l)).collect();
    let grid = gen_grid(&patches);
    
    grid.values.iter().filter(|x| **x > 1).count() as i32
}

fn part_2(input: &str) -> i32 {
    let patches: Vec<Patch> = input.lines().filter_map(|l| Patch::from_str(l)).collect();
    let grid = gen_grid(&patches);

    // check patches
    for p in patches {
        if grid.check_patch(&p) {
            return p.id as i32;
        }
    }

    panic!("No non-overlapping patch found")
}

struct BoundingBox{
    top: usize,
    left: usize,
    bottom: usize,
    right: usize,
}

impl BoundingBox {
    fn new() -> BoundingBox {
        BoundingBox {
            top: std::usize::MAX,
            left: std::usize::MAX,
            bottom: std::usize::MIN,
            right: std::usize::MIN,
        }
    }

    fn update(&mut self, p : &Patch){
        self.top = min(self.top, p.top as usize);
        self.left = min(self.left, p.left as usize);
        self.right = max(self.right, (p.left + p.width - 1) as usize);
        self.bottom = max(self.bottom, (p.top + p.heigth - 1) as usize);
    }

    fn width(&self) -> usize{
        self.right - self.left + 1
    }
    
    fn heigth(&self) -> usize{
        self.bottom - self.top + 1
    }
}

struct Grid {
    values: Vec<i32>,
    bb: BoundingBox,
    width: usize,
}

impl Grid {
    fn new(bb: BoundingBox) -> Grid{
        Grid{
            values : vec![0; bb.width() * bb. heigth()],
            width: bb.width(),
            bb,
        }
    }

    fn insert_patch(&mut self, p: &Patch){
        for y in p.top..p.top + p.heigth {
            for x in p.left..p.left + p.width{
                let idx = self.idx(x, y);
                self.values[idx] += 1;
            }
        }
    }

    fn check_patch(&self, p: &Patch) -> bool {
        for y in p.top..p.top + p.heigth {
            for x in p.left..p.left + p.width{
                if self.values[self.idx(x, y)] > 1 {
                    return false;
                }
            }
        }
        true
    }

    fn idx(&self, x: usize, y: usize) -> usize{
        let col = x - self.bb.left;
        let row = y - self.bb.top;
        row*self.width + col
    }
}

fn gen_grid(patches: &[Patch]) -> Grid {
    let mut bb = BoundingBox::new();
    for p in patches{
        bb.update(p);
    }
    let mut grid = Grid::new(bb);
    for p in patches {
        grid.insert_patch(p);
    }
    grid
}

#[derive(Debug)]
struct Patch {
    id: usize,
    left: usize,
    top: usize,
    width: usize,
    heigth: usize,
}

impl Patch {
    fn from_str(s: &str) -> Option<Patch> {
        lazy_static! {
            static ref RE: Regex = Regex::new(r"^#(\d+) @ (\d+),(\d+): (\d+)x(\d+)$").unwrap();
        }

        if let Some(cap) = RE.captures(s) {
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
