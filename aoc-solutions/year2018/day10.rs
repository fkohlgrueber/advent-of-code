#[allow(unused_imports)]
use aoc_tools::prelude::*;

pub struct Day();

impl Challenge for Day {
    type Input = Vec<Point>;

    fn parse(input: String) -> Self::Input {
        Point::from_str_multiple(&input)
    }

    fn part_1(input: Self::Input) -> String {
        let (i, bb) = calc_smallest_bb(&input);

        // generate picture
        let mut grid: Vec<Vec<bool>> = vec![vec![false; bb.width() as usize]; bb.heigth() as usize];
        for p in &input {
            let coord = p.get(i);
            grid[(coord.y - bb.min.y) as usize][(coord.x - bb.min.x) as usize] = true;
        }
        grid.iter().map(|row| row.iter().map(|x| {if *x {'█'} else {' '}}).collect::<String>()).collect::<Vec<String>>().join("\n")
    }
    
    fn part_2(input: Self::Input) -> String {
        let (i, _) = calc_smallest_bb(&input);
        i.to_string()
    }

    /// The result of part 1 needs to be interpreted, which I did without automating the task.
    /// This method maps the result (an image) to the interpreted String which is expected by
    /// adventofcode.com.
    /// UpTheAnte: Do this using OCR so that the solution works on all inputs.
    fn part_1_cvt(s: String) -> String {
        let exp = "█████   █    █     ███  █    █     ███    ██    ██████  █    █\n\
                   █    █  █    █      █   █    █      █    █  █   █       █    █\n\
                   █    █   █  █       █    █  █       █   █    █  █        █  █ \n\
                   █    █   █  █       █    █  █       █   █    █  █        █  █ \n\
                   █████     ██        █     ██        █   █    █  █████     ██  \n\
                   █    █    ██        █     ██        █   ██████  █         ██  \n\
                   █    █   █  █       █    █  █       █   █    █  █        █  █ \n\
                   █    █   █  █   █   █    █  █   █   █   █    █  █        █  █ \n\
                   █    █  █    █  █   █   █    █  █   █   █    █  █       █    █\n\
                   █████   █    █   ███    █    █   ███    █    █  ██████  █    █";
        match s {
            s if &s == exp => "BXJXJAEX".to_string(),
            other => other,
        }
    }
}

fn calc_smallest_bb(points: &[Point]) -> (i32, BoundingBox) {
    let mut maybe_bb_old : Option<BoundingBox> = None;
    let mut i = 1;
    loop {
        let mut bb = BoundingBox::new();

        for p in points {
            bb.update(p.get(i));
        }

        if let Some(bb_old) = maybe_bb_old {
            if bb.width() > bb_old.width() && bb.heigth() > bb_old.heigth() {
                break;
            }
        }
        maybe_bb_old = Some(bb);
        i += 1;
    }

    (i-1, maybe_bb_old.unwrap())
}

#[parse("< *{}, *{}>")]
#[derive(Clone, Copy)]
pub struct Coord {
    x: i32,
    y: i32
}

impl Coord {
    fn new(x: i32, y: i32) -> Coord {
        Coord { x, y }
    }

    fn min_inplace(&mut self, other: Coord){
        self.x = std::cmp::min(self.x, other.x);
        self.y = std::cmp::min(self.y, other.y);
    }

    fn max_inplace(&mut self, other: Coord){
        self.x = std::cmp::max(self.x, other.x);
        self.y = std::cmp::max(self.y, other.y);
    }
}

#[parse("position={} velocity={}")]
#[derive(Clone, Copy)]
pub struct Point {
    #[parse = "<.*>"]
    position: Coord,
    #[parse = "<.*>"]
    velocity: Coord,
}

impl Point {
    fn get(&self, i: i32) -> Coord {
        Coord {
            x: self.position.x + self.velocity.x * i,
            y: self.position.y + self.velocity.y * i,
        }
    }
}

#[derive(Clone, Copy)]
pub struct BoundingBox {
    min: Coord,
    max: Coord,
}

impl BoundingBox {
    fn new() -> BoundingBox {
        BoundingBox {
            min: Coord::new(std::i32::MAX, std::i32::MAX),
            max: Coord::new(std::i32::MIN, std::i32::MIN),
        }
    }

    fn update(&mut self, p: Coord) {
        self.min.min_inplace(p);
        self.max.max_inplace(p);
    }

    fn width(&self) -> i32 {
        self.max.x - self.min.x + 1
    }

    fn heigth(&self) -> i32 {
        self.max.y - self.min.y + 1
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_1() {
        Day::test_part_1(
            "position=< 9,  1> velocity=< 0,  2>\n\
            position=< 7,  0> velocity=<-1,  0>\n\
            position=< 3, -2> velocity=<-1,  1>\n\
            position=< 6, 10> velocity=<-2, -1>\n\
            position=< 2, -4> velocity=< 2,  2>\n\
            position=<-6, 10> velocity=< 2, -2>\n\
            position=< 1,  8> velocity=< 1, -1>\n\
            position=< 1,  7> velocity=< 1,  0>\n\
            position=<-3, 11> velocity=< 1, -2>\n\
            position=< 7,  6> velocity=<-1, -1>\n\
            position=<-2,  3> velocity=< 1,  0>\n\
            position=<-4,  3> velocity=< 2,  0>\n\
            position=<10, -3> velocity=<-1,  1>\n\
            position=< 5, 11> velocity=< 1, -2>\n\
            position=< 4,  7> velocity=< 0, -1>\n\
            position=< 8, -2> velocity=< 0,  1>\n\
            position=<15,  0> velocity=<-2,  0>\n\
            position=< 1,  6> velocity=< 1,  0>\n\
            position=< 8,  9> velocity=< 0, -1>\n\
            position=< 3,  3> velocity=<-1,  1>\n\
            position=< 0,  5> velocity=< 0, -1>\n\
            position=<-2,  2> velocity=< 2,  0>\n\
            position=< 5, -2> velocity=< 1,  2>\n\
            position=< 1,  4> velocity=< 2,  1>\n\
            position=<-2,  7> velocity=< 2, -2>\n\
            position=< 3,  6> velocity=<-1, -1>\n\
            position=< 5,  0> velocity=< 1,  0>\n\
            position=<-6,  0> velocity=< 2,  0>\n\
            position=< 5,  9> velocity=< 1, -2>\n\
            position=<14,  7> velocity=<-2,  0>\n\
            position=<-3,  6> velocity=< 2, -1>", 
            "█   █  ███\n\
             █   █   █ \n\
             █   █   █ \n\
             █████   █ \n\
             █   █   █ \n\
             █   █   █ \n\
             █   █   █ \n\
             █   █  ███"
        );
    }

    #[test]
    fn test_part_2() {
        Day::test_part_2(
            "position=< 9,  1> velocity=< 0,  2>\n\
            position=< 7,  0> velocity=<-1,  0>\n\
            position=< 3, -2> velocity=<-1,  1>\n\
            position=< 6, 10> velocity=<-2, -1>\n\
            position=< 2, -4> velocity=< 2,  2>\n\
            position=<-6, 10> velocity=< 2, -2>\n\
            position=< 1,  8> velocity=< 1, -1>\n\
            position=< 1,  7> velocity=< 1,  0>\n\
            position=<-3, 11> velocity=< 1, -2>\n\
            position=< 7,  6> velocity=<-1, -1>\n\
            position=<-2,  3> velocity=< 1,  0>\n\
            position=<-4,  3> velocity=< 2,  0>\n\
            position=<10, -3> velocity=<-1,  1>\n\
            position=< 5, 11> velocity=< 1, -2>\n\
            position=< 4,  7> velocity=< 0, -1>\n\
            position=< 8, -2> velocity=< 0,  1>\n\
            position=<15,  0> velocity=<-2,  0>\n\
            position=< 1,  6> velocity=< 1,  0>\n\
            position=< 8,  9> velocity=< 0, -1>\n\
            position=< 3,  3> velocity=<-1,  1>\n\
            position=< 0,  5> velocity=< 0, -1>\n\
            position=<-2,  2> velocity=< 2,  0>\n\
            position=< 5, -2> velocity=< 1,  2>\n\
            position=< 1,  4> velocity=< 2,  1>\n\
            position=<-2,  7> velocity=< 2, -2>\n\
            position=< 3,  6> velocity=<-1, -1>\n\
            position=< 5,  0> velocity=< 1,  0>\n\
            position=<-6,  0> velocity=< 2,  0>\n\
            position=< 5,  9> velocity=< 1, -2>\n\
            position=<14,  7> velocity=<-2,  0>\n\
            position=<-3,  6> velocity=< 2, -1>", 
            3
        );
    }
}
