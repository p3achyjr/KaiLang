use kai_ir::ir_gen;
use kai_parse::grammar::*;
use kai_typecheck::typecheck;
fn main() {
  println!("Hello, world!");

  let parser = FunctionParser::new();
  let ast = parser
    .parse("function main(): int { let x = 5 + 4 * 3 / 2 + 8; return x; }")
    .unwrap();
  println!("ast: {:#?}", ast);
  typecheck(&ast);
  let ir = ir_gen(&ast);
  println!("ir: {}", ir.to_string());
}

// pub fn parse() {
//   let parser = grammar::FunctionParser::new();
//   println!("ast: {:?}", parser.parse("function main(): int {}").unwrap());
// }
