use std::collections::VecDeque;

pub fn calc(input: &str) -> (String, String) {
    (part_1(input).to_string(), part_2(input).to_string())
}

fn part_1(input: &str) -> usize {
    let mut g = Game::new(input, 3);

    // rounds
    for i in 0.. {
        let unit_positions = g.get_units_in_order();

        let mut new_positions = vec!();
        for (y, x) in unit_positions {
            // if another unit moved to this position (because the unit that was 
            // there originally died), it should not take a turn again
            if new_positions.contains(&(y, x)) {continue; }
            // check for end of combat
            if let Some(sum_hit_points) = g.check_end() {
                return i * sum_hit_points;
            }
            // move unit
            let new_pos = g.move_unit(y, x);
            if let Some((y, x)) = new_pos {
                new_positions.push((y, x));
                g.attack(y, x);
            }
            //println!("\n...:");
            //g.print();
        }
        //println!("\n\nAfter Round {}:", i+1);
        //g.print();
    }
    0
}

fn part_2(input: &str) -> usize {
    for power in 4.. {
        println!("Simulating power {}", power);
        let mut g = Game::new(input, power);

        for i in 0.. {
            let units = g.get_units_in_order();
            let mut elf_died = false;
            let mut new_positions = vec!();
            for (y, x) in units {
                if new_positions.contains(&(y, x)) {continue; }
                // check for end of combat
                if let Some(sum_hit_points) = g.check_end() {
                    return i * sum_hit_points;
                }
                let new_pos = g.move_unit(y, x);
                if let Some((y, x)) = new_pos {
                    new_positions.push((y, x));
                    if g.attack(y, x) {
                        println!("Elf died (power={})!", power);
                        elf_died = true;
                        break;
                    }
                }
                //println!("\n...:");
                //g.print();
            }

            if elf_died {
                break;
            }
            //println!("\n\nAfter Round {}:", i+1);
            //g.print();
        }
    }
    0
}

type Pos = (usize, usize);

#[derive(Clone)]
struct Unit {
    hit_points: usize,
}

impl Unit {
    fn new() -> Unit {
        Unit {
            hit_points: 200,
        }
    }
}

#[derive(Clone)]
enum Cell {
    Free,
    Wall,
    Goblin(Unit),
    Elf(Unit)
}

impl Cell {
    fn is_goblin(&self) -> bool {
        match &self {
            Cell::Goblin(_) => true,
            _ => false,
        }
    }

    fn is_unit(&self) -> bool {
        match &self {
            Cell::Goblin(_) | Cell::Elf(_) => true,
            _ => false,
        }
    }

    fn unit(&self) -> &Unit {
        match &self {
            Cell::Goblin(u) | Cell::Elf(u) => u,
            _ => panic!("Cannot extract unit from non-unit variant."),
        }
    }
}

struct Game {
    field: Vec<Vec<Cell>>,
    elf_power: usize,
}

impl Game {
    fn new(input: &str, elf_power: usize) -> Game {
        let field: Vec<Vec<Cell>> = input.lines().map(|x| x.chars().map(|c| {
            match c {
                'G' => Cell::Goblin(Unit::new()),
                'E' => Cell::Elf(Unit::new()),
                '#' => Cell::Wall,
                '.' => Cell::Free,
                _ => panic!("Invalid character in input")
            }
        }
        ).collect::<Vec<Cell>>()).collect();
        Game {
            field,
            elf_power
        }
    }

    fn check_end(&self) -> Option<usize> {
        let units: Vec<&Cell> = self.field.iter().flatten().filter(|x| x.is_unit()).collect();
        if units.iter().all(|x| x.is_goblin()) || !units.iter().any(|x| x.is_goblin()) {
            Some(units.iter().map(|x| x.unit().hit_points).sum())
        } else {
            None
        }
    }

    fn get_units_in_order(&self) -> Vec<Pos> {
        // returns the indices of all units that are still alive in reading order.
        let mut coords : Vec<Pos> = vec!();
        for (y, row) in self.field.iter().enumerate() {
            for (x, e) in row.iter().enumerate() {
                match e {
                    Cell::Goblin(_) | Cell::Elf(_) => coords.push((y, x)),
                    _ => ()
                }
            }
        }
        coords
    }

    fn move_unit(&mut self, y: usize, x: usize) -> Option<Pos> {
        // unit may have been eliminated already, so check
        let unit = &self.field[y][x];
        if let Cell::Free = unit { return None; }
        if let Some((ny, nx)) = self.bfs(y, x) {
            self.field[ny][nx] = self.field[y][x].clone();
            self.field[y][x] = Cell::Free;
            Some((ny, nx))
        }else {
            Some((y, x))
        }
    }

    fn bfs(&self, y: usize, x: usize) -> Option<Pos> {
        let mut seen : Vec<Vec<bool>> = vec![vec![false; self.field[0].len()]; self.field.len()];
        let mut queue = VecDeque::new();

        let start_unit = &self.field[y][x];
        
        queue.push_back(((y, x), None, 0));

        let mut min_dist : Vec<(Pos, Option<Pos>, usize)> = vec!();
        while let Some(((y, x), first_step, num_steps)) = queue.pop_front() {
            if !min_dist.is_empty() && num_steps > min_dist[0].2 { break; }
            if seen[y][x] {continue; }
            seen[y][x] = true;
            let neighbors = [(y-1, x), (y, x-1), (y, x+1), (y+1, x)];
            for n in neighbors.iter() {
                if seen[n.0][n.1] { continue; }
                let next_first_step = match first_step {
                    Some(s) => s,
                    None => *n,
                };
                match (&start_unit, &self.field[n.0][n.1]) {
                    (Cell::Goblin(_), Cell::Elf(_)) | (Cell::Elf(_), Cell::Goblin(_))
                        => { min_dist.push((*n, first_step, num_steps))
                        },
                    (_, Cell::Free) => queue.push_back((*n, Some(next_first_step), num_steps+1)),
                    _ => ()
                }
            }
        }
        if min_dist.is_empty() {
            None
        } else {
            let mut m = min_dist[0];
            for x in min_dist.iter().skip(1) {
                if (x.0).0 < (m.0).0 || ((x.0).0 == (m.0).0 && (x.0).1 < (m.0).1) {
                    m = *x;
                }
            }
            m.1
        }
    }

    fn attack(&mut self, y: usize, x: usize) -> bool {
        let neighbors = [(y-1, x), (y, x-1), (y, x+1), (y+1, x)];

        let power = match &self.field[y][x] {
            Cell::Goblin(_) => 3,
            Cell::Elf(_) => self.elf_power,
            _ => panic!("AAAAAAAAAAAA!"),
        };

        let mut lowest_hitpoints = 1000;
        let mut target = None;
        for (ny, nx) in neighbors.iter() {
            match (&self.field[y][x], &self.field[*ny][*nx]) {
                (Cell::Goblin(_), Cell::Elf(n)) | (Cell::Elf(_), Cell::Goblin(n)) 
                    => if n.hit_points < lowest_hitpoints { 
                        target = Some((*ny, *nx));
                        lowest_hitpoints = n.hit_points;
                    },
                _ => ()
            }
        }
        if let Some((ty, tx)) = target {
            if lowest_hitpoints <= power {
                if let Cell::Elf(_) = &self.field[ty][tx] {
                    self.field[ty][tx] = Cell::Free;
                    return true;
                }
                self.field[ty][tx] = Cell::Free;
            } else {
                match &mut self.field[ty][tx] {
                    Cell::Goblin(n) | Cell::Elf(n) => n.hit_points -= power,
                    _ => ()
                }
            }
        }
        false
    }

    fn _print(&self) {
        for row in &self.field {
            let mut hp : Vec<String> = vec!();
            for n in row {
                match n {
                    Cell::Goblin(x) => hp.push(format!("G({})", x.hit_points)),
                    Cell::Elf(x) => hp.push(format!("E({})", x.hit_points)),
                    _ => ()
                }
            }
            println!("{} {}", row.iter().map(|e| match e {
                Cell::Goblin(_) => 'G',
                Cell::Elf(_) => 'E',
                Cell::Wall => '#',
                Cell::Free => '.',
            }).collect::<String>(), hp.join(", "));
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_units_in_order() {
        let g = Game::new("####\n#.E#\n#GG#\n#E.#\n####", 3);
        assert_eq!(g.get_units_in_order(), vec!((1, 2), (2, 1), (2, 2), (3, 1)));
    }

    #[test]
    fn test_bfs() {
        let g = Game::new("#######\n#E..G.#\n#...#.#\n#.G.#G#\n#######", 3);
        assert_eq!(g.bfs(1, 1), Some((1, 2)));
        let g = Game::new("#######\n#.E...#\n#.....#\n#...G.#\n#######", 3);
        assert_eq!(g.bfs(1, 2), Some((1, 3)));
        let g = Game::new("#######\n#.E...#\n#.G...#\n#...G.#\n#######", 3);
        assert_eq!(g.bfs(1, 2), None);

        let g = Game::new("#######\n\
                           #.E..G#\n\
                           #.#####\n\
                           #G#####\n\
                           #######", 3);
        assert_eq!(g.bfs(1, 2), Some((1, 3)));
    }

    #[test]
    fn test_part_1() {
        assert_eq!(
            part_1("####\n\
                    ##E#\n\
                    #GG#\n\
                    ####"), 
            200*67
        );
        assert_eq!(
            part_1("#######\n\
                    #.G...#\n\
                    #...EG#\n\
                    #.#.#G#\n\
                    #..G#E#\n\
                    #.....#\n\
                    #######"), 
            27730
        );
        assert_eq!(
            part_1("#######\n\
                    #G..#E#\n\
                    #E#E.E#\n\
                    #G.##.#\n\
                    #...#E#\n\
                    #...E.#\n\
                    #######"), 
            36334
        );
        assert_eq!(
            part_1("#######\n\
                    #E..EG#\n\
                    #.#G.E#\n\
                    #E.##E#\n\
                    #G..#.#\n\
                    #..E#.#\n\
                    #######"), 
            39514
        );
        assert_eq!(
            part_1("#######\n\
                    #E.G#.#\n\
                    #.#G..#\n\
                    #G.#.G#\n\
                    #G..#.#\n\
                    #...E.#\n\
                    #######"), 
            27755
        );
        assert_eq!(
            part_1("#######\n\
                    #.E...#\n\
                    #.#..G#\n\
                    #.###.#\n\
                    #E#G#G#\n\
                    #...#G#\n\
                    #######"), 
            28944
        );
        assert_eq!(
            part_1("#########\n\
                    #G......#\n\
                    #.E.#...#\n\
                    #..##..G#\n\
                    #...##..#\n\
                    #...#...#\n\
                    #.G...G.#\n\
                    #.....G.#\n\
                    #########"), 
            18740
        );
        assert_eq!(
            part_1("################################\n\
                    ##############.#################\n\
                    ##########G##....###############\n\
                    #########.....G.################\n\
                    #########...........############\n\
                    #########...........############\n\
                    ##########.....G...#############\n\
                    ###########.........############\n\
                    ########.#.#..#..G....##########\n\
                    #######..........G......########\n\
                    ##..GG..................###.####\n\
                    ##G..........................###\n\
                    ####G.G.....G.#####...E.#.G..###\n\
                    #....##......#######........####\n\
                    #.GG.#####.G#########.......####\n\
                    ###..####...#########..E...#####\n\
                    #...####....#########........###\n\
                    #.G.###.....#########....E....##\n\
                    #..####...G.#########E.....E..##\n\
                    #..###G......#######E.........##\n\
                    #..##.........#####..........###\n\
                    #......................#..E....#\n\
                    ##...G........G.......#...E...##\n\
                    ##............#..........#..####\n\
                    ###.....#...#.##..#......#######\n\
                    #####.###...#######...#..#######\n\
                    #########...E######....#########\n\
                    ###########...######.###########\n\
                    ############..#####..###########\n\
                    #############.E..##.############\n\
                    ################.#..############\n\
                    ################################"),
            250_594
        );
        
    }

    #[test]
    fn test_part_2() {
        assert_eq!(
            part_2("#######\n\
                    #.G...#\n\
                    #...EG#\n\
                    #.#.#G#\n\
                    #..G#E#\n\
                    #.....#\n\
                    #######"), 
            4988
        );

        assert_eq!(
            part_2("################################\n\
                    ##############.#################\n\
                    ##########G##....###############\n\
                    #########.....G.################\n\
                    #########...........############\n\
                    #########...........############\n\
                    ##########.....G...#############\n\
                    ###########.........############\n\
                    ########.#.#..#..G....##########\n\
                    #######..........G......########\n\
                    ##..GG..................###.####\n\
                    ##G..........................###\n\
                    ####G.G.....G.#####...E.#.G..###\n\
                    #....##......#######........####\n\
                    #.GG.#####.G#########.......####\n\
                    ###..####...#########..E...#####\n\
                    #...####....#########........###\n\
                    #.G.###.....#########....E....##\n\
                    #..####...G.#########E.....E..##\n\
                    #..###G......#######E.........##\n\
                    #..##.........#####..........###\n\
                    #......................#..E....#\n\
                    ##...G........G.......#...E...##\n\
                    ##............#..........#..####\n\
                    ###.....#...#.##..#......#######\n\
                    #####.###...#######...#..#######\n\
                    #########...E######....#########\n\
                    ###########...######.###########\n\
                    ############..#####..###########\n\
                    #############.E..##.############\n\
                    ################.#..############\n\
                    ################################"),
            52_133
        );
    }
}
