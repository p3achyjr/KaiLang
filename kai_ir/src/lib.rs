use kai_ast::ast;
use std::collections::HashMap;

pub mod cfg;
pub mod ir;
pub mod ir_gen;
pub mod ir_utils;
pub mod ssa;

struct IrGenContext {
  tmp_count: i32,
  label_count: i32,
  var_ty_map: HashMap<String, ast::Type>,
}

struct IrTempConversionContext {
  ident_tmp_map: HashMap<String, i32>,
  min_available_tmp: i32,
}

pub fn ir_gen(ast_func: &ast::Function, var_ty_map: HashMap<String, ast::Type>) -> ir::IrFunction {
  let mut ir_context = IrGenContext {
    tmp_count: 0,
    label_count: 0,
    var_ty_map,
  };
  let ir = ir_context.gen_ir_function(ast_func);
  // println!("{}", ir);
  let mut cfg = cfg::ControlFlowGraph {
    basic_blocks: &mut vec![],
  };
  let (mut ir_temps, tmp_count) = convert_vars_to_temps(ir);
  // cfg.build_cfg(&ir_temps);
  // println!("{}", cfg);
  // println!("ir_temps: {}", ir_temps);

  // let tmp_ty_map = ir_utils::get_tmp_to_type_map(&ir_temps);
  // ssa::gen_ssa(&mut ir_temps, &mut cfg, &tmp_ty_map, tmp_count as usize);

  ir_temps
}

pub fn convert_vars_to_temps(ir: ir::IrFunction) -> (ir::IrFunction, i32) {
  let mut tmp_conversion_ctx = IrTempConversionContext {
    ident_tmp_map: HashMap::new(),
    min_available_tmp: 0,
  };
  (
    tmp_conversion_ctx.convert_vars_to_temps(ir),
    tmp_conversion_ctx.min_available_tmp,
  )
}
