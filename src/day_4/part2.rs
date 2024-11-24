use crate::day_4::lib::{ parse_card_string, check_card, calculate_score };

pub fn part2(lines: Vec<String>) -> i64 {
  let mut called_numbers = vec![];
  let numbers_to_call = lines[0].split(",").map(|number| number.parse::<i32>().unwrap()).collect::<Vec<i32>>();
  let mut cards = parse_card_string(lines);
  for called_number in numbers_to_call {
    called_numbers.push(called_number);
    if cards.len() == 1 && check_card(&cards[0], &called_numbers) {
      return calculate_score(&cards[0], &called_numbers, called_number);
    }
    cards = cards.iter().filter(|card| !check_card(card, &called_numbers)).cloned().collect();
  }
  panic!("No card filled");
}
