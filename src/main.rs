mod solutions;

use std::fs;
use std::process;

use clap::Parser;

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct CliArgs {
    // Specify the day to use the input and solution from.
    #[arg(long)]
    day: u8,

    // Specify whether to use the solution for part 1 or 2.
    #[arg(long)]
    part: u8,

    // Optional, specify to use example input instead of the normal input.
    #[arg(long)]
    example: Option<u8>,
}

#[derive(Debug)]
enum Day {
    D1,
    D2,
    D3,
    D4,
    D5,
}

impl Day {
    fn padded(&self) -> &'static str {
        match self {
            Day::D1 => "01",
            Day::D2 => "02",
            Day::D3 => "03",
            Day::D4 => "04",
            Day::D5 => "05",
        }
    }
}

#[derive(Debug)]
enum Part {
    One,
    Two,
}

#[derive(Debug)]
struct Args {
    day: Day,
    part: Part,
    example: Option<u8>,
}

fn parse_args() -> Result<Args, String> {
    let args = CliArgs::parse();

    let day = match args.day {
        1 => Ok(Day::D1),
        2 => Ok(Day::D2),
        3 => Ok(Day::D3),
        4 => Ok(Day::D4),
        5 => Ok(Day::D5),
        n => Err(format!("Solution for day {n} is not yet implemented")),
    }?;

    let part = match args.part {
        1 => Ok(Part::One),
        2 => Ok(Part::Two),
        _ => Err(String::from("Specify part 1 or 2")),
    }?;

    Ok(Args {
        day,
        part,
        example: args.example,
    })
}

fn solve(args: &Args) -> Result<String, String> {
    // Pick the correct solver
    let solver: fn(&str) -> String = match (&args.day, &args.part) {
        (Day::D1, Part::One) => solutions::day01::solve_part1,
        (Day::D1, Part::Two) => solutions::day01::solve_part2,
        (Day::D2, Part::One) => solutions::day02::solve_part1,
        (Day::D2, Part::Two) => solutions::day02::solve_part2,
        (Day::D3, Part::One) => solutions::day03::solve_part1,
        (Day::D3, Part::Two) => solutions::day03::solve_part2,
        (Day::D4, Part::One) => solutions::day04::solve_part1,
        (Day::D4, Part::Two) => solutions::day04::solve_part2,
        (Day::D5, Part::One) => solutions::day05::solve_part1,
        (Day::D5, Part::Two) => solutions::day05::solve_part2,
    };

    // Build the filename for the correct input
    let input_filename = match args.example {
        None => format!("day{}.txt", args.day.padded()),
        Some(example) => format!("day{}_example{}.txt", args.day.padded(), example),
    };
    let input_filepath = format!("inputs/{}", input_filename);

    // Read the input
    let input = fs::read_to_string(&input_filepath).or(Err(format!(
        "Failed to read input from file: {}",
        &input_filepath
    )))?;

    // Run the solution and return the result
    Ok(solver(input.trim()))
}

fn _main() -> Result<String, String> {
    let args = parse_args()?;
    solve(&args)
}

fn main() {
    match _main() {
        Ok(result) => {
            println!("{result}");
            process::exit(0)
        }
        Err(err_msg) => {
            eprintln!("ERROR: {err_msg}");
            process::exit(1)
        }
    }
}
