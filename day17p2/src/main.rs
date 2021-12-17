use std::collections::HashSet;
use std::ops::RangeInclusive;

#[derive(Debug)]
enum Time {
    Transient(usize),
    Final(usize),
}

struct XGenerator {
    time: usize,
    x: isize,
    dx: isize,
    x_range: RangeInclusive<isize>,
}

impl XGenerator {
    fn new(dx: isize, x_range: RangeInclusive<isize>) -> Self {
        Self {
            time: 0,
            x: 0,
            dx,
            x_range,
        }
    }
}

impl Iterator for XGenerator {
    type Item = (Time, isize);

    fn next(&mut self) -> Option<Self::Item> {
        if self.dx == 0 {
            return None;
        }

        self.time += 1;
        self.x += self.dx;

        if self.dx > 0 {
            self.dx -= 1;
        } else if self.x < 0 {
            self.dx += 1;
        }

        if self.x > *self.x_range.end() {
            None
        } else if self.dx == 0 {
            Some((Time::Final(self.time), self.x))
        } else {
            Some((Time::Transient(self.time), self.x))
        }
    }
}

struct YGenerator {
    time: usize,
    y: isize,
    dy: isize,
    y_range: RangeInclusive<isize>,
}

impl YGenerator {
    fn new(dy: isize, y_range: RangeInclusive<isize>) -> Self {
        Self {
            time: 0,
            y: 0,
            dy,
            y_range,
        }
    }
}

impl Iterator for YGenerator {
    type Item = (usize, isize);

    fn next(&mut self) -> Option<Self::Item> {
        self.time += 1;
        self.y += self.dy;
        self.dy -= 1;

        if self.y < *self.y_range.start() {
            None
        } else {
            Some((self.time, self.y))
        }
    }
}

fn main() {
    let input = std::fs::read_to_string(std::env::args_os().nth(1).unwrap()).unwrap();
    let input: Vec<&str> = input.trim().split_whitespace().collect();

    let x_range: Vec<isize> = input[2][2..]
        .trim_end_matches(',')
        .split_terminator("..")
        .map(|v| v.parse::<isize>().unwrap())
        .collect();
    let min_x = isize::min(x_range[0], x_range[1]);
    let max_x = isize::max(x_range[0], x_range[1]);
    let x_range = min_x..=max_x;

    let y_range: Vec<isize> = input[3][2..]
        .split_terminator("..")
        .map(|v| v.parse::<isize>().unwrap())
        .collect();
    let min_y = isize::min(y_range[0], y_range[1]);
    let max_y = isize::max(y_range[0], y_range[1]);
    let y_range = min_y..=max_y;

    let dx_candidates: Vec<(Time, isize)> = (0..=max_x)
        .map(|dx| {
            XGenerator::new(dx, x_range.clone())
                .filter(|(_, x)| x_range.contains(x))
                .map(move |(t, _)| (t, dx))
        })
        .flatten()
        .collect();

    let dy_candidates: Vec<(usize, isize)> = (min_y..1000)
        .map(|dy| {
            YGenerator::new(dy, y_range.clone())
                .filter(|(_, y)| y_range.contains(y))
                .map(move |(t, _)| (t, dy))
        })
        .flatten()
        .collect();

    let mut d_candidates = HashSet::new();
    for (tx, dx) in dx_candidates {
        for dy in dy_candidates
            .iter()
            .filter(|(t, _)| match tx {
                Time::Final(tx) => *t >= tx,
                Time::Transient(tx) => *t == tx,
            })
            .map(|(_, dy)| *dy)
        {
            d_candidates.insert((dx, dy));
        }
    }

    println!("Trajectories: {}", d_candidates.len());
}
