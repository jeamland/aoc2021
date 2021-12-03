use std::fs::File;
use std::io::{BufRead, BufReader};

fn main() {
    let file = File::open(std::env::args_os().nth(1).unwrap()).unwrap();
    let reader = BufReader::new(file);

    let mut accumulator = Vec::new();
    let mut counter = 0;

    for mut digits in reader.lines().map(|line| {
        line.unwrap()
            .chars()
            .map(|c| c.to_digit(10).unwrap())
            .collect::<Vec<u32>>()
    }) {
        if accumulator.is_empty() {
            accumulator.append(&mut digits);
        } else {
            for (index, value) in digits.iter().enumerate() {
                accumulator[index] += value;
            }
        }

        counter += 1;
    }

    let gamma: u32 = accumulator
        .iter()
        .map(|v| if *v > (counter / 2) { 1 } else { 0 })
        .reduce(|a, i| (a << 1) + i)
        .unwrap();

    let width = accumulator.len() as u32;
    let epsilon = !gamma & ((1 << width) - 1);

    println!(
        "gamma={} ({:010b}) epsilon={} ({:010b}) product={}",
        gamma,
        gamma,
        epsilon,
        epsilon,
        gamma * epsilon
    );
}
