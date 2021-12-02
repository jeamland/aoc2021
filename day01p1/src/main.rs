use std::fs::File;
use std::io::{BufRead, BufReader};

use itertools::Itertools;

fn main() {
    let file = File::open("input").unwrap();
    let reader = BufReader::new(file);

    let mut increase_count = 0;
    for (n1, n2) in reader
        .lines()
        .map(|line| line.unwrap().parse::<usize>().unwrap())
        .tuple_windows()
    {
        if n2 > n1 {
            increase_count += 1;
        }
    }

    dbg!(increase_count);
}
