pub fn solve_part1(input: &str) -> String {
    let (orders, updates) = parse_input(input);

    format!("{:#?}\n\n{:#?}", orders, updates)
}

pub fn solve_part2(input: &str) -> String {
    format!("")
}

#[derive(Debug)]
struct Order {
    before: u32,
    after: u32,
}

type Update = Vec<u32>;

fn parse_input(input: &str) -> (Vec<Order>, Vec<Update>) {
    let parts: Vec<&str> = input.split("\n\n").into_iter().collect();
    let orders: Vec<Order> = parts[0]
        .trim()
        .split('\n')
        .map(|s| parse_order(s))
        .collect();
    let updates: Vec<Update> = parts[1]
        .trim()
        .split('\n')
        .map(|s| parse_update(s))
        .collect();

    (orders, updates)
}

fn parse_order(input: &str) -> Order {
    let parts: Vec<u32> = input
        .split('|')
        .into_iter()
        .map(|s| s.parse().unwrap())
        .collect();
    Order {
        before: parts[0],
        after: parts[1],
    }
}

fn parse_update(input: &str) -> Update {
    input
        .split(',')
        .into_iter()
        .map(|s| s.parse().unwrap())
        .collect()
}
