use std::collections::HashSet;
use std::fs::File;
use std::io::{BufRead, BufReader};

use itertools::Itertools;

fn main() {
    let file = File::open(std::env::args_os().nth(1).unwrap()).unwrap();
    let reader = BufReader::new(file);

    let mut cubes = HashSet::new();

    for (line_number, line) in reader.lines().enumerate() {
        let line = line.unwrap();
        let (state, ranges) = line.split_whitespace().collect_tuple().unwrap();
        let (x_range, y_range, z_range) = if let Some(r) = ranges
            .split_terminator(',')
            .filter_map(|r| {
                let (start, end) = r
                    .split_at(2)
                    .1
                    .split_terminator("..")
                    .collect_tuple()
                    .unwrap();
                let mut start = start.parse::<isize>().unwrap();
                let mut end = end.parse::<isize>().unwrap();

                if (start > 50 && end > 50) || (start < -50 && end < -50) {
                    return None;
                }

                if start < -50 {
                    start = -50;
                } else if start > 50 {
                    start = 50;
                }

                if end < -50 {
                    end = -50;
                } else if end > 50 {
                    end = 50;
                }

                Some(start..=end)
            })
            .collect_tuple()
        {
            r
        } else {
            continue;
        };

        for x in x_range.clone() {
            for y in y_range.clone() {
                for z in z_range.clone() {
                    if state == "on" {
                        cubes.insert((x, y, z));
                    } else {
                        cubes.remove(&(x, y, z));
                    }
                }
            }
        }

        println!("After line {}, {} cubes", line_number + 1, cubes.len());
    }

    println!("{} cubes", cubes.len());
}
