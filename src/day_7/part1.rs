pub fn part1(lines: Vec<String>) -> i64 {
  let mut values: Vec<i64> = lines[0].split(",").map(|v| v.parse().unwrap()).collect();
  values.sort();
  let median = values[values.len() / 2];
  values.into_iter().map(|v| (v - median).abs()).sum()
}
