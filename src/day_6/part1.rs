pub fn part1(lines: Vec<String>) -> i64 {
  let mut timers = lines[0].split(",").map(|v| v.parse().unwrap()).collect();
  let days = 80;
  for _ in 0..days {
    timers = process_timers(timers);
  }
  timers.len() as i64
}

fn process_timers(timers: Vec<i32>) -> Vec<i32> {
  let mut new_timers = vec![];
  for timer in timers {
    if timer == 0 {
      new_timers.push(6);
      new_timers.push(8);
    } else {
      new_timers.push(timer - 1);
    }
  }
  new_timers
}
