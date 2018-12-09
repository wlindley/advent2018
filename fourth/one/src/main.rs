use std::collections::HashMap;
use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;

fn main() {
    let guards = load_guards();
    let mut sleepiest_guard = 0;
    let mut most_sleep = 0;
    for (_, guard) in &guards {
        let sleep = guard.total_sleep();
        if sleep > most_sleep {
            sleepiest_guard = guard.id;
            most_sleep = sleep;
        }
    }
    println!("Sleepied guard {} slept {} minutes", sleepiest_guard, most_sleep);
    println!("Multiplied together: {}", sleepiest_guard * most_sleep);
}

fn load_guards() -> HashMap<u32, Guard> {
    let lines = load_sorted_log();
    let mut guards: HashMap<u32, Guard> = HashMap::new();
    let mut cur_guard = 0;
    let mut cur_nap = Nap::empty();
    for l in lines.iter().map(parse_line) {
        match l {
            Line::NewGuard(id) => {
                cur_guard = id;
                cur_nap = Nap::empty();
                guards.insert(cur_guard, Guard::default(&cur_guard));
            },
            Line::NapBegin(begin) => {
                cur_nap.begin = Option::Some(begin);
            },
            Line::NapEnd(end) => {
                cur_nap.end = Option::Some(end);
                match cur_nap.begin {
                    Option::Some(_) => {
                        guards.entry(cur_guard).or_insert(Guard::default(&cur_guard)).naps.push(cur_nap);
                        cur_nap = Nap::empty();
                    },
                    Option::None => {},
                }
            },
        }
    }
    return guards;
}

fn load_sorted_log() -> Vec<String> {
    let f = File::open("input.txt").expect("could not find file");
    let r = BufReader::new(&f);
    let mut lines: Vec<String> = r.lines().map(|l| l.unwrap()).collect();
    lines.sort();
    return lines;
}

fn parse_line(line: &String) -> Line {
    let tokens: Vec<&str> = line
        .split(|c| c == '[' || c == ']')
        .filter(|&t| t != "")
        .map(|t| t.trim())
        .collect();
    if tokens[1].starts_with("falls") {
        return Line::NapBegin(parse_minute(tokens[0]));
    }
    if tokens[1].starts_with("wakes") {
        return Line::NapEnd(parse_minute(tokens[0]));
    }
    return Line::NewGuard(parse_guard(tokens[1]));
}

fn parse_minute(timestamp: &str) -> u8 {
    let tokens: Vec<&str> = timestamp.split(':').skip(1).take(1).collect();
    return tokens[0].parse().unwrap();
}

fn parse_guard(input: &str) -> u32 {
    let tokens: Vec<&str> = input.split_whitespace().skip(1).take(1).collect();
    let guard_str: String = tokens[0].chars().skip(1).collect();
    return guard_str.parse().unwrap();
}

enum Line {
    NewGuard(u32),
    NapBegin(u8),
    NapEnd(u8),
}

struct Nap {
    begin: Option<u8>, // minute nap begins
    end: Option<u8>, // minute nap ends
}

impl Nap {
    fn empty() -> Nap { Nap{begin: Option::None, end: Option::None}}

    fn duration(&self) -> u8 {
        match self.begin {
            Option::None => 0,
            Option::Some(b) => {
                match self.end {
                    Option::None => 0,
                    Option::Some(e) => e - b,
                }
            },
        }
    }
}

struct Guard {
    id: u32,
    naps: Vec<Nap>,
}

impl Guard {
    fn default(id: &u32) -> Guard {
        return Guard{id: id.clone(), naps: Vec::default()};
    }

    fn total_sleep(&self) -> u32 {
        self.naps.iter().fold(0, |accum, nap| accum + nap.duration() as u32)
    }
}
