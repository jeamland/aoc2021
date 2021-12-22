use std::fs::File;
use std::io::{BufRead, BufReader};
use std::ops::RangeInclusive;

use itertools::Itertools;

#[derive(Clone, Debug)]
struct BunchOfCubes(
    RangeInclusive<isize>,
    RangeInclusive<isize>,
    RangeInclusive<isize>,
);

impl std::fmt::Display for BunchOfCubes {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "x={}..{},y={}..{},z={}..{} ({} cubes)",
            self.0.start(),
            self.0.end(),
            self.1.start(),
            self.1.end(),
            self.2.start(),
            self.2.end(),
            self.count()
        )
    }
}

impl BunchOfCubes {
    fn new(x: RangeInclusive<isize>, y: RangeInclusive<isize>, z: RangeInclusive<isize>) -> Self {
        Self(x, y, z)
    }

    fn encloses(&self, other: &Self) -> bool {
        (self.0.start() <= other.0.start() && self.0.end() >= other.0.end())
            && (self.1.start() <= other.1.start() && self.1.end() >= other.1.end())
            && (self.2.start() <= other.2.start() && self.2.end() >= other.2.end())
    }

    fn overlap_cubes(&self, other: &Self) -> Option<Self> {
        let ox_start = if self.0.start() >= other.0.start() && self.0.start() <= other.0.end() {
            *self.0.start()
        } else if other.0.start() >= self.0.start() && other.0.start() <= self.0.end() {
            *other.0.start()
        } else {
            return None;
        };

        let ox_end = if self.0.end() >= other.0.start() && self.0.end() <= other.0.end() {
            *self.0.end()
        } else if other.0.end() >= self.0.start() && other.0.end() <= self.0.end() {
            *other.0.end()
        } else {
            return None;
        };

        let oy_start = if self.1.start() >= other.1.start() && self.1.start() <= other.1.end() {
            *self.1.start()
        } else if other.1.start() >= self.1.start() && other.1.start() <= self.1.end() {
            *other.1.start()
        } else {
            return None;
        };

        let oy_end = if self.1.end() >= other.1.start() && self.1.end() <= other.1.end() {
            *self.1.end()
        } else if other.1.end() >= self.1.start() && other.1.end() <= self.1.end() {
            *other.1.end()
        } else {
            return None;
        };

        let oz_start = if self.2.start() >= other.2.start() && self.2.start() <= other.2.end() {
            *self.2.start()
        } else if other.2.start() >= self.2.start() && other.2.start() <= self.2.end() {
            *other.2.start()
        } else {
            return None;
        };

        let oz_end = if self.2.end() >= other.2.start() && self.2.end() <= other.2.end() {
            *self.2.end()
        } else if other.2.end() >= self.2.start() && other.2.end() <= self.2.end() {
            *other.2.end()
        } else {
            return None;
        };

        Some(Self(
            ox_start..=ox_end,
            oy_start..=oy_end,
            oz_start..=oz_end,
        ))
    }

    fn carve(&self, other: &Self) -> Vec<BunchOfCubes> {
        let removing = other.count();

        let mut victim = self.clone();
        let mut new_cubes = Vec::new();

        if victim.0.start() < other.0.start() {
            let new = BunchOfCubes::new(
                *victim.0.start()..=(*other.0.start() - 1),
                victim.1.clone(),
                victim.2.clone(),
            );
            println!("    x-start: {}", new);
            new_cubes.push(new);
            victim.0 = *other.0.start()..=*victim.0.end();
        }

        if victim.0.end() > other.0.end() {
            let new = BunchOfCubes::new(
                *other.0.end() + 1..=(*victim.0.end()),
                victim.1.clone(),
                victim.2.clone(),
            );
            println!("    x-end: {}", new);
            new_cubes.push(new);
            victim.0 = *victim.0.start()..=*other.0.end();
        }

        if victim.1.start() < other.1.start() {
            let new = BunchOfCubes::new(
                victim.0.clone(),
                *victim.1.start()..=(*other.1.start() - 1),
                victim.2.clone(),
            );
            println!("    y-start: {}", new);
            new_cubes.push(new);
            victim.1 = *other.1.start()..=*victim.1.end();
        }

        if victim.1.end() > other.1.end() {
            let new = BunchOfCubes::new(
                victim.0.clone(),
                *other.1.end() + 1..=(*victim.1.end()),
                victim.2.clone(),
            );
            println!("    y-end: {}", new);
            new_cubes.push(new);
            victim.1 = *victim.1.start()..=*other.1.end();
        }

        if victim.2.start() < other.2.start() {
            let new = BunchOfCubes::new(
                victim.0.clone(),
                victim.1.clone(),
                *victim.2.start()..=(*other.2.start() - 1),
            );
            println!("    z-start: {}", new);
            new_cubes.push(new);

            victim.2 = *other.2.start()..=*victim.2.end();
        }

        if victim.2.end() > other.2.end() {
            let new = BunchOfCubes::new(
                victim.0.clone(),
                victim.1.clone(),
                *other.2.end() + 1..=(*victim.2.end()),
            );
            println!("    z-end: {}", new);
            new_cubes.push(new);
            victim.2 = *victim.2.start()..=*other.2.end();
        }

        let remaining = new_cubes.iter().map(|c| c.count()).sum::<isize>();
        println!(
            "    remaining {} removed {} total {}",
            remaining,
            removing,
            remaining + removing
        );

        new_cubes
    }

    fn count(&self) -> isize {
        ((*self.0.end() + 1) - *self.0.start())
            * ((*self.1.end() + 1) - *self.1.start())
            * ((*self.2.end() + 1) - *self.2.start())
    }
}

fn main() {
    let file = File::open(std::env::args_os().nth(1).unwrap()).unwrap();
    let reader = BufReader::new(file);

    let mut cuboids: Vec<BunchOfCubes> = Vec::new();

    for (line_number, line) in reader.lines().enumerate() {
        let line = line.unwrap();
        println!("{}: {}", line_number + 1, line);
        let (state, ranges) = line.split_whitespace().collect_tuple().unwrap();
        let (x_range, y_range, z_range) = if let Some(r) = ranges
            .split_terminator(',')
            .map(|r| {
                let (start, end) = r
                    .split_at(2)
                    .1
                    .split_terminator("..")
                    .collect_tuple()
                    .unwrap();

                start.parse::<isize>().unwrap()..=end.parse::<isize>().unwrap()
            })
            .collect_tuple()
        {
            r
        } else {
            continue;
        };

        let on = state == "on";
        let cuboid = BunchOfCubes::new(x_range, y_range, z_range);
        let mut new_cuboids = Vec::new();
        let mut add = on;

        cuboids.reverse();

        for existing_cubes in cuboids {
            if on && existing_cubes.encloses(&cuboid) {
                println!("existing {} encloses {}", existing_cubes, cuboid);
                new_cuboids.push(existing_cubes);
                add = false;
                break;
            } else if on && cuboid.encloses(&existing_cubes) {
                println!("{} encloses existing {}", cuboid, existing_cubes);
            } else if !on && cuboid.encloses(&existing_cubes) {
                println!("{} encloses existing {} and is off", cuboid, existing_cubes);
            } else if let Some(overlap) = cuboid.overlap_cubes(&existing_cubes) {
                println!("carving {} out of existing {}", overlap, existing_cubes);
                new_cuboids.append(&mut existing_cubes.carve(&overlap));
            } else {
                println!("{} carried over", existing_cubes);
                new_cuboids.push(existing_cubes);
            }
        }

        new_cuboids.reverse();

        if add {
            println!("adding {}", cuboid);
            new_cuboids.push(cuboid.clone());
        }

        cuboids = new_cuboids;
        println!(
            "After line {}: {} cubes",
            line_number + 1,
            cuboids.iter().map(|cubes| cubes.count()).sum::<isize>(),
        );
    }

    let count = cuboids.iter().map(|cubes| cubes.count()).sum::<isize>();
    println!("{} cubes in {} cuboids", count, cuboids.len());
}
