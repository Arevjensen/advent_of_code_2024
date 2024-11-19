use anyhow::Result;
use aoc_2024::utils::enums::Day;
use clap::{command, Parser};

#[derive(Parser)]
#[command(version, about, long_about = None)]
struct Cli {
    #[arg(default_value_t = 1)]
    /// Day to run
    day: u32,
    /// part of the day to run
    #[arg(default_value_t = 1)]
    part: u8,
}

fn main() -> Result<()> {
    let args = Cli::parse();
    let day = Day::try_from(args.day)?;
    aoc_2024::run(day, args.part)
}
