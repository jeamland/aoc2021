use std::collections::{HashMap, HashSet};
use std::fs::File;
use std::io::{stdout, BufRead, BufReader, Write};

use crossterm::{cursor, style, style::Stylize, terminal, ExecutableCommand, QueueableCommand};

struct Pathfinder {
    cavern: Vec<Vec<usize>>,
    stdout: std::io::Stdout,
    current: (usize, usize),
    came_from: HashMap<(usize, usize), (usize, usize)>,
    previous_path: HashSet<(usize, usize)>,
}

impl Pathfinder {
    fn new(cavern: Vec<Vec<usize>>) -> Self {
        let mut stdout = stdout();
        stdout.execute(terminal::EnterAlternateScreen).unwrap();
        stdout.execute(cursor::Hide).unwrap();

        Self {
            cavern,
            stdout,
            current: (0, 0),
            came_from: HashMap::new(),
            previous_path: HashSet::new(),
        }
    }

    fn draw(&mut self) {
        for (y, row) in self.cavern.iter().enumerate() {
            self.stdout.queue(cursor::MoveTo(0, y as u16)).unwrap();
            for &v in row {
                self.stdout
                    .queue(style::PrintStyledContent(format!("{} ", v).reset()))
                    .unwrap();
            }
        }

        self.stdout
            .queue(cursor::MoveTo(0, self.cavern.len() as u16 + 1))
            .unwrap();
        self.stdout
            .queue(style::PrintStyledContent(
                format!("Current Score: {:4}", 0).reset(),
            ))
            .unwrap();

        self.stdout.flush().unwrap();
    }

    fn update(&mut self) {
        for &(x, y) in &self.previous_path {
            self.stdout
                .queue(cursor::MoveTo(x as u16 * 2, y as u16))
                .unwrap();
            self.stdout
                .queue(style::PrintStyledContent(
                    format!("{} ", self.cavern[y][x]).reset(),
                ))
                .unwrap();
        }

        let mut path = HashSet::from([self.current]);
        let mut current = self.current;
        loop {
            if current == (0, 0) {
                break;
            }
            current = *self.came_from.get(&current).unwrap();
            path.insert(current);
        }

        for &(x, y) in self.previous_path.difference(&path) {
            self.stdout
                .queue(cursor::MoveTo(x as u16 * 2, y as u16))
                .unwrap();
            self.stdout
                .queue(style::PrintStyledContent(
                    format!("{} ", self.cavern[y][x]).reset(),
                ))
                .unwrap();
        }

        for &(x, y) in &path {
            self.stdout
                .queue(cursor::MoveTo(x as u16 * 2, y as u16))
                .unwrap();
            self.stdout
                .queue(style::PrintStyledContent(
                    format!("{} ", self.cavern[y][x]).green().bold(),
                ))
                .unwrap();
        }

        let score = path.iter().map(|&(x, y)| self.cavern[y][x]).sum::<usize>() - self.cavern[0][0];

        self.stdout
            .queue(cursor::MoveTo(0, self.cavern.len() as u16 + 1))
            .unwrap();
        self.stdout
            .queue(style::PrintStyledContent(
                format!("Current Score: {:4}", score).reset(),
            ))
            .unwrap();

        self.previous_path = path;

        self.stdout.flush().unwrap();
        std::thread::sleep(std::time::Duration::from_millis(10));
    }

    fn find_path(&mut self) -> usize {
        self.draw();

        let x_max = self.cavern[0].len() - 1;
        let y_max = self.cavern.len() - 1;

        let mut f_score = vec![(0, 0, x_max + y_max)];
        let mut g_score = HashMap::from([((0, 0), 0)]);

        while !f_score.is_empty() {
            let (x, y, _) = f_score.remove(0);
            self.current = (x, y);
            self.update();

            if x == x_max && y == y_max {
                crossterm::event::read().unwrap();
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
                let tg = g + self.cavern[dy][dx];
                if tg < ng {
                    self.came_from.insert((dx, dy), (x, y));
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
}

impl Drop for Pathfinder {
    fn drop(&mut self) {
        self.stdout.execute(cursor::Show).unwrap();
        self.stdout.execute(terminal::LeaveAlternateScreen).unwrap();
    }
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

    let mut pathfinder = Pathfinder::new(bigger_cavern);
    let risk = pathfinder.find_path();
    println!("Lowest risk: {}", risk);
}
