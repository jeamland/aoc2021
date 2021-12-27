use std::collections::HashSet;
use std::fs::File;
use std::io::{BufRead, BufReader};

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum SeaCucumber {
    East,
    South,
}

impl From<char> for SeaCucumber {
    fn from(c: char) -> Self {
        match c {
            '>' => Self::East,
            'v' => Self::South,
            _ => panic!("eeek: {}", c),
        }
    }
}

impl std::fmt::Display for SeaCucumber {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::East => write!(f, ">"),
            Self::South => write!(f, "v"),
        }
    }
}

struct Floor {
    east_floor: HashSet<(usize, usize)>,
    south_floor: HashSet<(usize, usize)>,
    x_max: usize,
    y_max: usize,
}

impl Floor {
    fn new(
        east_floor: HashSet<(usize, usize)>,
        south_floor: HashSet<(usize, usize)>,
        x_max: usize,
        y_max: usize,
    ) -> Self {
        Self {
            east_floor,
            south_floor,
            x_max,
            y_max,
        }
    }

    fn step(&mut self) -> bool {
        let mut new_east = HashSet::new();
        let mut moved = false;

        for &(x, y) in &self.east_floor {
            let target = if x == self.x_max { (0, y) } else { (x + 1, y) };

            if self.east_floor.contains(&target) || self.south_floor.contains(&target) {
                new_east.insert((x, y));
            } else {
                new_east.insert(target);
                moved = true;
            }
        }

        self.east_floor = new_east;

        let mut new_south = HashSet::new();

        for &(x, y) in &self.south_floor {
            let target = if y == self.y_max { (x, 0) } else { (x, y + 1) };

            if self.east_floor.contains(&target) || self.south_floor.contains(&target) {
                new_south.insert((x, y));
            } else {
                new_south.insert(target);
                moved = true;
            }
        }

        self.south_floor = new_south;
        moved
    }

    fn print(&self) {
        for y in 0..=self.y_max {
            for x in 0..=self.x_max {
                if self.east_floor.contains(&(x, y)) {
                    print!(">");
                } else if self.south_floor.contains(&(x, y)) {
                    print!("v");
                } else {
                    print!(".");
                }
            }
            println!();
        }
    }
}

fn main() {
    let file = File::open(std::env::args_os().nth(1).unwrap()).unwrap();
    let reader = BufReader::new(file);

    let mut east_floor = HashSet::new();
    let mut south_floor = HashSet::new();
    let mut x_size = 0;
    let mut y_size = 0;

    for (y, line) in reader.lines().enumerate() {
        let line = line.unwrap();
        x_size = line.len();
        y_size = y;
        line.chars().enumerate().for_each(|(x, c)| match c {
            '>' => {
                east_floor.insert((x, y));
            }
            'v' => {
                south_floor.insert((x, y));
            }
            _ => (),
        });
    }

    let mut floor = Floor::new(east_floor, south_floor, x_size - 1, y_size);
    println!("Initial state:");
    floor.print();

    let mut counter = 0;
    loop {
        counter += 1;
        let moved = floor.step();

        if counter % 10 == 0 {
            println!();
            println!("After {} steps:", counter);
            floor.print();
        }

        if !moved {
            println!();
            println!("No movement after {} steps.", counter);
            floor.print();
            break;
        }
    }
}
