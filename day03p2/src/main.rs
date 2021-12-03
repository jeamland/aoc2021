use std::fs::File;
use std::io::{BufRead, BufReader};

fn main() {
    let file = File::open(std::env::args_os().nth(1).unwrap()).unwrap();
    let reader = BufReader::new(file);

    let mut values = Vec::new();

    for digits in reader.lines().map(|line| {
        line.unwrap()
            .chars()
            .map(|c| c.to_digit(10).unwrap())
            .collect::<Vec<u32>>()
    }) {
        values.push(digits);
    }

    let oxygen = filter_for_criteria(&values, |total, length| {
        (length % 2 == 0 && total >= length / 2) || total > length / 2
    });
    let co2 = filter_for_criteria(&values, |total, length| {
        !((length % 2 == 0 && total >= length / 2) || total > length / 2)
    });
    dbg!(oxygen, co2, oxygen * co2);
}

fn filter_for_criteria(values: &[Vec<u32>], filter: impl Fn(u32, u32) -> bool) -> u32 {
    let mut values = values.to_vec();

    for index in 0..values[0].len() {
        let total: u32 = values.iter().map(|d| d[index]).sum();
        let target = if filter(total, values.len() as u32) {
            1
        } else {
            0
        };

        values.retain(|d| d[index] == target);
        if values.len() <= 1 {
            break;
        }
    }

    values
        .pop()
        .unwrap()
        .into_iter()
        .reduce(|a, i| (a << 1) + i)
        .unwrap()
}
