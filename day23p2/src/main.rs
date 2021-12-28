use std::cmp::Reverse;
use std::collections::{hash_map::Entry, BTreeMap, HashMap, HashSet};
use std::fmt::Write as FmtWrite;
use std::hash::{Hash, Hasher};
use std::io::Write;

use crossterm::{cursor, terminal, QueueableCommand};
use priority_queue::PriorityQueue;

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

    const fn is_home_for_amphipod(&self, amphipod: char) -> bool {
        match (self, amphipod) {
            (Self::A, 'A') => true,
            (Self::B, 'B') => true,
            (Self::C, 'C') => true,
            (Self::D, 'D') => true,
            _ => false,
        }
    }

    fn home_for_amphipod(amphipod: char) -> Self {
        match amphipod {
            'A' => Self::A,
            'B' => Self::B,
            'C' => Self::C,
            'D' => Self::D,
            _ => panic!("eeek: {}", amphipod),
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
            (Self::Room(r, 0), Self::Hallway(_)) => Self::Hallway(r.hallway_outside()),
            (Self::Room(r, p), Self::Hallway(_)) => Self::Room(*r, *p - 1),
            (Self::Room(r1, 0), Self::Room(r2, _)) if *r1 != *r2 => {
                Self::Hallway(r1.hallway_outside())
            }
            (Self::Room(r1, p), Self::Room(r2, _)) if *r1 != *r2 => Self::Room(*r1, *p - 1),
            (Self::Room(r1, p1), Self::Room(r2, p2)) if *r1 == *r2 && *p1 > *p2 => {
                Self::Room(*r1, p1 - 1)
            }
            (Self::Room(r1, p1), Self::Room(r2, p2)) if *r1 == *r2 && *p1 < *p2 => {
                Self::Room(*r1, p1 + 1)
            }
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
            _ => panic!("eeek: {:?} {:?}", self, other),
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

    const fn is_home_for_amphipod(&self, amphipod: char) -> bool {
        match (self, amphipod) {
            (Self::Room(r, _), a) if r.is_home_for_amphipod(a) => true,
            _ => false,
        }
    }

    fn blocks_in_room(&self, other: &Self) -> bool {
        match (self, other) {
            (&Self::Room(r1, p1), &Self::Room(r2, p2)) if r1 == r2 => p1 < p2,
            _ => false,
        }
    }

    fn same_room(&self, other: &Self) -> bool {
        match (self, other) {
            (&Self::Room(r1, _), &Self::Room(r2, _)) => r1 == r2,
            _ => false,
        }
    }
}

type BurrowId = u64;

#[derive(Default)]
struct Burrowverse {
    burrows: HashMap<BurrowId, BTreeMap<Position, char>>,
    moves: HashMap<BurrowId, Vec<(BurrowId, usize)>>,
    approx_cost: HashMap<BurrowId, usize>,
    goal_burrows: HashSet<BurrowId>,
    dead_burrows: HashSet<BurrowId>,
}

impl Burrowverse {
    fn parse_burrow(input: &str) -> BTreeMap<Position, char> {
        let mut positions = BTreeMap::new();

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
                positions.insert(Position::Room(room, p as u8), marker);
            }
        }

        positions
    }

    fn print(&self, burrow: BurrowId) {
        println!("{}", self.format(burrow));
    }

    fn format(&self, burrow: BurrowId) -> String {
        let mut result = String::new();
        let positions = self.burrows.get(&burrow).unwrap();

        writeln!(result, "          #############").unwrap();

        write!(result, "          #").unwrap();
        for h in 0..=10 {
            if let Some(amphipod) = positions.get(&Position::Hallway(h)) {
                write!(result, "{}", amphipod).unwrap();
            } else {
                write!(result, ".").unwrap();
            }
        }
        writeln!(result, "#").unwrap();

        for room_position in 0..=3 {
            if room_position == 0 {
                write!(result, "          ###").unwrap();
            } else {
                write!(result, "            #").unwrap();
            }

            for room in [
                Position::Room(Room::A, room_position),
                Position::Room(Room::B, room_position),
                Position::Room(Room::C, room_position),
                Position::Room(Room::D, room_position),
            ] {
                if let Some(amphipod) = positions.get(&room) {
                    write!(result, "{}", amphipod).unwrap();
                } else {
                    write!(result, ".").unwrap();
                }
                write!(result, "#").unwrap();
            }

            if room_position == 0 {
                writeln!(result, "##").unwrap();
            } else {
                writeln!(result, "  ").unwrap();
            }
        }

        writeln!(result, "            #########  ").unwrap();
        writeln!(result).unwrap();
        writeln!(result, "Burrow ID:   {:20}", burrow).unwrap();

        result
    }

    fn get_or_insert(&mut self, positions: BTreeMap<Position, char>) -> BurrowId {
        let burrow_id = Self::generate_burrow_id(&positions);

        match self.burrows.entry(burrow_id) {
            Entry::Occupied(_) => (),
            Entry::Vacant(entry) => {
                let approx_cost = Self::calculate_approx_cost(&positions);
                let is_goal = Self::check_if_goal(&positions);
                entry.insert(positions);
                self.approx_cost.insert(burrow_id, approx_cost);
                if is_goal {
                    self.goal_burrows.insert(burrow_id);
                }
            }
        }

        burrow_id
    }

    fn approx_cost(&self, burrow: BurrowId) -> usize {
        *self.approx_cost.get(&burrow).unwrap()
    }

    fn is_goal(&self, burrow: BurrowId) -> bool {
        self.goal_burrows.contains(&burrow)
    }

    fn generate_burrow_id(positions: &BTreeMap<Position, char>) -> BurrowId {
        let mut hasher = std::collections::hash_map::DefaultHasher::new();
        for (pos, c) in positions {
            pos.hash(&mut hasher);
            c.hash(&mut hasher);
        }
        hasher.finish()
    }

    fn calculate_approx_cost(positions: &BTreeMap<Position, char>) -> usize {
        let mut cost = 0;

        let mut moved_amphipods = HashMap::new();

        for (&p, &a) in positions {
            if p.is_home_for_amphipod(a) {
                continue;
            }

            cost += Self::energy(
                p.distance(&Position::Room(Room::home_for_amphipod(a), 0)),
                a,
            );
            *moved_amphipods.entry(a).or_insert(0usize) += 1;
        }

        cost += moved_amphipods
            .iter()
            .map(|(a, c)| Self::energy((c * (c - 1)) / 2, *a))
            .sum::<usize>();

        cost
    }

    fn check_if_goal(positions: &BTreeMap<Position, char>) -> bool {
        positions.iter().all(|(p, &a)| p.is_home_for_amphipod(a))
    }

    fn energy(distance: usize, amphipod: char) -> usize {
        distance
            * match amphipod {
                'A' => 1,
                'B' => 10,
                'C' => 100,
                'D' => 1000,
                _ => panic!("eeeek: {}", amphipod),
            }
    }

    fn moves(&mut self, burrow: BurrowId) -> Vec<(BurrowId, usize)> {
        if self.dead_burrows.contains(&burrow) {
            return vec![];
        }

        if let Some(moves) = self.moves.get(&burrow) {
            let mut moves = moves.clone();
            moves.retain(|(b, _)| !self.dead_burrows.contains(b));

            if moves.is_empty() {
                self.dead_burrows.insert(burrow);
            }

            return moves;
        }

        let moves = self.calculate_moves(burrow);
        if moves.is_empty() {
            self.dead_burrows.insert(burrow);
        } else {
            self.moves.insert(burrow, moves.clone());
        }
        moves
    }

    fn calculate_moves(&mut self, burrow: BurrowId) -> Vec<(BurrowId, usize)> {
        let positions = self.burrows.get(&burrow).unwrap().clone();
        let occupied_positions: HashSet<Position> = positions.keys().copied().collect();

        let mut moves = HashSet::new();

        for (&position, &amphipod) in &positions {
            if positions.iter().any(|(p, _)| p.blocks_in_room(&position)) {
                // This amphipod is blocked into a room and can't move.
                continue;
            } else if position.is_home_for_amphipod(amphipod)
                && !positions.iter().any(|(p, &a)| {
                    amphipod != a && position.same_room(p) && position.blocks_in_room(p)
                })
            {
                // This amphipod is in its room and not blocking any other amphipod who needs to move.
                continue;
            }

            if !positions.iter().any(|(&p, &a)| {
                a != amphipod
                    && matches!(p, Position::Room(r, _) if r == Room::home_for_amphipod(amphipod))
            }) {
                // Our room is available, let's head there.
                if let Some(target) = (0..=3)
                    .rev()
                    .map(|rp| Position::Room(Room::home_for_amphipod(amphipod), rp))
                    .find(|p| !occupied_positions.contains(p))
                {
                    let path = position.path(&target);
                    if path.is_disjoint(&occupied_positions) {
                        moves.insert((
                            amphipod,
                            position,
                            target,
                            Self::energy(position.distance(&target), amphipod),
                        ));
                    }
                } else {
                    panic!("thought the room was free but it wasn't?");
                }
            } else if let Position::Room(r, rp) = position {
                if occupied_positions
                    .iter()
                    .any(|&p| matches!(p, Position::Room(r1, rp1) if r == r1 && rp1 < rp))
                {
                    // We're in a room and someone else is in front of us.
                    continue;
                }

                for h in 0..=10 {
                    // We can try to move to somewhere in the hallway although we can't stop outside a room.
                    if h != 2 && h != 4 && h != 6 && h != 8 {
                        let path = position.path(&Position::Hallway(h));
                        if path.is_disjoint(&occupied_positions) {
                            moves.insert((
                                amphipod,
                                position,
                                Position::Hallway(h),
                                Self::energy(position.distance(&Position::Hallway(h)), amphipod),
                            ));
                        }
                    }
                }
            }
        }

        let mut moves: Vec<(BurrowId, usize)> = moves
            .into_iter()
            .map(|(a, from, to, e)| {
                let mut new_positions = positions.clone();
                new_positions.remove(&from);
                new_positions.insert(to, a);
                (self.get_or_insert(new_positions), e)
            })
            .collect();
        moves.sort_unstable_by_key(|(_, e)| *e);
        moves
    }
}

#[derive(Default)]
struct Path(HashMap<BurrowId, (BurrowId, usize)>);

impl Path {
    fn insert(&mut self, to: BurrowId, from: BurrowId, cost: usize) {
        self.0.insert(to, (from, cost));
    }

    fn cost_from(&self, from: BurrowId) -> usize {
        let mut from = from;
        let mut energy_used = 0;

        while let Some(prev) = self.0.get(&from) {
            energy_used += prev.1;
            from = prev.0;
        }

        energy_used
    }

    fn path_to(&self, to: BurrowId) -> Vec<BurrowId> {
        let mut from = to;
        let mut path = vec![from];

        while let Some(prev) = self.0.get(&from) {
            from = prev.0;
            path.push(from);
        }

        path.reverse();
        path
    }
}

fn main() {
    let input = std::fs::read_to_string(std::env::args_os().nth(1).unwrap()).unwrap();
    let start = Burrowverse::parse_burrow(&input);

    let mut burrowverse = Box::new(Burrowverse::default());
    let mut current = burrowverse.get_or_insert(start);
    burrowverse.print(current);

    let mut open: PriorityQueue<BurrowId, Reverse<usize>> = PriorityQueue::new();
    open.push(current, Reverse(burrowverse.approx_cost(current)));
    let mut g = HashMap::from([(current, 0)]);
    let mut came_from = Path::default();

    let mut stdout = std::io::stdout();
    stdout
        .queue(terminal::Clear(terminal::ClearType::All))
        .unwrap();
    stdout.queue(cursor::Hide).unwrap();
    stdout.flush().unwrap();

    let mut cycle_counter: usize = 0;

    while let Some((next, _)) = open.pop() {
        let start = std::time::Instant::now();

        current = next;

        if burrowverse.is_goal(current) {
            break;
        }

        open.remove(&current);
        let moves = burrowverse.moves(current);

        for (next, energy) in moves {
            let tentative_g = g.get(&current).unwrap() + energy;
            let current_g = g.entry(next).or_insert(usize::MAX);
            if tentative_g < *current_g {
                came_from.insert(next, current, energy);
                *current_g = tentative_g;
                open.push(next, Reverse(tentative_g + burrowverse.approx_cost(next)));
            }
        }

        cycle_counter += 1;
        let cycle_time = std::time::Instant::now() - start;

        stdout.queue(cursor::MoveTo(0, 0)).unwrap();
        write!(stdout, "{}", burrowverse.format(current)).unwrap();
        stdout.queue(cursor::MoveTo(0, 12)).unwrap();
        write!(
            stdout,
            "cycles:      {:20}\nopen states: {:20}\nenergy:      {:20}\ncycle time:  {:20}µs",
            cycle_counter,
            open.len(),
            g.get(&current).unwrap(),
            cycle_time.as_micros()
        )
        .unwrap();
    }

    stdout.queue(cursor::Show).unwrap();
    stdout.flush().unwrap();

    if !burrowverse.is_goal(current) {
        println!("\nWE FAIL AFTER {}", cycle_counter);
    }

    println!();
    println!(
        "Energy used: {} ({})",
        came_from.cost_from(current),
        g.get(&current).unwrap(),
    );
    println!();

    for burrow in came_from.path_to(current) {
        burrowverse.print(burrow);
    }
}
