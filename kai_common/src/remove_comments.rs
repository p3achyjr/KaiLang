pub fn remove_comments(prog: &String) -> Result<String, String> {
  let mut elided_prog = "".to_string();
  let mut unmatched_block_comment = false;

  let mut prog_iter = prog.chars().peekable();
  while let Some(c0) = prog_iter.next() {
    let c1 = prog_iter.peek();
    if c1 == None {
      elided_prog.push(c0);
    }
    match (c0, c1) {
      ('/', Some('/')) => {
        // skip all characters until the next line
        let mut chr_in_comment = prog_iter.next();
        while chr_in_comment != Some('\n') && chr_in_comment != None {
          chr_in_comment = prog_iter.next();
        }
      }
      ('/', Some('*')) => {
        // skip all characters until we find the end brace
        prog_iter.next();
        let mut chr0 = prog_iter.next();
        loop {
          if chr0 == Some('*') && prog_iter.peek() == Some(&'/') {
            prog_iter.next();
            break;
          }
          if chr0 == None {
            unmatched_block_comment = true;
            break;
          }
          chr0 = prog_iter.next();
        }
      }
      ('*', Some('/')) => {
        // unmatched end brace
        return Err("unmatched closed comment brace".to_string());
      }
      _ => elided_prog.push(c0),
    }
  }

  if unmatched_block_comment {
    return Err("unmatched open comment brace".to_string());
  }

  Ok(elided_prog)
}
