use std::fs::File;
use std::io::{BufRead, BufReader};

use itertools::Itertools;

fn main() {
    let file = File::open(std::env::args_os().nth(1).unwrap()).unwrap();
    let reader = BufReader::new(file);

    let mut total_uniques = 0;

    for line in reader.lines() {
        let line = line.unwrap();

        let (_, patterns): (&str, &str) = line.split(" | ").collect_tuple().unwrap();

        total_uniques += patterns
            .split_whitespace()
            .filter(|n| n.len() == 2 || n.len() == 3 || n.len() == 4 || n.len() == 7)
            .count();
    }

    dbg!(total_uniques);
}
