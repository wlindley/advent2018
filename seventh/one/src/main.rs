use std::collections::HashMap;
use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;

const DEPENDER_INDEX: usize = 5;
const DEPENDEE_INDEX: usize = 36;

fn main() {
    let relationships: Vec<Relationship> = read_input().iter().map(parse_line).collect();
    let mut graph = build_graph(&relationships);

    loop {
        match next_item(&mut graph) {
            Option::None => break,
            Option::Some(next) => print!("{}", next),
        };
    }
    println!("");
}

fn read_input() -> Vec<String> {
    let f = File::open("input.txt").expect("could not find file");
    let r = BufReader::new(&f);
    return r.lines().map(|l| l.unwrap()).collect();
}

fn parse_line(line: &String) -> Relationship {
    return Relationship::from_tuple(line.chars().enumerate().fold((' ', ' '), |r, (i, c)| {
        if i == DEPENDER_INDEX {
            return (r.0, c);
        }
        if i == DEPENDEE_INDEX {
            return (c, r.1);
        }
        return r;
    }));
}

fn build_graph(relationships: &Vec<Relationship>) -> HashMap<char, Vec<char>> {
    let mut graph = HashMap::new();
    for rel in relationships {
        graph
            .entry(rel.depender)
            .or_insert(vec![])
            .push(rel.dependee);
        graph.entry(rel.dependee).or_insert(vec![]);
    }
    return graph;
}

fn next_item(mut graph: &mut HashMap<char, Vec<char>>) -> Option<char> {
    for cur in (b'A'..b'Z' + 1).map(|c| c as char) {
        match graph.get(&cur) {
            Option::None => continue,
            Option::Some(dependencies) => {
                if dependencies.len() == 0 {
                    remove(&mut graph, cur);
                    return Option::Some(cur);
                }
            }
        }
    }
    return Option::None;
}

fn remove(graph: &mut HashMap<char, Vec<char>>, to_remove: char) {
    graph.remove(&to_remove);
    for (_, value) in graph.iter_mut() {
        match value.iter().position(|&v| v == to_remove) {
            Option::None => continue,
            Option::Some(index) => value.remove(index),
        };
    }
}

#[derive(Debug, PartialEq, Eq)]
struct Relationship {
    depender: char,
    dependee: char,
}

impl Relationship {
    fn new(depender: char, dependee: char) -> Relationship {
        return Relationship { depender, dependee };
    }

    fn from_tuple((depender, dependee): (char, char)) -> Relationship {
        return Relationship { depender, dependee };
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_line() {
        assert_eq!(
            Relationship::new('M', 'Y'),
            parse_line(&String::from(
                "Step Y must be finished before step M can begin."
            ))
        );
        assert_eq!(
            Relationship::new('L', 'Q'),
            parse_line(&String::from(
                "Step Q must be finished before step L can begin."
            ))
        );
    }

    #[test]
    fn test_build_graph() {
        let relationships = vec![Relationship::new('B', 'A')];
        let graph = build_graph(&relationships);
        assert_eq!(vec!['A'], *graph.get(&'B').unwrap());

        let relationships = vec![
            Relationship::new('B', 'A'),
            Relationship::new('C', 'B'),
            Relationship::new('C', 'A'),
        ];
        let graph = build_graph(&relationships);
        let empty: Vec<char> = Vec::new();
        assert_eq!(vec!['A'], *graph.get(&'B').unwrap());
        assert_eq!(vec!['B', 'A'], *graph.get(&'C').unwrap());
        assert_eq!(empty, *graph.get(&'A').unwrap());
        assert_eq!(Option::None, graph.get(&'D'));
    }

    #[test]
    fn test_next_item() {
        let relationships = vec![Relationship::new('B', 'A'), Relationship::new('C', 'A')];
        let mut graph = build_graph(&relationships);

        assert_eq!(Option::Some('A'), next_item(&mut graph));
        assert_eq!(Option::Some('B'), next_item(&mut graph));
        assert_eq!(Option::Some('C'), next_item(&mut graph));
        assert_eq!(Option::None, next_item(&mut graph));
    }
}
