use std::collections::{HashMap, HashSet};
use std::hash::{Hash, Hasher};

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
enum Room {
    A,
    B,
    C,
    D,
}

impl Room {
    const fn hallway_outside(&self) -> u8 {
        match self {
            Self::A => 2,
            Self::B => 4,
            Self::C => 6,
            Self::D => 8,
        }
    }

    fn all() -> Vec<Self> {
        vec![Self::A, Self::B, Self::C, Self::D]
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
enum Position {
    Hallway(u8),
    Room(Room, u8),
}

impl Position {
    fn path(&self, other: &Self) -> HashSet<Self> {
        let mut path = HashSet::new();

        let next = match (self, other) {
            (Self::Hallway(h), _) | (_, Self::Hallway(h)) if *h > 10 => panic!("eek"),
            (Self::Room(r, 1), _) => Self::Room(*r, 0),
            (Self::Room(r1, 0), Self::Room(r2, 1)) if *r1 == *r2 => Self::Room(*r1, 1),
            (Self::Room(r, 0), _) => Self::Hallway(r.hallway_outside()),
            (Self::Hallway(h), Self::Room(r, _)) if *h == r.hallway_outside() => Self::Room(*r, 0),
            (Self::Hallway(h), Self::Room(r, _)) => {
                if *h < r.hallway_outside() {
                    Self::Hallway(h + 1)
                } else {
                    Self::Hallway(h - 1)
                }
            }
            (Self::Hallway(h1), Self::Hallway(h2)) => {
                if h1 > h2 {
                    Self::Hallway(h1 - 1)
                } else {
                    Self::Hallway(h1 + 1)
                }
            }
            _ => panic!("eeek"),
        };

        path.insert(next);
        if next != *other {
            path.extend(next.path(other));
        }

        path
    }

    fn distance(&self, other: &Self) -> usize {
        match (self, other) {
            (Self::Room(r1, p1), Self::Room(r2, p2)) => {
                (*p1 as usize + 1)
                    + (r1.hallway_outside() as i8 - r2.hallway_outside() as i8).abs() as usize
                    + (*p2 as usize + 1)
            }
            (Self::Room(r, p), Self::Hallway(h)) | (Self::Hallway(h), Self::Room(r, p)) => {
                (*p as usize + 1) + (r.hallway_outside() as i8 - *h as i8).abs() as usize
            }
            _ => panic!("eeeek"),
        }
    }

    fn is_home_for_amphipod(&self, amphipod: &Amphipod) -> bool {
        matches!((self, amphipod),
            (Self::Room(r, _), a) if *r == a.desired_room())
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
enum Amphipod {
    A(u8),
    B(u8),
    C(u8),
    D(u8),
}

impl Amphipod {
    fn new(marker: char, number: u8) -> Self {
        match marker {
            'A' => Self::A(number),
            'B' => Self::B(number),
            'C' => Self::C(number),
            'D' => Self::D(number),
            _ => panic!("amphieek: {}", marker),
        }
    }
}

impl Amphipod {
    fn is_same(&self, other: &Self) -> bool {
        matches!(
            (self, other),
            (Self::A(_), Self::A(_))
                | (Self::B(_), Self::B(_))
                | (Self::C(_), Self::C(_))
                | (Self::D(_), Self::D(_))
        )
    }

    const fn desired_room(&self) -> Room {
        match self {
            Self::A(_) => Room::A,
            Self::B(_) => Room::B,
            Self::C(_) => Room::C,
            Self::D(_) => Room::D,
        }
    }

    const fn marker(&self) -> char {
        match self {
            Self::A(_) => 'A',
            Self::B(_) => 'B',
            Self::C(_) => 'C',
            Self::D(_) => 'D',
        }
    }

    const fn energy(&self, distance: usize) -> usize {
        distance
            * match self {
                Self::A(_) => 1,
                Self::B(_) => 10,
                Self::C(_) => 100,
                Self::D(_) => 1000,
            }
    }
}

#[derive(Clone, Debug)]
struct Burrow(HashMap<Amphipod, Position>);

impl From<&str> for Burrow {
    fn from(input: &str) -> Self {
        let mut counters = HashMap::new();
        let mut burrow = HashMap::new();

        for (p, line) in input
            .split_terminator('\n')
            .filter(|&l| l.chars().any(|c| char::is_ascii_uppercase(&c)))
            .enumerate()
        {
            for room in Room::all() {
                let marker = line
                    .chars()
                    .nth(room.hallway_outside() as usize + 1)
                    .unwrap();
                let number = counters.entry(marker).or_insert(1);
                burrow.insert(
                    Amphipod::new(marker, *number),
                    Position::Room(room, p as u8),
                );
                *number += 1;
            }
        }

        Self(burrow)
    }
}

impl Hash for Burrow {
    fn hash<H>(&self, state: &mut H)
    where
        H: Hasher,
    {
        let mut amphipods: Vec<&Amphipod> = self.0.keys().collect();
        amphipods.sort_unstable();

        for amphipod in amphipods {
            amphipod.marker().hash(state);
            self.0.get(amphipod).unwrap().hash(state);
        }
    }
}

impl PartialEq for Burrow {
    fn eq(&self, other: &Self) -> bool {
        let mut amphipods_self: Vec<(char, &Position)> =
            self.0.iter().map(|(a, p)| (a.marker(), p)).collect();
        amphipods_self.sort_unstable();

        let mut amphipods_other: Vec<(char, &Position)> =
            other.0.iter().map(|(a, p)| (a.marker(), p)).collect();
        amphipods_other.sort_unstable();

        amphipods_self == amphipods_other
    }
}

impl Eq for Burrow {}

impl Burrow {
    fn print(&self) {
        let positions: HashMap<Position, Amphipod> = self.0.iter().map(|(a, p)| (*p, *a)).collect();

        println!("#############");

        print!("#");
        for h in 0..=10 {
            if let Some(amphipod) = positions.get(&Position::Hallway(h)) {
                print!("{}", amphipod.marker());
            } else {
                print!(".");
            }
        }
        println!("#");

        for room_position in 0..=1 {
            print!("###");
            for room in [
                Position::Room(Room::A, room_position),
                Position::Room(Room::B, room_position),
                Position::Room(Room::C, room_position),
                Position::Room(Room::D, room_position),
            ] {
                if let Some(amphipod) = positions.get(&room) {
                    print!("{}", amphipod.marker());
                } else {
                    print!(".");
                }
                print!("#");
            }
            println!("##");
        }

        println!("  #########  ");
        println!();
    }

    fn positions(&self) -> HashSet<Position> {
        self.0.values().copied().collect()
    }

    fn amphipod_is_home(&self, amphipod: &Amphipod) -> bool {
        if !self.0.get(amphipod).unwrap().is_home_for_amphipod(amphipod) {
            return false;
        }

        let room = amphipod.desired_room();

        !self.0.iter().any(|(a, p)| {
            matches!(p, Position::Room(r, _) if *r == room) && !p.is_home_for_amphipod(a)
        })
    }

    fn all_amphipods_are_home(&self) -> bool {
        self.0.iter().all(|(a, p)| p.is_home_for_amphipod(a))
    }

    fn moves(&self) -> Vec<(Amphipod, Position)> {
        let mut moves = HashSet::new();
        let positions = self.positions();

        for (amphipod, position) in &self.0 {
            if self.amphipod_is_home(amphipod) {
                continue;
            }

            match position {
                Position::Hallway(_) => {
                    if self.0.iter().any(|(a, p)| {
                        !amphipod.is_same(a)
                            && matches!(p, Position::Room(r, _) if *r == amphipod.desired_room())
                    }) {
                        // There's a different kind of amphipod in our room, can't go there.
                        continue;
                    } else if !positions.contains(&Position::Room(amphipod.desired_room(), 0)) {
                        // Look for a spot in a room and see if we can get there.
                        let mut target = Position::Room(amphipod.desired_room(), 0);
                        if !positions.contains(&Position::Room(amphipod.desired_room(), 1)) {
                            target = Position::Room(amphipod.desired_room(), 1);
                        }
                        let path = position.path(&target);
                        if path.is_disjoint(&positions) {
                            moves.insert((*amphipod, target));
                        }
                    }
                }
                Position::Room(r, p) => {
                    if *p == 1 && positions.contains(&Position::Room(*r, 0)) {
                        // We're at the back of a room and someone else is in front of us.
                        continue;
                    }

                    for h in 0..=10 {
                        // We can try to move to somewhere in the hallway.

                        if h == 2 || h == 4 || h == 6 || h == 8 {
                            // Can't stop outside a room.
                            continue;
                        }

                        let path = position.path(&Position::Hallway(h));
                        if path.is_disjoint(&positions) {
                            moves.insert((*amphipod, Position::Hallway(h)));
                        }
                    }

                    if !positions.contains(&Position::Room(amphipod.desired_room(), 0)) {
                        // Our room is available, let's head there.
                        let mut target = Position::Room(amphipod.desired_room(), 0);

                        if !positions.contains(&Position::Room(amphipod.desired_room(), 1)) {
                            target = Position::Room(amphipod.desired_room(), 1);
                        }

                        let path = position.path(&target);
                        if path.is_disjoint(&positions) {
                            moves.insert((*amphipod, target));
                        }
                    }
                }
            }
        }

        let mut moves: Vec<(Amphipod, Position)> = moves.into_iter().collect();
        moves.sort_unstable_by_key(|(a, p)| a.energy(p.distance(self.0.get(a).unwrap())));
        moves
    }
}

#[derive(Default)]
struct Burrower(HashMap<Burrow, Option<usize>>);

impl Burrower {
    fn score(&mut self, burrow: Burrow) -> usize {
        self.score_inner(burrow, 0).unwrap()
    }

    fn score_inner(&mut self, burrow: Burrow, depth: usize) -> Option<usize> {
        if let Some(score) = self.0.get(&burrow) {
            return *score;
        }

        if burrow.all_amphipods_are_home() {
            self.0.insert(burrow, Some(0));
            return Some(0);
        }

        let moves = burrow.moves();

        if moves.is_empty() {
            self.0.insert(burrow, None);
            return None;
        }

        let mut min_score = 0;

        for (amphipod, position) in moves {
            if depth == 0 {
                println!("{:?} -> {:?}", amphipod, position);
            }
            let energy = amphipod.energy(burrow.0.get(&amphipod).unwrap().distance(&position));

            if min_score != 0 && energy > min_score {
                break;
            }

            let mut next = burrow.clone();
            next.0.insert(amphipod, position);

            if let Some(score) = self.score_inner(next, depth + 1) {
                let score = score + energy;
                if min_score == 0 || score < min_score {
                    min_score = score
                }
            }
        }

        if min_score != 0 {
            self.0.insert(burrow, Some(min_score));
            Some(min_score)
        } else {
            self.0.insert(burrow, None);
            None
        }
    }
}

fn main() {
    let input = std::fs::read_to_string(std::env::args_os().nth(1).unwrap()).unwrap();
    let burrow = Burrow::from(input.as_str());
    burrow.print();
    println!("Minimum energy: {:?}", Burrower::default().score(burrow));
}
