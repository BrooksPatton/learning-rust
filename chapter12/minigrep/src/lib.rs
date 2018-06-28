pub mod grep {
  use std::fs::File;
  use std::io::prelude::*;

  pub struct Config {
    pub query: String,
    pub filename: String,
  }

  impl Config {
    pub fn new(input: Vec<String>) -> Result<Config, String> {
      if input.len() == 3 {
        Ok(Config{
          query: input[1].clone(),
          filename: input[2].clone(),
        })
      } else {
        Err(format!("Not the right amount of arguments, expected 2, got {}", input.len() - 1))
      }
    }
  }

  pub fn run(config: Config) -> Result<String, String> {
    let mut file = match File::open(&config.filename) {
      Ok(file) => file,
      Err(err) => return Err(String::from("Error opening the file")),
    };

    let mut file_content = String::new();

    match file.read_to_string(&mut file_content) {
      Err(err) => return Err(String::from("Error reading the file")),
      _ => {},
    };

    Ok(file_content)
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn parse_std_in() {
      let std_in: Vec<String> = vec![String::from("hello/world"), String::from("our_query"), String::from("the_filename")];

      let config = grep::Config::new(std_in).unwrap();

      assert_eq!(config.query, "our_query");
      assert_eq!(config.filename, "the_filename");
  }

  #[test]
  fn config_should_error_when_not_correct_number_of_arguments() {
    let std_in = vec![String::from("hello/world")];
    match grep::Config::new(std_in) {
      Err(e) => assert_eq!(e, "Not the right amount of arguments, expected 2, got 0"),
      Ok(_) => panic!("We should not be ok"),
    }
  }
}