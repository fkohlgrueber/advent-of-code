#[allow(unused_imports)]
use aoc_tools::prelude::*;

pub struct Day();

impl Challenge for Day {
    type Input = usize;

    fn parse(input: String) -> Self::Input {
        input.trim().parse().unwrap()
    }

    fn part_1(input: Self::Input) -> String {
        let mut width = 1;
        if input == 1 {
            return "0".to_string();
        }
        loop {
            let num_elmts = width * width;
            if num_elmts >= input {
                // this is the right ring
                let first_elmt_on_ring = (width-2)*(width-2) + 1;
                let num_on_ring = num_elmts - first_elmt_on_ring + 1;
                let per_side = num_on_ring / 4;
                let rel_pos = (input - first_elmt_on_ring) % per_side;
                let plus = (rel_pos as i32 - (((per_side + 1) / 2) - 1) as i32).abs();
                return (width / 2 + plus as usize).to_string();
            }
            
            width += 2;
        }
    }
    
    fn part_2(input: Self::Input) -> String {
        let mut grid = HashMap::new();
        grid.insert((0, 0), 1);
        let mut diam = 0;
        let mut x = 0;
        let mut y = 0;
        let mut insert = |x, y| {
            let val = grid.get(&(x+1, y)).unwrap_or(&0) + 
                    grid.get(&(x+1, y+1)).unwrap_or(&0) + 
                    grid.get(&(x, y+1)).unwrap_or(&0) + 
                    grid.get(&(x-1, y+1)).unwrap_or(&0) + 
                    grid.get(&(x-1, y)).unwrap_or(&0) + 
                    grid.get(&(x-1, y-1)).unwrap_or(&0) + 
                    grid.get(&(x, y-1)).unwrap_or(&0) + 
                    grid.get(&(x+1, y-1)).unwrap_or(&0);
            grid.insert((x, y), val);
            val
        };
        loop {
            diam += 1;
            while x < diam {
                x += 1;
                let val = insert(x, y);
                if val > input {
                    return val.to_string()
                }
            }
            while y < diam {
                y += 1;
                let val = insert(x, y);
                if val > input {
                    return val.to_string()
                }
            }
            while x > -diam {
                x -= 1;
                let val = insert(x, y);
                if val > input {
                    return val.to_string()
                }
            }
            while y > -diam {
                y -= 1;
                let val = insert(x, y);
                if val > input {
                    return val.to_string()
                }
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_1() {
        Day::test_part_1("1", 0);
        Day::test_part_1("12", 3);
        Day::test_part_1("23", 2);
        Day::test_part_1("1024", 31);
    }

    #[test]
    fn test_part_2() {
        Day::test_part_2("750", 806);
    }
}
