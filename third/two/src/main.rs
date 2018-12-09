use std::collections::HashMap;
use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;

fn main() {
    let rects = get_rects();
    let mut locations: HashMap<Point, u32> = HashMap::new();

    for rect in &rects {
        for point in rect.points() {
            *locations.entry(point).or_insert(0) += 1;
        }
    }

    for rect in &rects {
        let mut all_valid = true;
        for point in rect.points() {
            if locations[&point] != 1 {
                all_valid = false;
            }
        }
        if all_valid {
            println!("Unconflicted claim: {}", rect.id);
        }
    }
}

#[derive(PartialEq, Eq, Hash, Clone)]
struct Point {
    x: u32,
    y: u32,
}

#[derive(Clone)]
struct Rect {
    id: u32,
    x: u32,
    y: u32,
    width: u32,
    height: u32,
}

impl Rect {
    fn points(&self) -> PointIterator {
        return PointIterator::new(self);
    }
}

struct PointIterator {
    rect: Rect,
    cur: Point,
}

impl PointIterator {
    fn new(rect: &Rect) -> PointIterator {
        let first = Point {
            x: rect.x,
            y: rect.y,
        };
        return PointIterator {
            rect: rect.clone(),
            cur: first,
        };
    }
}

impl Iterator for PointIterator {
    type Item = Point;

    fn next(&mut self) -> Option<Point> {
        if self.cur.y >= self.rect.y + self.rect.height {
            return Option::None;
        }
        let point = self.cur.clone();
        self.cur.x += 1;
        if self.cur.x >= self.rect.x + self.rect.width {
            self.cur.x = self.rect.x;
            self.cur.y += 1;
        }
        return Option::Some(point);
    }
}

fn get_rects() -> Vec<Rect> {
    let f = File::open("input.txt").expect("could not find file");
    let r = BufReader::new(&f);
    return r.lines().map(|line| parse_line(&line.unwrap())).collect();
}

fn parse_line(line: &String) -> Rect {
    let tokens: Vec<&str> = line
        .split(|c| c == '#' || c == '@' || c == ':' || c == ',' || c == 'x')
        .map(|t| t.trim())
        .skip(1)
        .collect();
    return Rect {
        id: tokens[0].parse().unwrap(),
        x: tokens[1].parse().unwrap(),
        y: tokens[2].parse().unwrap(),
        width: tokens[3].parse().unwrap(),
        height: tokens[4].parse().unwrap(),
    };
}
