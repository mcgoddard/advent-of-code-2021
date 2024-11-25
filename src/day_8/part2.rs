use std::collections::HashSet;

pub fn part2(lines: Vec<String>) -> i64 {
  let parts: Vec<Vec<&str>> = lines.iter().map(| l | l.split(" | ").collect::<Vec<&str>>()).collect();
  let inputs: Vec<Vec<&str>> = parts.iter().map(| p | p[0]).collect::<Vec<&str>>().iter().map(| p | p.split(" ").collect::<Vec<&str>>()).collect();
  let outputs: Vec<Vec<&str>> = parts.iter().map(| p | p[1]).collect::<Vec<&str>>().iter().map(| p | p.split(" ").collect::<Vec<&str>>()).collect();
  let mut output_value = 0;
  for i in 0..inputs.len() {
    let one_set = inputs[i].iter().filter(| i | i.len() == 2).cloned().collect::<Vec<&str>>()[0].chars().collect::<HashSet<char>>();
    let four_set = inputs[i].iter().filter(| i | i.len() == 4).cloned().collect::<Vec<&str>>()[0].chars().collect::<HashSet<char>>();
    let four_one_diff = four_set.difference(&one_set).cloned().collect::<HashSet<char>>();
    let mut current_output = 0;
    for (index, result) in outputs[i].iter().enumerate() {
      let result_set = result.chars().collect::<HashSet<char>>();
      let multiplier = (10 as i64).pow((outputs[i].len() - index - 1) as u32);
      match result.len() {
        2 => current_output += 1 * multiplier,
        3 => current_output += 7 * multiplier,
        4 => current_output += 4 * multiplier,
        7 => current_output += 8 * multiplier,
        5 => {
          if result_set.is_superset(&one_set) {
            current_output += 3 * multiplier;
          } else if result_set.is_superset(&four_one_diff) {
            current_output += 5 * multiplier;
          } else {
            current_output += 2 * multiplier;
          }
        },
        6 => {
          if result_set.is_superset(&four_set) {
            current_output += 9 * multiplier;
          } else if result_set.is_superset(&four_one_diff) {
            current_output += 6 * multiplier;
          } else {
            current_output += 0 * multiplier;
          }
        }
        _ => panic!("Unexpected length"),
      }
    }
    output_value += current_output as i64;
  }
  output_value
}
