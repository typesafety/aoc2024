pub fn solve_part1(input: &str) -> String {
    let reports = parse_input(input);
    let safe = reports.into_iter().filter(|v| is_safe(v)).count();
    format!("{safe}")
}

pub fn solve_part2(_input: &str) -> String {
    unimplemented!()
}

#[derive(Debug)]
enum Trend {
    Increasing,
    Decreasing,
}

fn is_safe(report: &[u32]) -> bool {
    fn check(scrutinee: &u32, prev: &u32, trend: &Trend) -> bool {
        let trend_ok = match trend {
            Trend::Increasing => scrutinee > prev,
            Trend::Decreasing => scrutinee < prev,
        };
        let delta = scrutinee.abs_diff(*prev);
        let gap_ok = (1..=3).contains(&delta);
        !(trend_ok && gap_ok)
    }

    let trend = if report[0] > report[1] {
        Trend::Decreasing
    } else {
        Trend::Increasing
    };
    let mut prev = &report[0];
    for n in &report[1..] {
        if check(n, prev, &trend) {
            return false;
        }
        prev = n;
    }
    true
}

fn parse_input(s: &str) -> Vec<Vec<u32>> {
    s.split("\n").map(parse_report).collect()
}

fn parse_report(s: &str) -> Vec<u32> {
    s.split_whitespace().map(|c| c.parse().unwrap()).collect()
}
