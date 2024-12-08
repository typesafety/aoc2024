use std::ops::Rem;

pub fn solve_part1(input: &str) -> String {
    let equations: Vec<Equation> = parse_input(input);
    let result = equations
        .into_iter()
        .filter(|eq| possible_eq(eq, check_possible1))
        .fold(0, |acc, eq| acc + eq.test);

    format!("{result}")
}

pub fn solve_part2(input: &str) -> String {
    let equations: Vec<Equation> = parse_input(input);
    let result = equations
        .into_iter()
        .filter(|eq| possible_eq(eq, check_possible2))
        .fold(0, |acc, eq| acc + eq.test);

    format!("{result}")
}

fn possible_eq(eq: &Equation, check_possible: fn(&[u64], test: u64) -> bool) -> bool {
    check_possible(&eq.numbers, eq.test)
}

fn check_possible1(nums: &[u64], test: u64) -> bool {
    match nums.split_last().unwrap() {
        (last, []) => *last == test,
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
                .any(|new_test| check_possible1(rest, *new_test))
        }
    }
}

fn check_possible2(nums: &[u64], test: u64) -> bool {
    fn last_digit(n: u64) -> u64 {
        n.to_string()
            .chars()
            .last()
            .unwrap()
            .to_string()
            .parse()
            .unwrap()
    }

    match nums.split_last().unwrap() {
        (last, []) => *last == test,
        (last, rest) => {
            if *last > test {
                return false;
            }

            let mut new_tests: Vec<u64> = Vec::new();

            // Concat
            if last_digit(test) == last_digit(*last) {
                let last_len = last.to_string().len().try_into().unwrap();
                let new_test = test / (10u64.pow(last_len));
                new_tests.push(new_test);
            }

            // Mul
            if test.rem(last) == 0 {
                new_tests.push(test / last);
            }

            // Add
            new_tests.push(test - last);

            new_tests
                .iter()
                .any(|test| check_possible2(rest, *test))
        }
    }
}

#[derive(Debug)]
struct Equation {
    test: u64,
    numbers: Vec<u64>,
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
