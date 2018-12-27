use std::collections::HashMap;
use std::collections::VecDeque;
use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;

fn main() {
    let lines = read_input();
    let mut iter = lines.iter();
    let state = parse_pots(iter.next().unwrap());
    let iter = iter.skip(1); // blank line
    let patterns: Vec<Rule> = iter.map(parse_pattern).collect();
    let rules = build_rules(patterns);
    let gens = 50000000000;
    let state = generations(state, &rules, gens);
    let num_alive = count_living_pots(&state);
    println!("Alive after {} generations: {}", gens, num_alive);
}

fn read_input() -> Vec<String> {
    let f = File::open("input.txt").expect("could not find file");
    let r = BufReader::new(&f);
    return r.lines().map(|l| l.unwrap()).collect();
}

fn is_alive(c: char) -> bool {
    c == '#'
}

fn parse_pots(input: &String) -> State {
    let values = input.replace("initial state: ", "");
    values.chars().enumerate().map(|(i, c)| (i as i32, is_alive(c))).collect()
}

type Pot = (i32, bool);
type State = VecDeque<Pot>;
type Window = VecDeque<Pot>;
type PlantPattern = u8;
type Rule = (PlantPattern, bool);
type Rules = HashMap<PlantPattern, bool>;
type Count = i128;
type Generations = u64;
const PAD_SIZE: usize = 4;
const WINDOW_SIZE: usize = 5;
const BITMASK: u8 = 0b00011110;

fn parse_pattern(input: &String) -> Rule {
    let values = input.replace(" => ", "");
    let mut iter = values.chars().map(is_alive);
    let mut pattern = 0;
    for i in 0..WINDOW_SIZE {
        if iter.next().unwrap() {
            pattern |= 1 << i;
        }
    }
    return (pattern, iter.next().unwrap());
}

fn build_rules(rules: Vec<Rule>) -> Rules {
    let mut result = HashMap::new();
    for (pattern, alive) in rules {
        result.insert(pattern, alive);
    }
    return result;
}

fn next_generation(mut state: State, rules: &Rules) -> State {
    state = pad(state, PAD_SIZE);
    let mut next = State::with_capacity(state.len() + 2);
    let mut iter = state.iter();
    let mut window = 0u8;
    let mut cur_pot_num = state.front().unwrap().0 - 2;
    while let Some(pot) = iter.next() {
        window = (window & BITMASK) >> 1;
        if pot.1 {
            window |= 1 << (WINDOW_SIZE - 1);
        }
        let new_pot = match rules.get(&window) {
            None => (cur_pot_num, false),
            Some(alive) => (cur_pot_num, *alive),
        };
        next.push_back(new_pot);
        cur_pot_num += 1;
    }
    trim(next)
}

fn pad(mut state: State, pad_size: usize) -> State {
    // let mut min_index = state.front().unwrap().0;
    let mut max_index = state.back().unwrap().0;
    for _ in 0..pad_size {
        // min_index -= 1;
        max_index += 1;
        // state.push_front((min_index, false));
        state.push_back((max_index, false));
    }
    state
}

fn trim(mut state: State) -> State {
    while !state.front().unwrap().1 {
        state.pop_front();
    }
    while !state.back().unwrap().1 {
        state.pop_back();
    }
    state
}

fn pattern(pots: &Window) -> PlantPattern {
    let mut pattern = 0;
    for (i, pot) in pots.iter().enumerate() {
        if pot.1 {
            pattern |= 1 << i;
        }
    }
    pattern
}

fn stringify(state: &State) -> String {
    let mut buffer = String::with_capacity(state.len());
    for (_, alive) in state {
        if *alive {
            buffer.push('#');
        } else {
            buffer.push('.');
        }
    }
    buffer
}

fn generations(mut state: State, rules: &Rules, generations: Generations) -> State {
    for i in 0..generations {
        state = next_generation(state, rules);
        if i % 1000000 == 0 {
            println!("Completed generation {}", i);
        }
    }
    state
}

fn count_living_pots(state: &State) -> Count {
    return state.iter().fold(0, |accum, &(i, alive)| {
        if alive {
            accum + i as Count
        } else {
            accum
        }
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_pots() {
        assert_eq!(VecDeque::from(vec![(0, false)]), parse_pots(&String::from("initial state: .")));
        assert_eq!(VecDeque::from(vec![(0, true)]), parse_pots(&String::from("initial state: #")));
        assert_eq!(
            VecDeque::from(vec![(0, false), (1, true)]),
            parse_pots(&String::from("initial state: .#"))
        );
        assert_eq!(
            VecDeque::from(vec![(0, true), (1, false), (2, false), (3, true), (4, false)]),
            parse_pots(&String::from("initial state: #..#."))
        );
    }

    #[test]
    fn test_parse_pattern() {
        assert_eq!(
            (pattern([false, false, false, false, false]), false),
            parse_pattern(&String::from("..... => ."))
        );
        assert_eq!(
            (pattern([true, true, true, true, true]), true),
            parse_pattern(&String::from("##### => #"))
        );
        assert_eq!(
            (pattern([true, false, false, true, false]), true),
            parse_pattern(&String::from("#..#. => #"))
        );
        assert_eq!(
            (pattern([false, true, true, false, true]), false),
            parse_pattern(&String::from(".##.# => ."))
        );
    }

    #[test]
    fn test_next_generation() {
        let rules = build_rules(vec![
            parse_pattern(&String::from("..... => #")),
        ]);
        let state = parse_pots(&String::from("....."));
        assert_eq!(String::from("#########"), stringify(&next_generation(state, &rules)));

        let rules = build_rules(vec![
            parse_pattern(&String::from("..#.. => #")),
        ]);
        let state = parse_pots(&String::from("..#.."));
        assert_eq!(String::from("#"), stringify(&next_generation(state, &rules)));
    }

    #[test]
    fn test_example_generation() {
        let state = parse_pots(&String::from("#..#.#..##......###...###"));
        let raw_rules: Vec<String> = vec![
            String::from("...## => #"),
            String::from("..#.. => #"),
            String::from(".#... => #"),
            String::from(".#.#. => #"),
            String::from(".#.## => #"),
            String::from(".##.. => #"),
            String::from(".#### => #"),
            String::from("#.#.# => #"),
            String::from("#.### => #"),
            String::from("##.#. => #"),
            String::from("##.## => #"),
            String::from("###.. => #"),
            String::from("###.# => #"),
            String::from("####. => #"),
        ];
        let rules = build_rules(raw_rules.iter().map(parse_pattern).collect());

        let state = next_generation(state, &rules);
        assert_eq!(String::from("#...#....#.....#..#..#..#"), stringify(&state));
        let state = next_generation(state, &rules);
        assert_eq!(String::from("##..##...##....#..#..#..##"), stringify(&state));
        let state = next_generation(state, &rules);
        assert_eq!(String::from("#.#...#..#.#....#..#..#...#"), stringify(&state));
        let state = next_generation(state, &rules);
        assert_eq!(String::from("#.#..#...#.#...#..#..##..##"), stringify(&state));
        let state = next_generation(state, &rules);
        assert_eq!(String::from("#...##...#.#..#..#...#...#"), stringify(&state));
        let state = next_generation(state, &rules);
        assert_eq!(String::from("##.#.#....#...#..##..##..##"), stringify(&state));
        let state = next_generation(state, &rules);
        assert_eq!(String::from("#..###.#...##..#...#...#...#"), stringify(&state));
        let state = next_generation(state, &rules);
        assert_eq!(String::from("#....##.#.#.#..##..##..##..##"), stringify(&state));
        let state = next_generation(state, &rules);
        assert_eq!(String::from("##..#..#####....#...#...#...#"), stringify(&state));
        let state = next_generation(state, &rules);
        assert_eq!(String::from("#.#..#...#.##....##..##..##..##"), stringify(&state));
        let state = next_generation(state, &rules);
        assert_eq!(String::from("#...##...#.#...#.#...#...#...#"), stringify(&state));
        let state = next_generation(state, &rules);
        assert_eq!(String::from("##.#.#....#.#...#.#..##..##..##"), stringify(&state));
        let state = next_generation(state, &rules);
        assert_eq!(String::from("#..###.#....#.#...#....#...#...#"), stringify(&state));
        let state = next_generation(state, &rules);
        assert_eq!(String::from("#....##.#....#.#..##...##..##..##"), stringify(&state));
        let state = next_generation(state, &rules);
        assert_eq!(String::from("##..#..#.#....#....#..#.#...#...#"), stringify(&state));
        let state = next_generation(state, &rules);
        assert_eq!(String::from("#.#..#...#.#...##...#...#.#..##..##"), stringify(&state));
        let state = next_generation(state, &rules);
        assert_eq!(String::from("#...##...#.#.#.#...##...#....#...#"), stringify(&state));
        let state = next_generation(state, &rules);
        assert_eq!(String::from("##.#.#....#####.#.#.#...##...##..##"), stringify(&state));
        let state = next_generation(state, &rules);
        assert_eq!(String::from("#..###.#..#.#.#######.#.#.#..#.#...#"), stringify(&state));
        let state = next_generation(state, &rules);
        assert_eq!(String::from("#....##....#####...#######....#.#..##"), stringify(&state));
    }

    #[test]
    fn test_count_living_pots() {
        let state = parse_pots(&String::from("#..#.#..##......###...###"));
        assert_eq!(3+5+8+9+16+17+18+22+23+24, count_living_pots(&state));

        let raw_rules: Vec<String> = vec![
            String::from("...## => #"),
            String::from("..#.. => #"),
            String::from(".#... => #"),
            String::from(".#.#. => #"),
            String::from(".#.## => #"),
            String::from(".##.. => #"),
            String::from(".#### => #"),
            String::from("#.#.# => #"),
            String::from("#.### => #"),
            String::from("##.#. => #"),
            String::from("##.## => #"),
            String::from("###.. => #"),
            String::from("###.# => #"),
            String::from("####. => #"),
        ];
        let rules = build_rules(raw_rules.iter().map(parse_pattern).collect());
        let state = generations(state, &rules, 20);
        assert_eq!(325, count_living_pots(&state));
    }

    fn pattern(input: [bool;5]) -> PlantPattern {
        let mut pattern = 0;
        for i in 0..WINDOW_SIZE {
            if input[i] {
                pattern |= (1 << i);
            }
        }
        pattern
    }
}
