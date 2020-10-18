use kai_ast::ast;
use std::collections::HashMap;

pub mod tests;
pub mod typecheck;

pub fn typecheck(ast_func: &ast::Function) -> HashMap<String, ast::Type> {
  let mut type_check_ctx = typecheck::TypeCheckCtx {
    fn_type_map: HashMap::new(),
  };

  type_check_ctx.populate_fn_types(ast_func);
  type_check_ctx.typecheck_function(ast_func)
}
