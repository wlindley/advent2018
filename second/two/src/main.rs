use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;

fn main() {
    let ids = get_ids();
    for first in &ids {
        for second in &ids {
            if differ_by_one(first, second) {
                println!("IDs:\n{}\n{}", first, second);
                println!("Common chars:\n{}", remove_diff(first, second));
                return;
            }
        }
    }
}

fn get_ids() -> Vec<String> {
    let f = File::open("input.txt").expect("could not find file");
    let r = BufReader::new(&f);
    return r.lines().map(|line| line.unwrap()).collect();
}

fn differ_by_one(first: &String, second: &String) -> bool {
    let mut num_different = 0;
    for (ch1, ch2) in first.chars().zip(second.chars()) {
        if ch1 != ch2 {
            num_different += 1;
        }
    }
    return num_different == 1;
}

fn remove_diff(first: &String, second: &String) -> String {
    let mut result = String::new();
    for (ch1, ch2) in first.chars().zip(second.chars()) {
        if ch1 == ch2 {
            result.push(ch1);
        }
    }
    return result;
}
