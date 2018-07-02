pub mod grep {
  use std::fs::File;
  use std::io::prelude::*;

  pub struct Config {
    pub query: String,
    pub filename: String,
    pub case_sensitive: bool,
  }

  impl Config {
    pub fn new(input: Vec<String>, case_sensitive: bool) -> Result<Config, String> {
      if input.len() == 3 {
        Ok(Config{
          query: input[1].clone(),
          filename: input[2].clone(),
          case_sensitive: case_sensitive,
        })
      } else {
        Err(format!("Not the right amount of arguments, expected 2, got {}", input.len() - 1))
      }
    }
  }

  pub fn run(config: Config) -> Result<String, String> {
    let mut file = match File::open(&config.filename) {
      Ok(file) => file,
      Err(_) => return Err(String::from("Error opening the file")),
    };

    let mut file_content = String::new();

    match file.read_to_string(&mut file_content) {
      Err(_) => return Err(String::from("Error reading the file")),
      _ => {},
    };

    Ok(file_content)
  }

  pub fn search_case_insensitive<'a, 'b>(query: &'a str, content: &'b str) -> Vec<&'b str> {
    let query = query.to_lowercase();
    let mut result: Vec<&str> = Vec::new();

    for line in content.lines() {
      if line.to_lowercase().contains(&query) {
        result.push(line);
      }
    }

    result
  }

  pub fn search_case_sensitive<'a, 'b>(query: &'a str, content: &'b str) -> Vec<&'b str> {
    let mut result: Vec<&str> = Vec::new();
    for line in content.lines() {
      if line.contains(query) {
        result.push(line);
      }
    }

    result
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn parse_std_in() {
      let std_in: Vec<String> = vec![String::from("hello/world"), String::from("our_query"), String::from("the_filename")];

      let config = grep::Config::new(std_in, true).unwrap();

      assert_eq!(config.query, "our_query");
      assert_eq!(config.filename, "the_filename");
  }

  #[test]
  fn config_should_error_when_not_correct_number_of_arguments() {
    let std_in = vec![String::from("hello/world")];
    match grep::Config::new(std_in, true) {
      Err(e) => assert_eq!(e, "Not the right amount of arguments, expected 2, got 0"),
      Ok(_) => panic!("We should not be ok"),
    }
  }

  #[test]
  fn search_case_insensitive() {
    let query = String::from("duct");
    let content = String::from("Rust:
safe, fast, proDuctive.
Pick three.");

    assert_eq!(grep::search_case_insensitive(&query, &content), vec!["safe, fast, proDuctive."]);
  }

  #[test]
  fn search_case_sensitive() {
    let query = String::from("Duct");
    let content = String::from("Rust:
safe, fast, productive.
Pick three. ProDuctive.");

    assert_eq!(grep::search_case_sensitive(&query, &content), vec!["Pick three. ProDuctive."]);
  }

  #[test]
  fn it_should_search_and_find_multiple() {
    let query = String::from("hi");
    let content = String::from("Hello:
hi there.
Hi their.");
    let result = vec![
      "hi there.",
      "Hi their."
    ];

    assert_eq!(grep::search_case_insensitive(&query, &content), result);
  }

  #[test]
  fn it_should_search_and_find_nothing() {
    let query = String::from("goodbye");
    let content = String::from("Hello:
hi there.
hi their.");
    let result: Vec<&str> = vec![];

    assert_eq!(grep::search_case_insensitive(&query, &content), result);
  }
}