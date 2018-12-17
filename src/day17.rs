use regex::Regex;

pub fn calc(input: &str) -> (String, String) {
    (part_1(input).to_string(), part_2(input).to_string())
}

fn part_1(input: &str) -> usize {
    let grid = simulate_water(input);
    grid.iter().flatten().filter(|x| x.is_water()).count()
}

fn part_2(input: &str) -> usize {
    let grid = simulate_water(input);
    grid.iter().flatten().filter(|x| x.is_still_water()).count()
}

enum Dir {
    Left,
    Right,
    Both
}

fn simulate_water(input: &str) -> Grid {
    let re = Regex::new(r"([xy])=(\d+), [xy]=(\d+)..(\d+)").unwrap();
    let mut clays: Vec<Clay> = vec!();
    for c in re.captures_iter(input) {
        let xy = &c[1];
        let a = c[2].parse().unwrap();
        let b = c[3].parse().unwrap();
        let c = c[4].parse().unwrap();
        clays.push(if xy == "x" {
            Clay(a, a, b, c)
        } else {
            Clay(b, c, a, a)
        });
    }
    let min_x = clays.iter().map(|c| c.0).min().unwrap();
    let min_y = clays.iter().map(|c| c.2).min().unwrap();
    let max_x = clays.iter().map(|c| c.1).max().unwrap();
    let max_y = clays.iter().map(|c| c.3).max().unwrap();

    let mut grid: Grid = vec![vec![Cell::Sand; max_x - min_x + 3]; max_y - min_y + 1];
    
    for c in clays {
        for y in c.2..=c.3 {
            for x in c.0..=c.1 {
                grid[y-min_y][x-min_x+1] = Cell::Clay;
            }
        }
    }

    calc_cell(&mut grid, 500-min_x, 0, &Dir::Both);

    grid
}

fn calc_cell(grid: &mut Grid, x: usize, y: usize, dir: &Dir) -> bool {
    if y == grid.len() { return false }
    match grid[y][x] {
        Cell::Clay | Cell::WaterRest => true,
        Cell::WaterFlow => false,
        Cell::Sand => {
            grid[y][x] = Cell::WaterFlow;
            if !calc_cell(grid, x, y+1, &Dir::Both) {
                return false;
            }
            match dir {
                Dir::Both => {
                    let res_left = calc_cell(grid, x-1, y, &Dir::Left);
                    let res_right = calc_cell(grid, x+1, y, &Dir::Right);
                    if res_left && res_right {
                        fill_left_right(grid, x, y);
                        true
                    } else {
                        false
                    }
                },
                Dir::Left => calc_cell(grid, x-1, y, &Dir::Left),
                Dir::Right => calc_cell(grid, x+1, y, &Dir::Right)
            }
        }
    }
}

fn fill_left_right(grid: &mut Grid, x: usize, y: usize) {
    grid[y][x] = Cell::WaterRest;
    for tx in 1.. {
        if grid[y][x-tx] == Cell::WaterFlow {
            grid[y][x-tx] = Cell::WaterRest;
        } else {
            break;
        }
    }

    for tx in 1.. {
        if grid[y][x+tx] == Cell::WaterFlow {
            grid[y][x+tx] = Cell::WaterRest;
        } else {
            break;
        }
    }
}

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

impl Cell {
    fn is_water(&self) -> bool {
        match self {
            Cell::WaterFlow | Cell::WaterRest => true,
            _ => false
        }
    }

    fn is_still_water(&self) -> bool {
        match self {
            Cell::WaterRest => true,
            _ => false
        }
    }
}

#[derive(Debug)]
struct Clay(usize, usize, usize, usize);

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_1() {
        assert_eq!(
            part_1("x=495, y=2..7\n\
                    y=7, x=495..501\n\
                    x=501, y=3..7\n\
                    x=498, y=2..4\n\
                    x=506, y=1..2\n\
                    x=498, y=10..13\n\
                    x=504, y=10..13\n\
                    y=13, x=498..504"), 
            57);
    }

    #[test]
    fn test_part_2() {
        assert_eq!(
            part_2("x=495, y=2..7\n\
                    y=7, x=495..501\n\
                    x=501, y=3..7\n\
                    x=498, y=2..4\n\
                    x=506, y=1..2\n\
                    x=498, y=10..13\n\
                    x=504, y=10..13\n\
                    y=13, x=498..504"), 
            29);
    }
}
