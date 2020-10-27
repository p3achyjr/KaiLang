use std::collections::HashMap;

use crate::{ir::*, IrTempConversionContext};

impl IrTempConversionContext {
  pub fn convert_vars_to_temps(&mut self, ir: IrFunction) -> IrFunction {
    let ir_cmds = &ir.body;

    // get maximum temp number before replacing ident vars
    for cmd in ir_cmds {
      match cmd {
        IrCmd::Asgn(IrVar::Temp(n, _, _), _) => {
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
        IrVar::Ident(s, ty) => {
          self.ident_tmp_map.insert(s, self.min_available_tmp);
          new_args.push(IrFuncArg {
            ty: arg.ty,
            ident: IrVar::Temp(self.min_available_tmp, ty, 0),
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
        IrCmd::Asgn(IrVar::Ident(s, ty), expr) => {
          let tmp_number = if self.ident_tmp_map.contains_key(&s) {
            *self.ident_tmp_map.get(&s).unwrap()
          } else {
            self.ident_tmp_map.insert(s, self.min_available_tmp);
            self.min_available_tmp
          };
          new_cmds.push(IrCmd::Asgn(
            IrVar::Temp(tmp_number, ty, 0),
            self.convert_vars_to_temps_expr(expr),
          ));
          self.min_available_tmp += 1;
        }
        IrCmd::Asgn(tmp, expr) => {
          new_cmds.push(IrCmd::Asgn(tmp, self.convert_vars_to_temps_expr(expr)));
          self.min_available_tmp += 1;
        }
        IrCmd::Cond(IrLiteral::Var(IrVar::Ident(s, ty)), l1, l2) => {
          new_cmds.push(IrCmd::Cond(
            IrLiteral::Var(IrVar::Temp(*self.ident_tmp_map.get(&s).unwrap(), ty, 0)),
            l1,
            l2,
          ));
        }
        IrCmd::Return(lit) => new_cmds.push(IrCmd::Return(self.convert_vars_to_temps_lit(lit))),
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
      IrExpr::Phi(phis) => IrExpr::Phi(phis),
    }
  }

  fn convert_vars_to_temps_lit(&mut self, lit: IrLiteral) -> IrLiteral {
    match lit {
      IrLiteral::Var(IrVar::Ident(s, ty)) => {
        IrLiteral::Var(IrVar::Temp(*self.ident_tmp_map.get(&s).unwrap(), ty, 0))
      }
      _ => lit,
    }
  }
}

pub fn get_tmp_to_type_map(ir: &IrFunction) -> HashMap<i32, IrType> {
  let mut tmp_to_ty_map = HashMap::new();
  let cmds = &ir.body;

  for cmd in cmds {
    match cmd {
      IrCmd::Asgn(IrVar::Temp(var, ty, _), _) => {
        tmp_to_ty_map.insert(*var, *ty);
      }
      IrCmd::Asgn(IrVar::Ident(_, _), _) => {
        panic!("Do not call get_tmp_to_type_map before calling convert_vars_to_tmps")
      }
      _ => (),
    }
  }

  tmp_to_ty_map
}
