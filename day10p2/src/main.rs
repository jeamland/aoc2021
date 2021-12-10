use std::fs::File;
use std::io::{BufRead, BufReader};

const fn closer(c: char) -> char {
    match c {
        '(' => ')',
        '[' => ']',
        '{' => '}',
        '<' => '>',
        _ => panic!("eek"),
    }
}

const fn opener(c: char) -> char {
    match c {
        ')' => '(',
        ']' => '[',
        '}' => '{',
        '>' => '<',
        _ => panic!("eek"),
    }
}

const fn score(c: char) -> u64 {
    match c {
        '(' => 1,
        '[' => 2,
        '{' => 3,
        '<' => 4,
        _ => panic!("eeeeeeek"),
    }
}

fn main() {
    let file = File::open(std::env::args_os().nth(1).unwrap()).unwrap();
    let reader = BufReader::new(file);

    let mut scores = Vec::new();

    'outer: for line in reader.lines() {
        let line = line.unwrap();

        let mut stack = Vec::new();
        for c in line.chars() {
            match c {
                '(' | '[' | '{' | '<' => stack.push(c),
                ')' | ']' | '}' | '>' => {
                    let o = stack.pop().unwrap();
                    if o != opener(c) {
                        println!(
                            "{} - Expected {}, but found {} instead.",
                            line,
                            closer(o),
                            c
                        );
                        continue 'outer;
                    }
                }
                _ => panic!("eeeek"),
            }
        }

        let mut line_score = 0;
        while let Some(c) = stack.pop() {
            line_score = line_score * 5 + score(c);
        }
        scores.push(line_score);
    }

    scores.sort_unstable();
    println!("Scores: {:?}", scores);
    println!("Winner: {}", scores[scores.len() / 2]);
}
