use crate::remove_comments;

fn test_remove_comments(prog: &str, expected_prog: &str) {
  let res = remove_comments::remove_comments(prog.to_string());
  match res {
    Err(s) => panic!(
      "Expected removing comments to succeed
      error message: {:?}",
      s,
    ),
    Ok(actual_prog) => assert!(
      actual_prog == expected_prog,
      "program after removing comments does not match expected result
      original: `{:?}`
      expected: `{:?}`
      actual:   `{:?}`",
      prog,
      expected_prog,
      actual_prog,
    ),
  }
}

fn test_remove_comments_fail(prog: &str, expected_prog: &str) {
  let res = remove_comments::remove_comments(prog.to_string());
  match res {
    Err(_) => (),
    Ok(actual_prog) => panic!(
      "program succeeded in parsing, but should fail
      original: `{:?}`
      expected: `{:?}`
      actual:   `{:?}`",
      prog, expected_prog, actual_prog,
    ),
  }
}

#[test]
fn test_remove_comments_simple() {
  let prog = "//";
  let elided = "";

  test_remove_comments(prog, elided);
}

#[test]
fn test_remove_block_comment() {
  let prog = "/* i am a comment */";
  let elided = "";

  test_remove_comments(prog, elided);
}

#[test]
fn test_remove_block_comment_fail() {
  let prog = "/* i am a comment */*/";
  let elided = "";

  test_remove_comments_fail(prog, elided);
}

#[test]
fn test_remove_line_comments() {
  let prog = "
// this function does something */ should not throw
function main(): int {
  return 5; // this should also get removed
  // this whole line should be removed
}
// this should also get removed /* this should not count";
  let elided = "
function main(): int {
  return 5;   }
";

  test_remove_comments(prog, elided);
}

#[test]
fn test_remove_block_comments() {
  let prog = "
/*
 * fake docblock
 * @everything should get removed //including these
 // this should not count as a line comment
 * we allow arbitrary /* /* /*
 * so long as there is a closing 
 */
function main(): int {
  int y = /* we can insert comments here */ 6;
  return /* or here */ 5; /* block in function */
}
/* this should get removed */
";
  let elided = "

function main(): int {
  int y =  6;
  return  5; 
}


";

  test_remove_comments(prog, elided);
}
