// extern crate llvm_sys as llvm;

// use std::collections::HashMap;
// use std::mem;

// use llvm::core::*;
// use llvm::execution_engine::*;
// use llvm::target::*;
// use llvm::*;

// use kai_ir::ir::*;

// struct KaiLlvmResult {
//   llctx: *mut LLVMContext,
//   llmodule: *mut LLVMModule,
//   llbuilder: *mut LLVMBuilder,
//   arg_to_ll_param_map: HashMap<IrVar, *mut LLVMValue>,
// }

// impl KaiLlvmResult {
//   pub fn gen_llvm_ir(&mut self, ir: &IrFunction) -> KaiLlvmResult {
//     let ret_ty = self.ir_to_ll_type(ir.ret_ty);
//     let mut arg_tys: Vec<*mut LLVMType> = ir
//       .args
//       .into_iter()
//       .map(|arg| self.ir_to_ll_type(arg.ty))
//       .collect();

//     let fn_type = LLVMFunctionType(ret_ty, arg_tys.as_mut_ptr(), arg_tys.len() as u32, 0);
//     let function = llvm::core::LLVMAddFunction(self.llmodule, c_str(&ir.ident), fn_type);
//     for arg in ir.args {
//       self
//         .arg_to_ll_param_map
//         .insert(ar.ident, LLVMGetParam(function, i as u32));
//     }

//     let bb =
//       LLVMAppendBasicBlockInContext(self.llctx, function, c_str(&["entry", &ir.ident].join("_")));

//     LLVMPositionBuilderAtEnd(builder, bb);

//     self.gen_llvm_ir_body(&ir.body);
//     self
//   }

//   fn gen_llvm_ir_body(&mut self, ir_body: &Vec<IrCmd>) {
//     for cmd in ir_body {
//       match cmd {
//         IrCmd::Asgn(var, expr) => self.gen_llvm_asgn(var, expr)
//         // IrCmd::Label(IrLabel), // keep a counter
//         // IrCmd::Goto(IrLabel),
//         // IrCmd::Cond(IrLiteral, IrLabel, IrLabel),
//         // IrCmd::Return(IrExpr),
//       }
//     }
//   }

//   fn gen_llvm_asgn(var: IrVar, expr: IrExpr) {
//     match expr {
//       IrExpr::Num(n) => LLVMBuild
//     }
//   }

//   fn ir_to_ll_type(&self, ty: IrType) -> *mut LLVMType {
//     match ty {
//       Int => LLVMInt32TypeInContext(self.llctx),
//       Bool => LLVMInt1TypeInContext(self.llctx),
//       Addr => LLVMInt64TypeInContext(self.llctx),
//     }
//   }
// }

// fn c_str(s: &str) -> *const i8 {
//   s.as_bytes().as_ptr() as *const i8
// }

// pub fn llvm_ir_gen(ir: &IrFunction) -> *mut llvm_sys::LLVMModule {
//   unsafe {
//     let context = LLVMContextCreate();
//     let module = LLVMModuleCreateWithNameInContext(b"kai\0".as_ptr() as *const _, context);
//     let builder = LLVMCreateBuilderInContext(context);

//     let kai_llvm_result = KaiLlvmResult {
//       llctx: context,
//       llmodule: module,
//       llbuilder: builder,
//       arg_to_ll_param_map: HashMap::new(),
//     };

//     kai_llvm_result.gen_llvm_ir(ir)
//   }
// }
