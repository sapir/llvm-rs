//! This library provides wrappers for LLVM that are memory-safe and follow
//! Rust idioms.
//!
//! The original LLVM reference is available [here](http://llvm.org/doxygen/)
//! but take note that this isn't as thorough as this documentation.

extern crate cbox;
extern crate libc;
#[cfg(feature = "expose_bindings")]
pub extern crate llvm_sys as ffi;

#[cfg(not(feature = "expose_bindings"))]
extern crate llvm_sys as ffi;

#[macro_use]
mod macros;
mod buffer;
mod block;
mod builder;
mod compile;
mod context;
mod engine;
mod module;
mod object;
mod target;
pub mod types;
pub mod value;
mod util;
mod pass_manager;
pub mod lto;
pub mod link_time_optimizer;

pub use cbox::{CBox, CSemiBox};
pub use builder::Builder;
pub use block::BasicBlock;
pub use compile::Compile;
pub use context::{Context, GetContext};
pub use engine::{ExecutionEngine, GenericValue, GenericValueCast, Interpreter, JitEngine,
                 JitOptions};
pub use module::{AddressSpace, Functions, Module};
pub use object::{ObjectFile, Symbol, Symbols};
pub use target::{Target, TargetData, TargetMachine};
pub use types::*;
pub use value::{Alias, Arg, Function, GlobalValue, GlobalVariable, Linkage, Predicate, Value};
pub use util::Sub;
pub use pass_manager::{PassManager, PassManagerBuilder, PassRegistry};

pub mod prelude {

    pub type LLVMBool = ::libc::c_int;
    pub type LLVMMemoryBufferRef = *mut ::ffi::LLVMMemoryBuffer;
    pub type LLVMContextRef = *mut ::ffi::LLVMContext;
    pub type LLVMModuleRef = *mut ::ffi::LLVMModule;
    pub type LLVMTypeRef = *mut ::ffi::LLVMType;
    pub type LLVMValueRef = *mut ::ffi::LLVMValue;
    pub type LLVMBasicBlockRef = *mut ::ffi::LLVMBasicBlock;
    pub type LLVMBuilderRef = *mut ::ffi::LLVMBuilder;
    pub type LLVMModuleProviderRef = *mut ::ffi::LLVMModuleProvider;
    pub type LLVMPassManagerRef = *mut ::ffi::LLVMPassManager;
    pub type LLVMPassRegistryRef = *mut ::ffi::LLVMPassRegistry;
    pub type LLVMUseRef = *mut ::ffi::LLVMUse;
    pub type LLVMDiagnosticInfoRef = *mut ::ffi::LLVMDiagnosticInfo;
    pub type LLVMAttributeRef = *mut ::ffi::LLVMOpaqueAttributeRef;
}
