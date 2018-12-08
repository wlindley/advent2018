use std::collections::HashMap;
use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;

fn main() {
    let rects = get_rects();
    let mut locations: HashMap<Point, u32> = HashMap::new();

    for rect in rects {
        for w in 0..rect.width {
            for h in 0..rect.height {
                let x = rect.x + w;
                let y = rect.y + h;
                *locations.entry(Point { x, y }).or_insert(0) += 1;
            }
        }
    }

    let conflicting = locations.values().fold(0, |accum, claims| {
        if *claims == 1 {
            return accum;
        }
        return accum + 1;
    });
    println!("Square inches of conflict: {}", conflicting);
}

#[derive(PartialEq, Eq, Hash)]
struct Point {
    x: u32,
    y: u32,
}

struct Rect {
    x: u32,
    y: u32,
    width: u32,
    height: u32,
}

fn get_rects() -> Vec<Rect> {
    let f = File::open("input.txt").expect("could not find file");
    let r = BufReader::new(&f);
    return r.lines().map(|line| parse_line(&line.unwrap())).collect();
}

fn parse_line(line: &String) -> Rect {
    let tokens: Vec<&str> = line
        .split(|c| c == '@' || c == ':' || c == ',' || c == 'x')
        .map(|t| t.trim())
        .collect();
    return Rect {
        x: tokens[1].parse().unwrap(),
        y: tokens[2].parse().unwrap(),
        width: tokens[3].parse().unwrap(),
        height: tokens[4].parse().unwrap(),
    };
}
