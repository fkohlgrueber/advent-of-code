use regex::Regex;
use std::cmp::{Ordering, max};
use std::collections::{BinaryHeap, HashMap};

pub fn calc(input: &str) -> (String, String) {
    (part_1(input).to_string(), part_2(input).to_string())
}

fn part_1(input: &str) -> i32 {
    let (depth, target) = parse_input(input);
    let width = target.0 + 1;
    let height = target.1 + 1;

    let mut cave = Cave::new(target, depth, width, height);
    cave.fill();

    cave.grid
        .iter()
        .flatten()
        .map(|x| match x {
            Type::Rocky => 0,
            Type::Wet => 1,
            Type::Narrow => 2,
        })
        .sum()
}

fn part_2(input: &str) -> usize {
    let (depth, target) = parse_input(input);
    let width = (max(target.0, target.1)) * 8;
    let height = (max(target.0, target.1)) * 8;

    let mut cave = Cave::new(target, depth, width, height);
    cave.fill();

    //cave._print();

    // moving to adj region -> 1 min (only if tool is valid for new region)
    // switching tool in current region -> 7 min (only if new tool is valid for current region)

    let mut heap = BinaryHeap::new();
    let mut distances = HashMap::new();

    // push start node
    heap.push(Node {
        x: 0,
        y: 0,
        tool: Tool::Torch,
        d: 0,
    });

    distances.insert((0, 0, Tool::Torch), 0);

    loop {
        let elmt = heap.pop().unwrap();
        // return if we're looking at the target element
        if elmt.x == target.0 && elmt.y == target.1 && elmt.tool == Tool::Torch {
            return elmt.d;
        }

        let elmt_region_type = &cave.grid[elmt.y][elmt.x];
        let other_possible_tool = match elmt_region_type {
            Type::Rocky => {
                if elmt.tool == Tool::Torch {
                    Tool::ClimbingGear
                } else {
                    Tool::Torch
                }
            }
            Type::Narrow => {
                if elmt.tool == Tool::Torch {
                    Tool::Neither
                } else {
                    Tool::Torch
                }
            }
            Type::Wet => {
                if elmt.tool == Tool::Neither {
                    Tool::ClimbingGear
                } else {
                    Tool::Neither
                }
            }
        };

        // Process case of changing the tool
        if distances
            .get(&(elmt.x, elmt.y, other_possible_tool))
            .unwrap_or(&usize::max_value())
            <= &(elmt.d + 7)
        {
            // position equipped with other tool can be achieved in shorter time already
        } else {
            heap.push(Node {
                d: elmt.d + 7,
                tool: other_possible_tool,
                ..elmt
            });
            distances.insert((elmt.x, elmt.y, other_possible_tool), elmt.d + 7);
        }

        // Process possible neighbors
        for (dx, dy) in &[(-1i32, 0i32), (1, 0), (0, -1), (0, 1)] {
            let (x, y) = (elmt.x as i32 - dy, elmt.y as i32 - dx);
            if x >= 0
                && y >= 0
                && match (&cave.grid[y as usize][x as usize], &elmt.tool) {
                    (Type::Rocky, Tool::Neither) => false,
                    (Type::Narrow, Tool::ClimbingGear) => false,
                    (Type::Wet, Tool::Torch) => false,
                    _ => true,
                }
            {

                if distances
                    .get(&(x as usize, y as usize, elmt.tool))
                    .unwrap_or(&usize::max_value())
                    <= &(elmt.d + 1)
                {
                    // position equipped with other tool can be achieved in shorter time already
                } else {
                    heap.push(Node {
                        x: x as usize,
                        y: y as usize,
                        d: elmt.d + 1,
                        ..elmt
                    });
                    distances.insert((x as usize, y as usize, elmt.tool), elmt.d + 1);
                }
            }
        }
    }
}

#[derive(PartialEq, Eq, Debug)]
struct Node {
    x: usize,
    y: usize,
    tool: Tool,
    d: usize,
}

impl Ord for Node {
    fn cmp(&self, other: &Node) -> Ordering {
        self.d.cmp(&other.d).reverse()
    }
}

impl PartialOrd for Node {
    fn partial_cmp(&self, other: &Node) -> Option<Ordering> {
        Some(self.cmp(&other))
    }
}

#[derive(Eq, PartialEq, Hash, Copy, Clone, Debug)]
enum Tool {
    Torch,
    ClimbingGear,
    Neither,
}

#[derive(Clone)]
enum Type {
    Rocky,
    Narrow,
    Wet,
}

struct Cave {
    grid: Vec<Vec<Type>>,
    geo_idx: Vec<Vec<usize>>,
    target: (usize, usize),
    depth: usize,
}

impl Cave {
    fn new(target: (usize, usize), depth: usize, width: usize, height: usize) -> Cave {
        Cave {
            grid: vec![vec![Type::Rocky; width]; height],
            geo_idx: vec![vec![0; width]; height],
            target,
            depth,
        }
    }

    fn fill(&mut self) {
        for y in 0..self.grid.len() {
            for x in 0..self.grid[0].len() {
                self.grid[y][x] = self.get_type(x, y);
            }
        }
    }

    fn get_type(&mut self, x: usize, y: usize) -> Type {
        let geo_index = match (x, y) {
            (0, 0) => 0,
            t if t == self.target => 0,
            (x, 0) => x * 16807,
            (0, y) => y * 48271,
            (x, y) => ((self.geo_idx[y - 1][x] % 20183) * (self.geo_idx[y][x - 1] % 20183)) % 20183,
        };
        let erosion_lvl = (geo_index + self.depth) % 20183;
        self.geo_idx[y][x] = erosion_lvl;
        match erosion_lvl % 3 {
            0 => Type::Rocky,
            1 => Type::Wet,
            2 => Type::Narrow,
            _ => panic!("Impossible"),
        }
    }

    fn _print(&self) {
        for row in &self.grid {
            println!(
                "{}",
                row.iter()
                    .map(|x| match x {
                        Type::Rocky => '.',
                        Type::Wet => '=',
                        Type::Narrow => '|',
                    })
                    .collect::<String>()
            )
        }
    }
}

fn parse_input(input: &str) -> (usize, (usize, usize)) {
    let re = Regex::new(r"depth: (\d+)\ntarget: (\d+),(\d+)").unwrap();
    let cap = re.captures(input).unwrap();
    let depth = cap[1].parse().unwrap();
    let target = (cap[2].parse().unwrap(), cap[3].parse().unwrap());
    (depth, target)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_1() {
        assert_eq!(part_1("depth: 510\ntarget: 10,10"), 114);
    }

    #[test]
    fn test_part_2() {
        assert_eq!(part_2("depth: 510\ntarget: 10,10"), 45);
    }
}
