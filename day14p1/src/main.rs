use std::collections::HashMap;
use std::fs::File;
use std::io::{BufRead, BufReader};

use itertools::{Itertools, MinMaxResult};

#[derive(Debug)]
struct Polymeriserator {
    state: Vec<char>,
    rules: HashMap<(char, char), char>,
}

impl Polymeriserator {
    fn from_lines(mut lines: impl Iterator<Item = String>) -> Self {
        let template = lines.next().unwrap();
        lines.next().unwrap();

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
            state: template.chars().collect(),
            rules,
        }
    }

    fn step(&mut self) {
        let mut new_state = Vec::new();

        for (&a, &b) in self.state.iter().tuple_windows() {
            new_state.push(a);
            if let Some(&insert) = self.rules.get(&(a, b)) {
                new_state.push(insert);
            }
        }

        new_state.push(*self.state.last().unwrap());
        self.state = new_state;
    }

    fn state(&self) -> String {
        self.state.iter().join("")
    }

    fn value(&self) -> usize {
        let counts = self.state.iter().copied().counts();
        if let MinMaxResult::MinMax(min, max) = counts.values().minmax() {
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
    println!("Template:      {}", poly.state());
    for steps in 0..10 {
        poly.step();
        println!("After step {:2}: {}", steps + 1, poly.state());
    }
    println!("Value: {}", poly.value());
}
