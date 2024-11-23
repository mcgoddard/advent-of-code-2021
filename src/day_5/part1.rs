use std::{cmp::{max, min}, collections::HashMap};
use super::lib::{ Instruction, Point, parse_lines, count_overlaps };

pub fn part1(lines: Vec<String>) -> i32 {
  let instructions = parse_lines(lines);
  let grid = create_grid(instructions);
  count_overlaps(grid)
}

fn create_grid(instructions: Vec<Instruction>) -> HashMap<Point, i32> {
  let mut grid = HashMap::new();
  for instruction in instructions {
    if instruction.start.x == instruction.end.x || instruction.start.y == instruction.end.y {
      let start_x = min(instruction.start.x, instruction.end.x);
      let end_x = max(instruction.start.x, instruction.end.x);
      let start_y = min(instruction.start.y, instruction.end.y);
      let end_y = max(instruction.start.y, instruction.end.y);
      for x in start_x..end_x + 1 {
        for y in start_y..end_y + 1 {
          let point = Point { x, y };
          let count = grid.entry(point).or_insert(0);
          *count += 1;
        }
      }
    }
  }
  grid
}
