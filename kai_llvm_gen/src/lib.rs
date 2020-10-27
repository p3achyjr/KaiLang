extern crate llvm_sys as llvm;

use std::os::raw::c_ulong;
use std::{collections::HashMap, path::Path};
use std::{
  ffi::{CStr, CString},
  path::PathBuf,
};

use llvm::core::*;
use llvm::*;

use kai_common_types::ops::Opcode;
use kai_ir::ir::*;

struct KaiLlvmResult {
  llctx: *mut LLVMContext,
  llmodule: *mut LLVMModule,
  llbuilder: *mut LLVMBuilder,
  ll_tmp_counter: usize,
  var_to_alloca: HashMap<IrVar, *mut LLVMValue>,
  label_to_block: HashMap<IrLabel, *mut LLVMBasicBlock>,
}

impl KaiLlvmResult {
  pub unsafe fn gen_llvm_ir(&mut self, ir: &IrFunction) -> &KaiLlvmResult {
    let ret_ty = self.ir_to_ll_type(ir.ret_ty);
    let mut arg_tys: Vec<*mut LLVMType> = (&ir.args)
      .into_iter()
      .map(|arg| self.ir_to_ll_type(arg.ty))
      .collect();

    let fn_type = LLVMFunctionType(ret_ty, arg_tys.as_mut_ptr(), arg_tys.len() as u32, 0);
    let ll_function = llvm::core::LLVMAddFunction(self.llmodule, c_str(&ir.ident), fn_type);

    let alloca_bb = LLVMAppendBasicBlockInContext(
      self.llctx,
      ll_function,
      c_str(&["entry", ir.ident.as_str()].join("_")),
    );
    LLVMPositionBuilderAtEnd(self.llbuilder, alloca_bb);

    // allocate all fn args
    for i in 0..ir.args.len() {
      self.new_alloca(&ir.args[i].ident, ll_function);
    }

    // let entry_bb = LLVMAppendBasicBlockInContext(
    //   self.llctx,
    //   ll_function,
    //   c_str(&["entry", ir.ident.as_str()].join("_")),
    // );
    // LLVMPositionBuilderAtEnd(self.llbuilder, entry_bb);
    // then store all fn args into allocated slots
    for i in 0..ir.args.len() {
      let alloca_for_arg = self.var_to_alloca.get(&ir.args[i].ident).unwrap();
      LLVMBuildStore(
        self.llbuilder,
        LLVMGetParam(ll_function, i as u32),
        *alloca_for_arg,
      );
    }

    self.gen_llvm_body(&ir.body, ll_function);

    self
  }

  unsafe fn gen_llvm_body(&mut self, ir_cmds: &Vec<IrCmd>, ll_function: *mut LLVMValue) {
    let mut current_bb = LLVMGetEntryBasicBlock(ll_function);
    LLVMPositionBuilderAtEnd(self.llbuilder, current_bb);

    for cmd in ir_cmds {
      match cmd {
        IrCmd::Asgn(var, expr) => self.gen_llvm_asgn(var, expr, ll_function),
        IrCmd::Label(l) => {
          current_bb = self.get_or_create_bb(l, ll_function);
          LLVMPositionBuilderAtEnd(self.llbuilder, current_bb);
        }
        IrCmd::Goto(l) => {
          let bb = self.get_or_create_bb(l, ll_function);
          LLVMBuildBr(self.llbuilder, bb);
        }
        IrCmd::Cond(c, l1, l2) => {
          let bb1 = self.get_or_create_bb(l1, ll_function);
          let bb2 = self.get_or_create_bb(l2, ll_function);
          LLVMBuildCondBr(self.llbuilder, self.gen_llvm_lit(c, ll_function), bb1, bb2);
        }
        IrCmd::Return(lit) => {
          LLVMBuildRet(self.llbuilder, self.gen_llvm_lit(lit, ll_function));
        }
      }
    }
  }

  // unsafe fn gen_llvm_basic_block(
  //   &mut self,
  //   cfg: &ControlFlowGraph,
  //   bb_index: usize,
  //   ll_bbs: Vec<*mut LLVMBasicBlock>,
  // ) {
  //   let bb = cfg.at(bb_index);
  //   LLVMPositionBuilderAtEnd(self.llbuilder, ll_bbs[bb_index]);
  //   for cmd in bb.cmds() {
  //     match cmd {
  //       IrCmd::Asgn(var, expr) => self.gen_llvm_asgn(var, expr, bb_index, ll_bbs),
  //       // IrCmd::Label(IrLabel), // keep a counter
  //       // IrCmd::Goto(IrLabel),
  //       // IrCmd::Cond(IrLiteral, IrLabel, IrLabel),
  //       // IrCmd::Return(IrExpr),
  //     }
  //   }
  // }

  unsafe fn gen_llvm_asgn(&mut self, var: &IrVar, expr: &IrExpr, ll_function: *mut LLVMValue) {
    match expr {
      IrExpr::Literal(lit) => {
        let alloca = self.get_or_create_alloca(var, ll_function);
        let lit = self.gen_llvm_lit(lit, ll_function);

        LLVMBuildStore(self.llbuilder, lit, alloca);
      }
      IrExpr::Binop(op, lit1, lit2) => {
        if is_predicate(*op) {
          let alloca = self.get_or_create_alloca(var, ll_function);
          let lit1 = self.gen_llvm_lit(lit1, ll_function);
          let lit2 = self.gen_llvm_lit(lit2, ll_function);
          let binop_tmp = self.get_and_incr_ll_temp();
          let binop_inst =
            LLVMBuildICmp(self.llbuilder, to_ll_predicate(*op), lit1, lit2, binop_tmp);
          LLVMBuildStore(self.llbuilder, binop_inst, alloca);
        } else {
          let alloca = self.get_or_create_alloca(var, ll_function);
          let lit1 = self.gen_llvm_lit(lit1, ll_function);
          let lit2 = self.gen_llvm_lit(lit2, ll_function);
          let binop_tmp = self.get_and_incr_ll_temp();
          let binop_inst = LLVMBuildBinOp(self.llbuilder, to_ll_binop(*op), lit1, lit2, binop_tmp);
          LLVMBuildStore(self.llbuilder, binop_inst, alloca);
        }
      }
      IrExpr::Phi(_) => panic!("phi not supported"),
    }
  }

  unsafe fn gen_llvm_lit(
    &mut self,
    lit: &IrLiteral,
    ll_function: *mut LLVMValue,
  ) -> *mut LLVMValue {
    match lit {
      IrLiteral::Num(n) => LLVMConstInt(self.ir_to_ll_type(IrType::Int), *n as c_ulong, 0),
      IrLiteral::Bool(b) => LLVMConstInt(self.ir_to_ll_type(IrType::Bool), *b as c_ulong, 0),
      IrLiteral::Var(var) => {
        let alloc = self.get_or_create_alloca(var, ll_function);
        let ll_temp = self.get_and_incr_ll_temp();
        LLVMBuildLoad(self.llbuilder, alloc, ll_temp)
      }
    }
  }

  unsafe fn get_or_create_bb(
    &mut self,
    l: &IrLabel,
    ll_function: *mut LLVMValue,
  ) -> *mut LLVMBasicBlock {
    if self.label_to_block.contains_key(l) {
      return *self.label_to_block.get(l).unwrap();
    }

    let new_bb = LLVMAppendBasicBlockInContext(self.llctx, ll_function, c_str(&l.to_string()));
    self.label_to_block.insert(*l, new_bb);

    new_bb
  }

  unsafe fn get_or_create_alloca(
    &mut self,
    var: &IrVar,
    ll_function: *mut LLVMValue,
  ) -> *mut LLVMValue {
    if self.var_to_alloca.contains_key(var) {
      return *self.var_to_alloca.get(var).unwrap();
    }

    self.new_alloca(var, ll_function)
  }

  unsafe fn new_alloca(&mut self, var: &IrVar, function: *mut LLVMValue) -> *mut LLVMValue {
    match var {
      IrVar::Temp(_, ty, _) => {
        let entry_bb = LLVMGetEntryBasicBlock(function);
        let tmp_builder = LLVMCreateBuilderInContext(self.llctx);
        LLVMPositionBuilder(tmp_builder, entry_bb, LLVMGetFirstInstruction(entry_bb));
        let alloca = LLVMBuildAlloca(tmp_builder, self.ir_to_ll_type(*ty), tmp_c_str(&var));
        self.var_to_alloca.insert(var.clone(), alloca);

        return alloca;
      }
      _ => panic!("encountered string temp in new_alloca"),
    }
  }

  unsafe fn ir_to_ll_type(&self, ty: IrType) -> *mut LLVMType {
    match ty {
      IrType::Int => LLVMInt32TypeInContext(self.llctx),
      IrType::Bool => LLVMInt1TypeInContext(self.llctx),
      IrType::Addr => LLVMInt64TypeInContext(self.llctx),
    }
  }

  unsafe fn get_and_incr_ll_temp(&mut self) -> *const i8 {
    let tmp = self.ll_tmp_counter;
    self.ll_tmp_counter += 1;
    ll_tmp_c_str(tmp)
  }
}

fn to_ll_binop(op: Opcode) -> LLVMOpcode {
  match op {
    Opcode::Add => LLVMOpcode::LLVMAdd,
    Opcode::Sub => LLVMOpcode::LLVMSub,
    Opcode::Mul => LLVMOpcode::LLVMMul,
    Opcode::Div => LLVMOpcode::LLVMSDiv,
    Opcode::Mod => LLVMOpcode::LLVMSRem,
    Opcode::LogAnd => panic!("LogAnd should be short circuited now"),
    Opcode::LogOr => panic!("LogOr should be short circuited now"),
    op => panic!("cannot convert `{:?}` to binop", op),
  }
}

fn to_ll_predicate(op: Opcode) -> LLVMIntPredicate {
  match op {
    Opcode::Lt => LLVMIntPredicate::LLVMIntSLT,
    Opcode::Leq => LLVMIntPredicate::LLVMIntSLE,
    Opcode::Gt => LLVMIntPredicate::LLVMIntSGT,
    Opcode::Geq => LLVMIntPredicate::LLVMIntSGE,
    Opcode::LogEq => LLVMIntPredicate::LLVMIntEQ,
    Opcode::LogNeq => LLVMIntPredicate::LLVMIntNE,
    op => panic!("cannot convert `{:?}` to predicate", op),
  }
}

fn is_predicate(op: Opcode) -> bool {
  match op {
    Opcode::Lt => true,
    Opcode::Leq => true,
    Opcode::Gt => true,
    Opcode::Geq => true,
    Opcode::LogEq => true,
    Opcode::LogNeq => true,
    _ => false,
  }
}

fn tmp_c_str(var: &IrVar) -> *const i8 {
  match var {
    IrVar::Temp(var, _, version) => {
      c_str(&format!("t{}_{}", &var.to_string(), &version.to_string()))
    }
    _ => panic!("encountered string temp when converting to LL str"),
  }
}

fn ll_tmp_c_str(tmp: usize) -> *const i8 {
  c_str(&format!("ll_{}", tmp))
}

fn c_str(s: &str) -> *const i8 {
  let c_string = CString::new(s).expect("CString::new failed");
  c_string.into_raw()
}

pub unsafe fn llvm_ir_gen(ir: &IrFunction, filename: &str) -> *mut LLVMModule {
  //*mut llvm_sys::LLVMModule {
  let context = LLVMContextCreate();
  let module = LLVMModuleCreateWithNameInContext(c_str(filename), context);
  let builder = LLVMCreateBuilderInContext(context);

  let mut kai_llvm_result = KaiLlvmResult {
    llctx: context,
    llmodule: module,
    llbuilder: builder,
    ll_tmp_counter: 0,
    var_to_alloca: HashMap::new(),
    label_to_block: HashMap::new(),
  };

  let module = kai_llvm_result.gen_llvm_ir(ir);
  LLVMDumpModule(module.llmodule);

  module.llmodule
}

pub unsafe fn llvm_gen(ir: &IrFunction, pathbuf: &mut PathBuf) {
  let filename = pathbuf.file_name().unwrap().to_str().unwrap();
  let ll_module = llvm_ir_gen(ir, filename);
  let target_triple = llvm::target_machine::LLVMGetDefaultTargetTriple();

  llvm::target::LLVM_InitializeAllTargetInfos();
  llvm::target::LLVM_InitializeAllTargets();
  llvm::target::LLVM_InitializeAllTargetMCs();
  llvm::target::LLVM_InitializeAllAsmParsers();
  llvm::target::LLVM_InitializeAllAsmPrinters();

  let mut err = std::ptr::null_mut();
  let mut target = llvm::target_machine::LLVMGetFirstTarget();
  llvm::target_machine::LLVMGetTargetFromTriple(target_triple, &mut target, &mut err);

  let cpu = c_str("generic");
  let features = c_str("");
  let target_machine = llvm::target_machine::LLVMCreateTargetMachine(
    target,
    target_triple,
    cpu,
    features,
    llvm::target_machine::LLVMCodeGenOptLevel::LLVMCodeGenLevelDefault,
    llvm::target_machine::LLVMRelocMode::LLVMRelocDefault,
    llvm::target_machine::LLVMCodeModel::LLVMCodeModelDefault,
  );
  let data_layout = llvm::target_machine::LLVMCreateTargetDataLayout(target_machine);
  llvm::target::LLVMSetModuleDataLayout(ll_module, data_layout);
  LLVMSetTarget(ll_module, target_triple);

  let filename_ll = filename.to_string().replace(".kai", ".ll");
  let filename_obj = filename.to_string().replace(".kai", ".o");
  let mut err_ll = std::ptr::null_mut();
  let mut err_obj = std::ptr::null_mut();

  // println!("{}", filename);
  // println!("{:?}", filename_ll);
  // println!("{:?}", filename_obj);
  pathbuf.pop();
  pathbuf.push(filename_ll);
  let did_emit_asm = llvm::target_machine::LLVMTargetMachineEmitToFile(
    target_machine,
    ll_module,
    c_str(pathbuf.to_str().unwrap()) as *mut _,
    llvm::target_machine::LLVMCodeGenFileType::LLVMAssemblyFile,
    &mut err_ll,
  );
  pathbuf.pop();
  pathbuf.push(filename_obj);
  let did_emit_obj = llvm::target_machine::LLVMTargetMachineEmitToFile(
    target_machine,
    ll_module,
    c_str(pathbuf.to_str().unwrap()) as *mut _,
    llvm::target_machine::LLVMCodeGenFileType::LLVMObjectFile,
    &mut err_obj,
  );

  println!("{:?}, {:?}", did_emit_asm, did_emit_obj);
}
