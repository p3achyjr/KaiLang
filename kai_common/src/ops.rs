use std::fmt;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Opcode {
  Add, // +
  Sub, // -
  Mul, // *
  Div, // /
  Mod, // %

  Lt,  // <
  Leq, // <=
  Gt,  // >
  Geq, // >=

  LogAnd, // &&
  LogOr,  // ||
  LogEq,  // ==
  LogNeq, // !=
}

pub trait IsShortCircuit {
  fn is_short_circuit(self) -> bool;
}

impl IsShortCircuit for Opcode {
  fn is_short_circuit(self) -> bool {
    match self {
      Opcode::LogAnd => true,
      Opcode::LogOr => true,
      _ => false,
    }
  }
}

impl fmt::Display for Opcode {
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    match self {
      Opcode::Add => write!(f, "+"),
      Opcode::Sub => write!(f, "-"),
      Opcode::Mul => write!(f, "*"),
      Opcode::Div => write!(f, "/"),
      Opcode::Mod => write!(f, "%"),

      Opcode::Lt => write!(f, "<"),
      Opcode::Leq => write!(f, "<"),
      Opcode::Gt => write!(f, ">"),
      Opcode::Geq => write!(f, ">"),

      Opcode::LogAnd => write!(f, "&"),
      Opcode::LogOr => write!(f, "||"),
      Opcode::LogEq => write!(f, "=="),
      Opcode::LogNeq => write!(f, "!="),
    }
  }
}
