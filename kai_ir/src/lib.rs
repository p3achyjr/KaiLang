use kai_ast::ast;

pub mod ir;
pub mod ir_gen;

pub fn ir_gen(ast_func: &ast::Function) -> ir::IrFunction {
  let ir_context = ir_gen::IrGenContext {};
  ir_context.gen_ir_function(ast_func)
}
