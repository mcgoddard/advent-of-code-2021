use crate::day_2::lib::{ Command, Instruction };

struct Position {
  horizontal: i32,
  depth: i32,
}

pub fn part1(lines: Vec<String>) -> i64 {
  let instructions = lines.iter().map(|line| {
    let parts = line.split(" ").collect::<Vec<&str>>();
    let instruction = match parts[0] {
      "forward" => Instruction::Forward,
      "down" => Instruction::Down,
      "up" => Instruction::Up,
      _ => panic!("Invalid instruction"),
    };
    let value = parts[1].parse::<i32>().unwrap();
    Command { instruction, value }
  }).collect::<Vec<Command>>();
  let final_position = instructions.iter().fold(Position { horizontal: 0, depth: 0 }, |position, command| {
    match command.instruction {
      Instruction::Forward => Position { horizontal: position.horizontal + command.value, depth: position.depth },
      Instruction::Down => Position { horizontal: position.horizontal, depth: position.depth + command.value },
      Instruction::Up => Position { horizontal: position.horizontal, depth: position.depth - command.value },
    }
  });
  (final_position.horizontal * final_position.depth) as i64
}
