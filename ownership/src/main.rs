fn main() {
    let mut s = String::from("hello");
    s.push_str(", world!");
    println!("{}", s);

    let s2 = s;
    println!("{}, world!", s2);


    let s3 = takes_and_gives_back(s2);
    takes_ownership(s3);

    let s4 = String::from("hello");
    let (s5, len) = calculate_length(s4);
    println!("The length os '{}' is {}", s5, len);

    makes_copy(len);
}

fn takes_ownership(some_string: String) {
    println!("takes: {}", some_string);
}

fn makes_copy(some_integer: usize) {
    println!("{}", some_integer);
}

fn takes_and_gives_back(a_string: String) -> String {
    println!("takes and gives back: {}", a_string);
    a_string
}

fn calculate_length(s: String) -> (String, usize) {
    let length = s.len();
    (s, length)
}

fn string_slice() {
    let s = String::from("hello world");
    let hello = &s[0..5];
    let world = &s[6..11];
}

fn string_slice_drop0() {
    let s = String::from("hello");
    let slice = &s[0..2];
    let slice = &s[..2];
}

fn string_slice_dropend() {
    let s = String::from("hello");
    let len = s.len();

    let slice = &s[3..len];
    let slice = &s[3..];
}

fn string_slice_dropboth() {
    let s = String::from("hello");
    let len = s.len();

    let slice = &s[0..len];
    let slice = &s[..];
}