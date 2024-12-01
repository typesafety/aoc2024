pub fn solve_part1(input: &str) -> String {
    let (mut left, mut right) = parse_lists(input);
    left.sort();
    right.sort();

    let total_distance = left
        .into_iter()
        .zip(right)
        .fold(0, |total, (l, r)| total + l.abs_diff(r));

    format!("{total_distance}")
}

pub fn solve_part2(_input: &str) -> String {
    unimplemented!()
}

fn parse_lists(input: &str) -> (Vec<u32>, Vec<u32>) {
    parse_pairs(input).into_iter().unzip()
}

fn parse_pairs(input: &str) -> Vec<(u32, u32)> {
    input
        .split("\n")
        .map(|s| {
            let split: Vec<&str> = s.split_whitespace().collect();
            (split[0].parse().unwrap(), split[1].parse().unwrap())
        })
        .collect()
}
