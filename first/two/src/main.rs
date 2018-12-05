use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;
use std::collections::HashMap;

fn main() {
    let nums = get_list();
    let mut value = 0;
    let mut values = HashMap::new();
    loop {
        for num in &nums {
            if values.contains_key(&value) {
                println!("{}", value);
                return;
            }
            values.insert(value, true);

            value += num;
        }
    }
}

fn get_list() -> Vec<i64> {
    let f = File::open("input.txt").expect("could not find file");
    let r = BufReader::new(&f);
    return r.lines().map(|v| v.unwrap().parse::<i64>().unwrap()).collect();
}
