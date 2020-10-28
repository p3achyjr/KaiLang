use crate::TypeCheckCtx;
use kai_ast::ast;
use kai_common_types::ops::Opcode;
use std::collections::HashMap;

impl TypeCheckCtx {
  pub fn populate_fn_types(&mut self, ast_func: &ast::Function) {
    let func_args = &(*ast_func).args;
    self.fn_type_map.insert(
      ast_func.ident.clone(),
      ast::Type::FnType(
        func_args
          .into_iter()
          .map(|arg| Box::new((*arg).ty.clone()))
          .collect(),
        Box::new(ast_func.ret_ty.clone()),
      ),
    );
  }

  pub fn typecheck_function(&self, ast_func: &ast::Function) -> HashMap<String, ast::Type> {
    let mut var_ty_map = HashMap::new();
    for arg in &ast_func.args {
      var_ty_map.insert(arg.ident.clone(), arg.ty.clone());
    }
    let mut body_map = self.typecheck_stmt_list(&ast_func.body, &mut var_ty_map, &ast_func.ret_ty);
    body_map.extend(var_ty_map);
    body_map
  }

  fn typecheck_stmt_list(
    &self,
    stmts: &Vec<ast::Stmt>,
    var_ty_map: &mut HashMap<String, ast::Type>,
    ret_ty: &ast::Type,
  ) -> HashMap<String, ast::Type> {
    let mut current_scope_var_ty_map = HashMap::new();
    for stmt in stmts {
      match stmt {
        ast::Stmt::VarDecl(ident, expr) => {
          assert!(
            !var_ty_map.contains_key(ident),
            "variable ```{:?}``` already defined in scope",
            ident,
          );
          let e_ty = self.infer_expr_type(var_ty_map, expr);
          current_scope_var_ty_map.insert(ident.clone(), e_ty.clone());
          var_ty_map.insert((*ident).clone(), e_ty);
        }
        ast::Stmt::VarAsgn(ident, expr) => {
          assert!(
            var_ty_map.contains_key(ident),
            "trying to assign variable ```{:?}``` a value, but ```{:?}``` has not been defined",
            ident,
            ident,
          );
          let e_ty = self.infer_expr_type(var_ty_map, expr);
          let var_ty = var_ty_map.get(ident).unwrap_or(&ast::Type::Invalid);
          assert!(
            e_ty == *var_ty,
            "trying to assign variable ```{:?}``` a value of a different type than was defined",
            ident,
          );
        }
        ast::Stmt::If(c, b, e) => {
          let intersect_map = self.typecheck_if(var_ty_map, c, b, e, ret_ty);
          for (var, ty) in intersect_map.iter() {
            current_scope_var_ty_map.insert(var.clone(), ty.clone());
            var_ty_map.insert(var.clone(), ty.clone());
          }
        }
        ast::Stmt::Return(expr) => {
          let expr_type = self.infer_expr_type(var_ty_map, expr);
          assert!(
            expr_type == *ret_ty,
            "returning a value of a type ```{:?}``` that does not match function signature",
            expr_type,
          );
        }
      }
    }

    return current_scope_var_ty_map;
  }

  fn typecheck_if(
    &self,
    var_ty_map: &mut HashMap<String, ast::Type>,
    cond: &ast::Expr,
    if_stmts: &Vec<ast::Stmt>,
    else_if: &ast::ElseIf,
    ret_ty: &ast::Type,
  ) -> HashMap<String, ast::Type> {
    assert!(
      self.infer_expr_type(var_ty_map, cond) == ast::Type::Bool,
      "conditional in if block is not of type ```bool```",
    );
    let if_map = self.typecheck_stmt_list(if_stmts, var_ty_map, ret_ty);
    for (var, _) in if_map.iter() {
      // reset state
      var_ty_map.remove(var);
    }
    let else_map = match else_if {
      ast::ElseIf::Empty => HashMap::new(),
      ast::ElseIf::Else(else_stmts) => self.typecheck_stmt_list(else_stmts, var_ty_map, ret_ty),
      ast::ElseIf::ElseIf(c, b, e) => self.typecheck_if(var_ty_map, c, b, e, ret_ty),
    };
    for (var, _) in else_map.iter() {
      // reset state
      var_ty_map.remove(var);
    }

    let mut both_map = HashMap::new();
    for (var, ty1) in if_map.iter() {
      if !else_map.contains_key(var) {
        continue;
      }

      // var is in both maps
      let ty2 = else_map.get(var).unwrap();
      assert!(
        ty1 == ty2,
        "type for var ```{:?}``` do not match in if and else blocks",
        var
      );

      both_map.insert(var.clone(), ty1.clone());
    }

    println!("{:?}", both_map);

    both_map
  }

  fn infer_expr_type(
    &self,
    var_ty_map: &HashMap<String, ast::Type>,
    expr: &ast::Expr,
  ) -> ast::Type {
    match expr {
      ast::Expr::Num(_) => ast::Type::Int,
      ast::Expr::Bool(_) => ast::Type::Bool,
      ast::Expr::Ident(ident) => match var_ty_map.get(ident) {
        None => panic!("Variable ```{:?}``` does not have a type in scope", ident),
        Some(ty) => ty.clone(),
      },
      ast::Expr::Binop(op, e1_box, e2_box) => match op {
        Opcode::Add => self.infer_int_binop(var_ty_map, op, &**e1_box, &**e2_box),
        Opcode::Sub => self.infer_int_binop(var_ty_map, op, &**e1_box, &**e2_box),
        Opcode::Mul => self.infer_int_binop(var_ty_map, op, &**e1_box, &**e2_box),
        Opcode::Div => self.infer_int_binop(var_ty_map, op, &**e1_box, &**e2_box),
        Opcode::Mod => self.infer_int_binop(var_ty_map, op, &**e1_box, &**e2_box),

        Opcode::Lt => self.infer_cmp_binop(var_ty_map, op, &**e1_box, &**e2_box),
        Opcode::Leq => self.infer_cmp_binop(var_ty_map, op, &**e1_box, &**e2_box),
        Opcode::Gt => self.infer_cmp_binop(var_ty_map, op, &**e1_box, &**e2_box),
        Opcode::Geq => self.infer_cmp_binop(var_ty_map, op, &**e1_box, &**e2_box),

        Opcode::LogAnd => self.infer_log_binop(var_ty_map, op, &**e1_box, &**e2_box),
        Opcode::LogOr => self.infer_log_binop(var_ty_map, op, &**e1_box, &**e2_box),
        Opcode::LogEq => self.infer_poly_binop(var_ty_map, op, &**e1_box, &**e2_box),
        Opcode::LogNeq => self.infer_poly_binop(var_ty_map, op, &**e1_box, &**e2_box),
      },
    }
  }

  fn infer_int_binop(
    &self,
    var_ty_map: &HashMap<String, ast::Type>,
    opcode: &Opcode,
    expr1: &ast::Expr,
    expr2: &ast::Expr,
  ) -> ast::Type {
    return self.infer_binop(
      var_ty_map,
      opcode,
      expr1,
      expr2,
      ast::Type::Int, // expr1 type
      ast::Type::Int, // expr2 type
      ast::Type::Int, // expected return type
    );
  }

  fn infer_cmp_binop(
    &self,
    var_ty_map: &HashMap<String, ast::Type>,
    opcode: &Opcode,
    expr1: &ast::Expr,
    expr2: &ast::Expr,
  ) -> ast::Type {
    return self.infer_binop(
      var_ty_map,
      opcode,
      expr1,
      expr2,
      ast::Type::Int,  // expr1 type
      ast::Type::Int,  // expr2 type
      ast::Type::Bool, // expected return type
    );
  }

  fn infer_log_binop(
    &self,
    var_ty_map: &HashMap<String, ast::Type>,
    opcode: &Opcode,
    expr1: &ast::Expr,
    expr2: &ast::Expr,
  ) -> ast::Type {
    return self.infer_binop(
      var_ty_map,
      opcode,
      expr1,
      expr2,
      ast::Type::Bool, // expr1 type
      ast::Type::Bool, // expr2 type
      ast::Type::Bool, // expected return type
    );
  }

  fn infer_poly_binop(
    &self,
    var_ty_map: &HashMap<String, ast::Type>,
    opcode: &Opcode,
    expr1: &ast::Expr,
    expr2: &ast::Expr,
  ) -> ast::Type {
    let ty1 = self.infer_expr_type(var_ty_map, expr1);
    let ty2 = self.infer_expr_type(var_ty_map, expr2);

    if ty1 != ty2 {
      panic!(
        "Error when typechecking binary operand: lhs type does not match rhs {:?}, {:?}",
        ty1, ty2
      );
    }

    self.op_result_ty(opcode)
  }

  /*
   * Private, generic binop inference function
   *
   * @params:
   * - mapping from variables to types
   * - lhs expr
   * - rhs expr
   * - expected lhs type
   * - expected rhs type
   * - type to return if both sides typecheck
   */
  fn infer_binop(
    &self,
    var_ty_map: &HashMap<String, ast::Type>,
    opcode: &Opcode,
    expr1: &ast::Expr,
    expr2: &ast::Expr,
    ty1: ast::Type,
    ty2: ast::Type,
    ty_binop: ast::Type,
  ) -> ast::Type {
    let result1 = self.check_expr_type(var_ty_map, expr1, ty1);
    let result2 = self.check_expr_type(var_ty_map, expr2, ty2);

    match (result1, result2) {
      (Ok(_), Ok(_)) => ty_binop,
      _ => panic!("Wrong types to binary operand {:?}", opcode),
    }
  }

  fn check_expr_type(
    &self,
    var_ty_map: &HashMap<String, ast::Type>,
    expr: &ast::Expr,
    ty: ast::Type,
  ) -> Result<(), String> {
    let inferred_ty = self.infer_expr_type(var_ty_map, expr);
    // println!("{:?}, {:?}, {:?}", inferred_ty, ty, inferred_ty == ty);
    if inferred_ty == ty {
      return Ok(());
    }

    panic!(
      "Error when checking ```{:?}``` against type ```{:?}```",
      expr, ty
    );
  }

  fn op_result_ty(&self, opcode: &Opcode) -> ast::Type {
    match opcode {
      Opcode::Add => ast::Type::Int,
      Opcode::Sub => ast::Type::Int,
      Opcode::Mul => ast::Type::Int,
      Opcode::Div => ast::Type::Int,
      Opcode::Mod => ast::Type::Int,

      Opcode::Lt => ast::Type::Bool,
      Opcode::Leq => ast::Type::Bool,
      Opcode::Gt => ast::Type::Bool,
      Opcode::Geq => ast::Type::Bool,

      Opcode::LogAnd => ast::Type::Bool,
      Opcode::LogOr => ast::Type::Bool,
      Opcode::LogEq => ast::Type::Bool,
      Opcode::LogNeq => ast::Type::Bool,
    }
  }
}
