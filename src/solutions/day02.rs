pub fn solve_part1(input: &str) -> String {
    let reports = parse_input(input);
    let safe = reports.into_iter().filter(|v| is_safe(v, false)).count();
    format!("{safe}")
}

pub fn solve_part2(input: &str) -> String {
    let reports = parse_input(input);
    let safe = reports.into_iter().filter(|v| is_safe(v, true)).count();
    format!("{safe}")
}

#[derive(Debug)]
enum Trend {
    Increasing,
    Decreasing,
}

fn is_safe(report: &[u32], dampener_on: bool) -> bool {
    fn check(scrutinee: &u32, prev: &u32, trend: &Trend) -> bool {
        let trend_ok = match trend {
            Trend::Increasing => scrutinee > prev,
            Trend::Decreasing => scrutinee < prev,
        };
        let delta = scrutinee.abs_diff(*prev);
        let gap_ok = (1..=3).contains(&delta);
        trend_ok && gap_ok
    }

    fn check_all(nums: &[u32], allow_retry: bool) -> bool {
        fn calc_trend(x: &u32, y: &u32) -> Trend {
            if x > y {
                Trend::Decreasing
            } else {
                Trend::Increasing
            }
        }

        let trend = calc_trend(&nums[0], &nums[1]);
        let mut prev = &nums[0];

        for (ix, n) in nums.iter().enumerate() {
            if ix == 0 {
                continue;
            }
            match (check(n, prev, &trend), allow_retry) {
                (true, _) => prev = n,
                (false, true) => {
                    let mut variants: Vec<Vec<u32>> = [
                        [&nums[..ix - 1], &nums[ix..]].concat(),
                        [&nums[..ix], &nums[ix + 1..]].concat(),
                    ]
                    .to_vec();

                    if ix != 1 {
                        variants.extend([[&nums[..1], &nums[2..]].concat(), nums[1..].to_vec()]);
                    }

                    return variants
                        .into_iter()
                        .any(|variant| check_all(&variant, false));
                }
                (false, false) => {
                    return false;
                }
            }
        }
        true
    }

    check_all(report, dampener_on)
}

fn parse_input(s: &str) -> Vec<Vec<u32>> {
    s.split("\n").map(parse_report).collect()
}

fn parse_report(s: &str) -> Vec<u32> {
    s.split_whitespace().map(|c| c.parse().unwrap()).collect()
}
