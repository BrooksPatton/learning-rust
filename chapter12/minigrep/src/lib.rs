pub mod parse {
  pub fn parse_input<'a>(input: &'a Vec<&'a str>) -> &'a str {
    input[1]
  }
}

#[cfg(test)]
mod tests {
  use super::parse;

  #[test]
  fn parse_std_in() {
      let std_in = vec!["hello/world", "our_query", "the_filename"];
      let (query, filename) = parse::parse_input(&std_in);
      assert_eq!(query, "our_query");
      assert_eq!(filename, "the_filename");
  }
}