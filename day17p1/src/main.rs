use std::collections::HashSet;
use std::ops::RangeInclusive;

struct XGenerator {
    x: isize,
    dx: isize,
    x_range: RangeInclusive<isize>,
}

impl XGenerator {
    fn new(dx: isize, x_range: RangeInclusive<isize>) -> Self {
        Self { x: 0, dx, x_range }
    }
}

impl Iterator for XGenerator {
    type Item = isize;

    fn next(&mut self) -> Option<Self::Item> {
        self.x += self.dx;
        if self.dx > 0 {
            self.dx -= 1;
        } else if self.x < 0 {
            self.dx += 1;
        }

        if self.dx == 0 && !self.x_range.contains(&self.x) {
            None
        } else {
            Some(self.x)
        }
    }
}

struct YGenerator {
    y: isize,
    dy: isize,
    y_range: RangeInclusive<isize>,
}

impl YGenerator {
    fn new(dy: isize, y_range: RangeInclusive<isize>) -> Self {
        Self { y: 0, dy, y_range }
    }
}

impl Iterator for YGenerator {
    type Item = isize;

    fn next(&mut self) -> Option<Self::Item> {
        self.y += self.dy;
        self.dy -= 1;

        if self.y < *self.y_range.start() {
            None
        } else {
            Some(self.y)
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

    let dx_candidates: Vec<(usize, isize)> = (1..max_x)
        .enumerate()
        .filter(|&(_, dx)| XGenerator::new(dx, x_range.clone()).any(|x| x_range.contains(&x)))
        .collect();

    let dy_candidates: Vec<(usize, isize)> = (min_y..1000)
        .enumerate()
        .filter(|&(_, dy)| YGenerator::new(dy, y_range.clone()).any(|y| y_range.contains(&y)))
        .collect();

    let mut d_candidates = Vec::new();
    for (tx, dx) in dx_candidates {
        for dy in dy_candidates
            .iter()
            .filter(|(t, _)| *t >= tx)
            .map(|(_, dy)| *dy)
        {
            d_candidates.push((dx, dy));
        }
    }

    let candidates: HashSet<isize> = d_candidates.iter().map(|(_, dy)| *dy).collect();

    let max_height = candidates
        .iter()
        .map(|dy| YGenerator::new(*dy, y_range.clone()).max().unwrap())
        .max()
        .unwrap();
    dbg!(max_height);
}
