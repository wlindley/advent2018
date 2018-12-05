use std::collections::HashMap;
use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;

fn main() {
    let ids = get_ids();
    let mut num_twos = 0;
    let mut num_threes = 0;
    for id in ids {
        let counts = count_chars(id);
        if has_two(&counts) {
            num_twos += 1;
        }
        if has_three(&counts) {
            num_threes += 1;
        }
    }
    println!("{}", num_twos * num_threes);
}

fn get_ids() -> Vec<String> {
    let f = File::open("input.txt").expect("could not find file");
    let r = BufReader::new(&f);
    return r.lines().map(|line| line.unwrap()).collect();
}

fn count_chars(id: String) -> HashMap<char, u32> {
    let mut counts = HashMap::new();
    for c in id.chars() {
        *counts.entry(c).or_insert(0) += 1;
    }
    return counts;
}

fn has_two(counts: &HashMap<char, u32>) -> bool {
    for (_, count) in counts {
        if *count == 2 {
            return true;
        }
    }
    return false;
}

fn has_three(counts: &HashMap<char, u32>) -> bool {
    for (_, count) in counts {
        if *count == 3 {
            return true;
        }
    }
    return false;
}
