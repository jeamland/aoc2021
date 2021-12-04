use std::fs::File;
use std::io::{BufRead, BufReader};

#[derive(Debug)]
struct Board {
    numbers: [[u32; 5]; 5],
    matched: [[bool; 5]; 5],
}

impl From<Vec<Vec<u32>>> for Board {
    fn from(vec: Vec<Vec<u32>>) -> Self {
        let mut new_self = Board {
            numbers: [[0; 5]; 5],
            matched: [[false; 5]; 5],
        };
        for (index, row) in vec.iter().enumerate().take(5) {
            new_self.numbers[index].copy_from_slice(row);
        }
        new_self
    }
}

impl Board {
    fn mark_match(&mut self, number: u32) -> bool {
        for row in 0..5 {
            for column in 0..5 {
                if self.numbers[row][column] == number {
                    self.matched[row][column] = true;
                }
            }
        }

        self.bingo()
    }

    fn bingo(&mut self) -> bool {
        for row in 0..5 {
            if (0..5).all(|c| self.matched[row][c]) {
                return true;
            }
        }

        for column in 0..5 {
            if (0..5).all(|r| self.matched[r][column]) {
                return true;
            }
        }

        false
    }

    fn unmarked_sum(&self) -> u32 {
        let mut sum = 0;

        for row in 0..5 {
            for column in 0..5 {
                if !self.matched[row][column] {
                    sum += self.numbers[row][column];
                }
            }
        }

        sum
    }

    fn print_board(&self) {
        for row in 0..5 {
            println!(
                "{}",
                self.numbers[row]
                    .iter()
                    .map(|n| format!("{:2}", n))
                    .fold(String::new(), |a, i| format!("{} {}", a, i))
            );
        }
    }
}

fn main() {
    let file = File::open(std::env::args_os().nth(1).unwrap()).unwrap();
    let reader = BufReader::new(file);
    let mut lines = reader.lines();

    let numbers: Vec<u32> = lines
        .next()
        .unwrap()
        .unwrap()
        .split(',')
        .map(|n| n.parse::<u32>().unwrap())
        .collect();
    lines.next();

    let mut boards = Vec::new();
    let mut board_numbers = Vec::new();
    for line in lines {
        let line = line.unwrap();

        if line.is_empty() {
            boards.push(Board::from(board_numbers));
            board_numbers = Vec::new();
            continue;
        }

        board_numbers.push(
            line.split_whitespace()
                .map(|n| n.parse::<u32>().unwrap())
                .collect::<Vec<u32>>(),
        );
    }
    boards.push(Board::from(board_numbers));

    for number in numbers {
        let mut finished = false;
        for board in boards.iter_mut() {
            if board.mark_match(number) {
                dbg!(number);
                board.print_board();
                dbg!(board.unmarked_sum());
                dbg!(board.unmarked_sum() * number);
                finished = true;
                break;
            }
        }
        if finished {
            break;
        }
    }
}
