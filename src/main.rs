mod day_1;
mod day_2;
mod day_3;
mod day_4;
mod day_5;

use std::fs::read_to_string;
use clap::Parser;

/// Advent of Code 2024
#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
  /// Day of AoC
  #[arg(short, long)]
  day: u8,

  /// Part of the day
  #[arg(short, long)]
  part: u8,

  /// Use sample input file
  #[arg(short, long, default_value_t=false)]
  sample: bool,
}

fn read_lines(filename: &str) -> Vec<String> {
    let mut result = Vec::new();

    for line in read_to_string(filename).unwrap().lines() {
        result.push(line.to_string())
    }

    result
}

fn main() {
  let args = Args::parse();

  let mut filename = format!("./src/day_{}/input.txt", args.day);
  if args.sample {
    filename = format!("./src/day_{}/sample.txt", args.day);
  }
  let lines = read_lines(&filename);

  let result = match args.day {
    1 => match args.part {
      1 => day_1::part1::part1(lines),
      2 => day_1::part2::part2(lines),
      _ => panic!("Invalid part {}", args.part),
    },
    2 => match args.part {
      1 => day_2::part1::part1(lines),
      2 => day_2::part2::part2(lines),
      _ => panic!("Invalid part {}", args.part),
    },
    3 => match args.part {
      1 => day_3::part1::part1(lines),
      2 => day_3::part2::part2(lines),
      _ => panic!("Invalid part {}", args.part),
    },
    4 => match args.part {
      1 => day_4::part1::part1(lines),
      2 => day_4::part2::part2(lines),
      _ => panic!("Invalid part {}", args.part),
    },
    5 => match args.part {
      1 => day_5::part1::part1(lines),
      2 => day_5::part2::part2(lines),
      _ => panic!("Invalid part {}", args.part),
    },
    _ => panic!("Invalid day {}", args.day),
  };

  println!("Result: {}", result);
}
