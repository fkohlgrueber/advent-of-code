use std::ops::{Add, Sub};
use std::cmp::{min, max};

pub fn calc(input: &str) -> (String, String) {
    (part_1(input).to_string(), part_2(input).to_string())
}

fn part_1(input: &str) -> String {
    let (grid, mut carts) = gen_grid_carts(input);
    /*
    println!("{:?}", carts);
    println!("Length: {}", carts.len());
    for line in &grid {
        println!("{}", line.iter().collect::<String>());
    }
    */

    // simulate
    loop {
        // for each cart
        for id in 0..carts.len() {
            // calculate next position
            let new_pos = carts[id].next_coord();
            
            // check that no other cart is at new position
            for c2 in &carts {
                if c2.pos == new_pos {
                    // collision
                    return format!("{},{}", new_pos.x, new_pos.y);
                }
            }

            // move cart
            carts[id].pos = new_pos;

            // update direction based on new coordinate
            carts[id].turn(grid[new_pos.y][new_pos.x]);

            //println!("{:?}", &carts);
        }

        // if all carts have moved, sort the cart list by position and start over
        carts.sort_by_key(|x| x.pos);
    }
}

fn part_2(input: &str) -> String {
    let (grid, mut carts) = gen_grid_carts(input);

    // simulate
    loop {
        // for each cart
        let mut id = 0;
        while id < carts.len() {
            // calculate next position
            let new_pos = carts[id].next_coord();
            
            // check that no other cart is at new position
            let mut id_collision = None;
            for (id2, _) in carts.iter().enumerate() {
                if carts[id2].pos == new_pos {
                    // collision
                    id_collision = Some(id2);
                    break;
                }
            }
            if let Some(id2) = id_collision {
                carts.remove(max(id, id2));
                carts.remove(min(id, id2));
                if id2 < id {
                    id -= 1;
                }
                continue;
            }

            // move cart
            carts[id].pos = new_pos;

            // update direction based on new coordinate
            carts[id].turn(grid[new_pos.y][new_pos.x]);

            //println!("{:?}", &carts);
            id += 1;
        }

        if carts.len() == 1 {
            return format!("{},{}", carts[0].pos.x, carts[0].pos.y);
        }

        // if all carts have moved, sort the cart list by position and start over
        carts.sort_by_key(|x| x.pos);
    }
}

type Grid = Vec<Vec<char>>;

fn gen_grid_carts(input: &str) -> (Grid, Vec<Cart>){
    let mut grid : Vec<Vec<char>> = input.lines().map(|x| x.chars().collect::<Vec<_>>()).collect();
    let mut carts : Vec<Cart> = vec!();
    for (y, line) in grid.iter_mut().enumerate() {
        for (x, c) in line.iter_mut().enumerate() {
            let direction = match c {
                '^' => Some(Direction::North),
                '>' => Some(Direction::East),
                'v' => Some(Direction::South),
                '<' => Some(Direction::West),
                _ => None,
            };
            if let Some(d) = direction {
                // append cart to list
                carts.push(Cart{ pos: Coord{ x, y }, direction: d, next_direction_change: Turn::Left });
                // replace cart character in grid
                *c = match d {
                    Direction::West | Direction::East => '-',
                    Direction::North | Direction::South => '|',
                }
            }
        }
    }
    (grid, carts)
}

#[derive(Debug, Clone, Copy)]
enum Direction {
    North,
    East,
    South,
    West,
}

#[derive(Debug, Clone, Copy)]
enum Turn {
    Right,
    Left,
    Straight,
}

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Clone, Copy)]
struct Coord {
    y: usize,
    x: usize,
}

impl Add for &Coord {
    type Output = Coord;

    fn add(self, other: &Coord) -> Coord {
        Coord {
            x: self.x + other.x,
            y: self.y + other.y,
        }
    }
}

impl Sub for &Coord {
    type Output = Coord;

    fn sub(self, other: &Coord) -> Coord {
        Coord {
            x: self.x - other.x,
            y: self.y - other.y,
        }
    }
}


#[derive(Debug)]
struct Cart {
    pos: Coord,
    direction: Direction,
    next_direction_change: Turn,
}

impl Cart {
    fn next_coord(&self) -> Coord {
        match self.direction {
            Direction::North => &self.pos - &Coord { x: 0, y: 1 },
            Direction::East => &self.pos + &Coord { x: 1, y: 0 },
            Direction::South => &self.pos + &Coord { x: 0, y: 1 },
            Direction::West => &self.pos - &Coord { x: 1, y: 0 },
        }
    }

    fn turn(&mut self, elmt: char) {
        match elmt {
            '+' => self.turn_intersection(),
            '/' => self.direction = match self.direction {
                Direction::North => Direction::East,
                Direction::West => Direction::South,
                Direction::East => Direction::North,
                Direction::South => Direction::West,
            },
            '\\' => self.direction = match self.direction {
                Direction::North => Direction::West,
                Direction::East => Direction::South,
                Direction::West => Direction::North,
                Direction::South => Direction::East,
            },
            _ => ()
        }
    }

    fn turn_intersection(&mut self) {
        // update direction
        self.direction = match (self.next_direction_change, self.direction) {
            (Turn::Left, Direction::North) => Direction::West,
            (Turn::Left, Direction::South) => Direction::East,
            (Turn::Left, Direction::West) => Direction::South,
            (Turn::Left, Direction::East) => Direction::North,
            (Turn::Right, Direction::North) => Direction::East,
            (Turn::Right, Direction::South) => Direction::West,
            (Turn::Right, Direction::West) => Direction::North,
            (Turn::Right, Direction::East) => Direction::South,
            (Turn::Straight, d) => d,
        };
        // update next_direction
        self.next_direction_change = match self.next_direction_change {
            Turn::Left => Turn::Straight,
            Turn::Straight => Turn::Right,
            Turn::Right => Turn::Left,
        };
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_1() {
        assert_eq!(
            part_1(r"/->-\        
|   |  /----\
| /-+--+-\  |
| | |  | v  |
\-+-/  \-+--/
  \------/   "), "7,3");
    }

    #[test]
    fn test_part_2() {
        assert_eq!(part_2(r"/>-<\  
|   |  
| /<+-\
| | | v
\>+</ |
  |   ^
  \<->/"), "6,4");
    }

    #[test]
    fn test_coord() {
        assert_eq!(Coord{x: 1, y: 2}, Coord{x: 1, y: 2});
        assert_ne!(Coord{x: 2, y: 3}, Coord{x: 3, y: 1});
        assert!(Coord{x: 1, y: 2} > Coord{x: 0, y: 2});
        assert!(Coord{x: 1, y: 1} < Coord{x: 0, y: 2});
    }
}
