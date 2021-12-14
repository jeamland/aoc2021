use std::collections::HashMap;
use std::fs::File;
use std::io::{BufRead, BufReader};

use itertools::{Itertools, MinMaxResult};

#[derive(Debug)]
struct Polymeriserator {
    state: HashMap<(char, char), usize>,
    rules: HashMap<(char, char), char>,
    counts: HashMap<char, usize>,
}

impl Polymeriserator {
    fn from_lines(mut lines: impl Iterator<Item = String>) -> Self {
        let template = lines.next().unwrap();
        lines.next().unwrap();

        let counts = template.chars().counts();
        let state = template.chars().tuple_windows::<(char, char)>().counts();

        let rules: HashMap<(char, char), char> = lines
            .map(|l| {
                let (pair, insert) = l.split_terminator(" -> ").collect_tuple().unwrap();
                (
                    pair.chars().collect_tuple().unwrap(),
                    insert.chars().next().unwrap(),
                )
            })
            .collect();

        Self {
            state,
            rules,
            counts,
        }
    }

    fn step(&mut self) {
        let mut new_state = HashMap::new();
        for (&(a, b), &count) in &self.state {
            let insert = *self.rules.get(&(a, b)).unwrap();

            *new_state.entry((a, insert)).or_insert(0) += count;
            *new_state.entry((insert, b)).or_insert(0) += count;
            *self.counts.entry(insert).or_insert(0) += count;
        }
        self.state = new_state;
    }

    fn value(&self) -> usize {
        if let MinMaxResult::MinMax(min, max) = self.counts.values().minmax() {
            max - min
        } else {
            0
        }
    }
}

fn main() {
    let file = File::open(std::env::args_os().nth(1).unwrap()).unwrap();
    let reader = BufReader::new(file);

    let mut poly = Polymeriserator::from_lines(reader.lines().map(|l| l.unwrap()));
    for steps in 0..40 {
        println!("Step {}...", steps + 1);
        poly.step();
    }
    println!("Value: {}", poly.value());
}
