pub fn part2(lines: Vec<String>) -> i32 {
  let readings = lines.iter().map(|line| line.parse::<i32>().unwrap()).collect::<Vec<i32>>();
  let windows = readings.windows(3).map(|window| window.iter().sum::<i32>()).collect::<Vec<i32>>();
  let window_pairs = windows.iter().zip(windows.iter().skip(1));
  window_pairs.filter(|(window1, window2)| window1 < window2).count().try_into().unwrap()
}
