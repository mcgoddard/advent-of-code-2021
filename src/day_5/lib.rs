use std::collections::HashMap;

pub fn count_overlaps(grid: HashMap<Point, i32>) -> i32 {
  let mut count = 0;
  for (_, value) in grid {
    if value > 1 {
      count += 1;
    }
  }
  count
}

pub fn parse_lines(lines: Vec<String>) -> Vec<Instruction> {
  let mut instructions = Vec::new();
  for line in lines {
    let parts: Vec<&str> = line.split(" -> ").collect();
    let start = parts[0];
    let end = parts[1];
    let start_point = parse_point(start);
    let end_point = parse_point(end);
    let instruction = Instruction {
      start: start_point,
      end: end_point,
    };
    instructions.push(instruction);
  }
  instructions
}

pub fn parse_point(point: &str) -> Point {
  let parts: Vec<&str> = point.split(",").collect();
  let x = parts[0].parse().unwrap();
  let y = parts[1].parse().unwrap();
  Point { x, y }
}

#[derive(Debug, PartialEq, Eq, Hash)]
pub struct Point {
  pub x: i32,
  pub y: i32,
}

#[derive(Debug, PartialEq, Eq, Hash)]
pub struct Instruction {
  pub start: Point,
  pub end: Point,
}
