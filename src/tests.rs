
#[cfg(test)]
mod test {
  use crate::parse::tests::run as run_parse_tests;
  #[test]
  fn test_parse() {
    run_parse_tests();
  }
}
