use regex;

pub fn solve_part1(input: &str) -> String {
    let pattern = regex::Regex::new(r"mul\((?<multiplier>\d+),(?<multiplicant>\d+)\)").unwrap();
    let muls = pattern.captures_iter(input).map(Mul::from_captures);
    let result: u32 = muls.map(|mul| mul.eval()).sum();

    format!("{result}")
}

pub fn solve_part2(_input: &str) -> String {
    unimplemented!()
}

#[derive(Debug)]
struct Mul {
    multiplier: u32,
    multiplicant: u32,
}

impl Mul {
    fn from_captures(cap: regex::Captures) -> Mul {
        let multiplier = cap["multiplier"].parse().unwrap();
        let multiplicant = cap["multiplicant"].parse().unwrap();
        Mul {
            multiplier,
            multiplicant,
        }
    }

    fn eval(&self) -> u32 {
        self.multiplier * self.multiplicant
    }
}
