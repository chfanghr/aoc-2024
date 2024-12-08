use std::{
    fs::read_to_string,
    path::{Path, PathBuf},
};

use aoc_2024::{day_1, day_2, day_3, day_4, day_5, day_6, day_7, day_8};
use clap::Parser;

#[derive(Debug, clap::Parser)]
struct Cli {
    #[arg(short = 'i', long, global = true, default_value = "puzzle_input.txt")]
    puzzle_input_path: PathBuf,

    #[command(subcommand)]
    day: Day,
}

#[derive(Debug, clap::Subcommand)]
enum Day {
    Day1,
    Day2,
    Day3,
    Day4,
    Day5,
    Day6,
    Day7,
    Day8,
}

fn solve_puzzle_and_print<
    P: AsRef<Path>,
    F: FnOnce(&str) -> anyhow::Result<Box<dyn std::fmt::Debug>>,
>(
    input_path: P,
    solve: F,
) -> anyhow::Result<()> {
    let input = read_to_string(input_path)?;
    let answer = solve(&input)?;
    println!("{:?}", answer);
    Ok(())
}

fn box_solver<T: std::fmt::Debug + 'static, F: 'static + FnOnce(&str) -> anyhow::Result<T>>(
    solver: F,
) -> Box<dyn FnOnce(&str) -> anyhow::Result<Box<dyn std::fmt::Debug>>> {
    return Box::new(|input: &str| {
        solver(input).map(|r| -> Box<dyn std::fmt::Debug> { Box::new(r) })
    });
}

fn main() -> anyhow::Result<()> {
    let cli = Cli::try_parse()?;

    solve_puzzle_and_print(
        cli.puzzle_input_path,
        match cli.day {
            Day::Day1 => box_solver(day_1::solution),
            Day::Day2 => box_solver(day_2::solution),
            Day::Day3 => box_solver(day_3::solution),
            Day::Day4 => box_solver(day_4::solution),
            Day::Day5 => box_solver(day_5::solution),
            Day::Day6 => box_solver(day_6::solution),
            Day::Day7 => box_solver(day_7::solution),
            Day::Day8 => box_solver(day_8::solution),
        },
    )
}
