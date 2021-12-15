use std::collections::{HashMap, HashSet};
use std::fs::File;
use std::io::{BufRead, BufReader};

struct Pathfinder {
    cavern: Vec<Vec<u32>>,
    visited: HashSet<(usize, usize)>,
    distances: HashMap<(usize, usize), u32>,
}

impl Pathfinder {
    fn new(cavern: Vec<Vec<u32>>) -> Self {
        Self {
            cavern,
            visited: HashSet::from([(0, 0)]),
            distances: HashMap::from([((0, 0), 0)]),
        }
    }

    fn find_path(&mut self) -> u32 {
        let (mut x, mut y): (usize, usize) = (0, 0);
        let mut distance = 0;

        while x != self.cavern[0].len() - 1 || y != self.cavern.len() - 1 {
            let mut neighbours = Vec::new();
            if x > 0 {
                neighbours.push((x - 1, y));
            }
            if y > 0 {
                neighbours.push((x, y - 1));
            }
            if x < self.cavern[0].len() - 1 {
                neighbours.push((x + 1, y));
            }
            if y < self.cavern.len() - 1 {
                neighbours.push((x, y + 1));
            }

            for (dx, dy) in neighbours {
                let d = self.distances.entry((dx, dy)).or_insert(u32::MAX);
                let cd = distance + self.cavern[dy][dx];
                if cd < *d {
                    *d = cd;
                }
            }

            let new = self
                .distances
                .iter()
                .filter(|(&p, &d)| d > 0 && !self.visited.contains(&p))
                .min_by_key(|(_, &d)| d);
            if new.is_none() {
                break;
            }
            let new = new.unwrap();
            x = new.0 .0;
            y = new.0 .1;
            distance = *new.1;

            self.visited.insert((x, y));
        }

        distance
    }

    fn print(&self) {
        for y in 0..self.cavern.len() {
            for x in 0..self.cavern[y].len() {
                print!("{} [{:2}] ", self.cavern[y][x], self.distances[&(x, y)]);
            }
            println!();
        }
    }
}

fn main() {
    let file = File::open(std::env::args_os().nth(1).unwrap()).unwrap();
    let reader = BufReader::new(file);

    let cavern: Vec<Vec<u32>> = reader
        .lines()
        .map(|l| {
            l.unwrap()
                .chars()
                .map(|c| c.to_digit(10).unwrap())
                .collect()
        })
        .collect();

    let mut finder = Pathfinder::new(cavern);
    let risk = finder.find_path();
    finder.print();
    println!("Lowest risk: {}", risk);
}
