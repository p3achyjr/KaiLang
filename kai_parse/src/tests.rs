use crate::grammar::*;

fn test_expect_success(prog: &str) {
  let parser = FunctionParser::new();
  match parser.parse(prog) {
    Ok(_) => (),
    Err(e) => panic!(format!(
      "Failed in parsing {:?}, original error: {:?}",
      prog, e,
    )),
  }
}

fn test_expect_fail(prog: &str) {
  let parser = FunctionParser::new();
  match parser.parse(prog) {
    Err(_) => (),
    Ok(_) => panic!(format!("Succeeded in parsing {:?}, should fail", prog)),
  }
}

#[test]
fn test_basic() {
  test_expect_success("function main(): int {}");
}

#[test]
fn test_basic_fail() {
  test_expect_fail("function main(: int {}");
}

#[test]
fn test_fn_args() {
  test_expect_success("function main(x: int): int {}");
  test_expect_success("function main(x: int,): int {}");
  test_expect_success("function main(x111x1: int,): int {}");
  test_expect_success("function main(x: int, y: String, z: object): int {}");
  test_expect_success("function main(x: int, y: String, z: object,): int {}");
  test_expect_success("function main(___x: int, y: String,): int {}");

  test_expect_fail("function main(x: 2int,): int {}");
  test_expect_fail("function main(2x: int,): int {}");
  test_expect_fail("function main(x22: 6nt,     ): int {}");
  test_expect_fail("function main(, x: int,): int {}");
  test_expect_fail("function main(, x: int,     ): int {}");
}

#[test]
fn test_stmts() {
  test_expect_success("function main(): int { let x = 5; }");
  test_expect_success("function main(): int { let x = 5; x = 5; }");
  test_expect_success("function main(): int { let x = y; }");
  test_expect_success("function main(): int { let x = y + z; }");
  test_expect_success("function main(): int { let x = y + z * 3; }");

  test_expect_fail("function main(): int { 5; }");
  test_expect_fail("function main(): int { 5 + 5; }");
  test_expect_fail("function main(): int { let x = 5 }");
}
