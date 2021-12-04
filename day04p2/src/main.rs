use std::fs::File;
use std::io::{BufRead, BufReader};

use ansi_term::Style;

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

    fn bingo(&self) -> bool {
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
        let bold = Style::new().bold();

        for row in 0..5 {
            println!(
                "{}",
                self.numbers[row]
                    .iter()
                    .enumerate()
                    .map(|(c, n)| {
                        let s = format!("{:2}", n);
                        if self.matched[row][c] {
                            bold.paint(s).to_string()
                        } else {
                            s
                        }
                    })
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

    let mut final_number = 0;
    for number in numbers {
        dbg!(number);
        boards.iter_mut().for_each(|b| {
            b.mark_match(number);
        });
        if boards.len() > 1 {
            boards.retain(|b| !b.bingo());
        } else if boards[0].bingo() {
            final_number = number;
            break;
        }
    }

    let final_board = boards.pop().unwrap();
    dbg!(final_number);
    final_board.print_board();
    dbg!(
        final_board.unmarked_sum(),
        final_board.unmarked_sum() * final_number
    );
}
