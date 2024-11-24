pub fn part2(lines: Vec<String>) -> i64 {
  let parsed = lines.iter().map(|line| {
    let substrings: Vec<char> = line.chars().collect();
    substrings.iter().map(|substring| substring.to_digit(10).unwrap()).collect::<Vec<u32>>()
  }).collect::<Vec<Vec<u32>>>();
  let mut oxygen_ratings = parsed.clone();
  let mut co2_ratings = parsed.clone();
  for index in 0..parsed[0].len() {
    if oxygen_ratings.len() > 1 {
      oxygen_ratings = filter_rating(index, &oxygen_ratings, 1);
    }
    if co2_ratings.len() > 1 {
      co2_ratings = filter_rating(index, &co2_ratings, 0);
    }
  }
  let oxygen_rating = oxygen_ratings[0].iter().fold(0, |acc, digit| (acc << 1) + digit);
  let co2_rating = co2_ratings[0].iter().fold(0, |acc, digit| (acc << 1) + digit);
  (oxygen_rating * co2_rating).into()
}

fn sum_rating_index(index: usize, ratings: &Vec<Vec<u32>>) -> u32 {
  let mut sum = 0;
  for rating in ratings {
    sum += rating[index];
  }
  sum
}

fn filter_rating(index: usize, ratings: &Vec<Vec<u32>>, filter_to: u32) -> Vec<Vec<u32>> {
  if ratings.len() < 2 {
    return ratings.to_vec();
  }
  let sum = sum_rating_index(index, ratings);
  if sum as f32 >= ratings.len() as f32 / 2.0 {
    ratings.iter().filter(|rating| rating[index] == filter_to).cloned().collect()
  } else {
    ratings.iter().filter(|rating| rating[index] != filter_to).cloned().collect()
  }
}
