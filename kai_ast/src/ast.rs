use std::fmt::{Debug, Error, Formatter};

#[derive(Debug)]
pub struct Function {
  pub ident: String,
  pub args: Vec<Box<FuncArg>>,
  pub body: Vec<Box<Stmt>>,
  pub ret_ty_ident: String,
}

#[derive(Debug)]
pub struct FuncArg {
  pub ty_ident: String,
  pub ident: String,
}

#[derive(Debug)]
pub enum Stmt {
  VarDecl(String, Box<Expr>),
  VarAsgn(String, Box<Expr>),
  Return(Box<Expr>),
}

#[derive(Debug)]
pub enum Expr {
  Num(String),
  Bool(bool),
  Ident(String),
  Binop(Opcode, Box<Expr>, Box<Expr>),
}

#[derive(Debug)]
pub enum Opcode {
  Add,
  Sub,
  Mul,
  Div,
}

#[derive(Debug)]
pub enum Type {
  Int,
  Bool,
  FnType(Vec<Box<Type>>, Box<Type>),
}
