use std::collections::HashMap;
use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;

fn main() {
    let lines = load_sorted_log();
    let guards = parse_log(lines);
    let (guard, minute) = find_sleepiest_minute(&guards);
    println!("Guard {} had the sleepiest minute @ {}. Multiplies to: {}", guard.id, minute, guard.id * minute as u32);
}

fn find_sleepiest_minute(guards: &HashMap<GuardId, Guard>) -> (&Guard, Minute) {
    let mut sleepiest_guard = 0;
    let mut sleepiest_minute = 0;
    let mut num_naps = 0;
    for (_, guard) in guards.iter() {
        println!("examining guard {} with {} naps", guard.id, guard.naps.len());
        let (minute, count) = guard.sleepiest_minute();
        if count > num_naps {
            sleepiest_guard = guard.id;
            sleepiest_minute = minute;
            num_naps = count;
        }
    }
    return (&guards[&sleepiest_guard], sleepiest_minute);
}

fn parse_log(lines: Vec<Line>) -> HashMap<GuardId, Guard> {
    let mut guards = HashMap::new();
    let mut cur_guard = 0;
    let mut cur_nap = Nap::empty();
    for l in lines {
        match l {
            Line::NewGuard(id) => {
                cur_guard = id;
                cur_nap = Nap::empty();
                guards.entry(cur_guard).or_insert(Guard::default(&cur_guard));
            },
            Line::NapBegin(begin) => {
                cur_nap.begin = Option::Some(begin);
            },
            Line::NapEnd(end) => {
                cur_nap.end = Option::Some(end);
                guards.get_mut(&cur_guard).unwrap().naps.push(cur_nap.clone());
                cur_nap = Nap::empty();
            },
        }
    }
    return guards;
}

fn load_sorted_log() -> Vec<Line> {
    let f = File::open("input.txt").expect("could not find file");
    let r = BufReader::new(&f);
    let mut lines: Vec<String> = r.lines().map(|l| l.unwrap()).collect();
    lines.sort();
    return lines.iter().map(parse_line).collect();
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

type GuardId = u32;
type Minute = u8;
type Duration = u32;

#[derive(PartialEq, Eq, Debug)]
enum Line {
    NewGuard(GuardId),
    NapBegin(Minute),
    NapEnd(Minute),
}

#[derive(PartialEq, Eq, Debug, Clone)]
struct Nap {
    begin: Option<Minute>, // minute nap begins
    end: Option<Minute>, // minute nap ends
}

impl Nap {
    fn empty() -> Nap { Nap{begin: Option::None, end: Option::None}}

    fn duration(&self) -> Duration {
        match self.begin {
            Option::None => 0,
            Option::Some(b) => {
                match self.end {
                    Option::None => 0,
                    Option::Some(e) => (e - b) as u32,
                }
            },
        }
    }
}

struct Guard {
    id: GuardId,
    naps: Vec<Nap>,
}

impl Guard {
    fn default(id: &GuardId) -> Guard {
        return Guard{id: id.clone(), naps: Vec::default()};
    }

    fn total_sleep(&self) -> Duration {
        self.naps.iter().fold(0, |accum, nap| accum + nap.duration())
    }

    fn sleepiest_minute(&self) -> (Minute, Duration) {
        let mut minutes: HashMap<u8, u32> = HashMap::new();
        for nap in &self.naps {
            for minute in nap.begin.unwrap() .. nap.end.unwrap() {
                *minutes.entry(minute).or_insert(0) += 1;
            }
        }

        let mut sleepiest = Option::None;
        for (minute, count) in &minutes {
            match sleepiest {
                Option::None => sleepiest = Option::Some(minute),
                Option::Some(m) => {
                    if *count > minutes[&m] {
                        sleepiest = Option::Some(minute);
                    }
                },
            }
        }
        match sleepiest {
            Option::None => (0, 0),
            Option::Some(&minute) => (minute, *minutes.get(&minute).unwrap()),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_lines() {
        let input = vec![
            String::from("[0000-01-01 23:58] Guard #1 begins shift"),
            String::from("[0000-01-02 00:10] falls asleep"),
            String::from("[0000-01-02 00:52] wakes up"),
        ];
        let output: Vec<Line> = input.iter().map(parse_line).collect();
        assert_eq!(Line::NewGuard(1), output[0]);
        assert_eq!(Line::NapBegin(10), output[1]);
        assert_eq!(Line::NapEnd(52), output[2]);
    }

    #[test]
    fn test_total_sleep() {
        let guard = Guard{
            id: 0,
            naps: vec![
                Nap{begin: Option::Some(0), end: Option::Some(5)},
                Nap{begin: Option::Some(1), end: Option::Some(3)},
                Nap{begin: Option::Some(10), end: Option::Some(13)},
            ],
        };

        assert_eq!(10, guard.total_sleep());
    }

    #[test]
    fn test_sleepiest_minute() {
        let guard = Guard{
            id: 0,
            naps: vec![
                Nap{begin: Option::Some(0), end: Option::Some(5)},
                Nap{begin: Option::Some(3), end: Option::Some(10)},
                Nap{begin: Option::Some(4), end: Option::Some(5)},
            ],
        };

        assert_eq!((4, 3), guard.sleepiest_minute());
    }

    #[test]
    fn test_duration() {
        assert_eq!(0, Nap{begin: Option::None, end: Option::Some(5)}.duration());
        assert_eq!(0, Nap{begin: Option::Some(1), end: Option::None}.duration());
        assert_eq!(10, Nap{begin: Option::Some(2), end: Option::Some(12)}.duration());
    }

    #[test]
    fn test_find_sleepiest_minute() {
        let mut guards: HashMap<u32, Guard> = HashMap::new();
        guards.insert(1, Guard{id: 1, naps: vec![Nap{begin: Option::Some(1), end: Option::Some(5)}]});
        guards.insert(2, Guard{id: 2, naps: vec![Nap{begin: Option::Some(1), end: Option::Some(8)}]});
        guards.insert(3, Guard{id: 3, naps: vec![Nap{begin: Option::Some(1), end: Option::Some(6)}, Nap{begin: Option::Some(5), end: Option::Some(8)}]});
        let (guard, minute) = find_sleepiest_minute(&guards);
        assert_eq!(3, guard.id);
        assert_eq!(5, minute);
    }

    #[test]
    fn test_parse_log() {
        let log: Vec<Line> = vec![
            Line::NewGuard(1),
            Line::NapBegin(0),
            Line::NapEnd(5),
            Line::NapBegin(10),
            Line::NapEnd(20),
        ];
        let guards = parse_log(log);
        let guard = guards.get(&1).unwrap();
        assert_eq!(2, guard.naps.len());
        assert_eq!(15, guard.total_sleep());
    }
    #[test]
    fn test_parse_multiday_log() {
        let log: Vec<Line> = vec![
            Line::NewGuard(1),
            Line::NapBegin(0),
            Line::NapEnd(5),
            Line::NewGuard(1),
            Line::NapBegin(10),
            Line::NapEnd(20),
        ];
        let guards = parse_log(log);
        let guard = guards.get(&1).unwrap();
        assert_eq!(2, guard.naps.len());
        assert_eq!(15, guard.total_sleep());
    }

    #[test]
    fn test_parse_multiguard_log() {
        let log: Vec<Line> = vec![
            Line::NewGuard(1),
            Line::NapBegin(0),
            Line::NapEnd(5),

            Line::NewGuard(2),
            Line::NapBegin(10),
            Line::NapEnd(20),
        ];
        let guards = parse_log(log);
        let guard = guards.get(&1).unwrap();
        assert_eq!(1, guard.naps.len());
        assert_eq!(5, guard.total_sleep());
        let guard = guards.get(&2).unwrap();
        assert_eq!(1, guard.naps.len());
        assert_eq!(10, guard.total_sleep());
    }
}
