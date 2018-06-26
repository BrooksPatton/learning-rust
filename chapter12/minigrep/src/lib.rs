pub mod parse {
  pub fn input<'a>(input: &'a Vec<String>) -> Result<(&'a str, &'a str), String> {
    if input.len() == 3 {
      Ok((&input[1], &input[2]))
    } else {
      Err(format!("Not the right amount of arguments, expected 2, got {}", input.len() - 1))
    }
  }
}

#[cfg(test)]
mod tests {
  use super::parse;

  #[test]
  fn parse_std_in() {
      let std_in: Vec<String> = vec!["hello/world", "our_query", "the_filename"];
      let config = parse::input(&std_in).unwrap();

      assert_eq!(config.query, "our_query");
      assert_eq!(config.filename, "the_filename");
  }

  #[test]
  fn parse_input_should_error_when_not_correct_number_of_arguments() {
    let std_in = vec!["hello/world".to_string()];
    match parse::input(&std_in) {
      Err(e) => assert_eq!(e, "Not the right amount of arguments, expected 2, got 0"),
      Ok(_) => panic!("We should not be ok"),
    }
  }
}