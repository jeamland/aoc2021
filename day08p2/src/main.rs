use std::collections::{BTreeSet, HashMap};
use std::fs::File;
use std::io::{BufRead, BufReader};

use itertools::Itertools;

fn deduce_numbers(signals: &[BTreeSet<char>]) -> HashMap<BTreeSet<char>, u32> {
    let mut dictionary = HashMap::new();

    let one = signals.iter().find(|s| s.len() == 2).unwrap();
    let three = signals
        .iter()
        .find(|s| s.len() == 5 && s.is_superset(one))
        .unwrap();
    let four = signals.iter().find(|s| s.len() == 4).unwrap();
    let seven = signals.iter().find(|s| s.len() == 3).unwrap();
    let eight = signals.iter().find(|s| s.len() == 7).unwrap();
    let nine = signals
        .iter()
        .find(|s| s.len() == 6 && s.is_superset(one) && s.is_superset(three))
        .unwrap();

    let bottom_left = eight - nine;

    let two = signals
        .iter()
        .find(|s| s.len() == 5 && s.is_superset(&bottom_left))
        .unwrap();

    let top_right: BTreeSet<char> = two.intersection(one).copied().collect();

    let five = signals
        .iter()
        .find(|s| s.len() == 5 && s.is_disjoint(&top_right))
        .unwrap();
    let six = signals
        .iter()
        .find(|s| s.len() == 6 && s.is_disjoint(&top_right))
        .unwrap();
    let zero = signals
        .iter()
        .find(|s| s.len() == 6 && s != &six && s != &nine)
        .unwrap();

    dictionary.insert(zero.clone(), 0);
    dictionary.insert(one.clone(), 1);
    dictionary.insert(two.clone(), 2);
    dictionary.insert(three.clone(), 3);
    dictionary.insert(four.clone(), 4);
    dictionary.insert(five.clone(), 5);
    dictionary.insert(six.clone(), 6);
    dictionary.insert(seven.clone(), 7);
    dictionary.insert(eight.clone(), 8);
    dictionary.insert(nine.clone(), 9);

    dictionary
}

fn main() {
    let file = File::open(std::env::args_os().nth(1).unwrap()).unwrap();
    let reader = BufReader::new(file);

    let mut sum = 0;

    for line in reader.lines() {
        let line = line.unwrap();

        let (signals, patterns): (&str, &str) = line.split(" | ").collect_tuple().unwrap();

        let signals: Vec<BTreeSet<char>> = signals
            .split_whitespace()
            .map(|s| BTreeSet::from_iter(s.chars()))
            .collect();

        let numbers = deduce_numbers(&signals);

        let number = patterns
            .split_whitespace()
            .map(|d| *numbers.get(&BTreeSet::from_iter(d.chars())).unwrap())
            .reduce(|a, i| a * 10 + i)
            .unwrap();

        println!("{} -> {:04?}", patterns, number);
        sum += number;
    }

    println!("total: {}", sum);
}
