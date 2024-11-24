pub fn part2(lines: Vec<String>) -> i64 {
  let values: Vec<i64> = lines[0].split(",").map(|v| v.parse().unwrap()).collect();
  let mut results = Vec::new();
  let min = *values.iter().min().unwrap();
  let max = *values.iter().max().unwrap();
  for value in min..max + 1 {
    results.push(values.iter().map(|v| triangle_numbers((v - value).abs())).sum());
  }
  *results.iter().min().unwrap()
}

fn triangle_numbers(n: i64) -> i64 {
  (n * (n + 1)) / 2
}
