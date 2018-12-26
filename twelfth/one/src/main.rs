use std::collections::HashMap;
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
    let state = generations(state, &rules, 20);
    let num_alive = count_living_pots(&state);
    println!("Alive after 20 generations: {}", num_alive);
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
type State = Vec<Pot>;
type PlantPattern = [bool; 5];
type Rule = (PlantPattern, bool);
type Rules = HashMap<PlantPattern, bool>;

fn parse_pattern(input: &String) -> Rule {
    let values = input.replace(" => ", "");
    let mut iter = values.chars().map(is_alive);
    let pattern = [
        iter.next().unwrap(),
        iter.next().unwrap(),
        iter.next().unwrap(),
        iter.next().unwrap(),
        iter.next().unwrap(),
    ];
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
    state = pad(state, 4);
    let mut next: State = state.windows(5).map(|window| {
        match rules.get(&pattern(window)) {
            None => (window[2].0, false),
            Some(alive) => (window[2].0, *alive),
        }
    }).collect();
    let (index, _) = next.iter().enumerate().find(|(_, &(_, alive))| alive == true).unwrap();
    let mut next = next.split_off(index);
    loop {
        let pot = next.pop().unwrap();
        if pot.1 {
            next.push(pot);
            break;
        }
    }
    return next;
}

fn pad(mut state: State, pad_size: u32) -> State {
    let mut min_index = state[0].0;
    let mut max_index = state[state.len()-1].0;
    for _ in 0..pad_size {
        min_index -= 1;
        max_index += 1;
        state.insert(0, (min_index, false));
        state.push((max_index, false));
    }
    state
}

fn pattern(pots: &[Pot]) -> PlantPattern {
    let mut pattern = [false; 5];
    for i in 0..5 {
        pattern[i] = pots[i].1;
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

fn generations(mut state: State, rules: &Rules, generations: u32) -> State {
    for _ in 0..generations {
        state = next_generation(state, rules);
    }
    state
}

fn count_living_pots(state: &State) -> i32 {
    return state.iter().fold(0, |accum, &(i, alive)| {
        if alive {
            accum + i
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
        assert_eq!(vec![(0, false)], parse_pots(&String::from("initial state: .")));
        assert_eq!(vec![(0, true)], parse_pots(&String::from("initial state: #")));
        assert_eq!(
            vec![(0, false), (1, true)],
            parse_pots(&String::from("initial state: .#"))
        );
        assert_eq!(
            vec![(0, true), (1, false), (2, false), (3, true), (4, false)],
            parse_pots(&String::from("initial state: #..#."))
        );
    }

    #[test]
    fn test_parse_pattern() {
        assert_eq!(
            ([false, false, false, false, false], false),
            parse_pattern(&String::from("..... => ."))
        );
        assert_eq!(
            ([true, true, true, true, true], true),
            parse_pattern(&String::from("##### => #"))
        );
        assert_eq!(
            ([true, false, false, true, false], true),
            parse_pattern(&String::from("#..#. => #"))
        );
        assert_eq!(
            ([false, true, true, false, true], false),
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
}
