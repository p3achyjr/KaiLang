use kai_ast::ast;

pub mod ir;
pub mod ir_gen;

struct IrGenContext {
  tmp_count: i32,
  label_count: i32,
}

pub fn ir_gen(ast_func: &ast::Function) -> ir::IrFunction {
  let mut ir_context = IrGenContext {
    tmp_count: 0,
    label_count: 0,
  };
  ir_context.gen_ir_function(ast_func)
}
