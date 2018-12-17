use regex::Regex;
use itertools::iproduct;
use std::ops::RangeInclusive;

pub fn calc(input: &str) -> (String, String) {
    let grid = simulate(input);
    // count Water cells
    let (flow, still) = grid.iter().flatten()
        .map(|x| (*x == Cell::WaterFlow, *x == Cell::WaterRest))
        .fold((0, 0), |a, b| (a.0 + b.0 as usize, a.1 + b.1 as usize));
    ((flow+still).to_string(), still.to_string())
}

fn simulate(input: &str) -> Grid {
    // parse input
    let re = Regex::new(r"([xy])=(\d+), [xy]=(\d+)..(\d+)").unwrap();
    let mut clays: Vec<Clay> = vec!();
    for c in re.captures_iter(input) {
        let xy = &c[1];
        let a = c[2].parse().unwrap();
        let b = c[3].parse().unwrap();
        let c = c[4].parse().unwrap();
        clays.push(if xy == "x" {
            Clay { x: a..=a, y: b..=c }
        } else {
            Clay { x: b..=c, y: a..=a }
        });
    }
    let min_x = clays.iter().map(|c| *c.x.start()).min().unwrap();
    let min_y = clays.iter().map(|c| *c.y.start()).min().unwrap();
    let max_x = clays.iter().map(|c| *c.x.end()).max().unwrap();
    let max_y = clays.iter().map(|c| *c.y.end()).max().unwrap();

    // generate grid and set clay cells
    let mut grid: Grid = vec![vec![Cell::Sand; max_x - min_x + 3]; max_y - min_y + 1];    
    clays.into_iter()
         .flat_map(|c| iproduct!(c.y, c.x))
         .for_each(|(y, x)| grid[y - min_y][x - min_x + 1] = Cell::Clay);

    // run the simulation
    calc_cell(&mut grid, 500-min_x, 0, &Dir::Both);

    grid
}

fn calc_cell(grid: &mut Grid, x: usize, y: usize, dir: &Dir) -> Option<usize> {
    if y == grid.len() { return None }
    match grid[y][x] {
        Cell::Clay | Cell::WaterRest => Some(x),
        Cell::WaterFlow => None,
        Cell::Sand => {
            grid[y][x] = Cell::WaterFlow;
            calc_cell(grid, x, y+1, &Dir::Both)?;
            match dir {
                Dir::Both => {
                    match (calc_cell(grid, x-1, y, &Dir::Left), calc_cell(grid, x+1, y, &Dir::Right)) {
                        (Some(l), Some(r)) => {
                            grid[y].iter_mut().skip(l+1).take(r-l-1).for_each(|x| *x = Cell::WaterRest);
                            Some(x)
                        },
                        _ => None
                    }
                },
                Dir::Left => calc_cell(grid, x-1, y, &Dir::Left),
                Dir::Right => calc_cell(grid, x+1, y, &Dir::Right)
            }
        }
    }
}

enum Dir { Left, Right, Both }

type Grid = Vec<Vec<Cell>>;

#[allow(clippy::ptr_arg)]
fn _print_grid(g: &Grid) {
    for row in g {
        println!("{}", row.iter().map(|c| match c {
            Cell::Sand => '.',
            Cell::Clay => '#',
            Cell::WaterFlow => '|',
            Cell::WaterRest => '~',
        }).collect::<String>());
    }
}

#[derive(Clone, PartialEq)]
enum Cell {
    Sand,
    Clay,
    WaterFlow,
    WaterRest,
}

struct Clay {
    x: RangeInclusive<usize>,
    y: RangeInclusive<usize>,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_1() {
        assert_eq!(
            calc("x=495, y=2..7\n\
                  y=7, x=495..501\n\
                  x=501, y=3..7\n\
                  x=498, y=2..4\n\
                  x=506, y=1..2\n\
                  x=498, y=10..13\n\
                  x=504, y=10..13\n\
                  y=13, x=498..504").0, 
            "57");
    }

    #[test]
    fn test_part_2() {
        assert_eq!(
            calc("x=495, y=2..7\n\
                  y=7, x=495..501\n\
                  x=501, y=3..7\n\
                  x=498, y=2..4\n\
                  x=506, y=1..2\n\
                  x=498, y=10..13\n\
                  x=504, y=10..13\n\
                  y=13, x=498..504").1, 
            "29");
    }
}
