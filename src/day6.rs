
#[allow(unused_imports)]
use aoc_tools::prelude::*;

pub fn calc(input: &str) -> (String, String) {
    (part_1(input).to_string(), part_2(input, 10000).to_string())
}

fn part_1(input: &str) -> i32 {
    let points: Vec<Point> = input
        .lines()
        .map(|s| Point::from_str(s))
        .collect();
    closest(&points) as i32
}

fn part_2(input: &str, threshold: usize) -> i32 {
    let points: Vec<Point> = input
        .lines()
        .map(|s| Point::from_str(s))
        .collect();
    
    let min_y = points.iter().map(|p| p.y).min().unwrap_or(0);
    let min_x = points.iter().map(|p| p.x).min().unwrap_or(0);
    let max_y = points.iter().map(|p| p.y).max().unwrap_or(0);
    let max_x = points.iter().map(|p| p.x).max().unwrap_or(0);

    let mut num_coords = 0;
    for y in min_y..=max_y {
        for x in min_x..=max_x {
            let coord = Point { x, y };
            let distances: Vec<_> = points.iter().map(|x| x.distance_to(&coord)).collect();
            let total_distance: usize = distances.iter().sum();
            if total_distance < threshold {
                num_coords += 1;
            }
        }
    }
    num_coords
}

#[derive(PartialEq, Eq, Hash)]
struct Point {
    x: usize,
    y: usize,
}

impl Point {
    fn from_str(s: &str) -> Point {
        let coords: Vec<usize> = s.split(", ").map(|x| x.parse().unwrap()).collect();
        Point {
            x: coords[0],
            y: coords[1],
        }
    }

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

fn closest(points: &[Point]) -> usize{
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
        assert_eq!(part_1("1, 1\n1, 6\n8, 3\n3, 4\n5, 5\n8, 9"), 17);
    }

    #[test]
    fn test_part_2() {
        assert_eq!(part_2("1, 1\n1, 6\n8, 3\n3, 4\n5, 5\n8, 9", 32), 16);
    }
}
