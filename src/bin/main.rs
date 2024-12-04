use aoc_2024::{self, day_1};

fn main() -> anyhow::Result<()> {
    println!(
        "day_1 {:?}",
        day_1::solution(include_str!("./puzzle_inputs/day_1.input"))?
    );

    Ok(())
}
