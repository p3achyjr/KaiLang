use kai_common_types::ops::Opcode;
use std::fmt::Debug;

#[derive(Debug)]
pub struct Function {
  pub ident: String,
  pub args: Vec<FuncArg>,
  pub body: Vec<Stmt>,
  pub ret_ty: Type,
}

#[derive(Debug)]
pub struct FuncArg {
  pub ty: Type,
  pub ident: String,
}

#[derive(Debug)]
pub enum Stmt {
  VarDecl(String, Expr),
  VarAsgn(String, Expr),
  Return(Expr),
}

#[derive(Debug)]
pub enum Expr {
  Num(i32),
  Bool(bool),
  Ident(String),
  Binop(Opcode, Box<Expr>, Box<Expr>),
}

#[derive(Debug, PartialEq, Eq, Clone)]
pub enum Type {
  Invalid, // used to denote an error in the typechecker (e.g. type of undeclared variable)
  Int,
  Bool,
  FnType(Vec<Box<Type>>, Box<Type>),
}

impl Default for Type {
  fn default() -> Self {
    Type::Invalid
  }
}

pub trait IsLiteral {
  fn is_literal(&self) -> bool;
}

impl IsLiteral for Expr {
  fn is_literal(&self) -> bool {
    match *self {
      Expr::Num(_) => true,
      Expr::Bool(_) => true,
      Expr::Ident(_) => true,
      _ => false,
    }
  }
}
