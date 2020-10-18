use kai_common_types::ops::Opcode;
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
  pub ident: String,
}

#[derive(Debug)]
pub enum IrCmd {
  Asgn(IrVar, IrExpr),
  Return(IrExpr),
}

#[derive(Debug)]
pub enum IrExpr {
  Binop(Opcode, IrLiteral, IrLiteral),
}

#[derive(Debug)]
pub enum IrLiteral {
  Num(i32),
  Bool(bool),
  Ident(String),
}

#[derive(Debug)]
pub enum IrVar {
  Ident(String),
  Temp(i32),
}

#[derive(Debug, PartialEq, Eq, Clone)]
pub enum IrType {
  Int,
  Bool,
  Addr, // addresses
}
