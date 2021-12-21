use std::fs::File;
use std::io::{BufRead, BufReader};

use itertools::Itertools;

#[derive(Debug, Default)]
struct VeryBoringDice {
    value: usize,
}

impl VeryBoringDice {
    fn roll(&mut self) -> usize {
        let result = self.value + 1;
        self.value = (self.value + 1) % 100;
        result
    }
}

fn main() {
    let file = File::open(std::env::args_os().nth(1).unwrap()).unwrap();
    let reader = BufReader::new(file);

    let mut positions = Vec::new();

    for line in reader.lines() {
        let line = line.unwrap();

        let (_, position) = line.split_terminator(": ").collect_tuple().unwrap();
        positions.push(position.parse::<usize>().unwrap() - 1);
    }

    let mut scores = vec![0; positions.len()];
    let mut dice = VeryBoringDice::default();
    let mut player = 0;
    let mut roll_count = 0;

    while !scores.iter().any(|s| *s >= 1000) {
        let r1 = dice.roll();
        let r2 = dice.roll();
        let r3 = dice.roll();
        positions[player] = (positions[player] + r1 + r2 + r3) % 10;
        scores[player] += positions[player] + 1;

        println!(
            "Player {} rolls {}+{}+{} and moves to space {} for a total score of {}.",
            player + 1,
            r1,
            r2,
            r3,
            positions[player] + 1,
            scores[player]
        );

        player = (player + 1) % positions.len();
        roll_count += 3;
    }

    let losing_score = *scores.iter().min().unwrap();
    println!(
        "Losing score {} * roll count {} = {}",
        losing_score,
        roll_count,
        losing_score * roll_count
    );
}
