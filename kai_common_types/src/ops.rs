#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Opcode {
  Add,
  Sub,
  Mul,
  Div,
  Mod,

  Lt,
  Leq,
  Gt,
  Geq,

  LogAnd,
  LogOr,
  LogEq,
  LogNeq,
}
