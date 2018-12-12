use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;

fn main() {
    let f = File::open("input.txt").expect("could not find file");
    let mut contents = Vec::new();
    let mut r = BufReader::new(&f);
    r.read_to_end(&mut contents).unwrap();
    let chars: Vec<char> = contents.iter().map(|c| *c as char).collect();
    let result = react(chars);
    let result: String = result.iter().collect();
    println!("Reacted to {} polymers", result.len());
}

fn react(mut input: Vec<char>) -> Vec<char> {
    loop {
        let mut changed = false;
        for i in 0..input.len() - 1 {
            match next_char(&input, i + 1) {
                Option::None => break,
                Option::Some((next_i, next_c)) => {
                    if can_react(input[i], next_c) {
                        input[i] = '_';
                        input[next_i] = '_';
                        changed = true;
                    }
                }
            }
        }

        if !changed {
            break;
        }
    }
    return input.iter().filter(|&&c| c != '_').map(|&c| c).collect();
}

fn next_char(input: &Vec<char>, index: usize) -> Option<(usize, char)> {
    for (i, &c) in input.iter().enumerate().skip(index) {
        if c == '_' {
            continue;
        }
        return Option::Some((i, c));
    }
    return Option::None;
}

fn can_react(first: char, second: char) -> bool {
    return first != second && first.to_ascii_lowercase() == second.to_ascii_lowercase();
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_can_react() {
        assert_eq!(true, can_react('a', 'A'));
        assert_eq!(true, can_react('A', 'a'));
        assert_eq!(false, can_react('a', 'a'));
        assert_eq!(false, can_react('A', 'A'));
    }

    #[test]
    fn test_react() {
        assert_eq!(to_chars(""), react(to_chars("Aa")));
        assert_eq!(to_chars("bb"), react(to_chars("baAb")));
        assert_eq!(to_chars(""), react(to_chars("baAB")));
        assert_eq!(to_chars(""), react(to_chars("abBA")));
        assert_eq!(to_chars("abAB"), react(to_chars("abAB")));
        assert_eq!(to_chars("aabAAB"), react(to_chars("aabAAB")));
        assert_eq!(to_chars("bcB"), react(to_chars("baAcCcB")));
        assert_eq!(to_chars("dabCBAcaDA"), react(to_chars("dabAcCaCBAcCcaDA")));
    }

    #[test]
    fn test_next_char() {
        assert_eq!(Option::Some((2, 'A')), next_char(&to_chars("a_A"), 1));
        assert_eq!(Option::Some((3, 'B')), next_char(&to_chars("a__B"), 1));
        assert_eq!(Option::Some((1, 'z')), next_char(&to_chars("xz"), 1));
        assert_eq!(Option::None, next_char(&to_chars("xz_"), 2));
        assert_eq!(Option::None, next_char(&to_chars("xz"), 2));
    }

    fn to_chars(input: &str) -> Vec<char> {
        return String::from(input).chars().collect();
    }
}
