use std::collections::HashMap;

pub fn main() {
  let mut scores = HashMap::new();
  scores.insert(String::from("Blue"), 10);
  scores.insert(String::from("Yellow"), 5);

  let teams = vec![String::from("Blue"), String::from("Yellow")];
  let initial_scores = vec![10, 50];
  let mut scores: HashMap<_, _> =
    teams.into_iter().zip(initial_scores.into_iter()).collect();
  println!("{:?}", scores);

  let team_name = String::from("Blue");
  if let Some(score) = scores.get(&team_name) {
    println!("{} score is {}", team_name, score);
  }

  for (key, value) in &scores {
    println!("{} : {}", key, value);
  }

  scores.entry(String::from("Blue")).or_insert(100);
  scores.entry(String::from("Red")).or_insert(80);
  println!("{:?}", scores);

  let text = "hello world wonderful world";
  let mut map = HashMap::new();
  for word in text.split_whitespace() {
    let count = map.entry(word).or_insert(0);
    *count += 1;
  }

  println!("{:?}", map);
}