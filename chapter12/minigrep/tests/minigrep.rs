extern crate minigrep;
use minigrep::grep;

#[test]
fn case_insensitive_search() {
  let input = vec![String::from("/src/main.rs"), String::from("hello"), String::from("test_text.txt")];
  let case_sensitive_search = false;
  let config = grep::Config::new(input, case_sensitive_search).unwrap();
  let text = grep::run(config).unwrap();

  assert_eq!(text, "hello world
HELLo WoRld");
}

#[test]
fn case_sensitive_search() {
  let input = vec![String::from("/src/main.rs"), String::from("hello"), String::from("test_text.txt")];
  let case_sensitive_search = true;
  let config = grep::Config::new(input, case_sensitive_search).unwrap();
  let text = grep::run(config).unwrap();

  assert_eq!(text, "hello world");
}