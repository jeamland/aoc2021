use std::fs::File;
use std::io::{BufRead, BufReader};
use std::str::FromStr;

use anyhow::{anyhow, Error, Result};
use itertools::Itertools;

#[derive(Debug)]
struct Point(u32, u32);

impl FromStr for Point {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self> {
        let (x, y) = s
            .split(',')
            .map(|v| v.parse::<u32>())
            .collect_tuple()
            .ok_or(anyhow!("yikes"))?;
        Ok(Self(x?, y?))
    }
}

impl std::fmt::Display for Point {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "({}, {})", self.0, self.1)
    }
}

#[derive(Debug)]
struct Line(Point, Point);

impl FromStr for Line {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self> {
        let (start, end) = s
            .split_terminator(" -> ")
            .map(|p| p.parse::<Point>())
            .collect_tuple()
            .ok_or(anyhow!("yikes"))?;

        Ok(Self(start?, end?))
    }
}

impl std::fmt::Display for Line {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{} -> {}", self.0, self.1)
    }
}

struct Field(Vec<Vec<u32>>);

impl Field {
    fn new(x: usize, y: usize) -> Self {
        Self(vec![vec![0; x]; y])
    }

    fn mark_line(&mut self, line: &Line) {
        if line.0 .0 == line.1 .0 {
            let (start, end) = (line.0 .1.min(line.1 .1), line.0 .1.max(line.1 .1));

            for y in start..=end {
                self.0[y as usize][line.0 .0 as usize] += 1;
            }
        } else if line.0 .1 == line.1 .1 {
            let (start, end) = (line.0 .0.min(line.1 .0), line.0 .0.max(line.1 .0));

            for x in start..=end {
                self.0[line.0 .1 as usize][x as usize] += 1;
            }
        } else {
            let (start_x, end_x) = (line.0 .0.min(line.1 .0), line.0 .0.max(line.1 .0));
            let mut xs: Vec<u32> = (start_x..=end_x).collect();
            if line.0 .0 > line.1 .0 {
                xs.reverse();
            }

            let (start_y, end_y) = (line.0 .1.min(line.1 .1), line.0 .1.max(line.1 .1));
            let mut ys: Vec<u32> = (start_y..=end_y).collect();
            if line.0 .1 > line.1 .1 {
                ys.reverse();
            }

            for (x, y) in xs.into_iter().zip(ys.into_iter()) {
                self.0[y as usize][x as usize] += 1;
            }
        }
    }

    fn count_overlaps(&self) -> u32 {
        self.0
            .iter()
            .map(|row| row.iter().filter(|v| **v > 1).count())
            .sum::<usize>() as u32
    }

    fn print(&self) {
        for row in &self.0 {
            for value in row {
                if *value == 0 {
                    print!(".")
                } else {
                    print!("{}", value)
                };
            }
            println!();
        }
    }
}

fn main() -> Result<()> {
    let file = File::open(std::env::args_os().nth(1).unwrap()).unwrap();
    let reader = BufReader::new(file);

    let lines: Vec<Line> = reader
        .lines()
        .map(|line| line?.parse::<Line>())
        .collect::<Result<Vec<Line>>>()?;

    let max_x = lines
        .iter()
        .map(|l| l.0 .0.max(l.1 .0))
        .max()
        .ok_or(anyhow!("yikes"))? as usize;
    let max_y = lines
        .iter()
        .map(|l| l.0 .1.max(l.1 .1))
        .max()
        .ok_or(anyhow!("yikes"))? as usize;

    let mut field = Field::new(max_x + 1, max_y + 1);

    for line in lines {
        println!("{}", line);
        field.mark_line(&line);
    }

    field.print();
    dbg!(field.count_overlaps());

    Ok(())
}
