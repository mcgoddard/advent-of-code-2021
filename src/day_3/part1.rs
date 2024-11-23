pub fn part1(lines: Vec<String>) -> i64 {
  let parsed = lines.iter().map(|line| {
    let substrings: Vec<char> = line.chars().collect();
    substrings.iter().map(|substring| substring.to_digit(10).unwrap()).collect::<Vec<u32>>()
  }).collect::<Vec<Vec<u32>>>();
  let mut sums = vec![0; parsed[0].len()];
  for i in 0..parsed[0].len() {
    for parse in &parsed {
      sums[i] += parse[i];
    }
  }
  let mut gamma_bin = String::new();
  let mut epsilon_bin = String::new();
  for sum in &sums {
    if *sum > lines.len() as u32 / 2 {
      gamma_bin.push('1');
      epsilon_bin.push('0');
    } else if *sum < lines.len() as u32 / 2 {
      gamma_bin.push('0');
      epsilon_bin.push('1');
    } else {
      panic!("Same number of 1s and 0s");
    }
  }
  let gamma_rate = i32::from_str_radix(&gamma_bin, 2).unwrap();
  let epsilon_rate = i32::from_str_radix(&epsilon_bin, 2).unwrap();
  (gamma_rate * epsilon_rate) as i64
}
