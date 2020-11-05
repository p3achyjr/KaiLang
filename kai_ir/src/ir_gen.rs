use super::IrGenContext;
use crate::ir::*;
use kai_ast::ast;
use kai_ast::ast::IsLiteral;
use kai_common::ops::{IsShortCircuit, Opcode};

struct BlkGenResult {
  did_return: bool,
  cmds: Vec<IrCmd>,
}

impl IrGenContext {
  pub fn gen_ir_function(&mut self, ast_func: &ast::Function) -> IrFunction {
    IrFunction {
      ident: ast_func.ident.clone(),
      args: self.gen_ir_funcargs(&ast_func.args),
      body: self
        .gen_ir_body(&ast_func.body, self.gen_ir_type(&ast_func.ret_ty))
        .cmds,
      ret_ty: self.gen_ir_type(&ast_func.ret_ty),
    }
  }

  fn gen_ir_type(&self, ty: &ast::Type) -> IrType {
    match *ty {
      ast::Type::Int => IrType::Int,
      ast::Type::Bool => IrType::Bool,
      _ => panic!("cannot translate fn ast type to ir type"),
    }
  }

  fn gen_ir_funcargs(&self, ast_args: &Vec<ast::FuncArg>) -> Vec<IrFuncArg> {
    let mut ir_args = vec![];
    for arg in ast_args {
      ir_args.push(self.gen_ir_funcarg(&arg));
    }

    return ir_args;
  }

  fn gen_ir_funcarg(&self, arg: &ast::FuncArg) -> IrFuncArg {
    IrFuncArg {
      ty: self.gen_ir_type(&arg.ty),
      ident: IrVar::Ident(arg.ident.clone(), self.gen_ir_type(&arg.ty)),
    }
  }

  fn gen_ir_body(&mut self, stmts: &Vec<ast::Stmt>, ret_ty: IrType) -> BlkGenResult {
    let mut cmds = vec![];
    let mut did_return = false;
    for stmt in stmts {
      match stmt {
        ast::Stmt::VarDecl(ident, expr) => {
          let decl_cmds = self.gen_ir_expr_and_asgn(
            IrVar::Ident(
              ident.clone(),
              self.gen_ir_type(self.var_ty_map.get(ident).unwrap()),
            ),
            &expr,
          );
          cmds.extend(decl_cmds);
        }
        ast::Stmt::VarAsgn(ident, expr) => {
          let asgn_cmds = self.gen_ir_expr_and_asgn(
            IrVar::Ident(
              ident.clone(),
              self.gen_ir_type(self.var_ty_map.get(ident).unwrap()),
            ),
            &expr,
          );
          cmds.extend(asgn_cmds);
        }
        ast::Stmt::If(cond, blk, else_if) => {
          let if_result = self.gen_ir_if(cond, blk, else_if, ret_ty);
          cmds.extend(if_result.cmds);

          did_return = did_return || if_result.did_return;
        }
        ast::Stmt::Return(expr) => {
          // !("STATEMENTS: {:?}, RETURNSSSSS", stmts);
          if expr.is_literal() {
            cmds.push(IrCmd::Return(self.gen_ir_expr_for_lit(expr)));
            did_return = true;
            continue;
          }
          let ret_tmp = self.get_tmp_and_incr(ret_ty);
          let mut ret_cmds = self.gen_ir_expr_and_asgn(ret_tmp.clone(), &expr);
          ret_cmds.push(IrCmd::Return(lit_from_var(ret_tmp)));
          cmds.extend(ret_cmds);
          did_return = true;
        }
        ast::Stmt::Comment(_) => {}
      }
    }

    BlkGenResult { did_return, cmds }
  }

  // translate if-else block into list of commands w/ branches
  fn gen_ir_if(
    &mut self,
    cond: &ast::Expr,
    if_stmts: &Vec<ast::Stmt>,
    else_if: &ast::ElseIf,
    ret_ty: IrType,
  ) -> BlkGenResult {
    let mut cmds = vec![];
    let mut did_return = false;
    let cond_target = self.get_tmp_and_incr(IrType::Bool);
    let cond_cmds = self.gen_ir_expr_and_asgn(cond_target.clone(), cond);
    let if_label = self.get_label_and_incr();
    let else_label = self.get_label_and_incr();
    let join_label = self.get_label_and_incr();

    cmds.extend(cond_cmds);
    cmds.push(IrCmd::Cond(
      lit_from_var(cond_target.clone()),
      if_label,
      else_label,
    ));
    match else_if {
      ast::ElseIf::Empty => {
        cmds.push(IrCmd::Label(if_label));
        let if_res = self.gen_ir_body(if_stmts, ret_ty);
        cmds.extend(if_res.cmds);
        cmds.extend(if if_res.did_return {
          vec![]
        } else {
          vec![IrCmd::Goto(else_label)]
        });

        cmds.push(IrCmd::Label(else_label));
      }
      ast::ElseIf::Else(else_stmts) => {
        let if_res = self.gen_ir_body(if_stmts, ret_ty);
        let else_res = self.gen_ir_body(else_stmts, ret_ty);

        cmds.push(IrCmd::Label(if_label));
        cmds.extend(if_res.cmds);
        cmds.extend(if if_res.did_return {
          vec![]
        } else {
          vec![IrCmd::Goto(join_label)]
        });
        cmds.push(IrCmd::Label(else_label));
        cmds.extend(else_res.cmds);
        cmds.extend(if else_res.did_return {
          vec![]
        } else {
          vec![IrCmd::Goto(join_label)]
        });
        if if_res.did_return && else_res.did_return {
          did_return = true
        } else {
          // if both branches return, we don't need this
          cmds.push(IrCmd::Label(join_label));
          did_return = false
        }
      }
      ast::ElseIf::ElseIf(else_cond, else_if_stmts, else_if) => {
        let if_res = self.gen_ir_body(if_stmts, ret_ty);
        let else_res = self.gen_ir_if(else_cond, else_if_stmts, else_if, ret_ty);

        cmds.push(IrCmd::Label(if_label));
        cmds.extend(if_res.cmds);
        cmds.extend(if if_res.did_return {
          vec![]
        } else {
          vec![IrCmd::Goto(join_label)]
        });
        cmds.push(IrCmd::Label(else_label));
        cmds.extend(else_res.cmds);
        cmds.extend(if else_res.did_return {
          vec![]
        } else {
          vec![IrCmd::Goto(join_label)]
        });
        if if_res.did_return && else_res.did_return {
          did_return = true
        } else {
          // if both branches return, we don't need this
          cmds.push(IrCmd::Label(join_label));
          did_return = false
        }
      }
    }

    BlkGenResult { did_return, cmds }
  }

  // translate expression into list of commands, and assign it to target
  fn gen_ir_expr_and_asgn(&mut self, target: IrVar, expr: &ast::Expr) -> Vec<IrCmd> {
    match expr {
      ast::Expr::Num(n) => vec![IrCmd::Asgn(target, IrExpr::Literal(IrLiteral::Num(*n)))],
      ast::Expr::Bool(b) => vec![IrCmd::Asgn(target, IrExpr::Literal(IrLiteral::Bool(*b)))],
      ast::Expr::Ident(ident) => vec![IrCmd::Asgn(
        target,
        IrExpr::Literal(IrLiteral::Var(IrVar::Ident(
          ident.clone(),
          self.gen_ir_type(self.var_ty_map.get(ident).unwrap()),
        ))),
      )],
      ast::Expr::Binop(op, e1_box, e2_box) => {
        if op.is_short_circuit() {
          return self.gen_ir_expr_for_short_circuit(target, *op, e1_box, e2_box);
        }
        match (&**e1_box, &**e2_box) {
          (ast::Expr::Binop(op1, _, _), ast::Expr::Binop(op2, _, _)) => {
            // need to generate tmps for both
            let target1 = self.get_tmp_and_incr(gen_op_arg_type(*op1));
            let target2 = self.get_tmp_and_incr(gen_op_arg_type(*op2));
            let mut cmds = self.gen_ir_expr_and_asgn(target1.clone(), e1_box);
            cmds.extend(self.gen_ir_expr_and_asgn(target2.clone(), e2_box));
            cmds.push(IrCmd::Asgn(
              target,
              IrExpr::Binop(
                *op,
                lit_from_var(target1.clone()),
                lit_from_var(target2.clone()),
              ),
            ));
            cmds
          }
          (ast::Expr::Binop(op1, _, _), e2) => {
            // only needs temps for left side
            let target1 = self.get_tmp_and_incr(gen_op_arg_type(*op1));
            let mut cmds = self.gen_ir_expr_and_asgn(target1.clone(), &*e1_box);
            cmds.push(IrCmd::Asgn(
              target,
              IrExpr::Binop(
                *op,
                IrLiteral::Var(target1.clone()),
                self.gen_ir_expr_for_lit(&e2),
              ),
            ));
            cmds
          }
          (e1, ast::Expr::Binop(op2, _, _)) => {
            // only needs temps for left side
            let target1 = self.get_tmp_and_incr(gen_op_arg_type(*op2));
            let mut cmds = self.gen_ir_expr_and_asgn(target1.clone(), &*e2_box);
            cmds.push(IrCmd::Asgn(
              target,
              IrExpr::Binop(
                *op,
                IrLiteral::Var(target1.clone()),
                self.gen_ir_expr_for_lit(&e1),
              ),
            ));
            cmds
          }
          // should both be literal values
          (e1, e2) => vec![IrCmd::Asgn(
            target,
            IrExpr::Binop(
              *op,
              self.gen_ir_expr_for_lit(&e1),
              self.gen_ir_expr_for_lit(&e2),
            ),
          )],
        }
      }
    }
  }

  fn gen_ir_expr_for_short_circuit(
    &mut self,
    target: IrVar,
    op: Opcode,
    e1: &ast::Expr,
    e2: &ast::Expr,
  ) -> Vec<IrCmd> {
    let cmds1 = self.gen_ir_expr_and_asgn(target.clone(), e1);
    let cmds2 = self.gen_ir_expr_and_asgn(target.clone(), e2);

    let (label1, label2) = (self.get_label_and_incr(), self.get_label_and_incr());

    /*
     * Determines which block to jump to
     * if we are in && case, we should evaluate e2 is tmp1 is true, and skip if
     * tmp1 is false.
     *
     * if we are in || case, we do the opposite.
     */
    let cond = if op == Opcode::LogAnd {
      IrCmd::Cond(IrLiteral::Var(target.clone()), label1, label2)
    } else {
      IrCmd::Cond(IrLiteral::Var(target.clone()), label2, label1)
    };
    match op {
      Opcode::LogAnd => {
        let mut cmds = cmds1;
        // jump to label1 if tmp1 is true, label2 o/w
        cmds.extend(vec![cond, IrCmd::Label(label1)]);
        cmds.extend(cmds2);
        cmds.push(IrCmd::Goto(label2));
        cmds.extend(vec![
          // value depends solely on tmp2
          IrCmd::Label(label2),
          // we reach here only if tmp1 is false, so result of computation is false
        ]);
        cmds
      }
      Opcode::LogOr => {
        // similar logic, but inverted
        let mut cmds = cmds1;
        cmds.extend(vec![cond, IrCmd::Label(label1)]);
        cmds.extend(cmds2);
        cmds.push(IrCmd::Goto(label2));
        cmds.extend(vec![
          // value depends solely on tmp2
          IrCmd::Label(label2),
        ]);
        cmds
      }
      _ => panic!("calling short circuit elab on non-short circuit expr"),
    }
  }

  /*
   * translate expression into list of commands when expr is simple
   * SHOULD NOT be called directly
   */
  fn gen_ir_expr_for_lit(&self, expr: &ast::Expr) -> IrLiteral {
    assert!(
      expr.is_literal(),
      "called gen_ir_expr_for_lit on non-lit value {:?}",
      *expr,
    );

    match expr {
      ast::Expr::Num(n) => IrLiteral::Num(*n),
      ast::Expr::Bool(b) => IrLiteral::Bool(*b),
      ast::Expr::Ident(ident) => IrLiteral::Var(IrVar::Ident(
        ident.clone(),
        self.gen_ir_type(self.var_ty_map.get(ident).unwrap()),
      )),
      _ => panic!("impossible case ```gen_ir_expr_for_lit```, should have caught in assert"),
    }
  }

  fn get_tmp_and_incr(&mut self, ty: IrType) -> IrVar {
    let tmp = IrVar::Temp(self.tmp_count, ty, 0);
    self.tmp_count += 1;
    tmp
  }

  fn get_label_and_incr(&mut self) -> IrLabel {
    let label = IrLabel {
      label: self.label_count,
    };
    self.label_count += 1;
    label
  }
}
