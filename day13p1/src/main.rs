use std::collections::HashSet;
use std::fs::File;
use std::io::{BufRead, BufReader};

use itertools::Itertools;

#[derive(Debug)]
enum Fold {
    Horizontal(u32),
    Vertical(u32),
}

impl Fold {
    fn translate(&self, point: (u32, u32)) -> (u32, u32) {
        match *self {
            Self::Horizontal(y) => {
                if point.1 > y {
                    (point.0, y - (point.1 - y))
                } else {
                    (point.0, point.1)
                }
            }
            Self::Vertical(x) => {
                if point.0 > x {
                    (x - (point.0 - x), point.1)
                } else {
                    (point.0, point.1)
                }
            }
        }
    }
}

#[derive(Debug)]
struct TransparentPaper(HashSet<(u32, u32)>);

impl TransparentPaper {
    fn fold(&mut self, fold: &Fold) {
        self.0 = self.0.iter().map(|&p| fold.translate(p)).collect();
    }

    fn dot_count(&self) -> usize {
        self.0.len()
    }

    fn print(&self) {
        for y in 0..=self.0.iter().map(|(_, y)| *y).max().unwrap() {
            for x in 0..=self.0.iter().map(|(x, _)| *x).max().unwrap() {
                if self.0.contains(&(x, y)) {
                    print!("#");
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

    let mut points = HashSet::new();
    let mut folds = Vec::new();
    let mut reading_points = true;

    for line in reader.lines() {
        let line = line.unwrap();
        if line.is_empty() {
            reading_points = false;
            continue;
        }

        if reading_points {
            points.insert(
                line.split_terminator(',')
                    .map(|v| v.parse::<u32>().unwrap())
                    .collect_tuple::<(u32, u32)>()
                    .unwrap(),
            );
        } else {
            let (_, _, linespec) = line.split_whitespace().collect_tuple().unwrap();
            let (axis, value) = linespec.split_terminator('=').collect_tuple().unwrap();
            let value = value.parse::<u32>().unwrap();
            let fold = match axis {
                "x" => Fold::Vertical(value),
                "y" => Fold::Horizontal(value),
                _ => panic!("eek"),
            };
            folds.push(fold);
        }
    }

    let mut paper = TransparentPaper(points);
    paper.print();
    println!();
    let fold = folds.first().unwrap();
    paper.fold(fold);
    paper.print();
    println!("Dots: {}", paper.dot_count());
}
