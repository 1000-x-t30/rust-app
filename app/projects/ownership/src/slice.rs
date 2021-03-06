fn main() {
  let my_string = String::from("hello world");

  // first_word works on slices of `String`s
  // first_wordは`String`のスライスに対して機能する
  let word = first_word(&my_string[..]);

  let my_string_literal = "hello world";

  // first_word works on slices of string literals
  // first_wordは文字列リテラルのスライスに対して機能する
  let word = first_word(&my_string_literal[..]);

  // Because string literals *are* string slices already,
  // this works too, without the slice syntax!
  // 文字列リテラルは「それ自体すでに文字列スライスなので」、
  // スライス記法なしでも機能するのだ！
  let word = first_word(my_string_literal);
}

fn first_word(s: &str) -> &str {
  let bytes = s.as_bytes();
  println!("{:?}", bytes);

  for (i, &item) in bytes.iter().enumerate() {
      if item == b' ' {
          return &s[0..i];
      }
  }
  &s[..]
}