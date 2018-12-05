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
    let num_different = first.chars().zip(second.chars()).fold(0, |accum, val| {
        if val.0 != val.1 {
            return accum + 1;
        }
        return accum;
    });
    return num_different == 1;
}

fn remove_diff(first: &String, second: &String) -> String {
    return first.chars().zip(second.chars()).fold(String::new(), |accum, val| {
        if val.0 == val.1 {
            let mut next = String::from(accum);
            next.push(val.0);
            return next;
        }
        return accum;
    });
}
