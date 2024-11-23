pub fn parse_card_string(lines: Vec<String>) -> Vec<Vec<Vec<i32>>> {
  let mut cards_strings = vec![vec![]];
  for index in 0..(lines[2..].len() / 5) {
    cards_strings.push(lines[2..].iter().skip(index * 6).take(5).map(|s| s.to_string()).collect());
  }
  let mut cards = vec![];
  for card_string in cards_strings[1..].iter() {
    let mut card = vec![];
    for card_line_string in card_string {
      let card_line_parts = card_line_string.split_whitespace().collect::<Vec<&str>>();
      card.push(card_line_parts.iter().map(|part| part.parse::<i32>().unwrap()).collect::<Vec<i32>>());
    }
    if card.len() == 5 {
      cards.push(card);
    }
  }
  cards
}

pub fn check_card(card: &Vec<Vec<i32>>, called_numbers: &Vec<i32>) -> bool {
  for row in card.iter() {
    let mut row_filled = true;
    for number in row.iter() {
      if !called_numbers.contains(number) {
        row_filled = false;
      }
    }
    if row_filled {
      return true;
    }
  }
  for column_index in 0..card.len() {
    let mut column_filled = true;
    for row in card.iter() {
      if !called_numbers.contains(&row[column_index]) {
        column_filled = false;
      }
    }
    if column_filled {
      return true;
    }
  }
  false
}

pub fn calculate_score(card: &Vec<Vec<i32>>, called_numbers: &Vec<i32>, called_number: i32) -> i64 {
  let unmarked = card.iter().map(|row| {
    row.iter().filter(|number| !called_numbers.contains(number)).collect::<Vec<&i32>>()
  }).collect::<Vec<Vec<&i32>>>();
  let unmarked_sum = unmarked.iter().map(|row| {
    row.iter().fold(0, |acc, number| acc + *number)
  }).fold(0, |acc, number| acc + number);
  (unmarked_sum * called_number) as i64
}

