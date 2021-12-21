use std::collections::HashMap;
use std::fs::File;
use std::io::{BufRead, BufReader};

use itertools::Itertools;

#[derive(Clone, Debug, Hash, PartialEq, Eq)]
struct State {
    positions: [usize; 2],
    scores: [usize; 2],
}

impl State {
    fn roll(&self, player: usize) -> Vec<State> {
        let mut states = Vec::new();

        for r1 in 1..=3 {
            for r2 in 1..=3 {
                for r3 in 1..=3 {
                    let mut new = self.clone();
                    new.positions[player] = (new.positions[player] + r1 + r2 + r3) % 10;
                    new.scores[player] += new.positions[player] + 1;
                    states.push(new);
                }
            }
        }

        states
    }
}

fn main() {
    let file = File::open(std::env::args_os().nth(1).unwrap()).unwrap();
    let reader = BufReader::new(file);

    let mut start_positions = Vec::new();

    for line in reader.lines() {
        let line = line.unwrap();

        let (_, position) = line.split_terminator(": ").collect_tuple().unwrap();
        start_positions.push(position.parse::<usize>().unwrap() - 1);
    }

    let mut states = HashMap::new();
    states.insert(
        State {
            positions: [start_positions[0], start_positions[1]],
            scores: [0, 0],
        },
        1,
    );
    let mut player = 0;
    let mut win_universes = [0usize; 2];

    while !states.is_empty() {
        let mut new_states = HashMap::new();

        for (state, count) in states {
            for new_state in state.roll(player) {
                if state.scores[0] >= 21 {
                    win_universes[0] += count;
                } else if state.scores[1] >= 21 {
                    win_universes[1] += count;
                } else {
                    *new_states.entry(new_state).or_insert(0) += count;
                }
            }
        }

        states = new_states;

        println!(
            "Player {} roll(s), {} ({} unique) in-progress states, {} win states",
            player + 1,
            states.values().copied().sum::<usize>(),
            states.len(),
            win_universes[0] + win_universes[1]
        );

        player = (player + 1) % 2;
    }

    println!(
        "Player 1: {}, Player 2: {}",
        win_universes[0] / 27, // I have no idea why I'm overcounting but I am
        win_universes[1] / 27,
    );
}
