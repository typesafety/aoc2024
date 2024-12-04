pub fn solve_part1(input: &str) -> String {
    let grid = parse_input(input);
    let mut count = 0;

    for (y, row) in grid.iter().enumerate() {
        for (x, _) in row.iter().enumerate() {
            let x = x.try_into().unwrap();
            let y = y.try_into().unwrap();
            count += read_word("XMAS", &grid, (x, y));
        }
    }

    format!("{}", count)
}

pub fn solve_part2(input: &str) -> String {
    let grid = parse_input(input);
    let mut count = 0;

    for (y, row) in grid.iter().enumerate() {
        for (x, _) in row.iter().enumerate() {
            let x = x.try_into().unwrap();
            let y = y.try_into().unwrap();

            let is_a = get(&(x, y), &grid) == Some('A');
            if is_a && is_xmas((x, y), &grid) {
                count += 1;
            }
        }
    }

    format!("{}", count)
}

fn is_xmas(point: Point, grid: &Grid) -> bool {
    fn get_letters(middle: Point, dirs: impl Iterator<Item = Direction>, grid: &Grid) -> Vec<char> {
        dirs.filter_map(|d| get(&next_point(middle, &d), grid))
            .collect()
    }

    fn is_mas(chars: Vec<char>) -> bool {
        let ms: Vec<char> = "MS".chars().collect();
        let mut cs: Vec<char> = chars.clone();
        cs.sort();
        cs == ms
    }

    let d7_d3 = [Direction::D7, Direction::D3];
    let d1_d9 = [Direction::D1, Direction::D9];

    let d7_d3_is_mas = is_mas(get_letters(point, d7_d3.into_iter(), grid));
    let d1_d9_is_mas = is_mas(get_letters(point, d1_d9.into_iter(), grid));

    d7_d3_is_mas && d1_d9_is_mas
}

fn get(point: &Point, grid: &Grid) -> Option<char> {
    if inside(*point, grid) {
        Some(grid[get_y(point)][get_x(point)])
    } else {
        None
    }
}

type Grid = Vec<Vec<char>>;
type Point = (i32, i32);

#[derive(Debug)]
enum Direction {
    D1,
    D2,
    D3,
    D4,
    D6,
    D7,
    D8,
    D9,
}

fn read_word(word: &str, grid: &Grid, start: Point) -> u32 {
    let directions = [
        Direction::D1,
        Direction::D2,
        Direction::D3,
        Direction::D4,
        Direction::D6,
        Direction::D7,
        Direction::D8,
        Direction::D9,
    ];

    directions
        .iter()
        .map(|dir| try_read_word_dir(word, grid, start, dir))
        .filter(|b| *b)
        .count()
        .try_into()
        .unwrap()
}

fn get_x(p: &Point) -> usize {
    p.0.try_into().unwrap()
}

fn get_y(p: &Point) -> usize {
    p.1.try_into().unwrap()
}

fn try_read_word_dir(word: &str, grid: &Grid, start: Point, dir: &Direction) -> bool {
    let mut point = start;

    for (ix, c) in word.chars().enumerate() {
        let grid_char = grid[get_y(&point)][get_x(&point)];
        if c != grid_char {
            // If the char does not match
            return false;
        }

        if ix == word.len() - 1 {
            // Last iteration -- don't try to step
            break;
        }

        match step(point, dir, grid) {
            Some(next_point) => {
                point = next_point;
            }
            None => return false, // If we go out of bounds
        }
    }
    true
}

fn step(point: Point, dir: &Direction, grid: &Grid) -> Option<Point> {
    let next = next_point(point, dir);
    if inside(next, grid) {
        Some(next)
    } else {
        None
    }
}

fn next_point(point: Point, dir: &Direction) -> Point {
    let (x, y) = point;
    match dir {
        Direction::D1 => (x - 1, y + 1),
        Direction::D2 => (x, y + 1),
        Direction::D3 => (x + 1, y + 1),
        Direction::D4 => (x - 1, y),
        Direction::D6 => (x + 1, y),
        Direction::D7 => (x - 1, y - 1),
        Direction::D8 => (x, y - 1),
        Direction::D9 => (x + 1, y - 1),
    }
}

fn inside(point: Point, grid: &Grid) -> bool {
    let grid_width = grid[0].len().try_into().unwrap();
    let grid_height = grid.len().try_into().unwrap();

    let (x, y) = point;
    let x_inside = x < grid_width && x >= 0;
    let y_inside = y < grid_height && y >= 0;

    x_inside && y_inside
}

fn parse_input(input: &str) -> Grid {
    input.split("\n").map(parse_line).collect()
}

fn parse_line(input: &str) -> Vec<char> {
    input.chars().collect()
}
