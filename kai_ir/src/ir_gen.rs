use crate::ir::*;
use kai_ast::ast;
use kai_common_types::ops::Opcode;

pub struct IrGenContext {}

impl IrGenContext {
  pub fn gen_ir_function(&self, ast_func: &ast::Function) -> IrFunction {
    IrFunction {
      ident: ast_func.ident.clone(),
      args: ast_func
        .args
        .into_iter()
        .map(|arg| self.gen_ir_funcarg(arg))
        .collect(),
      body: self.gen_ir_body(ast_func.body),
      ret_ty: self.gen_ir_type(ast_func.ret_ty),
    }
  }

  fn gen_ir_type(&self, ty: &ast::Type) -> IrType {
    match *ty {
      ast::Type::Int => IrType::Int,
      ast::Type::Bool => IrType::Bool,
      _ => panic!("cannot translate fn ast type to ir type"),
    }
  }

  fn gen_ir_funcarg(&self, arg: &ast::FuncArg) -> IrFuncArg {
    IrFuncArg {
      ty: self.gen_ir_type(arg.ty),
      ident: arg.ident.clone(),
    }
  }

  fn gen_ir_body(stmts: &Vec<ast::Stmt>) -> Vec<IrCmd> {
    return vec![];
  }
}
