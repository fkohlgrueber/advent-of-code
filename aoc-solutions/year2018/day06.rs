#[allow(unused_imports)]
use aoc_tools::prelude::*;

pub struct Day();

impl Challenge for Day {
    type Input = Vec<Point>;

    fn parse(input: String) -> Self::Input {
        Point::from_str_multiple(&input)
    }

    fn part_1(input: Self::Input) -> String {
        closest(&input).to_string()
    }
    
    fn part_2(input: Self::Input) -> String {
        Self::_part_2(input, 10000)
    }
}

impl Day {
    fn _part_2(input: Vec<Point>, threshold: usize) -> String {
        let min_y = input.iter().map(|p| p.y).min().unwrap_or(0);
        let min_x = input.iter().map(|p| p.x).min().unwrap_or(0);
        let max_y = input.iter().map(|p| p.y).max().unwrap_or(0);
        let max_x = input.iter().map(|p| p.x).max().unwrap_or(0);

        let mut num_coords = 0;
        for y in min_y..=max_y {
            for x in min_x..=max_x {
                let coord = Point { x, y };
                let distances: Vec<_> = input.iter().map(|x| x.distance_to(&coord)).collect();
                let total_distance: usize = distances.iter().sum();
                if total_distance < threshold {
                    num_coords += 1;
                }
            }
        }
        num_coords.to_string()
    }
}

#[parse("{}, {}")]
#[derive(PartialEq, Eq, Hash, Clone)]
pub struct Point {
    x: usize,
    y: usize,
}

impl Point {
    fn distance_to(&self, other: &Point) -> usize {
        abs_diff(self.x, other.x) + abs_diff(self.y, other.y)
    }
}

fn abs_diff(a: usize, b: usize) -> usize {
    if a > b {
        a - b
    } else {
        b - a
    }
}

fn closest(points: &[Point]) -> usize {
    let min_y = points.iter().map(|p| p.y).min().unwrap_or(0);
    let min_x = points.iter().map(|p| p.x).min().unwrap_or(0);
    let max_y = points.iter().map(|p| p.y).max().unwrap_or(0);
    let max_x = points.iter().map(|p| p.x).max().unwrap_or(0);

    let mut border_points = HashSet::new();
    let mut num_closest: HashMap<&Point, usize> = HashMap::new();
    for y in min_y..=max_y {
        for x in min_x..=max_x {
            let coord = Point { x, y };
            let distances: Vec<_> = points.iter().map(|x| x.distance_to(&coord)).collect();
            let min_distance = *distances.iter().min().unwrap();
            let min_dist_points: Vec<_> = points.iter().zip(distances).filter(|pair| pair.1 == min_distance).collect();
            if min_dist_points.len() == 1{
                let point = min_dist_points[0].0;
                let entry = num_closest.entry(point).or_insert(0);
                *entry += 1;
                if y == min_y || y == max_y || x == min_x || x == max_x {
                    border_points.insert(point);
                }
            }
        }
    }

    num_closest.keys().filter(|k| !border_points.contains(*k)).map(|k| num_closest[k]).max().unwrap()
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_1() {
        Day::test_part_1("1, 1\n1, 6\n8, 3\n3, 4\n5, 5\n8, 9", 17);
    }

    #[test]
    fn test_part_2() {
        assert_eq!(Day::_part_2(
            Day::parse("1, 1\n1, 6\n8, 3\n3, 4\n5, 5\n8, 9".to_string()), 32
        ), "16");
    }
}
