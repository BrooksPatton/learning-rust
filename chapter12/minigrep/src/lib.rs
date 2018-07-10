//!# Minigrep
//! 
//! A crate for searching through files

pub use grep::Config;

pub mod grep {
  use std::fs::File;
  use std::io::prelude::*;
  use std::env::Args;

  pub struct Config {
    pub query: String,
    pub filename: String,
    pub case_sensitive: bool,
  }

  impl Config {
    pub fn new(mut args: Args, case_sensitive: bool) -> Result<Config, String> {
      args.next();
      
      let query = match args.next() {
        Some(q) => q,
        None => return Err(String::from("Query option missing")),
      };

      let filename = match args.next() {
        Some(f) => f,
        None => return Err(String::from("Filename option mission")),
      };

      Ok(Config {
        query,
        filename,
         case_sensitive,
      })
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

    if config.case_sensitive {
      Ok(search_case_sensitive(&config.query, &file_content).join("\n"))
    } else {
      Ok(search_case_insensitive(&config.query, &file_content).join("\n"))
    }
  }

  /// # Search case-insensitive
  /// 
  /// Allows the caller to search through a string while ignoring case.
  /// 
  /// ```
  ///   let content = String::from("Rust:
  ///safe, fast, proDuctive.
  ///Pick three.");
  /// 
  ///   let query = String::from("duct");
  /// 
  ///   assert_eq!(minigrep::grep::search_case_insensitive(&query, &content), vec!["safe, fast, proDuctive."]);
  /// ```
  pub fn search_case_insensitive<'a, 'b>(query: &'a str, content: &'b str) -> Vec<&'b str> {
    let query = query.to_lowercase();

    content.lines()
      .filter(|line| line.to_lowercase().contains(&query))
      .collect()
  }

  pub fn search_case_sensitive<'a, 'b>(query: &'a str, content: &'b str) -> Vec<&'b str> {
    content.lines()
      .filter(|line| line.contains(query))
      .collect()
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  // #[test]
//   fn search_case_insensitive() {
//     let query = String::from("duct");
//     let content = String::from("Rust:
// safe, fast, proDuctive.
// Pick three.");

//     assert_eq!(grep::search_case_insensitive(&query, &content), vec!["safe, fast, proDuctive."]);
//   }

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