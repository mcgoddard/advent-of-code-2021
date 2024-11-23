use crate::day_4::lib::{ parse_card_string, check_card, calculate_score };

pub fn part1(lines: Vec<String>) -> i32 {
  let mut called_numbers = vec![];
  let numbers_to_call = lines[0].split(",").map(|number| number.parse::<i32>().unwrap()).collect::<Vec<i32>>();
  let cards = parse_card_string(lines);
  for called_number in numbers_to_call {
    called_numbers.push(called_number);
    for card in cards.iter() {
      if check_card(card, &called_numbers) {
        return calculate_score(card, &called_numbers, called_number);
      }
    }
  }
  panic!("No card filled");
}
