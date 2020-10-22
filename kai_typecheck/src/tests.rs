use cached::proc_macro::cached;

use std::path::{Path, PathBuf};
use std::str::FromStr;

#[allow(unused_imports)]
use crate::typecheck;
#[allow(unused_imports)]
use kai_ast::ast;
#[allow(unused_imports)]
use kai_parse::grammar::*;
#[allow(unused_imports)]
use std::fs;
#[allow(unused_imports)]
use std::io;

// const KAI_ARITHMETIC: &str = "basic/kai_arithmetic.kai";
//   "basic/kai_basic.kai",
//   "basic/kai_multi_variables.kai",
//   "basic/kai_varasgn_fail_redecl.kai",
//   "basic/kai_multi_variables.kai",
// ]

// let entries = fs::read_dir(example_dir)?
// .map(|res| res.map(|e| e.path()))
// .collect::<Result<Vec<_>, io::Error>>()?;

#[cached]
pub fn get_example_dir() -> Box<PathBuf> {
  let mut dir = PathBuf::from_str(env!("CARGO_MANIFEST_DIR")).unwrap();
  dir.pop();
  dir.push(Path::new("example").clone());

  Box::new(dir)
}

#[test]
fn test_kai_arithmetic() -> io::Result<()> {
  let mut dir = get_example_dir();
  (*dir).push("basic/kai_multi_variables.kai");

  let prog = fs::read_to_string(*dir)?;
  let parser = FunctionParser::new();
  let ast = parser.parse(prog.as_str()).unwrap();
  let var_ty_map = typecheck(&ast);

  assert!(*var_ty_map.get("x").unwrap() == ast::Type::Int);
  assert!(*var_ty_map.get("y").unwrap() == ast::Type::Int);
  assert!(*var_ty_map.get("z").unwrap() == ast::Type::Bool);
  assert!(*var_ty_map.get("z0").unwrap() == ast::Type::Bool);
  assert!(*var_ty_map.get("x1").unwrap() == ast::Type::Int);
  assert!(*var_ty_map.get("x2").unwrap() == ast::Type::Bool);
  assert!(*var_ty_map.get("x3").unwrap() == ast::Type::Bool);
  assert!(*var_ty_map.get("x4").unwrap() == ast::Type::Bool);

  Ok(())
}

#[test]
fn test_kai_fn_args() -> io::Result<()> {
  let mut dir = get_example_dir();
  (*dir).push("basic/kai_fn_args.kai");

  let prog = fs::read_to_string(*dir)?;
  let parser = FunctionParser::new();
  let ast = parser.parse(prog.as_str()).unwrap();
  let var_ty_map = typecheck(&ast);

  assert!(*var_ty_map.get("a").unwrap() == ast::Type::Int);
  assert!(*var_ty_map.get("b").unwrap() == ast::Type::Bool);

  Ok(())
}
