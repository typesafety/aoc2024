use std::collections::{HashMap, HashSet};

pub fn solve_part1(input: &str) -> String {
    let (rules, updates) = parse_input(input);

    let page_map = make_page_map(rules);
    let good_updates = updates
        .into_iter()
        .filter(|update| check_update(&update, &page_map));
    let result: u32 = good_updates.map(|update| update[update.len() / 2]).sum();

    format!("{}", result)
}

pub fn solve_part2(input: &str) -> String {
    format!("")
}

fn check_update(update: &Update, page_map: &HashMap<u32, Page>) -> bool {
    let mut page_map = filter_page_map_for_update(&page_map, update.clone());

    for rule in update {
        let page = page_map.remove(&rule).unwrap();

        if page.before.iter().any(|n| page_map.contains_key(n)) {
            return false;
        }
    }
    true
}

fn filter_page_map_for_update(page_map: &HashMap<u32, Page>, update: Update) -> HashMap<u32, Page> {
    let update: HashSet<u32> = HashSet::from_iter(update);
    page_map
        .clone()
        .into_iter()
        .filter(|(k, _)| update.contains(k))
        .collect()
}

fn make_page_map(rules: Vec<Rule>) -> HashMap<u32, Page> {
    let mut map: HashMap<u32, Page> = HashMap::new();

    for rule in rules {
        let page_left = map.entry(rule.before).or_insert(Page {
            _page: rule.before,
            before: HashSet::new(),
            after: HashSet::new(),
        });
        page_left.after.insert(rule.after);

        let page_right = map.entry(rule.after).or_insert(Page {
            _page: rule.after,
            before: HashSet::new(),
            after: HashSet::new(),
        });
        page_right.before.insert(rule.before);
    }

    map
}

#[derive(Debug, Clone)]
struct Page {
    _page: u32,
    before: HashSet<u32>,
    after: HashSet<u32>,
}

#[derive(Debug)]
struct Rule {
    before: u32,
    after: u32,
}

type Update = Vec<u32>;

fn parse_input(input: &str) -> (Vec<Rule>, Vec<Update>) {
    let parts: Vec<&str> = input.split("\n\n").into_iter().collect();
    let orders: Vec<Rule> = parts[0]
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

fn parse_order(input: &str) -> Rule {
    let parts: Vec<u32> = input
        .split('|')
        .into_iter()
        .map(|s| s.parse().unwrap())
        .collect();
    Rule {
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
