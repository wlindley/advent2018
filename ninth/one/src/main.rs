use std::collections::HashMap;
use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;

fn main() {
    let (players, marbles) = parse_line(&read_input());
    let mut game = Game::new(players, marbles);
    game.play();
    println!("high score {}", game.high_score());
}

fn read_input() -> String {
    let f = File::open("input.txt").expect("could not find file");
    let mut r = BufReader::new(&f);
    let mut contents = String::new();
    r.read_to_string(&mut contents);
    return contents;
}

fn parse_line(input: &String) -> (u32, u32) {
    let tokens: Vec<String> = input.split_whitespace().map(String::from).collect();
    return (tokens[0].parse().unwrap(), tokens[6].parse().unwrap());
}

fn wrapping_insert(vector: &mut Vec<u32>, mut index: usize, value: u32) {
    index = wrap_index(&vector, index, 0);
    vector.insert(index, value);
}

fn wrap_index(vector: &Vec<u32>, mut index: usize, delta: i32) -> usize {
    index += (vector.len() as i32 + delta) as usize;
    while index > vector.len() {
        index -= vector.len();
    }
    return index;
}

type Marble = u32;
type Player = u32;
type Score = u32;

struct Game {
    num_players: u32,
    max_marble: Marble,
    pub marbles: Vec<Marble>,
    next_marble: Marble,
    cur_index: usize,
    cur_player: Player,
    scores: HashMap<Player, Score>,
}

impl Game {
    pub fn new(num_players: u32, max_marble: Marble) -> Game {
        return Game {
            num_players: num_players,
            max_marble: max_marble,
            marbles: vec![0],
            next_marble: 1,
            cur_index: 0,
            cur_player: 0,
            scores: HashMap::new(),
        };
    }

    pub fn play(&mut self) {
        for _ in 0..self.max_marble + 1 {
            self.place_marble();
        }
    }

    fn place_marble(&mut self) {
        if self.next_marble % 23 == 0 {
            self.cur_index = wrap_index(&self.marbles, self.cur_index, -7);
            let marble = self.marbles.remove(self.cur_index);
            *self.scores.entry(self.cur_player + 1).or_insert(0) += self.next_marble + marble;
        } else {
            self.cur_index = wrap_index(&self.marbles, self.cur_index, 2);
            self.marbles.insert(self.cur_index, self.next_marble);
        }
        self.next_marble += 1;
        self.cur_player = (self.cur_player + 1) % self.num_players;
    }

    pub fn scores(&self) -> HashMap<Player, Score> {
        return self.scores.clone();
    }

    pub fn high_score(&self) -> Score {
        let mut max = 0;
        for (_, score) in &self.scores {
            max = std::cmp::max(max, *score);
        }
        return max;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_line() {
        assert_eq!(
            (426, 72058),
            parse_line(&String::from(
                "426 players; last marble is worth 72058 points"
            ))
        );
    }

    #[test]
    fn test_wrapping_insert() {
        let mut marbles = vec![0, 1, 2, 3];
        wrapping_insert(&mut marbles, 4, 4);
        assert_eq!(vec![0, 1, 2, 3, 4], marbles);
        wrapping_insert(&mut marbles, 2, 5);
        assert_eq!(vec![0, 1, 5, 2, 3, 4], marbles);
        wrapping_insert(&mut marbles, 1, 6);
        assert_eq!(vec![0, 6, 1, 5, 2, 3, 4], marbles);
        wrapping_insert(&mut marbles, 8, 7);
        assert_eq!(vec![0, 7, 6, 1, 5, 2, 3, 4], marbles);
    }

    #[test]
    fn test_example_game() {
        let mut game = Game::new(9, 25);
        assert_eq!(vec![0], game.marbles);
        game.place_marble();
        assert_eq!(vec![0, 1], game.marbles);
        game.place_marble();
        assert_eq!(vec![0, 2, 1], game.marbles);
        game.place_marble();
        assert_eq!(vec![0, 2, 1, 3], game.marbles);
        game.place_marble();
        assert_eq!(vec![0, 4, 2, 1, 3], game.marbles);
        game.place_marble();
        assert_eq!(vec![0, 4, 2, 5, 1, 3], game.marbles);
        game.place_marble();
        assert_eq!(vec![0, 4, 2, 5, 1, 6, 3], game.marbles);
        game.place_marble();
        assert_eq!(vec![0, 4, 2, 5, 1, 6, 3, 7], game.marbles);
        game.place_marble();
        assert_eq!(vec![0, 8, 4, 2, 5, 1, 6, 3, 7], game.marbles);
        game.place_marble();
        assert_eq!(vec![0, 8, 4, 9, 2, 5, 1, 6, 3, 7], game.marbles);
        game.place_marble();
        assert_eq!(vec![0, 8, 4, 9, 2, 10, 5, 1, 6, 3, 7], game.marbles);
        game.place_marble();
        game.place_marble();
        game.place_marble();
        game.place_marble();
        game.place_marble(); // 15
        assert_eq!(
            vec![0, 8, 4, 9, 2, 10, 5, 11, 1, 12, 6, 13, 3, 14, 7, 15],
            game.marbles
        );
        game.place_marble();
        game.place_marble();
        game.place_marble();
        game.place_marble();
        game.place_marble(); // 20
        assert_eq!(
            vec![0, 16, 8, 17, 4, 18, 9, 19, 2, 20, 10, 5, 11, 1, 12, 6, 13, 3, 14, 7, 15],
            game.marbles
        );
        game.place_marble();
        game.place_marble();
        assert_eq!(
            vec![0, 16, 8, 17, 4, 18, 9, 19, 2, 20, 10, 21, 5, 22, 11, 1, 12, 6, 13, 3, 14, 7, 15],
            game.marbles
        );
        game.place_marble(); // 23, special rules trigger
        assert_eq!(
            vec![0, 16, 8, 17, 4, 18, 19, 2, 20, 10, 21, 5, 22, 11, 1, 12, 6, 13, 3, 14, 7, 15],
            game.marbles
        );
        game.place_marble();
        assert_eq!(
            vec![0, 16, 8, 17, 4, 18, 19, 2, 24, 20, 10, 21, 5, 22, 11, 1, 12, 6, 13, 3, 14, 7, 15],
            game.marbles
        );
        game.place_marble();
        assert_eq!(
            vec![
                0, 16, 8, 17, 4, 18, 19, 2, 24, 20, 25, 10, 21, 5, 22, 11, 1, 12, 6, 13, 3, 14, 7,
                15
            ],
            game.marbles
        );

        let scores = game.scores();
        assert_eq!(32, scores[&5]);
    }

    #[test]
    fn test_additional_examples() {
        let mut game = Game::new(9, 25);
        game.play();
        assert_eq!(32, game.high_score());

        let mut game = Game::new(10, 1618);
        game.play();
        assert_eq!(8317, game.high_score());

        let mut game = Game::new(13, 7999);
        game.play();
        assert_eq!(146373, game.high_score());

        let mut game = Game::new(17, 1104);
        game.play();
        assert_eq!(2764, game.high_score());

        let mut game = Game::new(21, 6111);
        game.play();
        assert_eq!(54718, game.high_score());

        let mut game = Game::new(30, 5807);
        game.play();
        assert_eq!(37305, game.high_score());
    }
}
