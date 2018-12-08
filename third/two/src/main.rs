use std::collections::HashMap;
use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;

fn main() {
    let rects = get_rects();
    let mut locations: HashMap<Point, u32> = HashMap::new();

    for rect in &rects {
        for w in 0..rect.width {
            for h in 0..rect.height {
                let x = rect.x + w;
                let y = rect.y + h;
                *locations.entry(Point { x, y }).or_insert(0) += 1;
            }
        }
    }

    for rect in &rects {
        let mut all_valid = true;
        for w in 0..rect.width {
            for h in 0..rect.height {
                let x = rect.x + w;
                let y = rect.y + h;
                if locations[&Point { x, y }] != 1 {
                    all_valid = false;
                }
            }
        }
        if all_valid {
            println!("Unconflicted claim: {}", rect.id);
        }
    }
}

#[derive(PartialEq, Eq, Hash)]
struct Point {
    x: u32,
    y: u32,
}

struct Rect {
    id: u32,
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
        .split(|c| c == '#' || c == '@' || c == ':' || c == ',' || c == 'x')
        .map(|t| t.trim())
        .collect();
    return Rect {
        id: tokens[1].parse().unwrap(),
        x: tokens[2].parse().unwrap(),
        y: tokens[3].parse().unwrap(),
        width: tokens[4].parse().unwrap(),
        height: tokens[5].parse().unwrap(),
    };
}
