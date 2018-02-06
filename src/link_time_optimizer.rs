use ffi::link_time_optimizer::*;
use libc::{c_char, c_void};
use std::ops::Drop;

/// Dummy type for pointers to the LTO object
type LTOObject = *mut c_void;

/// This struct represents a llvm LinkTimeOptimize
pub struct LinkTimeOptimizer(LTOObject);

pub enum LTOStatus {
    Uknown,
    OptSuccess,
    ReadSuccess,
    ReadFailure,
    WriteFailure,
    NoTarget,
    NoWork,
    ModuleMergeFailure,
    ASMFailure,
    NullObject,
}

impl LinkTimeOptimizer {
    pub fn new() -> LinkTimeOptimizer {
        LinkTimeOptimizer(unsafe { llvm_create_optimizer() })
    }

    pub fn optimize_module(&self, output_file: &str) -> LTOStatus {
        unsafe { llvm_optimize_modules(self.0, output_file.as_ptr() as *const c_char) }.into()
    }

    pub fn read_object_file(&self, input_file: &str) -> LTOStatus {
        unsafe { llvm_read_object_file(self.0, input_file.as_ptr() as *const c_char) }.into()
    }
}

impl Drop for LinkTimeOptimizer {
    fn drop(&mut self) {
        unsafe { llvm_destroy_optimizer(self.0) }
    }
}

impl From<llvm_lto_status_t> for LTOStatus {
    fn from(error: llvm_lto_status_t) -> LTOStatus {
        match error {
            llvm_lto_status_t::LLVM_LTO_UNKNOWN => LTOStatus::Uknown,
            llvm_lto_status_t::LLVM_LTO_OPT_SUCCESS => LTOStatus::OptSuccess,
            llvm_lto_status_t::LLVM_LTO_READ_SUCCESS => LTOStatus::ReadSuccess,
            llvm_lto_status_t::LLVM_LTO_READ_FAILURE => LTOStatus::ReadFailure,
            llvm_lto_status_t::LLVM_LTO_WRITE_FAILURE => LTOStatus::WriteFailure,
            llvm_lto_status_t::LLVM_LTO_NO_TARGET => LTOStatus::NoTarget,
            llvm_lto_status_t::LLVM_LTO_NO_WORK => LTOStatus::NoWork,
            llvm_lto_status_t::LLVM_LTO_MODULE_MERGE_FAILURE => LTOStatus::ModuleMergeFailure,
            llvm_lto_status_t::LLVM_LTO_ASM_FAILURE => LTOStatus::ASMFailure,
            llvm_lto_status_t::LLVM_LTO_NULL_OBJECT => LTOStatus::NullObject,
        }
    }
}
