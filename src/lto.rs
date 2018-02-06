use ffi::lto::*;
use std::ops::Drop;
use std::ffi::{CStr, CString};
use libc::{c_char, c_uint, c_void};
use object::Symbol;
use context::Context;
use std::mem;

pub struct LTOCodeGenerator(*mut LLVMOpaqueLTOCodeGenerator);

pub struct LTOModule(*mut LLVMOpaqueLTOModule);

pub enum LTODebugModel {
    DWARF,
    None,
}

pub enum LTOPicModel {
    Static,
    Dynamic,
    DynamicNoPic,
    Default,
}

pub enum LTOSymbolAttributes {
    MaskAlignment,
    MaskPermissions,
    CodePermissions,
    DataPermissions,
    RoDataPermissions,
    MaskDefinition,
    RegularDefinition,
    TentativeDefinition,
    WeakDefinition,
    UndefinedDefinition,
    WeakUndefindedDefinition,
    MaskScope,
    InternalScope,
    HiddenScope,
    ProtectedScope,
    DefualtScope,
    ScopeCanBeHidden,
    Comdat,
    Alias,
}

pub enum LTODiagnosticSeverity {

}

impl LTOCodeGenerator {
    pub fn new() -> Self {
        LTOCodeGenerator(unsafe { lto_codegen_create() })
    }

    fn get(&self) -> *mut LLVMOpaqueLTOCodeGenerator {
        self.0
    }

    pub fn add_module(&self, module: LTOModule) -> Result<(), String> {
        if unsafe { lto_codegen_add_module(self.get(), module.0) } == (false as u8) {
            Ok(())
        } else {
            let error = unsafe { CStr::from_ptr(lto_get_error_message()) };

            Err(CString::from(error).into_string().unwrap())
        }
    }

    /// Adds a symbol to the list of global symbols that must exist in the final
    /// generated code. If a function is not listed there, it might be inlined into
    ///  every usage and optimized away. For every single module, the functions
    /// referenced from code outside of the ThinLTO modules need to be added here.
    pub fn preserve_symbol(&self, symbol: Symbol) {
        unsafe { lto_codegen_add_must_preserve_symbol(self.get(), symbol.get()) }
    }

    pub fn compile(&self) {
        unimplemented!()
    }

    /// Sets debug option
    pub fn debug_options(&self, opts: &str) {
        unsafe { lto_codegen_debug_options(self.get(), opts.as_ptr() as *const c_char) }
    }

    /// Runs optimization for the merged module
    pub fn optimize(&self) -> Result<(), String> {
        if unsafe { lto_codegen_optimize(self.get()) } == true as u8 {
            let error = unsafe { CStr::from_ptr(lto_get_error_message()) };
            Err(CString::from(error).into_string().unwrap())
        } else {
            Ok(())
        }
    }

    /// Sets extra arguments that libLTO should pass to the assembler
    pub fn assembler_args(&self, args: Vec<&str>) {
        unsafe {
            lto_codegen_set_assembler_args(
                self.get(),
                args.join("").as_ptr() as *mut *const c_char,
                args.len() as i32,
            )
        }
    }

    /// Sets the location of the assembler tool to run. If not set, libLTO
    /// will use gcc to invoke the assembler
    pub fn assembler_path(&self, path: &str) {
        unsafe { lto_codegen_set_assembler_path(self.get(), path.as_ptr() as *const c_char) }
    }

    /// Sets the cpu to generate code for
    pub fn set_cpu(&self, cpu: &str) {
        unsafe { lto_codegen_set_cpu(self.get(), cpu.as_ptr() as *const c_char) }
    }
    /// Sets if debug info should be generated.
    pub fn set_debug_model(&self, model: LTODebugModel) -> Result<(), String> {
        if unsafe { lto_codegen_set_debug_model(self.get(), model.into()) } == true as u8 {
            let error = unsafe { CStr::from_ptr(lto_get_error_message()) };
            Err(CString::from(error).into_string().unwrap())
        } else {
            Ok(())
        }
    }

    ///Sets the object module for code gneeration. This will transfer ownership of the module to the code generator.
    pub fn set_module(&self, module: LTOModule) {
        unsafe { lto_codegen_set_module(self.get(), module.0) }
    }

    /// Sets which PIC code model to generated
    pub fn set_pic_model(&self, model: LTOPicModel) -> Result<(), String> {
        if unsafe { lto_codegen_set_pic_model(self.get(), model.into()) } == true as u8 {
            let error = unsafe { CStr::from_ptr(lto_get_error_message()) };
            Err(CString::from(error).into_string().unwrap())
        } else {
            Ok(())
        }
    }

    /// Set whether to embed uselists in bitcode.
    pub fn should_use_embed_lists(&self, flag: bool) {
        unsafe { lto_codegen_set_should_embed_uselists(self.get(), flag as u8) }
    }

    pub fn should_internalize(&self, flag: bool) {
        unsafe { lto_codegen_set_should_internalize(self.get(), flag as u8) }
    }

    ///  Writes a new object file at the specified path that contains the
    /// merged contents of all modules added so far
    pub fn write_modules(&self, path: &str) -> Result<(), String> {
        if unsafe { lto_codegen_write_merged_modules(self.get(), path.as_ptr() as *const c_char) }
            == true as u8
        {
            let error = unsafe { CStr::from_ptr(lto_get_error_message()) };
            Err(CString::from(error).into_string().unwrap())
        } else {
            Ok(())
        }
    }

    /// Initializes LLVM disassemblers
    pub fn init_disassembler(&self) {
        unsafe { lto_initialize_disassembler() }
    }
}

impl Drop for LTOCodeGenerator {
    fn drop(&mut self) {
        unsafe { lto_codegen_dispose(self.get()) }
    }
}

impl From<LTODebugModel> for lto_debug_model {
    fn from(model: LTODebugModel) -> lto_debug_model {
        match model {
            LTODebugModel::None => lto_debug_model::LTO_DEBUG_MODEL_NONE,
            LTODebugModel::DWARF => lto_debug_model::LTO_DEBUG_MODEL_DWARF,
        }
    }
}

impl From<LTOPicModel> for lto_codegen_model {
    fn from(model: LTOPicModel) -> lto_codegen_model {
        match model {
            LTOPicModel::Static => lto_codegen_model::LTO_CODEGEN_PIC_MODEL_STATIC,
            LTOPicModel::Dynamic => lto_codegen_model::LTO_CODEGEN_PIC_MODEL_DYNAMIC,
            LTOPicModel::DynamicNoPic => lto_codegen_model::LTO_CODEGEN_PIC_MODEL_DYNAMIC_NO_PIC,
            LTOPicModel::Default => lto_codegen_model::LTO_CODEGEN_PIC_MODEL_DEFAULT,
        }
    }
}

impl From<lto_symbol_attributes> for LTOSymbolAttributes {
    fn from(attribute: lto_symbol_attributes) -> LTOSymbolAttributes {
        match attribute {
            lto_symbol_attributes::LTO_SYMBOL_ALIGNMENT_MASK => LTOSymbolAttributes::MaskAlignment,
            lto_symbol_attributes::LTO_SYMBOL_PERMISSIONS_MASK => {
                LTOSymbolAttributes::MaskPermissions
            }
            lto_symbol_attributes::LTO_SYMBOL_PERMISSIONS_CODE => {
                LTOSymbolAttributes::CodePermissions
            }
            lto_symbol_attributes::LTO_SYMBOL_PERMISSIONS_DATA => {
                LTOSymbolAttributes::DataPermissions
            }
            lto_symbol_attributes::LTO_SYMBOL_PERMISSIONS_RODATA => {
                LTOSymbolAttributes::RoDataPermissions
            }
            lto_symbol_attributes::LTO_SYMBOL_DEFINITION_MASK => {
                LTOSymbolAttributes::MaskDefinition
            }
            lto_symbol_attributes::LTO_SYMBOL_DEFINITION_REGULAR => {
                LTOSymbolAttributes::RegularDefinition
            }
            lto_symbol_attributes::LTO_SYMBOL_DEFINITION_TENTATIVE => {
                LTOSymbolAttributes::TentativeDefinition
            }
            lto_symbol_attributes::LTO_SYMBOL_DEFINITION_WEAK => {
                LTOSymbolAttributes::WeakDefinition
            }
            lto_symbol_attributes::LTO_SYMBOL_DEFINITION_UNDEFINED => {
                LTOSymbolAttributes::UndefinedDefinition
            }
            lto_symbol_attributes::LTO_SYMBOL_DEFINITION_WEAKUNDEF => {
                LTOSymbolAttributes::WeakUndefindedDefinition
            }
            lto_symbol_attributes::LTO_SYMBOL_SCOPE_MASK => LTOSymbolAttributes::MaskScope,
            lto_symbol_attributes::LTO_SYMBOL_SCOPE_INTERNAL => LTOSymbolAttributes::InternalScope,
            lto_symbol_attributes::LTO_SYMBOL_SCOPE_HIDDEN => LTOSymbolAttributes::HiddenScope,
            lto_symbol_attributes::LTO_SYMBOL_SCOPE_PROTECTED => {
                LTOSymbolAttributes::ProtectedScope
            }
            lto_symbol_attributes::LTO_SYMBOL_SCOPE_DEFAULT => LTOSymbolAttributes::DefualtScope,
            lto_symbol_attributes::LTO_SYMBOL_SCOPE_DEFAULT_CAN_BE_HIDDEN => {
                LTOSymbolAttributes::ScopeCanBeHidden
            }
            lto_symbol_attributes::LTO_SYMBOL_COMDAT => LTOSymbolAttributes::Comdat,
            lto_symbol_attributes::LTO_SYMBOL_ALIAS => LTOSymbolAttributes::Alias,
        }
    }
}

impl LTOModule {
    pub fn new(path: &str) -> LTOModule {
        LTOModule(unsafe { lto_module_create(path.as_ptr() as *const c_char) })
    }

    fn get(&self) -> *mut LLVMOpaqueLTOModule {
        self.0
    }

    /// Loads an object file into the same context as codegenerator. The module is safe to
    // add using `lto_codegen_add_module()`
    pub fn with_context(mut ctx: &Context, path: &str) -> Result<(), String> {
        let length = mem::size_of::<Context>();
        let ctx_ptr: *mut c_void = &mut ctx as *mut _ as *mut c_void;
        if unsafe {
            lto_module_create_in_local_context(ctx_ptr, length, path.as_ptr() as *const c_char)
        }.is_null()
        {
            let error = unsafe { CStr::from_ptr(lto_get_error_message()) };
            Err(CString::from(error).into_string().unwrap())
        } else {
            Ok(())
        }
    }

    /// Returns the module's linker options
    pub fn linker_opts(&self) -> &str {
        let opts = unsafe { CStr::from_ptr(lto_module_get_linkeropts(self.get())) };
        opts.to_str().unwrap()
    }

    /// Returns the number of symbols in the object module
    pub fn num_symbols(&self) -> c_uint {
        unsafe { lto_module_get_num_symbols(self.get()) }
    }

    ///  Returns the attributes of the ith symbol in the object module.
    pub fn symbol_attribute(&self, index: c_uint) -> LTOSymbolAttributes {
        unsafe { lto_module_get_symbol_attribute(self.get(), index) }.into()
    }

    /// Returns the name of the ith symbol in the object module
    pub fn symbol_name(&self, index: c_uint) -> &str {
        unsafe {
            CStr::from_ptr(lto_module_get_symbol_name(self.get(), index))
                .to_str()
                .unwrap()
        }
    }

    pub fn target_triple(&self) -> &str {
        unsafe {
            CStr::from_ptr(lto_module_get_target_triple(self.get()))
                .to_str()
                .unwrap()
        }
    }

    pub fn is_object_file(&self, path: &str) -> bool {
        unsafe { lto_module_is_object_file(path.as_ptr() as *const c_char) != 0 }
    }

    pub fn is_for_target(&self, path: &str, target: &str) -> bool {
        unsafe {
            lto_module_is_object_file_for_target(
                path.as_ptr() as *const c_char,
                target.as_ptr() as *const c_char,
            ) != 0
        }
    }

    pub fn is_thinlto(&self) -> bool {
        unsafe { lto_module_is_thinlto(self.get()) != 0 }
    }
}

impl Drop for LTOModule {
    fn drop(&mut self) {
        unsafe { lto_module_dispose(self.get()) }
    }
}
