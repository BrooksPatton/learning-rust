extern crate minigrep;
use minigrep::grep;

#[test]
fn case_insensitive_search() {
  let config = grep::Config {
    query: String::from("hello"),
    filename: String::from("test_text.txt"),
    case_sensitive: false,
  };
  
  let text = grep::run(config).unwrap();

  assert_eq!(text, "hello world
HELLo WoRld");
}

#[test]
fn case_sensitive_search() {
  let config = grep::Config {
    query: String::from("hello"),
    filename: String::from("test_text.txt"),
    case_sensitive: true,
  };

  let text = grep::run(config).unwrap();

  assert_eq!(text, "hello world");
}