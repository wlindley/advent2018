use std::collections::HashMap;
use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;

const DEPENDER_INDEX: usize = 5;
const DEPENDEE_INDEX: usize = 36;

fn main() {
    let relationships: Vec<Relationship> = read_input().iter().map(parse_line).collect();
    let graph = build_graph(&relationships);
    let workgroup = WorkGroup::new(5, graph);
    let (seconds, result) = run_to_completion(workgroup);
    println!(
        "Took {} seconds to produce solution {}",
        seconds,
        result,
    );
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
                    start_item(&mut graph, cur);
                    return Option::Some(cur);
                }
            }
        }
    }
    return Option::None;
}

fn start_item(graph: &mut HashMap<char, Vec<char>>, item: char) {
    graph.remove(&item);
}

fn complete_item(graph: &mut HashMap<char, Vec<char>>, item: char) {
    for (_, value) in graph.iter_mut() {
        match value.iter().position(|&v| v == item) {
            Option::None => continue,
            Option::Some(index) => value.remove(index),
        };
    }
}

fn cost(task: &char) -> u32 {
    return 61 + (*task as u8 - 'A' as u8) as u32;
}

fn run_to_completion(mut workgroup: WorkGroup) -> (u32, String) {
    let mut ticks = 0;
    loop {
        print!("tick {:02}:", ticks);
        for (i, elf) in workgroup.elves.iter().enumerate() {
            match elf.task {
                Option::None => print!(" E{},*", i),
                Option::Some(t) => print!(" E{},{}", i, t),
            };
        }
        println!("");

        match workgroup.tick() {
            WorkGroupState::Working => {
                ticks += 1;
                continue;
            },
            WorkGroupState::Complete => break,
        };
    }
    return (ticks, workgroup.result());
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

#[derive(Debug, PartialEq, Eq)]
struct Elf {
    time: u32,
    task: Option<char>,
}

#[derive(Debug, PartialEq, Eq)]
enum ElfState {
    Working,
    Complete(char),
    Idle,
}

impl Elf {
    fn new() -> Elf {
        return Elf {
            task: Option::None,
            time: 0,
        };
    }

    fn start(&mut self, task: char, duration: u32) {
        self.task = Option::Some(task);
        self.time = duration;
    }

    fn tick(&mut self) -> ElfState {
        match self.task {
            Option::None => return ElfState::Idle,
            Option::Some(c) => {
                self.time -= 1;
                if self.time != 0 {
                    return ElfState::Working;
                }
                self.task = Option::None;
                return ElfState::Complete(c);
            }
        };
    }

    fn state(&self) -> ElfState {
        match self.task {
            Option::None => return ElfState::Idle,
            Option::Some(_) => return ElfState::Working,
        };
    }
}

struct WorkGroup {
    elves: Vec<Elf>,
    graph: HashMap<char, Vec<char>>,
    result: Vec<char>,
    cost: fn(&char) -> u32,
}

enum WorkGroupState {
    Working,
    Complete,
}

impl WorkGroup {
    fn new(num_workers: u32, graph: HashMap<char, Vec<char>>) -> WorkGroup {
        return WorkGroup {
            elves: (0..num_workers).map(|_| Elf::new()).collect(),
            graph: graph,
            result: Vec::new(),
            cost: cost,
        };
    }

    fn result(&self) -> String {
        return self.result.iter().collect();
    }

    fn tick(&mut self) -> WorkGroupState {
        let mut all_idle = true;
        for elf in &mut self.elves {
            match elf.tick() {
                ElfState::Working => {
                    all_idle = false;
                    continue;
                }
                ElfState::Idle => {
                    if start_next(elf, &mut self.graph, self.cost) {
                        all_idle = false;
                    }
                }
                ElfState::Complete(task) => {
                    self.result.push(task);
                    complete_item(&mut self.graph, task);
                    if start_next(elf, &mut self.graph, self.cost) {
                        all_idle = false;
                    }
                }
            };
        }

        for elf in &mut self.elves {
            match elf.state() {
                ElfState::Working => continue,
                ElfState::Complete(_) => continue,
                ElfState::Idle => {
                    if start_next(elf, &mut self.graph, self.cost) {
                        all_idle = false;
                    }
                },
            };
        }

        if all_idle && self.graph.len() == 0 {
            return WorkGroupState::Complete;
        }
        return WorkGroupState::Working;
    }
}

fn start_next(elf: &mut Elf, graph: &mut HashMap<char, Vec<char>>, cost: fn(&char) -> u32) -> bool {
    match next_item(graph) {
        Option::None => return false,
        Option::Some(task) => elf.start(task, cost(&task)),
    };
    return true;
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
        let relationships = vec![
            Relationship::new('B', 'A'),
            Relationship::new('C', 'A'),
            Relationship::new('B', 'C'),
        ];
        let mut graph = build_graph(&relationships);

        assert_eq!(Option::Some('A'), next_item(&mut graph));
        assert_eq!(Option::None, next_item(&mut graph));
        complete_item(&mut graph, 'A');
        assert_eq!(Option::Some('C'), next_item(&mut graph));
        assert_eq!(Option::None, next_item(&mut graph));
        complete_item(&mut graph, 'C');
        assert_eq!(Option::Some('B'), next_item(&mut graph));
        complete_item(&mut graph, 'B');
        assert_eq!(Option::None, next_item(&mut graph));
    }

    #[test]
    fn test_elf() {
        let mut elf = Elf::new();
        elf.start('A', 2);
        assert_eq!(ElfState::Working, elf.tick());
        assert_eq!(ElfState::Complete('A'), elf.tick());
        assert_eq!(ElfState::Idle, elf.tick());
    }

    #[test]
    fn test_tick_cost() {
        let mut elf = Elf::new();
        elf.start('C', cost(&'C'));
        for _ in 0..62 {
            assert_eq!(ElfState::Working, elf.tick());
        }
        assert_eq!(ElfState::Complete('C'), elf.tick());
        assert_eq!(ElfState::Idle, elf.tick());
    }

    #[test]
    fn test_cost() {
        assert_eq!(61, cost(&'A'));
        assert_eq!(62, cost(&'B'));
        assert_eq!(63, cost(&'C'));
        assert_eq!(86, cost(&'Z'));
    }

    #[test]
    fn test_work_group() {
        let relationships = vec![Relationship::new('B', 'A'), Relationship::new('D', 'C')];
        let graph = build_graph(&relationships);
        let mut group = WorkGroup::new(2, graph);
        for _ in 0..65 {
            group.tick();
        }
        assert_eq!("AC", group.result());
        loop {
            match group.tick() {
                WorkGroupState::Complete => break,
                WorkGroupState::Working => continue,
            }
        }
        assert_eq!("ACBD", group.result());
    }

    #[test]
    fn test_example() {
        let relationships = vec![
            Relationship::new('A', 'C'),
            Relationship::new('F', 'C'),
            Relationship::new('B', 'A'),
            Relationship::new('D', 'A'),
            Relationship::new('E', 'B'),
            Relationship::new('E', 'D'),
            Relationship::new('E', 'F'),
        ];
        let graph = build_graph(&relationships);
        let mut group = WorkGroup::new(2, graph);
        group.cost = |&task| 1 + (task as u8 - b'A' as u8) as u32;
        assert_eq!(1, (group.cost)(&'A'));
        assert_eq!(2, (group.cost)(&'B'));
        assert_eq!(3, (group.cost)(&'C'));
        assert_eq!(26, (group.cost)(&'Z'));
        let (seconds, result) = run_to_completion(group);
        assert_eq!(String::from("CABFDE"), result);
        assert_eq!(15, seconds);
    }
}
