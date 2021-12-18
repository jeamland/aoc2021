use std::fs::File;
use std::io::{BufRead, BufReader};
use std::ops::Add;

use num_integer::Integer;
use serde_json::Value;

#[derive(Clone, Debug)]
enum SnailDigit {
    PairOpen,
    PairSeparator,
    PairClose,
    Number(i64),
}

#[derive(Clone, Debug)]
struct SnailNumber(Vec<SnailDigit>);

impl From<Value> for SnailNumber {
    fn from(value: Value) -> Self {
        SnailNumber::from(&value)
    }
}

impl From<&Value> for SnailNumber {
    fn from(value: &Value) -> Self {
        let mut number = Vec::new();

        match value {
            Value::Number(n) => number.push(SnailDigit::Number(n.as_i64().unwrap())),
            Value::Array(array) => {
                assert!(array.len() == 2, "pair isn't a pair");
                let mut a = SnailNumber::from(&array[0]);
                let mut b = SnailNumber::from(&array[1]);

                number.push(SnailDigit::PairOpen);
                number.append(&mut a.0);
                number.push(SnailDigit::PairSeparator);
                number.append(&mut b.0);
                number.push(SnailDigit::PairClose);
            }
            _ => panic!("unexpected value"),
        }

        SnailNumber(number)
    }
}

impl std::fmt::Display for SnailNumber {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for digit in &self.0 {
            match digit {
                SnailDigit::PairOpen => write!(f, "[")?,
                SnailDigit::PairSeparator => write!(f, ",")?,
                SnailDigit::PairClose => write!(f, "]")?,
                SnailDigit::Number(n) => write!(f, "{}", n)?,
            }
        }

        Ok(())
    }
}

impl Add for SnailNumber {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        let mut sum = vec![SnailDigit::PairOpen];
        sum.extend(self.0.iter().cloned());
        sum.push(SnailDigit::PairSeparator);
        sum.extend(rhs.0.iter().cloned());
        sum.push(SnailDigit::PairClose);
        let mut sum = SnailNumber(sum);

        let mut reduced = false;
        while !reduced {
            reduced = true;

            if sum.explode() {
                reduced = false;
                continue;
            }

            if sum.split() {
                reduced = false;
                continue;
            }
        }

        sum
    }
}

impl SnailNumber {
    fn explode(&mut self) -> bool {
        let mut depth = 0;
        let mut explode_pos = None;

        for (index, digit) in self.0.iter().enumerate() {
            match digit {
                SnailDigit::PairOpen => depth += 1,
                SnailDigit::PairClose => depth -= 1,
                _ => continue,
            }

            if depth > 4 {
                explode_pos = Some(index);
                break;
            }
        }

        if let Some(pos) = explode_pos {
            let left = if let SnailDigit::Number(n) = &self.0[pos + 1] {
                *n
            } else {
                panic!("explode pair not both regular");
            };
            assert!(matches!(self.0[pos + 2], SnailDigit::PairSeparator));
            let right = if let SnailDigit::Number(n) = &self.0[pos + 3] {
                *n
            } else {
                panic!("explode pair not both regular");
            };

            for index in (0..pos).rev() {
                if let SnailDigit::Number(n) = &mut self.0[index] {
                    *n += left;
                    break;
                }
            }

            for index in pos + 4..self.0.len() {
                if let SnailDigit::Number(n) = &mut self.0[index] {
                    *n += right;
                    break;
                }
            }

            self.0.remove(pos + 4);
            self.0.remove(pos + 3);
            self.0.remove(pos + 2);
            self.0.remove(pos + 1);
            self.0.remove(pos);
            self.0.insert(pos, SnailDigit::Number(0));

            true
        } else {
            false
        }
    }

    fn split(&mut self) -> bool {
        let mut split_pos = None;

        for (index, digit) in self.0.iter().enumerate() {
            match digit {
                SnailDigit::Number(n) if *n >= 10 => {
                    split_pos = Some(index);
                    break;
                }
                _ => continue,
            };
        }

        if let Some(pos) = split_pos {
            let number = if let SnailDigit::Number(n) = &self.0[pos] {
                *n
            } else {
                panic!("eek");
            };

            let a = number.div_floor(&2);
            let b = number.div_ceil(&2);

            self.0.remove(pos);
            self.0.insert(pos, SnailDigit::PairOpen);
            self.0.insert(pos + 1, SnailDigit::Number(a));
            self.0.insert(pos + 2, SnailDigit::PairSeparator);
            self.0.insert(pos + 3, SnailDigit::Number(b));
            self.0.insert(pos + 4, SnailDigit::PairClose);

            true
        } else {
            false
        }
    }

    fn magnitude(&self) -> i64 {
        let mut stack = Vec::new();
        let mut number = 0;
        let mut left = 0;

        for digit in &self.0 {
            match digit {
                SnailDigit::Number(n) => number = *n,
                SnailDigit::PairOpen => stack.push(left),
                SnailDigit::PairSeparator => left = number * 3,
                SnailDigit::PairClose => {
                    number = left + number * 2;
                    left = stack.pop().unwrap();
                }
            }
        }

        number
    }
}

fn main() {
    let file = File::open(std::env::args_os().nth(1).unwrap()).unwrap();
    let reader = BufReader::new(file);

    let numbers: Vec<SnailNumber> = reader
        .lines()
        .map(|l| {
            let l = l.unwrap();
            serde_json::from_str::<Value>(&l).unwrap().into()
        })
        .collect();

    for number in &numbers {
        println!("{}", number);
    }

    println!();

    let total = numbers.into_iter().reduce(|a, e| a + e).unwrap();
    println!("Total: {}", total);
    println!("Magnitude: {}", total.magnitude());
}
