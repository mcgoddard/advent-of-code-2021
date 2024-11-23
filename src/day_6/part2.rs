pub fn part2(lines: Vec<String>) -> i64 {
  let timers = lines[0].split(",").map(|v| v.parse().unwrap()).collect();
  let days = 256;
  let fish = process_timers(timers, days);
  fish.iter().sum()
}

fn process_timers(timers: Vec<u32>, days: i32) -> Vec<i64> {
  let mut fish = vec![0; 9];
  for timer in timers {
    fish[timer as usize] += 1;
  }
  for _ in 0..days {
    let num = fish.remove(0);
    fish[6] += num;
    fish.push(num);
  }
  fish
}
