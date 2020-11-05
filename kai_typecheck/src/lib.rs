use kai_ast::ast;
use std::collections::HashMap;

pub mod tests;
pub mod typecheck;

struct TypeCheckCtx {
  fn_type_map: HashMap<String, ast::Type>,
  // var_map: HashMap<String, ast::Type>,
}

pub fn typecheck(ast_func: &ast::Function) -> Result<HashMap<String, ast::Type>, String> {
  let mut type_check_ctx = TypeCheckCtx {
    fn_type_map: HashMap::new(),
  };

  type_check_ctx.populate_fn_types(ast_func);
  type_check_ctx.typecheck_function(ast_func)
}
