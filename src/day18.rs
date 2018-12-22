use itertools::iproduct;

pub fn calc(input: &str) -> (String, String) {
    (part_1(input).to_string(), part_2(input).to_string())
}

fn part_1(input: &str) -> usize {
    sim(input, 10)
}

fn part_2(input: &str) -> usize {
    sim(input, 1_000_000_000)
}

fn sim(input: &str, iterations: usize) -> usize{
    let cells : Vec<Vec<char>> = input.lines().map(|x| x.chars().collect()).collect();
    let width = cells[0].len();
    let height = cells.len();
    let mut grid_a = vec![vec![' '; width+2]; height+2];
    let mut grid_b = vec![vec![' '; width+2]; height+2];
    for y in 1..=height {
        grid_a[y][1..=width].clone_from_slice(&cells[y-1][0..width]);
    }

    let mut prev = vec!();

    let mut i = 0;
    while i < iterations {
        let (grid, grid_old) = if i % 2 == 0 {(&mut grid_b, &mut grid_a)} else {(&mut grid_a, &mut grid_b)};
        for y in 1..=height {
            for x in 1..=width {
                grid[y][x] = match grid_old[y][x] {
                    '.' => if iproduct!(y-1..=y+1, x-1..=x+1).filter(|(y, x)| grid_old[*y][*x] == '|').count() >= 3 {'|'} else {'.'},
                    '|' => if iproduct!(y-1..=y+1, x-1..=x+1).filter(|(y, x)| grid_old[*y][*x] == '#').count() >= 3 {'#'} else {'|'},
                    '#' => if iproduct!(y-1..=y+1, x-1..=x+1).filter(|(y, x)| grid_old[*y][*x] == '#').count() >= 2 && iproduct!(y-1..=y+1, x-1..=x+1).filter(|(y, x)| grid_old[*y][*x] == '|').count() >= 1 {'#'} else {'.'},
                    _ => panic!("aaah!"),
                };
            }
        }

        for (idx, g) in prev.iter().enumerate() {
            if g == grid {
                let cycle_len = i - idx;
                i += ((iterations - i) / cycle_len) * cycle_len;
                prev.clear();
                break;
            }
        }
        prev.push(grid.clone());

        /*
        for row in grid.iter() {
            println!("{}", row.iter().collect::<String>());
        }
        println!("\n\n");
        */
        i += 1;
        
    }

    grid_a.iter().flatten().filter(|x| **x == '|').count() * grid_a.iter().flatten().filter(|x| **x == '#').count()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_1() {
        assert_eq!(
            part_1(".#.#...|#.\n\
                    .....#|##|\n\
                    .|..|...#.\n\
                    ..|#.....#\n\
                    #.#|||#|#|\n\
                    ...#.||...\n\
                    .|....|...\n\
                    ||...#|.#|\n\
                    |.||||..|.\n\
                    ...#.|..|."), 1147);
    }
}
