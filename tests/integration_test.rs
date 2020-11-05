use std::{fs::File, io::Read, path::PathBuf, process::Command, str::FromStr};

use kai_ast;
use kai_common;
use kai_ir;
use kai_llvm_gen;
use kai_parse;
use kai_typecheck;
use regex::Regex;

fn test_compile(mut path: PathBuf) -> Result<(), String> {
  let mut file = File::open(&path).unwrap();
  let mut program = String::new();
  let re = Regex::new(r"(?P<e>@expect\(\d*\))|(?P<f>@fail)|(?P<i>@ignore)").unwrap();
  let expect_re = Regex::new(r"@expect\((?P<val>\d*)\)").unwrap();

  file.read_to_string(&mut program).unwrap();

  let caps = re.captures(&program).unwrap();
  let expect_cap = caps.name("e");
  let fail_cap = caps.name("f");
  let ignore_cap = caps.name("i");

  if let Some(_) = ignore_cap {
    return Ok(());
  }

  let prog = kai_common::remove_comments::remove_comments(&program).unwrap();

  let parser = kai_parse::grammar::FunctionParser::new();
  let ast = parser.parse(&prog).unwrap();
  let typecheck_result = kai_typecheck::typecheck(&ast);
  match typecheck_result {
    Ok(var_ty_map) => {
      if let None = expect_cap {
        return Err(format!("Expected failure, but got success on {:?}", path));
      }
      let ir = kai_ir::ir_gen(&ast, var_ty_map);
      unsafe {
        kai_llvm_gen::llvm_gen(&ir, &mut path);
      }

      let obj_stem = path.file_stem().unwrap().to_str().unwrap();
      let obj_name = format!("{}.o", obj_stem);
      path.pop();
      path.push(obj_name);
      Command::new("gcc")
        .arg(path.to_str().unwrap())
        .status()
        .expect("Could not invoke gcc");

      let expected_res = expect_re
        .captures(expect_cap.unwrap().to_owned().as_str())
        .unwrap()
        .name("val")
        .unwrap()
        .as_str()
        .parse::<i32>()
        .unwrap();
      let res = Command::new("./a.out")
        .status()
        .expect("Could not run executable");
      if res.code().unwrap() != expected_res {
        return Err(format!(
          "expected result does not match actual result.\nexpected: {:?}\nactual: {:?}\npath: {:?}",
          expected_res,
          res.code(),
          path,
        ));
      }
      return Ok(());
    }
    Err(_) => {
      if let Some(_) = fail_cap {
        return Ok(());
      }

      return Err(format!(
        "found failure parsing file {:?} when expecting success",
        path,
      ));
    }
  }
}

#[test]
fn test_basic() {
  let basic_path = "example/basic/";
  let mut dir = PathBuf::from_str(env!("CARGO_MANIFEST_DIR")).unwrap();
  dir.push(basic_path);

  for entry in dir.read_dir().expect("could not read basic dir") {
    if let Ok(entry) = entry {
      let path = entry.path();
      // println!("{:?}", path);
      // println!("{:?}", path.to_str().unwrap().ends_with(".kai"));
      if !path.to_str().unwrap().ends_with(".kai") {
        continue;
      }

      test_compile(path).unwrap();
    }
  }
}
