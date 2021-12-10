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

const fn score(c: char) -> u32 {
    match c {
        ')' => 3,
        ']' => 57,
        '}' => 1197,
        '>' => 25137,
        _ => panic!("eeeeeeek"),
    }
}

fn main() {
    let file = File::open(std::env::args_os().nth(1).unwrap()).unwrap();
    let reader = BufReader::new(file);

    let mut total_score = 0;

    for line in reader.lines() {
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
                        total_score += score(c);
                        break;
                    }
                }
                _ => panic!("eeeek"),
            }
        }
    }

    println!("Total score: {}", total_score);
}
