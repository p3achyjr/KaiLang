use rust_ast::ast;
use std::collections::HashMap;
use std::iter::Map;

impl TypeChecker {
  // fn coerce_type_ident_into_type(ty_ident: &String) -> ast::Type {
  //   match ty_ident {
  //     "int" => ast::Type
  //   }
  // }
  
  fn populate_fn_types(&self, ast_func: ast::Function) {
    &self.fn_type_map.insert(
      ast_func.ident,
      ast::Type::FnType(
        ast_func.args.into_iter().map(|arg| coerce_type_ident_into_type(arg.ty_ident)),
      )
    );
  }

  fn typecheck_function(&self, ast_func: ast::Function) -> Result<ast::Function, String> {
    ast::Function {
      ident: ast_func.ident,
      args: ast_func.args,
      body: typecheck_stmt_list(ast_func.body),
      ret_ty_ident: ast_func.args,
    };
  }

  fn typecheck_stmt_list(&self, stmts: Vec<Box<ast::Stmt>>) -> HashMap<String, ast::Type> {
    let stmts_typed = vec![];
    for stmt_box in ast_fn_body {
      let stmt = *stmt_box;
      
    }
    ast_fn_body.into_iter().map(
      |x| match *stmt {
        ast::VarDecl(ident, expr, ty) => 
        ast::
      }
    );
  }
}
