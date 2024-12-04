use std::{
    fs::read_to_string,
    path::{Path, PathBuf},
};

use aoc_2024::{day_1, day_2, day_3};
use clap::Parser;

#[derive(Debug, clap::Parser)]
struct Cli {
    #[command(subcommand)]
    command: Command,
}

#[derive(Debug, clap::Subcommand)]
enum Command {
    Day1 {
        #[arg(short = 'i', long, default_value = "./testdata/day1")]
        puzzle_input_path: PathBuf,
    },
    Day2 {
        #[arg(short = 'i', long, default_value = "./testdata/day2")]
        puzzle_input_path: PathBuf,
    },
    Day3 {
        #[arg(short = 'i', long, default_value = "./testdata/day3")]
        puzzle_input_path: PathBuf,
    },
}

fn load_puzzle_input(path: impl AsRef<Path>) -> anyhow::Result<String> {
    let input = read_to_string(path)?;
    Ok(input)
}

fn main() -> anyhow::Result<()> {
    let cli = Cli::try_parse()?;
    match cli.command {
        Command::Day1 { puzzle_input_path } => {
            let puzzle_input = load_puzzle_input(puzzle_input_path)?;
            println!("day1 {:?}", day_1::solution(&puzzle_input)?)
        }
        Command::Day2 { puzzle_input_path } => {
            let puzzle_input = load_puzzle_input(puzzle_input_path)?;
            println!("day2 {:?}", day_2::solution(&puzzle_input)?)
        }
        Command::Day3 { puzzle_input_path } => {
            let puzzle_input = load_puzzle_input(puzzle_input_path)?;
            println!("day3 {:?}", day_3::solution(&puzzle_input)?)
        }
    }

    Ok(())
}
