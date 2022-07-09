pub fn main() {
  let v = vec![1, 2, 3, 4, 5];

  let third: &i32 = &v[2];
  println!("The third element is {}", third);

  match v.get(5) {
      Some(third) => println!("The sixth element is {}", third),
      None => println!("There is no sixth element"),
  }

  for i in &v {
      println!("{}", i);
  }

  let mut v1 = vec![100, 32, 57];
  for i in &mut v1 {
      *i += 50;
      println!("{}", i);
  }

  #[derive(Debug)]
  enum SpreadsheetCell {
      Int(i32),
      Float(f64),
      Text(String),
  }

  let row = vec![
      SpreadsheetCell::Int(3),
      SpreadsheetCell::Text(String::from("blue")),
      SpreadsheetCell::Float(10.12),
  ];

  for i in row {
      println!("{:?}", i);
  }

}