use std::collections::HashSet;

pub fn part1(lines: Vec<String>) -> i64 {
  let outputs: Vec<Vec<&str>> = lines.iter().map(| l | l.split(" | ").collect::<Vec<&str>>()[1].split(" ").collect::<Vec<&str>>()).collect();
  let mut counts = 0;
  let unique_lengths = HashSet::from([2, 3, 4, 7]);
  for output in outputs {
    for result in output {
      if unique_lengths.contains(&result.len()) {
        counts += 1;
      }
    }
  }
  counts
}
