use kai_ast::ast;
use kai_common::ops::Opcode;
use std::fmt;
use std::fmt::Debug;

#[derive(Debug)]
pub struct IrFunction {
  pub ident: String,
  pub args: Vec<IrFuncArg>,
  pub body: Vec<IrCmd>,
  pub ret_ty: IrType,
}

#[derive(Debug)]
pub struct IrFuncArg {
  pub ty: IrType,
  pub ident: IrVar,
}

#[derive(Debug, Clone)]
pub enum IrCmd {
  Asgn(IrVar, IrExpr),
  Label(IrLabel), // keep a counter
  Goto(IrLabel),
  Cond(IrLiteral, IrLabel, IrLabel),
  Return(IrLiteral),
}

#[derive(Debug, Clone)]
pub enum IrExpr {
  Phi(Vec<(IrVar, IrLabel)>),
  Literal(IrLiteral),
  Binop(Opcode, IrLiteral, IrLiteral),
}

#[derive(Debug, Clone)]
pub enum IrLiteral {
  Num(i32),
  Bool(bool),
  Var(IrVar),
}

#[derive(Debug, PartialEq, Eq, Clone, Hash)]
pub enum IrVar {
  Ident(String, IrType),
  // ident, version (ssa)
  Temp(i32, IrType, usize),
}

#[derive(Debug, PartialEq, Eq, Clone, Copy, Hash)]
pub enum IrType {
  Int,
  Bool,
  Addr, // addresses
}

#[derive(Debug, PartialEq, Eq, Clone, Copy, Hash)]
pub struct IrLabel {
  pub label: i32,
}

pub fn lit_from_var(var: IrVar) -> IrLiteral {
  return IrLiteral::Var(var);
}

pub fn expr_from_var(var: IrVar) -> IrExpr {
  return IrExpr::Literal(IrLiteral::Var(var));
}

pub fn cmd_from_label(label: IrLabel) -> IrCmd {
  return IrCmd::Label(label);
}

pub fn expr_from_lit(lit: IrLiteral) -> IrExpr {
  return IrExpr::Literal(lit);
}

pub fn gen_op_arg_type(op: Opcode) -> IrType {
  match op {
    Opcode::Add => IrType::Int,
    Opcode::Sub => IrType::Int,
    Opcode::Mul => IrType::Int,
    Opcode::Div => IrType::Int,
    Opcode::Mod => IrType::Int,
    Opcode::Lt => IrType::Int,
    Opcode::Leq => IrType::Int,
    Opcode::Gt => IrType::Int,
    Opcode::Geq => IrType::Int,
    Opcode::LogAnd => IrType::Bool,
    Opcode::LogOr => IrType::Bool,
    Opcode::LogEq => IrType::Bool,
    Opcode::LogNeq => IrType::Bool,
  }
}

pub fn gen_op_result_type(op: Opcode) -> IrType {
  match op {
    Opcode::Add => IrType::Int,
    Opcode::Sub => IrType::Int,
    Opcode::Mul => IrType::Int,
    Opcode::Div => IrType::Int,
    Opcode::Mod => IrType::Int,
    Opcode::Lt => IrType::Bool,
    Opcode::Leq => IrType::Bool,
    Opcode::Gt => IrType::Bool,
    Opcode::Geq => IrType::Bool,
    Opcode::LogAnd => IrType::Bool,
    Opcode::LogOr => IrType::Bool,
    Opcode::LogEq => IrType::Bool,
    Opcode::LogNeq => IrType::Bool,
  }
}

impl fmt::Display for IrFunction {
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    let mut arg_tokens = vec![];
    let mut stmt_tokens = vec![];
    for arg in &self.args {
      arg_tokens.push(arg.to_string());
    }
    for i in 0..self.body.len() {
      stmt_tokens.push(format!("{}: {}", i, self.body[i].to_string()));
    }
    return write!(
      f,
      "{}({}): {}\n{}",
      (*self).ident.to_string(),
      arg_tokens.join("\n"),
      (*self).ret_ty,
      stmt_tokens.join("\n"),
    );
  }
}

impl fmt::Display for IrFuncArg {
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    return write!(f, "{}: {}", ((*self).ident).to_string(), (*self).ty);
  }
}

impl fmt::Display for IrCmd {
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    match self {
      IrCmd::Asgn(v, e) => write!(f, "{} = {}", v, e),
      IrCmd::Label(l) => write!(f, "{}", l),
      IrCmd::Goto(l) => write!(f, "goto {}", l),
      IrCmd::Cond(v, l1, l2) => write!(f, "branch {}: {}, {}", v, l1, l2),
      IrCmd::Return(e) => write!(f, "ret {}", e),
    }
  }
}

impl fmt::Display for IrExpr {
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    match self {
      IrExpr::Literal(l) => write!(f, "{}", l),
      IrExpr::Binop(o, l, r) => write!(f, "{} {} {}", l, o, r),
      IrExpr::Phi(phis) => write!(f, "phi({:?})", phis),
    }
  }
}

impl fmt::Display for IrLiteral {
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    match self {
      IrLiteral::Num(n) => write!(f, "{}", n),
      IrLiteral::Bool(b) => write!(f, "{}", b),
      IrLiteral::Var(v) => write!(f, "{}", v),
    }
  }
}

impl fmt::Display for IrVar {
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    match self {
      IrVar::Ident(ident, ty) => write!(f, "{}({})", ident, ty),
      IrVar::Temp(n, ty, v) => write!(f, "t{}_{}({})", n, v, ty),
    }
  }
}

impl fmt::Display for IrType {
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    match self {
      IrType::Int => write!(f, "int"),
      IrType::Bool => write!(f, "bool"),
      IrType::Addr => write!(f, "addr"),
    }
  }
}

impl fmt::Display for IrLabel {
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    write!(f, ".L{}", self.label)
  }
}
