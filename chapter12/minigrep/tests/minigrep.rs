extern crate minigrep;
use minigrep::grep;

#[test]
fn get_entire_text() {
  let input = vec![String::from("/src/main.rs"), String::from("HELLO"), String::from("single_line.txt")];
  let config = grep::Config::new(input, false).unwrap();
  let text = grep::run(config).unwrap();

  assert_eq!(text, "hello world");
}