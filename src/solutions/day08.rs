use std::collections::{HashMap, HashSet};

pub fn solve_part1(input: &str) -> String {
    let (x_size, y_size) = get_grid_size(input);
    let antennas = parse_input(input);

    let mut antis = Vec::new();
    for (_, points) in antennas.into_iter() {
        let perms = permutations(points);
        for (p1, p2) in perms {
            let (a1, a2) = antipoints(&p1, &p2);
            antis.push(a1);
            antis.push(a2);
        }
    }

    antis.retain(|p| p.x < x_size && p.x >= 0 && p.y < y_size && p.y >= 0);

    let unique_antis: HashSet<Point> = HashSet::from_iter(antis);

    format!("{:?}", unique_antis.len())
}

pub fn solve_part2(input: &str) -> String {
    format!("{input}")
}

fn get_grid_size(input: &str) -> (i32, i32) {
    let rows: Vec<&str> = input.split("\n").collect();
    (
        rows[0].len().try_into().unwrap(),
        rows.len().try_into().unwrap(),
    )
}

fn delta(a: &Point, b: &Point) -> (u32, u32) {
    (a.x.abs_diff(b.x), a.y.abs_diff(b.y))
}

fn antipoints(a: &Point, b: &Point) -> (Point, Point) {
    let delta = delta(a, b);
    let delta = (i32::try_from(delta.0).unwrap(), i32::try_from(delta.1).unwrap());

    let a1 = Point {
        x: if a.x > b.x { a.x + delta.0 } else { a.x - delta.0},
        y: if a.y > b.y { a.y + delta.1 } else { a.y - delta.1},
    };
    let a2 = Point {
        x: if b.x > a.x { b.x + delta.0 } else { b.x - delta.0},
        y: if b.y > a.y { b.y + delta.1 } else { b.y - delta.1},
    };

    (a1, a2)
}

fn permutations(points: HashSet<Point>) -> Vec<(Point, Point)> {
    let mut res = Vec::new();

    for (ix, p) in points.iter().enumerate() {
        let ps: Vec<Point> = points.iter().cloned().collect();
        for p2 in &ps[ix + 1..] {
            res.push((p.clone(), p2.clone()))
        }
    }

    res
}

#[derive(Clone, Debug, Eq, Hash, PartialEq)]
struct Point {
    x: i32,
    y: i32,
}
impl Point {
    fn pp(&self) -> String {
        format!("({}, {})", self.x, self.y)
    }
}

type Antennas = HashMap<char, HashSet<Point>>;

fn parse_input(input: &str) -> Antennas {
    let mut antennas: Antennas = HashMap::new();

    for (y, row) in input.split("\n").enumerate() {
        for (x, char) in row.chars().enumerate() {
            if char == '.' {
                continue;
            }

            let point = Point {
                x: x.try_into().unwrap(),
                y: y.try_into().unwrap(),
            };
            antennas
                .entry(char)
                .and_modify(|points| {
                    points.insert(point.clone());
                })
                .or_insert(HashSet::from([point]));
        }
    }

    antennas
}
