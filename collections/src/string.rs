pub fn main() {
  // create new string
  let mut s = String::new();

  let data = "initial contents";
  let s = data.to_string();
  let s = "initial contents".to_string();

  let s = String::from("initial contents");

  // updating a string
  let mut s = String::from("foo");
  s.push_str("bar");

  let mut s1 = String::from("foo");
  let s2 = "bar";
  s1.push_str(s2);
  println!("s2 is {}", s2);

  let mut s = String::from("lo");
  s.push('l');
  println!("s is {}", s);

  // concatenation
  let s1 = String::from("Hello, ");
  let s2 = String::from("World!");
  let s3 = s1 + &s2;
  println!("s3 is {}", s3);

  let s1 = String::from("tic");
  let s2 = String::from("tac");
  let s3 = String::from("toe");

  let sss = format!("{}-{}-{}", s1, s2, s3);
  println!("sss is {}", sss);

  for c in "नमस्ते".chars() {
    println!("{}", c);
  }

  for b in "नमस्ते".bytes() {
    println!("{}", b);
  }
}