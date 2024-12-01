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
}

impl Day {
    fn padded(&self) -> &'static str {
        match self {
            Day::D1 => "01",
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

fn run_solution(args: &Args) -> Result<String, String> {
    // Pick the correct solver
    let solver: fn(&str) -> String = match (&args.day, &args.part) {
        (Day::D1, Part::One) => solutions::day01::solve_part1,
        (Day::D1, Part::Two) => solutions::day01::solve_part2,
    };

    // Build the filename for the correct input
    let input_filename = match args.example {
        None => format!("day{}.txt", args.day.padded()),
        Some(example) => format!("day{}_example{}.txt", args.day.padded(), example),
    };
    let input_filepath = format!("inputs/{}", input_filename);

    // Read the input
    let input = fs::read_to_string(&input_filepath)
        .unwrap_or_else(|_| panic!("Failed to read input from file: {}", &input_filepath));

    // Run the solution and return the result
    Ok(solver(&input))
}

fn _main() -> Result<String, String> {
    let args = parse_args()?;
    match run_solution(&args) {
        Ok(result) => {
            println!("{result}");
            Ok(result)
        }
        Err(err_msg) => {
            println!("{err_msg}");
            Err(err_msg)
        }
    }
}

fn main() {
    match _main() {
        Ok(_) => process::exit(0),
        Err(_) => process::exit(1),
    }
}
