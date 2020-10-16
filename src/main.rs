use kai_ast::ast::*;
use kai_parse::grammar::*;
fn main() {
  println!("Hello, world!");

  let parser = FunctionParser::new();
  println!(
    "ast: {:#?}",
    parser
      .parse("function main(): int { var x = 5 + 4 * 3 / 2 + 8; }")
      .unwrap()
  );
}

// pub fn parse() {
//   let parser = grammar::FunctionParser::new();
//   println!("ast: {:?}", parser.parse("function main(): int {}").unwrap());
// }
