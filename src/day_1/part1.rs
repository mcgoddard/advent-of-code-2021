pub fn part1(lines: Vec<String>) -> i32 {
  let readings = lines.iter().map(|line| line.parse::<i32>().unwrap()).collect::<Vec<i32>>();
  let pairs = readings.iter().zip(readings.iter().skip(1));
  pairs.filter(|(reading1, reading2)| reading1 < reading2).count().try_into().unwrap()
}
