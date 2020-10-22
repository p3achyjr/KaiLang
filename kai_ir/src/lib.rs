use kai_ast::ast;
use std::collections::HashMap;

pub mod cfg;
pub mod ir;
pub mod ir_gen;
pub mod ir_utils;

struct IrGenContext {
  tmp_count: i32,
  label_count: i32,
}

struct IrTempConversionContext {
  ident_tmp_map: HashMap<String, i32>,
  min_available_tmp: i32,
}

pub fn ir_gen(ast_func: &ast::Function) -> ir::IrFunction {
  let mut ir_context = IrGenContext {
    tmp_count: 0,
    label_count: 0,
  };
  let ir = ir_context.gen_ir_function(ast_func);
  let mut cfg = cfg::ControlFlowGraph {
    basic_blocks: &mut vec![],
  };
  cfg.build_cfg(&ir);
  println!("{}", cfg);

  ir
}

pub fn convert_vars_to_temps(ir: ir::IrFunction) {
  let mut tmp_conversion_ctx = IrTempConversionContext {
    ident_tmp_map: HashMap::new(),
    min_available_tmp: 0,
  };
  tmp_conversion_ctx.convert_vars_to_temps(ir);
}
