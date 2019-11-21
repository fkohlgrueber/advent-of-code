#[allow(unused_imports)]
use aoc_tools::prelude::*;

pub struct Day();

impl Challenge for Day {
    type Input = (Vec<Patch>, Grid);

    fn parse(input: String) -> Self::Input {
        let patches: Vec<Patch> = Patch::from_str_multiple(&input);
        let grid = gen_grid(&patches);
        (patches, grid)
    }

    fn part_1(input: Self::Input) -> String {
        input.1.values.iter().filter(|x| **x > 1).count().to_string()
    }
    
    fn part_2(input: Self::Input) -> String {
        // check patches
        for p in input.0 {
            if input.1.check_patch(&p) {
                return p.id.to_string();
            }
        }

        panic!("No non-overlapping patch found")
    }
}

#[parse(r"#{} @ {},{}: {}x{}")]
#[derive(Debug, Clone)]
pub struct Patch {
    id: usize,
    left: usize,
    top: usize,
    width: usize,
    heigth: usize,
}

#[derive(Debug, Clone)]
pub struct BoundingBox {
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

    fn update(&mut self, p: &Patch){
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

#[derive(Debug, Clone)]
pub struct Grid {
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


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_1() {
        Day::test_part_1("#1 @ 1,3: 4x4\n#2 @ 3,1: 4x4\n#3 @ 5,5: 2x2", 4);
    }

    #[test]
    fn test_part_2() {
        Day::test_part_2("#1 @ 1,3: 4x4\n#2 @ 3,1: 4x4\n#3 @ 5,5: 2x2", 3);
    }
}
