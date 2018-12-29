use std::cmp::Ordering;
use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;

fn main() {
    let input_rows: Vec<(Row, Vec<Cart>)> = read_input()
        .iter()
        .enumerate()
        .map(|item| parse_line(item.0, item.1))
        .collect();
    let (map, carts) = combine_input(input_rows);
    let mut sim = Simulation::new(map, carts);
    let coord = sim.until_crash();
    println!("Crash occurred at {},{}", coord.0, coord.1);
}

type Row = Vec<Cell>;
type Map = Vec<Row>;
type Coord = (usize, usize);

#[derive(Debug, PartialEq, Eq)]
enum Cell {
    Empty,
    Track,
    TurnOne, // Left if going horizontal, right if vertical
    TurnTwo, // Right if going horizontal, left if vertical
    Intersection,
}

#[derive(Debug, PartialEq, Eq)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
}

#[derive(Debug, PartialEq, Eq)]
enum Turn {
    Left,
    Straight,
    Right,
}

impl Turn {
    fn next(&self) -> Self {
        match self {
            Turn::Left => Turn::Straight,
            Turn::Straight => Turn::Right,
            Turn::Right => Turn::Left,
        }
    }
}

#[derive(Debug)]
struct Cart {
    row: usize,
    col: usize,
    dir: Direction,
    next_turn: Turn,
    id: u32,
}

static mut CART_ID: u32 = 0;

impl Cart {
    pub fn new(row: usize, col: usize, dir: Direction) -> Self {
        let mut id = 0;
        unsafe {
            id = CART_ID;
            CART_ID += 1;
        }
        Self {
            row,
            col,
            dir,
            next_turn: Turn::Left,
            id,
        }
    }

    pub fn is_same(&self, other: &Self) -> bool {
        self.id == other.id
    }

    pub fn advance(&mut self, cell: &Cell) {
        match cell {
            Cell::Track => self.track(),
            Cell::TurnOne => self.turn_one(),
            Cell::TurnTwo => self.turn_two(),
            Cell::Intersection => self.intersection(),
            Cell::Empty => panic!("Cart on empty cell"),
        };
    }

    fn track(&mut self) {
        match self.dir {
            Direction::Right => self.col += 1,
            Direction::Left => self.col -= 1,
            Direction::Up => self.row -= 1,
            Direction::Down => self.row += 1,
        }
    }

    fn turn_one(&mut self) {
        match self.dir {
            Direction::Up | Direction::Down => self.turn_right(),
            Direction::Left | Direction::Right => self.turn_left(),
        }
    }

    fn turn_two(&mut self) {
        match self.dir {
            Direction::Up | Direction::Down => self.turn_left(),
            Direction::Left | Direction::Right => self.turn_right(),
        }
    }

    fn turn_left(&mut self) {
        match self.dir {
            Direction::Right => {
                self.dir = Direction::Up;
                self.row -= 1;
            }
            Direction::Left => {
                self.dir = Direction::Down;
                self.row += 1;
            }
            Direction::Up => {
                self.dir = Direction::Left;
                self.col -= 1;
            }
            Direction::Down => {
                self.dir = Direction::Right;
                self.col += 1;
            }
        }
    }

    fn turn_right(&mut self) {
        match self.dir {
            Direction::Right => {
                self.dir = Direction::Down;
                self.row += 1;
            }
            Direction::Left => {
                self.dir = Direction::Up;
                self.row -= 1;
            }
            Direction::Up => {
                self.dir = Direction::Right;
                self.col += 1;
            }
            Direction::Down => {
                self.dir = Direction::Left;
                self.col -= 1;
            }
        }
    }

    fn intersection(&mut self) {
        match self.next_turn {
            Turn::Left => self.turn_left(),
            Turn::Straight => self.track(),
            Turn::Right => self.turn_right(),
        };
        self.next_turn = self.next_turn.next();
    }
}

impl PartialEq for Cart {
    fn eq(&self, other: &Self) -> bool {
        self.row == other.row && self.col == other.col && self.dir == other.dir
    }
}

impl Eq for Cart {}

impl Ord for Cart {
    fn cmp(&self, other: &Self) -> Ordering {
        let result = self.row.cmp(&other.row);
        if result == Ordering::Equal {
            return self.col.cmp(&other.col);
        }
        result
    }
}

impl PartialOrd for Cart {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

struct Simulation {
    map: Map,
    carts: Vec<Cart>,
}

impl Simulation {
    pub fn new(map: Map, carts: Vec<Cart>) -> Self {
        Self { map, carts }
    }

    pub fn ticks(&mut self, ticks: u32) {
        for _ in 0..ticks {
            self.tick();
        }
    }

    pub fn until_crash(&mut self) -> Coord {
        loop {
            match self.tick() {
                None => continue,
                Some(coord) => return coord,
            }
        }
    }

    fn tick(&mut self) -> Option<Coord> {
        self.carts.sort_unstable();
        let num_carts = self.carts.len();
        for i in 0..num_carts {
            let mut cart = &mut self.carts[i];
            match location(&self.map, cart.row, cart.col) {
                None => panic!(
                    "Cart out of bounds @ {},{} going {:?}",
                    cart.row, cart.col, cart.dir
                ),
                Some(cell) => {
                    cart.advance(cell);
                    let coord = (cart.row, cart.col);

                    for j in 0..num_carts {
                        if i == j {
                            continue;
                        }
                        let other = &self.carts[j];
                        if coord.0 == other.row && coord.1 == other.col {
                            return Some(coord);
                        }
                    }
                },
            }
        }
        None
    }

    fn detect_crash(&self) -> Option<Coord> {
        for cart1 in &self.carts {
            for cart2 in &self.carts {
                if cart1.is_same(cart2) {
                    continue;
                }
                match cart1.cmp(cart2) {
                    Ordering::Equal => return Some((cart1.row, cart1.col)),
                    _ => continue,
                }
            }
        }
        None
    }
}

fn location(map: &Map, row: usize, col: usize) -> Option<&Cell> {
    match map.get(row) {
        None => None,
        Some(cells) => cells.get(col),
    }
}

fn read_input() -> Vec<String> {
    let f = File::open("input.txt").expect("could not find file");
    let r = BufReader::new(&f);
    return r.lines().map(|l| l.unwrap()).collect();
}

fn parse_line(row: usize, input: &String) -> (Row, Vec<Cart>) {
    let mut carts: Vec<Cart> = Vec::new();
    let row = input
        .chars()
        .enumerate()
        .map(|(i, c)| match c {
            '-' | '|' => Cell::Track,
            '/' => Cell::TurnOne,
            '\\' => Cell::TurnTwo,
            '+' => Cell::Intersection,
            '<' => {
                carts.push(Cart::new(row, i, Direction::Left));
                Cell::Track
            }
            '>' => {
                carts.push(Cart::new(row, i, Direction::Right));
                Cell::Track
            }
            '^' => {
                carts.push(Cart::new(row, i, Direction::Up));
                Cell::Track
            }
            'v' => {
                carts.push(Cart::new(row, i, Direction::Down));
                Cell::Track
            }
            _ => Cell::Empty,
        })
        .collect();
    return (row, carts);
}

fn combine_input(input_rows: Vec<(Row, Vec<Cart>)>) -> (Map, Vec<Cart>) {
    let mut map: Map = Map::new();
    let mut carts: Vec<Cart> = Vec::new();
    for mut row in input_rows {
        map.push(row.0);
        carts.append(&mut row.1);
    }
    (map, carts)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_line() {
        assert_eq!(
            (
                vec![
                    Cell::TurnOne,
                    Cell::Track,
                    Cell::Track,
                    Cell::Track,
                    Cell::Track,
                    Cell::TurnTwo
                ],
                vec![]
            ),
            parse_line(0, &String::from("/----\\"))
        );
        assert_eq!(
            (
                vec![
                    Cell::TurnTwo,
                    Cell::Track,
                    Cell::Track,
                    Cell::Track,
                    Cell::TurnOne,
                    Cell::Empty,
                    Cell::Empty,
                    Cell::Empty,
                    Cell::Empty,
                    Cell::Empty,
                    Cell::Empty,
                    Cell::Empty,
                    Cell::Empty
                ],
                vec![Cart::new(4, 2, Direction::Right)]
            ),
            parse_line(4, &String::from("\\->-/        "))
        );
        assert_eq!(
            (
                vec![
                    Cell::Track,
                    Cell::Empty,
                    Cell::Empty,
                    Cell::Empty,
                    Cell::Track,
                    Cell::Empty,
                    Cell::Empty,
                    Cell::TurnOne,
                    Cell::Track,
                    Cell::Track,
                    Cell::Track,
                    Cell::Track,
                    Cell::TurnTwo
                ],
                vec![]
            ),
            parse_line(0, &String::from("|   |  /----\\"))
        );
        assert_eq!(
            (
                vec![
                    Cell::Track,
                    Cell::Empty,
                    Cell::TurnOne,
                    Cell::Track,
                    Cell::Intersection,
                    Cell::Track,
                    Cell::Track,
                    Cell::Intersection,
                    Cell::Track,
                    Cell::TurnTwo,
                    Cell::Empty,
                    Cell::Empty,
                    Cell::Track
                ],
                vec![]
            ),
            parse_line(0, &String::from("| /-+--+-\\  |"))
        );
    }

    #[test]
    fn test_track() {
        let mut cart = Cart::new(5, 5, Direction::Right);
        cart.advance(&Cell::Track);
        assert_eq!(Cart::new(5, 6, Direction::Right), cart);

        let mut cart = Cart::new(5, 5, Direction::Left);
        cart.advance(&Cell::Track);
        assert_eq!(Cart::new(5, 4, Direction::Left), cart);

        let mut cart = Cart::new(5, 5, Direction::Up);
        cart.advance(&Cell::Track);
        assert_eq!(Cart::new(4, 5, Direction::Up), cart);

        let mut cart = Cart::new(5, 5, Direction::Down);
        cart.advance(&Cell::Track);
        assert_eq!(Cart::new(6, 5, Direction::Down), cart);
    }

    #[test]
    fn test_turn_one() {
        let mut cart = Cart::new(5, 5, Direction::Right);
        cart.advance(&Cell::TurnOne);
        assert_eq!(Cart::new(4, 5, Direction::Up), cart);

        let mut cart = Cart::new(5, 5, Direction::Left);
        cart.advance(&Cell::TurnOne);
        assert_eq!(Cart::new(6, 5, Direction::Down), cart);

        let mut cart = Cart::new(5, 5, Direction::Up);
        cart.advance(&Cell::TurnOne);
        assert_eq!(Cart::new(5, 6, Direction::Right), cart);

        let mut cart = Cart::new(5, 5, Direction::Down);
        cart.advance(&Cell::TurnOne);
        assert_eq!(Cart::new(5, 4, Direction::Left), cart);
    }

    #[test]
    fn test_turn_two() {
        let mut cart = Cart::new(5, 5, Direction::Right);
        cart.advance(&Cell::TurnTwo);
        assert_eq!(Cart::new(6, 5, Direction::Down), cart);

        let mut cart = Cart::new(5, 5, Direction::Left);
        cart.advance(&Cell::TurnTwo);
        assert_eq!(Cart::new(4, 5, Direction::Up), cart);

        let mut cart = Cart::new(5, 5, Direction::Up);
        cart.advance(&Cell::TurnTwo);
        assert_eq!(Cart::new(5, 4, Direction::Left), cart);

        let mut cart = Cart::new(5, 5, Direction::Down);
        cart.advance(&Cell::TurnTwo);
        assert_eq!(Cart::new(5, 6, Direction::Right), cart);
    }

    #[test]
    fn test_intersection() {
        let mut cart = Cart::new(5, 5, Direction::Right);
        cart.advance(&Cell::Intersection);
        assert_eq!(Cart::new(4, 5, Direction::Up), cart);
        cart.advance(&Cell::Intersection);
        assert_eq!(Cart::new(3, 5, Direction::Up), cart);
        cart.advance(&Cell::Intersection);
        assert_eq!(Cart::new(3, 6, Direction::Right), cart);
        cart.advance(&Cell::Intersection);
        assert_eq!(Cart::new(2, 6, Direction::Up), cart);

        let mut cart = Cart::new(5, 5, Direction::Left);
        cart.advance(&Cell::Intersection);
        assert_eq!(Cart::new(6, 5, Direction::Down), cart);
        cart.advance(&Cell::Intersection);
        assert_eq!(Cart::new(7, 5, Direction::Down), cart);
        cart.advance(&Cell::Intersection);
        assert_eq!(Cart::new(7, 4, Direction::Left), cart);
        cart.advance(&Cell::Intersection);
        assert_eq!(Cart::new(8, 4, Direction::Down), cart);

        let mut cart = Cart::new(5, 5, Direction::Up);
        cart.advance(&Cell::Intersection);
        assert_eq!(Cart::new(5, 4, Direction::Left), cart);
        cart.advance(&Cell::Intersection);
        assert_eq!(Cart::new(5, 3, Direction::Left), cart);
        cart.advance(&Cell::Intersection);
        assert_eq!(Cart::new(4, 3, Direction::Up), cart);
        cart.advance(&Cell::Intersection);
        assert_eq!(Cart::new(4, 2, Direction::Left), cart);

        let mut cart = Cart::new(5, 5, Direction::Down);
        cart.advance(&Cell::Intersection);
        assert_eq!(Cart::new(5, 6, Direction::Right), cart);
        cart.advance(&Cell::Intersection);
        assert_eq!(Cart::new(5, 7, Direction::Right), cart);
        cart.advance(&Cell::Intersection);
        assert_eq!(Cart::new(6, 7, Direction::Down), cart);
        cart.advance(&Cell::Intersection);
        assert_eq!(Cart::new(6, 8, Direction::Right), cart);
    }

    #[test]
    fn test_simulation_ticks() {
        let input_rows: Vec<(Row, Vec<Cart>)> = read_test()
            .iter()
            .enumerate()
            .map(|item| parse_line(item.0, item.1))
            .collect();
        let (map, carts) = combine_input(input_rows);
        let mut sim = Simulation::new(map, carts);

        assert_eq!(2, sim.carts.len());
        assert_eq!(Cart::new(0, 2, Direction::Right), sim.carts[0]);
        assert_eq!(Cart::new(3, 9, Direction::Down), sim.carts[1]);

        sim.ticks(10);

        assert_eq!(2, sim.carts.len());
        assert_eq!(Cart::new(1, 9, Direction::Left), sim.carts[0]);
        assert_eq!(Cart::new(3, 9, Direction::Down), sim.carts[1]);
    }

    #[test]
    fn test_simulation_until_crash() {
        let input_rows: Vec<(Row, Vec<Cart>)> = read_test()
            .iter()
            .enumerate()
            .map(|item| parse_line(item.0, item.1))
            .collect();
        let (map, carts) = combine_input(input_rows);
        let mut sim = Simulation::new(map, carts);

        assert_eq!((3, 7), sim.until_crash());
    }

    fn read_test() -> Vec<String> {
        let f = File::open("test.txt").expect("could not find file");
        let r = BufReader::new(&f);
        return r.lines().map(|l| l.unwrap()).collect();
    }
}
