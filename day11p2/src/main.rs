use std::fs::File;
use std::io::{BufRead, BufReader};

struct Field(Vec<Vec<u32>>, Box<term::StdoutTerminal>);

impl Field {
    fn step(&mut self) -> usize {
        let mut flashes = 0;

        for row in self.0.iter_mut() {
            for o in row.iter_mut() {
                *o += 1;
            }
        }

        let mut finished = false;
        while !finished {
            finished = true;

            for y in 0..self.0.len() {
                for x in 0..self.0[y].len() {
                    if self.0[y][x] == 10 {
                        for dx in x.saturating_sub(1)..usize::min(self.0[y].len(), x + 2) {
                            for dy in y.saturating_sub(1)..usize::min(self.0.len(), y + 2) {
                                if self.0[dy][dx] <= 9 {
                                    self.0[dy][dx] += 1;
                                }
                            }
                        }
                        flashes += 1;
                        self.0[y][x] = 11;
                        finished = false;
                    }
                }
            }
        }

        for row in self.0.iter_mut() {
            for o in row.iter_mut() {
                if *o > 9 {
                    *o = 0;
                }
            }
        }

        flashes
    }

    fn print(&mut self) {
        for row in &self.0 {
            for o in row {
                if *o == 0 {
                    self.1.fg(term::color::WHITE).unwrap();
                }
                print!("{}", o);
                if *o == 0 {
                    self.1.reset().unwrap();
                }
            }
            println!();
        }
        println!();
    }
}

fn main() {
    let t = term::stdout().unwrap();
    let file = File::open(std::env::args_os().nth(1).unwrap()).unwrap();
    let reader = BufReader::new(file);

    let mut field = Field(
        reader
            .lines()
            .map(|l| {
                l.unwrap()
                    .chars()
                    .map(|c| c.to_digit(10).unwrap())
                    .collect()
            })
            .collect(),
        t,
    );

    let mut counter = 0;

    println!("Before any steps:");
    field.print();

    while field.step() != 100 {
        counter += 1;

        if counter % 10 == 0 {
            println!("After step {}:", counter);
            field.print();
        }
    }

    println!("After step: {}", counter + 1);
    field.print();
}
