use core::panic;

use regex;

pub fn solve_part1(input: &str) -> String {
    let pattern =
        regex::Regex::new(r"mul\((?<multiplier>\d{1,3}),(?<multiplicant>\d{1,3})\)").unwrap();
    let muls = pattern.captures_iter(input).map(Mul1::from_captures);
    let result: u32 = muls.map(|mul| mul.eval()).sum();

    format!("{result}")
}

pub fn solve_part2(input: &str) -> String {
    let pattern = regex::Regex::new(
        r"(?x)
        (?:
            (?<do>do\(\))
            |
            (?<dont>don't\(\))
            |
            (?<mul>mul\((?<multiplier>\d{1,3}),(?<multiplicant>\d{1,3})\))
        )",
    )
    .unwrap();
    let instructions = pattern.captures_iter(input).map(Instruction::from_captures);
    let result: u32 = interpret(instructions).iter().sum();

    format!("{result}")
}

// Part 2 logic

fn interpret(instructions: impl Iterator<Item = Instruction>) -> Vec<u32> {
    let mut ret = Vec::new();
    let mut mul_on = true;
    for instr in instructions {
        match instr {
            Instruction::Do => mul_on = true,
            Instruction::Dont => mul_on = false,
            Instruction::Mul {
                multiplier,
                multiplicant,
            } => {
                if mul_on {
                    ret.push(multiplier * multiplicant);
                }
            }
        }
    }
    ret
}

#[derive(Debug)]
enum Instruction {
    Do,
    Dont,
    Mul { multiplier: u32, multiplicant: u32 },
}

impl Instruction {
    fn from_captures(cap: regex::Captures) -> Instruction {
        if cap.name("do").is_some() {
            Instruction::Do
        } else if cap.name("dont").is_some() {
            Instruction::Dont
        } else if cap.name("mul").is_some() {
            Instruction::Mul {
                multiplier: cap["multiplier"].parse().unwrap(),
                multiplicant: cap["multiplicant"].parse().unwrap(),
            }
        } else {
            panic!("Could not parse an instruction from Captures: {:?}", cap)
        }
    }
}

// Part 1 logic

#[derive(Debug)]
struct Mul1 {
    multiplier: u32,
    multiplicant: u32,
}

impl Mul1 {
    fn from_captures(cap: regex::Captures) -> Mul1 {
        Mul1 {
            multiplier: cap["multiplier"].parse().unwrap(),
            multiplicant: cap["multiplicant"].parse().unwrap(),
        }
    }

    fn eval(&self) -> u32 {
        self.multiplier * self.multiplicant
    }
}
