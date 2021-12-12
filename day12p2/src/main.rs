use std::collections::{HashMap, HashSet};
use std::fs::File;
use std::io::{BufRead, BufReader};

use itertools::Itertools;

#[derive(Debug)]
struct Cave {
    name: String,
    big: bool,
    connections: HashSet<String>,
}

impl Cave {
    fn new(name: &str) -> Self {
        let name = name.to_string();
        let big = name.chars().all(|c| c.is_ascii_uppercase());
        Self {
            name,
            big,
            connections: HashSet::new(),
        }
    }
}

struct Pathfinder {
    caves: HashMap<String, Cave>,
    visited_small: HashSet<String>,
    visited_small_extra: Option<String>,
    stack: Vec<(String, Vec<String>)>,
}

impl Pathfinder {
    fn new(caves: HashMap<String, Cave>) -> Self {
        let start = caves.get("start").unwrap();
        let visited_small = HashSet::from([start.name.clone()]);
        let stack = vec![(
            start.name.clone(),
            start.connections.iter().cloned().collect(),
        )];

        Self {
            caves,
            visited_small,
            visited_small_extra: None,
            stack,
        }
    }
}

impl Iterator for Pathfinder {
    type Item = String;

    fn next(&mut self) -> Option<Self::Item> {
        while !self.stack.is_empty() {
            let (cave, connections) = self.stack.last_mut().unwrap();

            if connections.is_empty() {
                if self.visited_small_extra == Some(cave.clone()) {
                    self.visited_small_extra = None;
                } else {
                    self.visited_small.remove(cave);
                }
                self.stack.pop();
                continue;
            }

            let next_cave = self.caves.get(&connections.pop().unwrap()).unwrap();
            if next_cave.name == "end" {
                let mut path: Vec<&str> = self.stack.iter().map(|(n, _)| n.as_str()).collect();
                path.push("end");
                return Some(path.join(","));
            } else if next_cave.name == "start" {
                continue;
            }

            if !next_cave.big {
                if self.visited_small.contains(&next_cave.name) {
                    if self.visited_small_extra.is_some() {
                        continue;
                    } else {
                        self.visited_small_extra = Some(next_cave.name.clone());
                    }
                } else {
                    self.visited_small.insert(next_cave.name.clone());
                }
            }

            self.stack.push((
                next_cave.name.clone(),
                next_cave.connections.iter().cloned().collect(),
            ));
        }

        None
    }
}

fn main() {
    let file = File::open(std::env::args_os().nth(1).unwrap()).unwrap();
    let reader = BufReader::new(file);

    let mut caves = HashMap::new();

    for line in reader.lines() {
        let line = line.unwrap();

        let (first, second): (&str, &str) = line.split_terminator('-').collect_tuple().unwrap();
        let first = first.to_string();
        let second = second.to_string();

        let first_cave = caves
            .entry(first.clone())
            .or_insert_with(|| Cave::new(&first));
        first_cave.connections.insert(second.clone());

        let second_cave = caves
            .entry(second.clone())
            .or_insert_with(|| Cave::new(&second));
        second_cave.connections.insert(first.clone());
    }

    dbg!(&caves);

    let mut counter = 0;

    for path in Pathfinder::new(caves) {
        println!("{}", path);
        counter += 1;
    }

    println!("{} paths", counter);
}
