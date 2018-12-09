use regex::Regex;

pub fn calc(input: &str) -> (String, String) {
    (part_1(input).to_string(), part_2(input).to_string())
}

fn part_1(input: &str) -> usize {
    let mut game = MarbleGame::from_str(input, 1);
    game.calc();
    *game.player_scores.iter().max().unwrap()
}

fn part_2(input: &str) -> usize {
    let mut game = MarbleGame::from_str(input, 100);
    game.calc();
    *game.player_scores.iter().max().unwrap()
}

struct Marble {
    prev: usize,
    next: usize,
}

impl Marble {
    fn new(id: usize) -> Marble {
        Marble {
            prev: id,
            next: id,
        }
    }
}

struct MarbleGame {
    marbles: Vec<Marble>,
    player_scores: Vec<usize>,
    current_marble: usize,
    next_marbles: std::ops::RangeInclusive<usize>,
    player_turns: std::iter::Cycle<std::ops::Range<usize>>,
}

impl MarbleGame {
    fn from_str(input: &str, multiplier: usize) -> MarbleGame {
        let re = Regex::new(r"(\d+) players; last marble is worth (\d+) points").unwrap();
        let cap = re.captures(input).unwrap();
        let num_players : usize = cap[1].parse().unwrap();
        let max_marble : usize = cap[2].parse::<usize>().unwrap() * multiplier;

        MarbleGame{
            marbles: (0..=max_marble).map(Marble::new).collect(),
            player_scores: vec![0; num_players],
            current_marble: 0,
            next_marbles: 1..=max_marble,
            player_turns: (0..num_players).cycle()
        }
    }

    fn calc(&mut self){
        while let Some(new_number) = self.next_marbles.next() {
            let player = self.player_turns.next().unwrap();
            if new_number % 23 == 0 {
                // special case
                let to_remove = self.iter_back().nth(6).unwrap();
                let prev = self.marbles[to_remove].prev;
                let next = self.marbles[to_remove].next;
                self.marbles[prev].next = next;
                self.marbles[next].prev = prev;
                self.player_scores[player] += new_number + to_remove;
                self.current_marble = next;
            } else {
                // normal case
                let a = self.marbles[self.current_marble].next;
                let b = self.marbles[a].next;
                self.marbles[new_number].next = b;
                self.marbles[new_number].prev = a;
                self.marbles[a].next = new_number;
                self.marbles[b].prev = new_number;
                self.current_marble = new_number;
            }
        }
    }

    fn iter_back(&self) -> MarbleIterator {
        MarbleIterator::new(self.current_marble, &self)
    }
}

struct MarbleIterator<'a> {
    current_marble: usize,
    marble_game: &'a MarbleGame,
}

impl<'a> MarbleIterator<'a> {
    fn new(current_marble: usize, marble_game: &MarbleGame) -> MarbleIterator {
        MarbleIterator{
            current_marble,
            marble_game,
        }
    }
}

impl<'a> Iterator for MarbleIterator<'a> {
    type Item = usize;

    fn next(&mut self) -> Option<usize> {
        self.current_marble = self.marble_game.marbles[self.current_marble].prev;
        Some(self.current_marble)
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_1() {
        assert_eq!(part_1("9 players; last marble is worth 25 points"), 32);
        assert_eq!(part_1("10 players; last marble is worth 1618 points"), 8317);
        assert_eq!(part_1("13 players; last marble is worth 7999 points"), 146373);
        assert_eq!(part_1("17 players; last marble is worth 1104 points"), 2764);
        assert_eq!(part_1("21 players; last marble is worth 6111 points"), 54718);
        assert_eq!(part_1("30 players; last marble is worth 5807 points"), 37305);
    }

    #[test]
    fn test_part_2() {
        assert_eq!(part_2(""), 0);
    }
}
