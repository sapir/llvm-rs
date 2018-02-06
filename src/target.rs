use libc::{c_char, c_uint};
use ffi::target_machine::*;
use ffi::target::*;
use ffi::core::LLVMDisposeMessage;
use std::ffi::CStr;
use std::fmt;
use types::Type;
use util;
use std::ptr;
use pass_manager::PassManager;

/// Represents an LLVM Target
pub struct TargetData(LLVMTargetDataRef);
native_ref!(&TargetData = LLVMTargetDataRef);

impl TargetData {
    /// Create a target data from a target layout string.
    pub fn from_string(rep: &str) -> TargetData {
        TargetData(unsafe { LLVMCreateTargetData(rep.as_ptr() as *const c_char) })
    }

    /// Returns true if the target is big endian.
    pub fn is_big_endian(&self) -> bool {
        let order = unsafe { LLVMByteOrder(self.0) } as c_uint;
        order == 0
    }
    /// Returns the size of a pointer on the target.
    pub fn get_pointer_size(&self) -> usize {
        unsafe { LLVMPointerSize(self.0) as usize }
    }
    /// Returns the size of the type given in bits.
    pub fn size_of_in_bits(&self, ty: &Type) -> u64 {
        unsafe { LLVMSizeOfTypeInBits(self.0, ty.into()) }
    }
    /// Returns the size of the type given in bytes.
    pub fn size_of(&self, ty: &Type) -> u64 {
        unsafe { LLVMStoreSizeOfType(self.0, ty.into()) }
    }
    /// Returns the alignment of the type given in bytes.
    pub fn alignment_of(&self, ty: &Type) -> usize {
        unsafe { LLVMABIAlignmentOfType(self.0, ty.into()) as usize }
    }
    /// Computes the structure element that contains the byte offset for a target.
    pub fn element_at(&self, struct_ty: &Type, offset: u64) -> usize {
        unsafe { LLVMElementAtOffset(self.0, struct_ty.into(), offset) as usize }
    }
    /// Compute the byte offset of an element in the struct type given.
    pub fn offset_of(&self, struct_ty: &Type, element: usize) -> u64 {
        unsafe { LLVMOffsetOfElement(self.0, struct_ty.into(), element as c_uint) }
    }
    /// Returns the string representation of this target data.
    pub fn as_str(&self) -> &str {
        unsafe {
            CStr::from_ptr(LLVMCopyStringRepOfTargetData(self.0))
                .to_str()
                .unwrap()
        }
    }
}

impl fmt::Display for TargetData {
    fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
        fmt.write_str(&self.as_str())
    }
}

impl Drop for TargetData {
    fn drop(&mut self) {
        unsafe { LLVMDisposeTargetData(self.0) }
    }
}

pub struct Target(*mut LLVMTarget);
native_ref!(&Target = LLVMTargetRef);

impl Target {
    /// Returns the name of this target.
    pub fn get_name(&self) -> &str {
        unsafe { util::to_str(LLVMGetTargetName(self.0) as *mut c_char) }
    }
    /// Returns the description of this target.
    pub fn get_description(&self) -> &str {
        unsafe { util::to_str(LLVMGetTargetDescription(self.0) as *mut c_char) }
    }

    /// Returns true if this target has an assembly generation backend implemented.
    pub fn has_asm_backend(&self) -> bool {
        unsafe { LLVMTargetHasAsmBackend(self.0) != 0 }
    }
    /// Returns true if this target supports JIT compilation.
    pub fn has_jit(&self) -> bool {
        unsafe { LLVMTargetHasJIT(self.0) != 0 }
    }
    /// Returns true if this target has a target machine.
    pub fn has_target_machine(&self) -> bool {
        unsafe { LLVMTargetHasTargetMachine(self.0) != 0 }
    }
}

pub struct TargetMachine(*mut LLVMOpaqueTargetMachine);

impl TargetMachine {
    pub fn new() -> Result<TargetMachine, String> {
        let triple = unsafe { LLVMGetDefaultTargetTriple() };

        let triple_str = unsafe { CStr::from_ptr(triple) }
            .to_str()
            .expect("Invalid target triple");
        let mut target = ptr::null_mut();

        let mut error = ptr::null_mut();

        if unsafe { LLVMGetTargetFromTriple(triple, &mut target, &mut error) } != 0 {
            let msg = unsafe { CStr::from_ptr(error) }
                .to_str()
                .expect("Invalid C string");
            let e = format!(
                "Unable to get an LLVM target reference for {}: {}",
                triple_str, msg
            );
            unsafe { LLVMDisposeMessage(error) };
            unsafe { LLVMDisposeMessage(triple) };
            return Err(e);
        }

        let target_machine = unsafe {
            LLVMCreateTargetMachine(
                target,
                triple,
                "".as_ptr() as *const c_char,
                "".as_ptr() as *const c_char,
                LLVMCodeGenOptLevel::LLVMCodeGenLevelDefault,
                LLVMRelocMode::LLVMRelocPIC,
                LLVMCodeModel::LLVMCodeModelDefault,
            )
        };

        if target_machine.is_null() {
            let e = format!("Unable to get a LLVM target machine for {}", triple_str);
            return Err(e);
        }

        Ok(TargetMachine(target_machine))
    }

    pub fn first_target(&self) -> Target {
        unsafe { Target(LLVMGetFirstTarget()) }
    }

    pub fn next_target(&self, target: Target) -> Target {
        unsafe { Target(LLVMGetNextTarget(target.0)) }
    }

    pub fn get_description(&self, target: &Target) -> &str {
        unsafe {
            CStr::from_ptr(LLVMGetTargetDescription(target.0))
                .to_str()
                .expect("unable to get decription")
        }
    }

    pub fn analysis_passes(&self, pass_manager: &PassManager) {
        unsafe { LLVMAddAnalysisPasses(self.0, pass_manager.into()) }
    }
}

impl Drop for TargetMachine {
    fn drop(&mut self) {
        unsafe { LLVMDisposeTargetMachine(self.0) }
    }
}
