fn main() {
  let s1 = String::from("hello");
  let len = calculate_length(&s1);
  println!("The length of '{}' is {}.", s1, len);

  let mut t = String::from("hello");
  change(&mut t);

  let reference_to_nothing = dangle();
}

fn calculate_length(s: &String) -> usize {
  s.len()
}

fn change(some_string: &mut String) {
  some_string.push_str(", world");
}

fn dangle() -> &String {
  let s = String::from("hello");
  s
}
