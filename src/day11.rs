
const SIZE : usize = 300;

pub fn calc(input: &str) -> (String, String) {
    (part_1(input), part_2(input))
}

fn part_1(input: &str) -> String {
    let grid = gen_grid(input);

    // get best 3x3 patch
    let mut max_val = std::i32::MIN;
    let mut coords = (0, 0);
    for y in 1..=SIZE-2 {
        for x in 1..SIZE-2 {
            let sum = calc_patch_sum(x, y, 3, &grid);
            if sum > max_val {
                max_val = sum;
                coords = (x, y);
            }
        }
    }

    format!("{},{}", coords.0, coords.1)
}

fn part_2(input: &str) -> String {
    let grid = gen_grid(input);

    // get best NxN patch
    let mut max_val = std::i32::MIN;
    let mut coords = (0, 0, 0);
    for n in 1..=300 {
        for y in 1..=SIZE-n+1 {
            for x in 1..=SIZE-n+1 {
                let sum = calc_patch_sum(x, y, n, &grid);
                if sum > max_val {
                    max_val = sum;
                    coords = (x, y, n);
                }
            }
        }
    }

    format!("{},{},{}", coords.0, coords.1, coords.2)
}

fn gen_grid(input: &str) -> [[i32; SIZE]; SIZE] {
    let grid_serial_number : usize = input.parse().unwrap();
    
    let mut grid = [[0i32; SIZE]; SIZE];

    // fill grid
    for y in 1..=SIZE {
        for x in 1..SIZE {
            grid[y-1][x-1] = calc_power_level(x, y, grid_serial_number);
        }
    }

    grid
}

fn calc_power_level(x: usize, y: usize, grid_serial_number: usize) -> i32 {
    let rack_id = x + 10;
    let power_level = (rack_id * y + grid_serial_number) * rack_id;
    let hundrets = (power_level / 100) % 10;
    hundrets as i32 - 5
}

fn calc_patch_sum(x: usize, y: usize, n: usize, grid: &[[i32; SIZE]; SIZE]) -> i32 {
    // x and y are 1-indexed
    let mut sum : i32 = 0;
    for line in grid.iter().skip(y-1).take(n) {
        sum += line.iter().skip(x-1).take(n).sum::<i32>();
    }
    sum
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_calc_power_level() {
        assert_eq!(calc_power_level(3, 5, 8), 4);
        assert_eq!(calc_power_level(122, 79, 57), -5);
        assert_eq!(calc_power_level(217, 196, 39), 0);
        assert_eq!(calc_power_level(101, 153, 71), 4);
    }

    #[test]
    fn test_part_1() {
        assert_eq!(part_1("18"), "33,45");
        assert_eq!(part_1("42"), "21,61");
    }

    #[test]
    fn test_part_2() {
        assert_eq!(part_2("18"), "90,251,12");
        assert_eq!(part_2("42"), "232,251,12");
    }
}
