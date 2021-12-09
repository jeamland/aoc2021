use std::fs::File;
use std::io::{BufRead, BufReader};

use itertools::Itertools;

struct Field(Vec<Vec<u32>>);

impl Field {
    fn find_candidate(&self) -> Option<(usize, usize)> {
        for y in 0..self.0.len() {
            for x in 0..self.0[0].len() {
                if self.0[y][x] < 9 {
                    return Some((y, x));
                }
            }
        }

        None
    }

    fn mark_horizontal(&mut self, y: usize, x: usize, marker: u32) {
        if self.0[y][x] >= 9 {
            return;
        }

        for dx in x..self.0[y].len() {
            if self.0[y][dx] >= 9 {
                break;
            }

            self.0[y][dx] = marker;

            if y > 0 && self.0[y - 1][dx] < 9 {
                self.mark_horizontal(y - 1, dx, marker);
            }
            if y < self.0.len() - 1 && self.0[y + 1][dx] < 9 {
                self.mark_horizontal(y + 1, dx, marker);
            }
        }

        for dx in (0..x).rev() {
            if self.0[y][dx] >= 9 {
                break;
            }

            self.0[y][dx] = marker;

            if y > 0 && self.0[y - 1][dx] < 9 {
                self.mark_horizontal(y - 1, dx, marker);
            }
            if y < self.0.len() - 1 && self.0[y + 1][dx] < 9 {
                self.mark_horizontal(y + 1, dx, marker);
            }
        }
    }

    fn basin_sizes(&self) -> Vec<usize> {
        let counts = self.0.iter().flatten().copied().filter(|&v| v > 9).counts();
        let mut counts: Vec<usize> = counts.values().copied().collect();
        counts.sort_unstable();
        counts.reverse();
        counts
    }

    fn print(&self) {
        for row in &self.0 {
            for c in row {
                print!("{:3}", c);
            }
            println!();
        }
    }
}

fn main() {
    let file = File::open(std::env::args_os().nth(1).unwrap()).unwrap();
    let reader = BufReader::new(file);

    let field: Vec<Vec<u32>> = reader
        .lines()
        .map(|l| {
            l.unwrap()
                .chars()
                .map(|c| c.to_digit(10).unwrap())
                .collect()
        })
        .collect();

    let mut field = Field(field);
    let mut marker = 10;

    while let Some((y, x)) = field.find_candidate() {
        field.mark_horizontal(y, x, marker);
        marker += 1;
    }

    field.print();
    println!("{:?}", field.basin_sizes());
    println!(
        "{:?}",
        field.basin_sizes().iter().take(3).product::<usize>()
    );
}
