use std::collections::HashMap;

pub fn solve_part1(input: &str) -> String {
    let (mut left, mut right) = parse_input(input);
    left.sort();
    right.sort();

    let total_distance = left
        .into_iter()
        .zip(right)
        .fold(0, |total, (l, r)| total + l.abs_diff(r));

    format!("{total_distance}")
}

pub fn solve_part2(input: &str) -> String {
    let (left, right) = parse_input(input);

    let mut occurrences = HashMap::<u32, u32>::new();
    for num in right {
        occurrences.entry(num).and_modify(|n| *n += 1).or_insert(1);
    }
    format!(
        "{}",
        left.into_iter()
            .fold(0, |acc, n| acc + n * occurrences.get(&n).unwrap_or(&0))
    )
}

fn parse_input(input: &str) -> (Vec<u32>, Vec<u32>) {
    input
        .split("\n")
        .map(|s| {
            let split: Vec<&str> = s.split_whitespace().collect();
            (
                split[0].parse::<u32>().unwrap(),
                split[1].parse::<u32>().unwrap(),
            )
        })
        .unzip()
}
