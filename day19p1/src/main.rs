use std::collections::{HashMap, HashSet};
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::ops::{Add, AddAssign, Neg, Sub};

use itertools::Itertools;
use ndarray::Axis;

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Hash, PartialOrd, Ord)]
struct Pointlike(i64, i64, i64);

impl Pointlike {
    fn rotate(&self, rotation: Rotation) -> Self {
        let mut rotated = self.clone();

        rotated = match rotation.0 {
            AxisRotation::None => rotated,
            AxisRotation::First => Self(rotated.0, rotated.2, -rotated.1),
            AxisRotation::Second => Self(rotated.0, -rotated.1, -rotated.2),
            AxisRotation::Third => Self(rotated.0, -rotated.2, rotated.1),
        };

        rotated = match rotation.1 {
            AxisRotation::None => rotated,
            AxisRotation::First => Self(-rotated.2, rotated.1, rotated.0),
            AxisRotation::Second => Self(-rotated.0, rotated.1, -rotated.2),
            AxisRotation::Third => Self(rotated.2, rotated.1, -rotated.0),
        };

        match rotation.2 {
            AxisRotation::None => rotated,
            AxisRotation::First => Self(-rotated.1, rotated.0, rotated.2),
            AxisRotation::Second => Self(-rotated.0, -rotated.1, rotated.2),
            AxisRotation::Third => Self(rotated.1, -rotated.0, rotated.2),
        }
    }
}

impl Neg for Pointlike {
    type Output = Self;

    fn neg(self) -> Self::Output {
        Self(-self.0, -self.1, -self.2)
    }
}

impl Add for Pointlike {
    type Output = Self;

    fn add(self, rhs: Pointlike) -> Self::Output {
        Self(self.0 + rhs.0, self.1 + rhs.1, self.2 + rhs.2)
    }
}

impl Sub for Pointlike {
    type Output = Self;

    fn sub(self, rhs: Pointlike) -> Self::Output {
        Self(self.0 - rhs.0, self.1 - rhs.1, self.2 - rhs.2)
    }
}

impl std::fmt::Display for Pointlike {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "({:5}, {:5}, {:5})", self.0, self.1, self.2)
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum AxisRotation {
    None,
    First,
    Second,
    Third,
}

impl AxisRotation {
    fn increment(&self) -> AxisRotation {
        match self {
            AxisRotation::None => AxisRotation::First,
            AxisRotation::First => AxisRotation::Second,
            AxisRotation::Second => AxisRotation::Third,
            AxisRotation::Third => AxisRotation::None,
        }
    }
}

impl std::fmt::Display for AxisRotation {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            AxisRotation::None => write!(f, "0"),
            AxisRotation::First => write!(f, "1"),
            AxisRotation::Second => write!(f, "2"),
            AxisRotation::Third => write!(f, "3"),
        }
    }
}

impl Neg for AxisRotation {
    type Output = Self;

    fn neg(self) -> Self::Output {
        match self {
            AxisRotation::None => AxisRotation::None,
            AxisRotation::First => AxisRotation::Third,
            AxisRotation::Second => AxisRotation::Second,
            AxisRotation::Third => AxisRotation::First,
        }
    }
}

impl Add<AxisRotation> for AxisRotation {
    type Output = Self;

    fn add(self, rhs: Self) -> Self {
        let rhs = match rhs {
            AxisRotation::None => 0,
            AxisRotation::First => 1,
            AxisRotation::Second => 2,
            AxisRotation::Third => 3,
        };

        self + rhs
    }
}

impl Sub<AxisRotation> for AxisRotation {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self {
        let rhs = match rhs {
            AxisRotation::None => 0,
            AxisRotation::First => 3,
            AxisRotation::Second => 2,
            AxisRotation::Third => 1,
        };

        self + rhs
    }
}

impl Add<usize> for AxisRotation {
    type Output = Self;

    fn add(self, rhs: usize) -> Self {
        let mut new = self.clone();
        new += rhs;
        new
    }
}

impl AddAssign<usize> for AxisRotation {
    fn add_assign(&mut self, rhs: usize) {
        for _ in 0..rhs % 4 {
            *self = match self {
                Self::None => Self::First,
                Self::First => Self::Second,
                Self::Second => Self::Third,
                Self::Third => Self::None,
            }
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
struct Rotation(AxisRotation, AxisRotation, AxisRotation);

impl Default for Rotation {
    fn default() -> Self {
        Self(AxisRotation::None, AxisRotation::None, AxisRotation::None)
    }
}

impl Rotation {
    fn is_last(&self) -> bool {
        self.0 == AxisRotation::Third
            && self.1 == AxisRotation::Third
            && self.2 == AxisRotation::Third
    }
}

impl Neg for Rotation {
    type Output = Self;

    fn neg(self) -> Self::Output {
        Self(-self.0, -self.1, -self.2)
    }
}

impl Add<Rotation> for Rotation {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        Self(self.0 + rhs.0, self.1 + rhs.1, self.2 + rhs.2)
    }
}

impl Sub<Rotation> for Rotation {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self::Output {
        Self(self.0 - rhs.0, self.1 - rhs.1, self.2 - rhs.2)
    }
}

impl AddAssign<usize> for Rotation {
    fn add_assign(&mut self, rhs: usize) {
        assert!(rhs == 1);

        match (self.0, self.1, self.2) {
            (_, _, z) if z != AxisRotation::Third => self.2 += 1,
            (_, y, _) if y != AxisRotation::Third => {
                self.2 = AxisRotation::None;
                self.1 += 1
            }
            (_, _, _) => {
                self.2 = AxisRotation::None;
                self.1 = AxisRotation::None;
                self.0 += 1
            }
        }
    }
}

impl std::fmt::Display for Rotation {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "[{}, {}, {}]", self.0, self.1, self.2)
    }
}

struct Rotator {
    beacons: Vec<(usize, usize, Pointlike)>,
    rotation: Rotation,
}

impl Rotator {
    fn new(beacons: &[(usize, usize, Pointlike)]) -> Self {
        Self {
            beacons: beacons.to_vec(),
            rotation: Rotation::default(),
        }
    }
}

impl Iterator for Rotator {
    type Item = (Rotation, Vec<(usize, usize, Pointlike)>);

    fn next(&mut self) -> Option<Self::Item> {
        if self.rotation.is_last() {
            return None;
        }

        let rotated = self
            .beacons
            .iter()
            .map(|(a, b, point)| (*a, *b, point.rotate(self.rotation)))
            .collect();
        let rotation = self.rotation.clone();

        self.rotation += 1;

        Some((rotation, rotated))
    }
}

#[derive(Debug, Default)]
struct Region {
    scanners: HashMap<usize, (Pointlike, Rotation, Vec<Pointlike>)>,
    beacons: HashSet<Pointlike>,
}

impl Region {
    fn offsets(locations: &[Pointlike]) -> Vec<(usize, usize, Pointlike)> {
        locations
            .iter()
            .enumerate()
            .combinations(2)
            .map(|beacons| {
                let (id_a, location_a) = beacons[0];
                let (id_b, location_b) = beacons[1];

                (id_a, id_b, *location_a - *location_b)
            })
            .collect()
    }

    fn compare(first: &[Pointlike], second: &[Pointlike]) -> Option<(Rotation, Pointlike)> {
        let offsets_first = Self::offsets(first);
        let offsets_second = Self::offsets(second);

        for (rotation, rotated) in Rotator::new(&offsets_second) {
            let mut matches = HashSet::new();
            for (a, b, offset_first) in &offsets_first {
                if let Some((oa, ob, _)) = rotated
                    .iter()
                    .find(|(_, _, offset_second)| *offset_first == *offset_second)
                {
                    matches.insert((*a, *oa));
                    matches.insert((*b, *ob));
                } else if let Some((oa, ob, _)) = rotated
                    .iter()
                    .find(|(_, _, offset_second)| -*offset_first == *offset_second)
                {
                    matches.insert((*a, *ob));
                    matches.insert((*b, *oa));
                }
            }

            if matches.len() == usize::min(first.len(), 12) {
                let (id, oid) = matches.iter().next().unwrap();
                let translation = first[*id] - second[*oid].rotate(rotation);

                assert!(matches.iter().all(|(id, oid)| {
                    let t = first[*id] - second[*oid].rotate(rotation);
                    t == translation
                }));

                return Some((rotation, translation));
            }
        }

        None
    }

    fn register(&mut self, id: usize, beacon_locations: &[Pointlike]) -> bool {
        let mut rotation = Rotation::default();
        let mut translation = Pointlike::default();

        if self.scanners.len() != 0 {
            let mut found = false;

            for (eid, (location, er, existing)) in &self.scanners {
                if let Some((r, t)) = Self::compare(existing, beacon_locations) {
                    found = true;
                    rotation = r - *er;
                    translation = *location + t.rotate(-*er);
                    println!(
                        "Scanner {} matched scanner {} @ translation {}, rotation {}",
                        id, eid, t, r
                    );
                    break;
                }
            }

            if !found {
                return false;
            }
        }

        for location in beacon_locations {
            println!(
                "scanner {} beacon @ offset {} -> rotation {} = {} -> translation {} = {} or {}",
                id,
                location,
                rotation,
                location.rotate(rotation),
                translation,
                location.rotate(rotation) + translation,
                *location + translation.rotate(rotation),
            );
            self.beacons.insert(location.rotate(rotation) + translation);
        }

        self.scanners
            .insert(id, (translation, rotation, beacon_locations.to_vec()));

        true
    }

    fn describe(&self) {
        let mut scanners: Vec<(usize, Pointlike)> = self
            .scanners
            .iter()
            .map(|(id, (l, _, _))| (*id, *l))
            .collect();
        scanners.sort_unstable_by_key(|(id, _)| *id);

        for (id, location) in scanners {
            println!(
                "Scanner {} @ ({:5}, {:5}, {:5})",
                id, location.0, location.1, location.2
            );
        }

        println!();

        let mut beacons: Vec<&Pointlike> = self.beacons.iter().collect();
        beacons.sort_unstable();

        for beacon in beacons {
            println!("Beacon @ ({:5}, {:5}, {:5})", beacon.0, beacon.1, beacon.2);
        }

        println!();

        println!(
            "Scanners: {}, Beacons: {}",
            self.scanners.len(),
            self.beacons.len()
        );
    }
}

impl FromIterator<(usize, Vec<Pointlike>)> for Region {
    fn from_iter<T>(iter: T) -> Self
    where
        T: IntoIterator<Item = (usize, Vec<Pointlike>)>,
    {
        let mut region = Region::default();
        let mut unregistered = Vec::new();

        for (id, beacon_locations) in iter {
            if !region.register(id, &beacon_locations) {
                unregistered.push((id, beacon_locations));
            }
        }

        while !unregistered.is_empty() {
            let (id, beacon_locations) = unregistered.remove(0);
            if !region.register(id, &beacon_locations) {
                unregistered.push((id, beacon_locations));
            }
        }

        region
    }
}

fn main() {
    let file = File::open(std::env::args_os().nth(1).unwrap()).unwrap();
    let reader = BufReader::new(file);

    let mut scanners = Vec::new();
    let mut beacons = Vec::new();
    let mut scanner_id = 0;

    for line in reader.lines() {
        let line = line.unwrap();
        if line.starts_with("---") {
            continue;
        } else if line.is_empty() {
            scanners.push((scanner_id, beacons));
            scanner_id += 1;
            beacons = Vec::new();
            continue;
        }

        let point: Vec<i64> = line
            .split_terminator(',')
            .map(|v| v.parse::<i64>().unwrap())
            .collect();

        let x = point[0];
        let y = point[1];
        let z = if point.len() == 3 { point[2] } else { 0 };

        beacons.push(Pointlike(x, y, z));
    }

    if !beacons.is_empty() {
        scanners.push((scanner_id, beacons));
    }

    let region: Region = scanners.into_iter().collect();
    region.describe();

    let point = Pointlike(0, 3, 3);
    let points = vec![(0, 0, point.clone())];
    for (rotation, rotated) in Rotator::new(&points) {
        if rotated[0].2 .0 == 3 && rotated[0].2 .1 == 0 && rotated[0].2 .2 == -3 {
            println!("{} -> {} => {}", point, rotation, rotated[0].2);
        }
    }
}
