use std::collections::HashMap;
use std::fs::File;
use std::io::{BufRead, BufReader};

fn find_path(cavern: Vec<Vec<usize>>) -> usize {
    let x_max = cavern[0].len() - 1;
    let y_max = cavern.len() - 1;

    let mut f_score = vec![(0, 0, x_max + y_max)];
    let mut g_score = HashMap::from([((0, 0), 0)]);

    while !f_score.is_empty() {
        let (x, y, _) = f_score.remove(0);
        println!("({:3}, {:3}) {:6} {:6}", x, y, f_score.len(), g_score.len());

        if x == x_max && y == y_max {
            return *g_score.get(&(x, y)).unwrap();
        }

        let g = g_score.get(&(x, y)).copied().unwrap_or(usize::MAX);

        let mut neighbours = Vec::new();
        if x > 0 {
            neighbours.push((x - 1, y));
        }
        if y > 0 {
            neighbours.push((x, y - 1));
        }
        if x < x_max {
            neighbours.push((x + 1, y));
        }
        if y < y_max {
            neighbours.push((x, y + 1));
        }

        for (dx, dy) in neighbours {
            let ng = g_score.get(&(dx, dy)).copied().unwrap_or(usize::MAX);
            let tg = g + cavern[dy][dx];
            if tg < ng {
                g_score.insert((dx, dy), tg);
                let f = tg + (x_max - dx) + (y_max - dy);
                if let Some(pos) = f_score.iter().position(|&(_, _, ef)| ef > f) {
                    f_score.insert(pos, (dx, dy, f));
                } else {
                    f_score.push((dx, dy, f));
                }
            }
        }
    }

    panic!("all is lost");
}

fn main() {
    let file = File::open(std::env::args_os().nth(1).unwrap()).unwrap();
    let reader = BufReader::new(file);

    let cavern: Vec<Vec<usize>> = reader
        .lines()
        .map(|l| {
            l.unwrap()
                .chars()
                .map(|c| c.to_digit(10).unwrap() as usize)
                .collect()
        })
        .collect();

    let mut bigger_cavern = Vec::new();
    for row in &cavern {
        let mut bigger_row = row.clone();
        for n in 1..5 {
            for p in 0..row.len() {
                let mut v = bigger_row[p] + n;
                if v > 9 {
                    v -= 9;
                }
                bigger_row.push(v);
            }
        }
        bigger_cavern.push(bigger_row);
    }
    for n in 1..5 {
        for p in 0..cavern.len() {
            let bigger_row = bigger_cavern[p]
                .iter()
                .map(|&v| {
                    let v = v + n;
                    if v > 9 {
                        v - 9
                    } else {
                        v
                    }
                })
                .collect();
            bigger_cavern.push(bigger_row);
        }
    }

    let risk = find_path(bigger_cavern);
    println!("Lowest risk: {}", risk);
}
