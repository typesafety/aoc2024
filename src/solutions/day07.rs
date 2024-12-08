use std::ops::Rem;

pub fn solve_part1(input: &str) -> String {
    let equations: Vec<Equation> = parse_input(input);
    let result = equations
        .into_iter()
        .filter(|eq| possible_eq(eq))
        .fold(0, |acc, eq| acc + eq.test);

    format!("{result}")
}

pub fn solve_part2(input: &str) -> String {
    format!("{}", input)
}

fn possible_eq(eq: &Equation) -> bool {
    fn possible(nums: &Vec<u64>, test: u64) -> bool {
        match nums.split_last().unwrap() {
            (last, []) => test == *last,
            (last, rest) => {
                if *last > test {
                    return false;
                }

                let mut new_tests: Vec<u64> = Vec::new();
                if test.rem(last) == 0 {
                    new_tests.push(test / last);
                };
                new_tests.push(test - last);

                new_tests
                    .iter()
                    .any(|new_test| possible(&rest.into(), *new_test))
            }
        }
    }

    possible(&eq.numbers, eq.test)
}

#[derive(Debug)]
struct Equation {
    test: u64,
    numbers: Vec<u64>,
}

#[derive(Debug)]
enum Op {
    Add,
    Mul,
}

fn parse_input(input: &str) -> Vec<Equation> {
    input.split("\n").map(parse_line).collect()
}

fn parse_line(input: &str) -> Equation {
    let split: Vec<&str> = input.split(": ").collect();
    let test: u64 = split[0].parse().unwrap();
    let numbers: Vec<u64> = split[1]
        .split_whitespace()
        .map(|n| n.parse().unwrap())
        .collect();
    Equation { test, numbers }
}
