use std::fs::File;
use std::io::{BufRead, BufReader};

use itertools::Itertools;

fn main() {
    let file = File::open("input").unwrap();
    let reader = BufReader::new(file);

    let mut aim = 0;
    let mut position = 0;
    let mut depth = 0;

    for line in reader.lines() {
        let line = line.unwrap();
        let (direction, value) = line.split_whitespace().collect_tuple().unwrap();
        let value = value.parse::<usize>().unwrap();

        match direction {
            "forward" => {
                position += value;
                depth += aim * value;
            }
            "up" => aim -= value,
            "down" => aim += value,
            _ => panic!("wtf"),
        }
    }

    dbg!(position, depth, position * depth);
}
