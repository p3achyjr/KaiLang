use kai_parse::grammar::*;
use kai_typecheck::typecheck;
fn main() {
  println!("Hello, world!");

  let parser = FunctionParser::new();
  let ast = parser
    .parse("function main(): int { let x = 5 + 4 * 3 / 2 + 8; }")
    .unwrap();
  println!("ast: {:#?}", ast);
  typecheck(&ast);
}

// pub fn parse() {
//   let parser = grammar::FunctionParser::new();
//   println!("ast: {:?}", parser.parse("function main(): int {}").unwrap());
// }
