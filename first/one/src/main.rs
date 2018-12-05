use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;

fn main() {
    let f = File::open("input.txt").expect("could not find file");
    let r = BufReader::new(&f);
    let mut value = 0;
    for line in r.lines() {
        let l = line.unwrap();
        let delta = l.parse::<i64>().unwrap();
        value += delta
    }
    println!("result: {}", value);
}
