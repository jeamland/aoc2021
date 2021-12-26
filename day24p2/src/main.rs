use std::fs::File;
use std::io::{BufRead, BufReader};
use std::str::FromStr;

use anyhow::anyhow;

#[derive(Clone, Debug)]
enum Variable {
    W,
    X,
    Y,
    Z,
}

impl FromStr for Variable {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "w" => Ok(Self::W),
            "x" => Ok(Self::X),
            "y" => Ok(Self::Y),
            "z" => Ok(Self::Z),
            _ => Err(anyhow!("unknown variable: {}", s)),
        }
    }
}

impl std::fmt::Display for Variable {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::W => write!(f, "w"),
            Self::X => write!(f, "x"),
            Self::Y => write!(f, "y"),
            Self::Z => write!(f, "z"),
        }
    }
}

#[derive(Clone, Debug)]
enum VariableOrImmediate {
    Variable(Variable),
    Immediate(i64),
}

impl FromStr for VariableOrImmediate {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if let Ok(variable) = Variable::from_str(s) {
            return Ok(Self::Variable(variable));
        }

        Ok(Self::Immediate(i64::from_str(s).map_err(|_| {
            anyhow!("couldn't parse as variable or immediate: {}", s)
        })?))
    }
}

impl std::fmt::Display for VariableOrImmediate {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Variable(v) => write!(f, "{}", v),
            Self::Immediate(i) => write!(f, "{}", i),
        }
    }
}

#[derive(Clone, Debug)]
enum Insn {
    Inp(Variable),
    Add(Variable, VariableOrImmediate),
    Mul(Variable, VariableOrImmediate),
    Div(Variable, VariableOrImmediate),
    Mod(Variable, VariableOrImmediate),
    Eql(Variable, VariableOrImmediate),
}

impl FromStr for Insn {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let parts: Vec<&str> = s.split_whitespace().collect();
        match parts[0] {
            "inp" => Ok(Self::Inp(parts[1].parse()?)),
            "add" => Ok(Self::Add(parts[1].parse()?, parts[2].parse()?)),
            "mul" => Ok(Self::Mul(parts[1].parse()?, parts[2].parse()?)),
            "div" => Ok(Self::Div(parts[1].parse()?, parts[2].parse()?)),
            "mod" => Ok(Self::Mod(parts[1].parse()?, parts[2].parse()?)),
            "eql" => Ok(Self::Eql(parts[1].parse()?, parts[2].parse()?)),
            _ => Err(anyhow!("unknown instruction: {}", s)),
        }
    }
}

impl std::fmt::Display for Insn {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Inp(v) => write!(f, "inp {}", v),
            Self::Add(a1, a2) => write!(f, "add {} {}", a1, a2),
            Self::Mul(a1, a2) => write!(f, "mul {} {}", a1, a2),
            Self::Div(a1, a2) => write!(f, "div {} {}", a1, a2),
            Self::Mod(a1, a2) => write!(f, "mod {} {}", a1, a2),
            Self::Eql(a1, a2) => write!(f, "eql {} {}", a1, a2),
        }
    }
}

#[derive(Debug, Default)]
struct Alu {
    w: i64,
    x: i64,
    y: i64,
    z: i64,
}

impl Alu {
    fn variable(&self, variable: &Variable) -> i64 {
        match variable {
            Variable::W => self.w,
            Variable::X => self.x,
            Variable::Y => self.y,
            Variable::Z => self.z,
        }
    }

    fn set_variable(&mut self, variable: &Variable, value: i64) {
        match variable {
            Variable::W => self.w = value,
            Variable::X => self.x = value,
            Variable::Y => self.y = value,
            Variable::Z => self.z = value,
        }
    }

    fn value(&self, value: &VariableOrImmediate) -> i64 {
        match value {
            VariableOrImmediate::Variable(v) => self.variable(v),
            VariableOrImmediate::Immediate(i) => *i,
        }
    }

    fn run(&mut self, program: &[Insn], inputs: &[i64]) {
        let mut inputs = inputs.iter().copied();
        for insn in program {
            match insn {
                Insn::Inp(v) => {
                    if let Some(input) = inputs.next() {
                        self.set_variable(&v, input);
                    } else {
                        break;
                    }
                }
                Insn::Add(a1, a2) => {
                    self.set_variable(&a1, self.variable(&a1) + self.value(&a2));
                }
                Insn::Mul(a1, a2) => {
                    self.set_variable(&a1, self.variable(&a1) * self.value(&a2));
                }
                Insn::Div(a1, a2) => {
                    self.set_variable(&a1, self.variable(&a1) / self.value(&a2));
                }
                Insn::Mod(a1, a2) => {
                    self.set_variable(&a1, self.variable(&a1) % self.value(&a2));
                }
                Insn::Eql(a1, a2) => {
                    self.set_variable(
                        &a1,
                        if self.variable(&a1) == self.value(&a2) {
                            1
                        } else {
                            0
                        },
                    );
                }
            }
        }
    }
}

fn main() {
    let file = File::open(std::env::args_os().nth(1).unwrap()).unwrap();
    let reader = BufReader::new(file);

    let program: Vec<Insn> = reader
        .lines()
        .map(|l| l.unwrap().parse().unwrap())
        .collect();

    for inputs in InputGenerator::<14>::new() {
        let mut alu = Alu::default();

        alu.run(&program, &inputs);

        for digit in &inputs {
            print!("{}", digit);
        }
        println!(" -> w={} x={} y={} z={}", alu.w, alu.x, alu.y, alu.z);

        if alu.z == 0 {
            for digit in &inputs {
                print!("{}", digit);
            }
            println!(" -> w={} x={} y={} z={}", alu.w, alu.x, alu.y, alu.z);
            break;
            // } else if alu.x == 0 {
            //     for digit in &inputs {
            //         print!("{}", digit);
            //     }
            //     println!(" -> w={} x={} y={} z={}", alu.w, alu.x, alu.y, alu.z);
        }
    }
}

struct InputGenerator<const LENGTH: usize> {
    digits: [i64; LENGTH],
    prefix_4: Vec<[i64; 4]>,
    prefix_8: Vec<[i64; 4]>,
    prefix_10: Vec<[i64; 2]>,
}

impl<const LENGTH: usize> InputGenerator<LENGTH> {
    fn new() -> Self {
        let mut digits = [1; LENGTH];
        let mut prefix_4: Vec<[i64; 4]> = Vec::from(PREFIX_4);
        let mut prefix_8: Vec<[i64; 4]> = Vec::from(PREFIX_8);
        let mut prefix_10: Vec<[i64; 2]> = Vec::from(PREFIX_10);
        digits[..4].copy_from_slice(&prefix_4.remove(0));
        digits[4..8].copy_from_slice(&prefix_8.remove(0));
        digits[8..10].copy_from_slice(&prefix_10.remove(0));

        Self {
            digits,
            prefix_4,
            prefix_8,
            prefix_10,
        }
    }

    fn refresh_prefix_8(&mut self) {
        self.prefix_8 = Vec::from(PREFIX_8);
    }

    fn refresh_prefix_10(&mut self) {
        self.prefix_10 = Vec::from(PREFIX_10);
    }
}

impl<const LENGTH: usize> Iterator for InputGenerator<LENGTH> {
    type Item = [i64; LENGTH];

    fn next(&mut self) -> Option<Self::Item> {
        let digits = self.digits.clone();
        let mut next_prefix = true;

        for pos in (10..self.digits.len()).rev() {
            self.digits[pos] += 1;
            if self.digits[pos] == 10 {
                self.digits[pos] = 1;
            } else {
                next_prefix = false;
                break;
            }
        }

        if next_prefix {
            if self.prefix_10.is_empty() {
                if self.prefix_8.is_empty() {
                    if self.prefix_4.is_empty() {
                        return None;
                    }
                    self.digits[..4].copy_from_slice(&self.prefix_4.remove(0));
                    self.refresh_prefix_8();
                }
                self.digits[4..8].copy_from_slice(&self.prefix_8.remove(0));
                self.refresh_prefix_10();
            }
            self.digits[8..10].copy_from_slice(&self.prefix_10.remove(0));
        }

        Some(digits)
    }
}

const PREFIX_4: [[i64; 4]; 1] = [
    // [1, 1, 8, 1],
    // [1, 1, 9, 2],
    // [1, 2, 8, 1],
    // [1, 2, 9, 2],
    // [1, 3, 8, 1],
    // [1, 3, 9, 2],
    // [1, 4, 8, 1],
    // [1, 4, 9, 2],
    // [1, 5, 8, 1],
    // [1, 5, 9, 2],
    // [1, 6, 8, 1],
    // [1, 6, 9, 2],
    // [1, 7, 8, 1],
    // [1, 7, 9, 2],
    // [1, 8, 8, 1],
    // [1, 8, 9, 2],
    // [1, 9, 8, 1],
    // [1, 9, 9, 2],
    // [2, 1, 8, 1],
    // [2, 1, 9, 2],
    // [2, 2, 8, 1],
    // [2, 2, 9, 2],
    // [2, 3, 8, 1],
    // [2, 3, 9, 2],
    // [2, 4, 8, 1],
    // [2, 4, 9, 2],
    // [2, 5, 8, 1],
    // [2, 5, 9, 2],
    // [2, 6, 8, 1],
    // [2, 6, 9, 2],
    // [2, 7, 8, 1],
    // [2, 7, 9, 2],
    // [2, 8, 8, 1],
    // [2, 8, 9, 2],
    // [2, 9, 8, 1],
    // [2, 9, 9, 2],
    // [3, 1, 8, 1],
    // [3, 1, 9, 2],
    // [3, 2, 8, 1],
    // [3, 2, 9, 2],
    // [3, 3, 8, 1],
    // [3, 3, 9, 2],
    // [3, 4, 8, 1],
    // [3, 4, 9, 2],
    // [3, 5, 8, 1],
    // [3, 5, 9, 2],
    // [3, 6, 8, 1],
    // [3, 6, 9, 2],
    // [3, 7, 8, 1],
    // [3, 7, 9, 2],
    // [3, 8, 8, 1],
    // [3, 8, 9, 2],
    // [3, 9, 8, 1],
    // [3, 9, 9, 2],
    [4, 1, 8, 1],
    // [4, 1, 9, 2],
    // [4, 2, 8, 1],
    // [4, 2, 9, 2],
    // [4, 3, 8, 1],
    // [4, 3, 9, 2],
    // [4, 4, 8, 1],
    // [4, 4, 9, 2],
    // [4, 5, 8, 1],
    // [4, 5, 9, 2],
    // [4, 6, 8, 1],
    // [4, 6, 9, 2],
    // [4, 7, 8, 1],
    // [4, 7, 9, 2],
    // [4, 8, 8, 1],
    // [4, 8, 9, 2],
    // [4, 9, 8, 1],
    // [4, 9, 9, 2],
    // [5, 1, 8, 1],
    // [5, 1, 9, 2],
    // [5, 2, 8, 1],
    // [5, 2, 9, 2],
    // [5, 3, 8, 1],
    // [5, 3, 9, 2],
    // [5, 4, 8, 1],
    // [5, 4, 9, 2],
    // [5, 5, 8, 1],
    // [5, 5, 9, 2],
    // [5, 6, 8, 1],
    // [5, 6, 9, 2],
    // [5, 7, 8, 1],
    // [5, 7, 9, 2],
    // [5, 8, 8, 1],
    // [5, 8, 9, 2],
    // [5, 9, 8, 1],
    // [5, 9, 9, 2],
    // [6, 1, 8, 1],
    // [6, 1, 9, 2],
    // [6, 2, 8, 1],
    // [6, 2, 9, 2],
    // [6, 3, 8, 1],
    // [6, 3, 9, 2],
    // [6, 4, 8, 1],
    // [6, 4, 9, 2],
    // [6, 5, 8, 1],
    // [6, 5, 9, 2],
    // [6, 6, 8, 1],
    // [6, 6, 9, 2],
    // [6, 7, 8, 1],
    // [6, 7, 9, 2],
    // [6, 8, 8, 1],
    // [6, 8, 9, 2],
    // [6, 9, 8, 1],
    // [6, 9, 9, 2],
    // [7, 1, 8, 1],
    // [7, 1, 9, 2],
    // [7, 2, 8, 1],
    // [7, 2, 9, 2],
    // [7, 3, 8, 1],
    // [7, 3, 9, 2],
    // [7, 4, 8, 1],
    // [7, 4, 9, 2],
    // [7, 5, 8, 1],
    // [7, 5, 9, 2],
    // [7, 6, 8, 1],
    // [7, 6, 9, 2],
    // [7, 7, 8, 1],
    // [7, 7, 9, 2],
    // [7, 8, 8, 1],
    // [7, 8, 9, 2],
    // [7, 9, 8, 1],
    // [7, 9, 9, 2],
    // [8, 1, 8, 1],
    // [8, 1, 9, 2],
    // [8, 2, 8, 1],
    // [8, 2, 9, 2],
    // [8, 3, 8, 1],
    // [8, 3, 9, 2],
    // [8, 4, 8, 1],
    // [8, 4, 9, 2],
    // [8, 5, 8, 1],
    // [8, 5, 9, 2],
    // [8, 6, 8, 1],
    // [8, 6, 9, 2],
    // [8, 7, 8, 1],
    // [8, 7, 9, 2],
    // [8, 8, 8, 1],
    // [8, 8, 9, 2],
    // [8, 9, 8, 1],
    // [8, 9, 9, 2],
    // [9, 1, 8, 1],
    // [9, 1, 9, 2],
    // [9, 2, 8, 1],
    // [9, 2, 9, 2],
    // [9, 3, 8, 1],
    // [9, 3, 9, 2],
    // [9, 4, 8, 1],
    // [9, 4, 9, 2],
    // [9, 5, 8, 1],
    // [9, 5, 9, 2],
    // [9, 6, 8, 1],
    // [9, 6, 9, 2],
    // [9, 7, 8, 1],
    // [9, 7, 9, 2],
    // [9, 8, 8, 1],
    // [9, 8, 9, 2],
    // [9, 9, 8, 1],
    // [9, 9, 9, 2],
];

const PREFIX_8: [[i64; 4]; 324] = [
    [1, 1, 6, 1],
    [1, 1, 7, 2],
    [1, 1, 8, 3],
    [1, 1, 9, 4],
    [1, 2, 6, 1],
    [1, 2, 7, 2],
    [1, 2, 8, 3],
    [1, 2, 9, 4],
    [1, 3, 6, 1],
    [1, 3, 7, 2],
    [1, 3, 8, 3],
    [1, 3, 9, 4],
    [1, 4, 6, 1],
    [1, 4, 7, 2],
    [1, 4, 8, 3],
    [1, 4, 9, 4],
    [1, 5, 6, 1],
    [1, 5, 7, 2],
    [1, 5, 8, 3],
    [1, 5, 9, 4],
    [1, 6, 6, 1],
    [1, 6, 7, 2],
    [1, 6, 8, 3],
    [1, 6, 9, 4],
    [1, 7, 6, 1],
    [1, 7, 7, 2],
    [1, 7, 8, 3],
    [1, 7, 9, 4],
    [1, 8, 6, 1],
    [1, 8, 7, 2],
    [1, 8, 8, 3],
    [1, 8, 9, 4],
    [1, 9, 6, 1],
    [1, 9, 7, 2],
    [1, 9, 8, 3],
    [1, 9, 9, 4],
    [2, 1, 6, 1],
    [2, 1, 7, 2],
    [2, 1, 8, 3],
    [2, 1, 9, 4],
    [2, 2, 6, 1],
    [2, 2, 7, 2],
    [2, 2, 8, 3],
    [2, 2, 9, 4],
    [2, 3, 6, 1],
    [2, 3, 7, 2],
    [2, 3, 8, 3],
    [2, 3, 9, 4],
    [2, 4, 6, 1],
    [2, 4, 7, 2],
    [2, 4, 8, 3],
    [2, 4, 9, 4],
    [2, 5, 6, 1],
    [2, 5, 7, 2],
    [2, 5, 8, 3],
    [2, 5, 9, 4],
    [2, 6, 6, 1],
    [2, 6, 7, 2],
    [2, 6, 8, 3],
    [2, 6, 9, 4],
    [2, 7, 6, 1],
    [2, 7, 7, 2],
    [2, 7, 8, 3],
    [2, 7, 9, 4],
    [2, 8, 6, 1],
    [2, 8, 7, 2],
    [2, 8, 8, 3],
    [2, 8, 9, 4],
    [2, 9, 6, 1],
    [2, 9, 7, 2],
    [2, 9, 8, 3],
    [2, 9, 9, 4],
    [3, 1, 6, 1],
    [3, 1, 7, 2],
    [3, 1, 8, 3],
    [3, 1, 9, 4],
    [3, 2, 6, 1],
    [3, 2, 7, 2],
    [3, 2, 8, 3],
    [3, 2, 9, 4],
    [3, 3, 6, 1],
    [3, 3, 7, 2],
    [3, 3, 8, 3],
    [3, 3, 9, 4],
    [3, 4, 6, 1],
    [3, 4, 7, 2],
    [3, 4, 8, 3],
    [3, 4, 9, 4],
    [3, 5, 6, 1],
    [3, 5, 7, 2],
    [3, 5, 8, 3],
    [3, 5, 9, 4],
    [3, 6, 6, 1],
    [3, 6, 7, 2],
    [3, 6, 8, 3],
    [3, 6, 9, 4],
    [3, 7, 6, 1],
    [3, 7, 7, 2],
    [3, 7, 8, 3],
    [3, 7, 9, 4],
    [3, 8, 6, 1],
    [3, 8, 7, 2],
    [3, 8, 8, 3],
    [3, 8, 9, 4],
    [3, 9, 6, 1],
    [3, 9, 7, 2],
    [3, 9, 8, 3],
    [3, 9, 9, 4],
    [4, 1, 6, 1],
    [4, 1, 7, 2],
    [4, 1, 8, 3],
    [4, 1, 9, 4],
    [4, 2, 6, 1],
    [4, 2, 7, 2],
    [4, 2, 8, 3],
    [4, 2, 9, 4],
    [4, 3, 6, 1],
    [4, 3, 7, 2],
    [4, 3, 8, 3],
    [4, 3, 9, 4],
    [4, 4, 6, 1],
    [4, 4, 7, 2],
    [4, 4, 8, 3],
    [4, 4, 9, 4],
    [4, 5, 6, 1],
    [4, 5, 7, 2],
    [4, 5, 8, 3],
    [4, 5, 9, 4],
    [4, 6, 6, 1],
    [4, 6, 7, 2],
    [4, 6, 8, 3],
    [4, 6, 9, 4],
    [4, 7, 6, 1],
    [4, 7, 7, 2],
    [4, 7, 8, 3],
    [4, 7, 9, 4],
    [4, 8, 6, 1],
    [4, 8, 7, 2],
    [4, 8, 8, 3],
    [4, 8, 9, 4],
    [4, 9, 6, 1],
    [4, 9, 7, 2],
    [4, 9, 8, 3],
    [4, 9, 9, 4],
    [5, 1, 6, 1],
    [5, 1, 7, 2],
    [5, 1, 8, 3],
    [5, 1, 9, 4],
    [5, 2, 6, 1],
    [5, 2, 7, 2],
    [5, 2, 8, 3],
    [5, 2, 9, 4],
    [5, 3, 6, 1],
    [5, 3, 7, 2],
    [5, 3, 8, 3],
    [5, 3, 9, 4],
    [5, 4, 6, 1],
    [5, 4, 7, 2],
    [5, 4, 8, 3],
    [5, 4, 9, 4],
    [5, 5, 6, 1],
    [5, 5, 7, 2],
    [5, 5, 8, 3],
    [5, 5, 9, 4],
    [5, 6, 6, 1],
    [5, 6, 7, 2],
    [5, 6, 8, 3],
    [5, 6, 9, 4],
    [5, 7, 6, 1],
    [5, 7, 7, 2],
    [5, 7, 8, 3],
    [5, 7, 9, 4],
    [5, 8, 6, 1],
    [5, 8, 7, 2],
    [5, 8, 8, 3],
    [5, 8, 9, 4],
    [5, 9, 6, 1],
    [5, 9, 7, 2],
    [5, 9, 8, 3],
    [5, 9, 9, 4],
    [6, 1, 6, 1],
    [6, 1, 7, 2],
    [6, 1, 8, 3],
    [6, 1, 9, 4],
    [6, 2, 6, 1],
    [6, 2, 7, 2],
    [6, 2, 8, 3],
    [6, 2, 9, 4],
    [6, 3, 6, 1],
    [6, 3, 7, 2],
    [6, 3, 8, 3],
    [6, 3, 9, 4],
    [6, 4, 6, 1],
    [6, 4, 7, 2],
    [6, 4, 8, 3],
    [6, 4, 9, 4],
    [6, 5, 6, 1],
    [6, 5, 7, 2],
    [6, 5, 8, 3],
    [6, 5, 9, 4],
    [6, 6, 6, 1],
    [6, 6, 7, 2],
    [6, 6, 8, 3],
    [6, 6, 9, 4],
    [6, 7, 6, 1],
    [6, 7, 7, 2],
    [6, 7, 8, 3],
    [6, 7, 9, 4],
    [6, 8, 6, 1],
    [6, 8, 7, 2],
    [6, 8, 8, 3],
    [6, 8, 9, 4],
    [6, 9, 6, 1],
    [6, 9, 7, 2],
    [6, 9, 8, 3],
    [6, 9, 9, 4],
    [7, 1, 6, 1],
    [7, 1, 7, 2],
    [7, 1, 8, 3],
    [7, 1, 9, 4],
    [7, 2, 6, 1],
    [7, 2, 7, 2],
    [7, 2, 8, 3],
    [7, 2, 9, 4],
    [7, 3, 6, 1],
    [7, 3, 7, 2],
    [7, 3, 8, 3],
    [7, 3, 9, 4],
    [7, 4, 6, 1],
    [7, 4, 7, 2],
    [7, 4, 8, 3],
    [7, 4, 9, 4],
    [7, 5, 6, 1],
    [7, 5, 7, 2],
    [7, 5, 8, 3],
    [7, 5, 9, 4],
    [7, 6, 6, 1],
    [7, 6, 7, 2],
    [7, 6, 8, 3],
    [7, 6, 9, 4],
    [7, 7, 6, 1],
    [7, 7, 7, 2],
    [7, 7, 8, 3],
    [7, 7, 9, 4],
    [7, 8, 6, 1],
    [7, 8, 7, 2],
    [7, 8, 8, 3],
    [7, 8, 9, 4],
    [7, 9, 6, 1],
    [7, 9, 7, 2],
    [7, 9, 8, 3],
    [7, 9, 9, 4],
    [8, 1, 6, 1],
    [8, 1, 7, 2],
    [8, 1, 8, 3],
    [8, 1, 9, 4],
    [8, 2, 6, 1],
    [8, 2, 7, 2],
    [8, 2, 8, 3],
    [8, 2, 9, 4],
    [8, 3, 6, 1],
    [8, 3, 7, 2],
    [8, 3, 8, 3],
    [8, 3, 9, 4],
    [8, 4, 6, 1],
    [8, 4, 7, 2],
    [8, 4, 8, 3],
    [8, 4, 9, 4],
    [8, 5, 6, 1],
    [8, 5, 7, 2],
    [8, 5, 8, 3],
    [8, 5, 9, 4],
    [8, 6, 6, 1],
    [8, 6, 7, 2],
    [8, 6, 8, 3],
    [8, 6, 9, 4],
    [8, 7, 6, 1],
    [8, 7, 7, 2],
    [8, 7, 8, 3],
    [8, 7, 9, 4],
    [8, 8, 6, 1],
    [8, 8, 7, 2],
    [8, 8, 8, 3],
    [8, 8, 9, 4],
    [8, 9, 6, 1],
    [8, 9, 7, 2],
    [8, 9, 8, 3],
    [8, 9, 9, 4],
    [9, 1, 6, 1],
    [9, 1, 7, 2],
    [9, 1, 8, 3],
    [9, 1, 9, 4],
    [9, 2, 6, 1],
    [9, 2, 7, 2],
    [9, 2, 8, 3],
    [9, 2, 9, 4],
    [9, 3, 6, 1],
    [9, 3, 7, 2],
    [9, 3, 8, 3],
    [9, 3, 9, 4],
    [9, 4, 6, 1],
    [9, 4, 7, 2],
    [9, 4, 8, 3],
    [9, 4, 9, 4],
    [9, 5, 6, 1],
    [9, 5, 7, 2],
    [9, 5, 8, 3],
    [9, 5, 9, 4],
    [9, 6, 6, 1],
    [9, 6, 7, 2],
    [9, 6, 8, 3],
    [9, 6, 9, 4],
    [9, 7, 6, 1],
    [9, 7, 7, 2],
    [9, 7, 8, 3],
    [9, 7, 9, 4],
    [9, 8, 6, 1],
    [9, 8, 7, 2],
    [9, 8, 8, 3],
    [9, 8, 9, 4],
    [9, 9, 6, 1],
    [9, 9, 7, 2],
    [9, 9, 8, 3],
    [9, 9, 9, 4],
];

const PREFIX_10: [[i64; 2]; 2] = [[1, 8], [2, 9]];
