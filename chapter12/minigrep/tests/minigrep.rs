extern crate minigrep;
use minigrep::grep;

#[test]
fn get_entire_text() {
  let input = vec![String::from("/src/main.rs"), String::from("something"), String::from("single_line.txt")];
  let config = grep::Config::new(input).unwrap();
  let text = grep::run(config).unwrap();

  assert_eq!(text, "hello world");
}