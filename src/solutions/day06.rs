use std::collections::HashMap;

pub fn solve_part1(input: &str) -> String {
    let (start_point, map) = parse_map(input);

    let mut map = map;
    let mut state = GuardState {
        point: start_point,
        dir: Direction::Up,
    };

    loop {
        match step(&state, &map) {
            None => break,
            Some(new_state) => {
                if state.point != new_state.point {
                    map.entry(new_state.point)
                        .and_modify(|pos| pos.visited = true);
                }
                state = new_state;
            }
        }
    }

    let num_visited = map.into_values().filter(|pos| pos.visited).count();

    format!("{:#?}", num_visited)
}

pub fn solve_part2(input: &str) -> String {
    format!("{}", input)
}

fn step(state: &GuardState, map: &Map) -> Option<GuardState> {
    fn move_forward(point: Point, dir: &Direction) -> Point {
        let delta = match dir {
            Direction::Up => (0, -1),
            Direction::Down => (0, 1),
            Direction::Left => (-1, 0),
            Direction::Right => (1, 0),
        };
        (point.0 + delta.0, point.1 + delta.1)
    }

    fn turn_right(dir: &Direction) -> Direction {
        match dir {
            Direction::Up => Direction::Right,
            Direction::Down => Direction::Left,
            Direction::Left => Direction::Up,
            Direction::Right => Direction::Down,
        }
    }

    let next_point = move_forward(state.point, &state.dir);
    let next_pos = map.get(&next_point)?; // Out of bounds

    if next_pos.obstruction {
        Some(GuardState {
            point: state.point,
            dir: turn_right(&state.dir),
        })
    } else {
        Some(GuardState {
            point: next_point,
            dir: state.dir.clone(),
        })
    }
}

#[derive(Debug)]
struct GuardState {
    point: Point,
    dir: Direction,
}

type Point = (i32, i32);
type Map = HashMap<Point, Pos>;

#[derive(Debug)]
struct Pos {
    obstruction: bool,
    visited: bool,
}

#[derive(Debug, Clone)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
}

fn parse_map(input: &str) -> (Point, Map) {
    let mut map = HashMap::new();
    let mut start = None;

    for (y, row) in input.split("\n").enumerate() {
        for (x, char) in row.chars().enumerate() {
            let mut visited = false;
            let point = (x.try_into().unwrap(), y.try_into().unwrap());

            if char == '^' {
                start = Some(point);
                visited = true;
            };

            let pos = Pos {
                obstruction: char == '#',
                visited: visited,
            };

            map.insert(point, pos);
        }
    }

    (start.unwrap(), map)
}
