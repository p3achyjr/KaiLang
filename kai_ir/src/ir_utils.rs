use crate::{ir::*, IrTempConversionContext};

impl IrTempConversionContext {
  pub fn convert_vars_to_temps(&mut self, ir: IrFunction) -> IrFunction {
    let ir_cmds = &ir.body;

    for cmd in ir_cmds {
      match cmd {
        IrCmd::Asgn(IrVar::Temp(n), _) => {
          self.min_available_tmp = if *n >= self.min_available_tmp {
            n + 1
          } else {
            self.min_available_tmp
          };
        }
        _ => continue,
      }
    }

    IrFunction {
      ident: ir.ident,
      args: self.convert_vars_to_temps_args(ir.args),
      body: self.convert_vars_to_temps_body(ir.body),
      ret_ty: ir.ret_ty,
    }
  }

  fn convert_vars_to_temps_args(&mut self, args: Vec<IrFuncArg>) -> Vec<IrFuncArg> {
    let mut new_args = vec![];
    for arg in args {
      match arg.ident {
        IrVar::Ident(s) => {
          self.ident_tmp_map.insert(s, self.min_available_tmp);
          new_args.push(IrFuncArg {
            ty: arg.ty,
            ident: IrVar::Temp(self.min_available_tmp),
          });
          self.min_available_tmp += 1;
        }
        _ => panic!("No fn args should be temps before conversion"),
      }
    }

    new_args
  }

  fn convert_vars_to_temps_body(&mut self, cmds: Vec<IrCmd>) -> Vec<IrCmd> {
    let mut new_cmds = vec![];
    for cmd in cmds {
      match cmd {
        IrCmd::Asgn(IrVar::Ident(s), expr) => {
          self.ident_tmp_map.insert(s, self.min_available_tmp);
          new_cmds.push(IrCmd::Asgn(
            IrVar::Temp(self.min_available_tmp),
            self.convert_vars_to_temps_expr(expr),
          ));
          self.min_available_tmp += 1;
        }
        IrCmd::Cond(IrLiteral::Var(IrVar::Ident(s)), l1, l2) => {
          new_cmds.push(IrCmd::Cond(
            IrLiteral::Var(IrVar::Temp(*self.ident_tmp_map.get(&s).unwrap())),
            l1,
            l2,
          ));
        }
        IrCmd::Return(expr) => new_cmds.push(IrCmd::Return(self.convert_vars_to_temps_expr(expr))),
        _ => new_cmds.push(cmd),
      }
    }

    new_cmds
  }

  fn convert_vars_to_temps_expr(&mut self, expr: IrExpr) -> IrExpr {
    match expr {
      IrExpr::Literal(lit) => IrExpr::Literal(self.convert_vars_to_temps_lit(lit)),
      IrExpr::Binop(op, lit1, lit2) => IrExpr::Binop(
        op,
        self.convert_vars_to_temps_lit(lit1),
        self.convert_vars_to_temps_lit(lit2),
      ),
    }
  }

  fn convert_vars_to_temps_lit(&mut self, lit: IrLiteral) -> IrLiteral {
    match lit {
      IrLiteral::Var(IrVar::Ident(s)) => {
        IrLiteral::Var(IrVar::Temp(*self.ident_tmp_map.get(&s).unwrap()))
      }
      _ => lit,
    }
  }
}
