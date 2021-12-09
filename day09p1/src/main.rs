use std::fs::File;
use std::io::{BufRead, BufReader};

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

    let mut total_risk = 0;

    for y in 0..field.len() {
        for (x, &value) in field[y].iter().enumerate() {
            if x > 0 && field[y][x - 1] <= value {
                continue;
            } else if x < field[y].len() - 1 && field[y][x + 1] <= value {
                continue;
            } else if y > 0 && field[y - 1][x] <= value {
                continue;
            } else if y < field.len() - 1 && field[y + 1][x] <= value {
                continue;
            }

            println!("low point {} @ {}, {}", value, x, y);
            total_risk += 1 + value;
        }
    }

    println!("total risk level: {}", total_risk);
}
