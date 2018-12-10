use regex::Regex;
use lazy_static::lazy_static;

pub fn calc(input: &str) -> (String, String) {
    let mut p1 = String::from("\n");
    p1.push_str(&part_1(input));
    (p1, part_2(input).to_string())
}

fn part_1(input: &str) -> String {
    let points : Vec<Point> = input.lines().map(Point::from_str).collect();
    let (i, bb) = calc_smallest_bb(&points);

    // generate picture
    let mut grid : Vec<Vec<bool>> = vec![vec![false; bb.width() as usize]; bb.heigth() as usize];
    for p in &points {
        let coord = p.get(i);
        grid[(coord.y - bb.min.y) as usize][(coord.x - bb.min.x) as usize] = true;
    }
    grid.iter().map(|row| row.iter().map(|x| {if *x {'█'} else {' '}}).collect::<String>()).collect::<Vec<String>>().join("\n")
}

fn part_2(input: &str) -> i32 {
    let points : Vec<Point> = input.lines().map(Point::from_str).collect();
    let (i, _) = calc_smallest_bb(&points);
    i
}

fn calc_smallest_bb(points: &[Point]) -> (i32, BoundingBox) {
    let mut maybe_bb_old : Option<BoundingBox> = None;
    let mut i = 1;
    loop {
        let mut bb = BoundingBox::new();

        for p in points {
            bb.update(&p.get(i));
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

#[derive(Clone, Copy)]
struct Coord {
    x: i32,
    y: i32
}

impl Coord {
    fn new(x: i32, y: i32) -> Coord {
        Coord { x, y }
    }

    fn min_inplace(&mut self, other: &Coord){
        self.x = std::cmp::min(self.x, other.x);
        self.y = std::cmp::min(self.y, other.y);
    }

    fn max_inplace(&mut self, other: &Coord){
        self.x = std::cmp::max(self.x, other.x);
        self.y = std::cmp::max(self.y, other.y);
    }
}

#[derive(Clone, Copy)]
struct Point {
    position: Coord,
    velocity: Coord,
}

impl Point {
    fn from_str(input: &str) -> Point {
        lazy_static! {
            static ref RE : Regex = Regex::new(r"position=< *(.+), *(.+)> velocity=< *(.+), *(.+)>").unwrap();
        }
        let cap = RE.captures(input).unwrap();
        Point {
            position: Coord::new(cap[1].parse().unwrap(), cap[2].parse().unwrap()),
            velocity: Coord::new(cap[3].parse().unwrap(), cap[4].parse().unwrap()),
        }
    }

    fn get(&self, i: i32) -> Coord {
        Coord {
            x: self.position.x + self.velocity.x * i,
            y: self.position.y + self.velocity.y * i,
        }
    }
}

#[derive(Clone, Copy)]
struct BoundingBox {
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

    fn update(&mut self, p: &Coord) {
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
        assert_eq!(
            part_1("position=< 9,  1> velocity=< 0,  2>\n\
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
                    position=<-3,  6> velocity=< 2, -1>"), 
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
        assert_eq!(
            part_2("position=< 9,  1> velocity=< 0,  2>\n\
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
                    position=<-3,  6> velocity=< 2, -1>"), 
            3
        );
    }
}
