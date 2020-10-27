use kai_ir::ir_gen;
use kai_llvm_gen::llvm_gen;
use kai_parse::grammar::*;
use kai_typecheck::typecheck;
use std::path::{Path, PathBuf};
use std::{env, str::FromStr};
use std::{fs::File, io::Read};

fn main() -> std::io::Result<()> {
  let args: Vec<String> = env::args().collect();
  let filename = &args[1];

  let mut dir = PathBuf::from_str(env!("CARGO_MANIFEST_DIR")).unwrap();
  dir.push(Path::new(filename));
  let mut file = File::open(Path::new(filename))?;
  let mut program = String::new();
  println!("filename {:?}", filename);
  println!("dir {:?}", dir);
  file.read_to_string(&mut program)?;
  // println!("program: {:?}", program);

  let parser = FunctionParser::new();
  let ast = parser.parse(&program).unwrap();
  // println!("ast: {:#?}", ast);
  let var_ty_map = typecheck(&ast);
  let ir = ir_gen(&ast, var_ty_map);
  // println!("ir: {}", ir.to_string());
  unsafe {
    llvm_gen(&ir, &mut dir);
  }
  Ok(())
}
