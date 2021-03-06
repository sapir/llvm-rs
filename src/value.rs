use libc::{c_char, c_int, c_uint};
use ffi::prelude::LLVMValueRef;
use ffi::core;
use ffi::LLVMLinkage;
use std::ffi::CString;
use std::{fmt, mem};
use std::ops::{Deref, Index};
use std::marker::PhantomData;
use block::{BasicBlock, BlockIter};
use context::{Context, GetContext};
use types::{FunctionType, Type};
use util::{self, Sub};

macro_rules! sub {
    ($this:ty, $name:ident) => (
        sub!{$this, $name, ::Value}
    );
    ($this:ty, $name:ident, $sup:ty) => (
unsafe impl Sub<$sup> for $this {
    fn is(value: &$sup) -> bool {
        unsafe {
            !core::$name(value.into()).is_null()
        }
    }
    fn from_super(value: &$sup) -> Option<&$this> {
        unsafe { mem::transmute(core::$name(value.into())) }
    }
}
impl Deref for $this {
    type Target = $sup;
    fn deref(&self) -> &$sup {
        self.to_super()
    }
}
    )
}

/// A typed value that can be used as an operand in instructions.
#[derive(Clone)]
pub struct Value(PhantomData<[u8]>);
native_ref!(&Value = LLVMValueRef);
to_str!{Value, LLVMPrintValueToString}
impl Value {
    /// Create a new constant struct from the values given.
    pub fn new_struct<'a>(context: &'a Context, vals: &[&'a Value], packed: bool) -> &'a Value {
        unsafe {
            core::LLVMConstStructInContext(
                context.into(),
                vals.as_ptr() as *mut LLVMValueRef,
                vals.len() as c_uint,
                packed as c_int,
            )
        }.into()
    }

    /// Create a new null value from any type
    pub fn new_null<'a>(ty: &'a Type) -> &'a Value {
        unsafe { core::LLVMConstNull(ty.into()).into() }
    }
    /// Create a new constant vector from the values given.
    pub fn new_vector<'a>(vals: &[&'a Value]) -> &'a Value {
        unsafe {
            core::LLVMConstVector(vals.as_ptr() as *mut LLVMValueRef, vals.len() as c_uint).into()
        }
    }
    /// Create a new constant C string from the text given.
    pub fn new_string<'a>(context: &'a Context, text: &str, rust_style: bool) -> &'a Value {
        unsafe {
            let ptr = text.as_ptr() as *const c_char;
            let len = text.len() as c_uint;
            core::LLVMConstStringInContext(context.into(), ptr, len, rust_style as c_int).into()
        }
    }
    /// Create a new constant undefined value of the given type.
    pub fn new_undef<'a>(ty: &'a Type) -> &'a Value {
        unsafe { core::LLVMGetUndef(ty.into()).into() }
    }
    /// Returns the name of this value, or `None` if it lacks a name
    pub fn get_name(&self) -> Option<&str> {
        unsafe {
            let c_name = core::LLVMGetValueName(self.into());
            util::to_null_str(c_name as *mut i8)
        }
    }
    /// Sets the name of this value
    pub fn set_name(&self, name: &str) {
        let c_name = CString::new(name).unwrap();
        unsafe { core::LLVMSetValueName(self.into(), c_name.as_ptr()) }
    }
    /// Returns the type of this value
    pub fn get_type(&self) -> &Type {
        unsafe { core::LLVMTypeOf(self.into()) }.into()
    }
}
/// Comparative operations on values.
#[derive(Copy, Clone, Eq, PartialEq)]
pub enum Predicate {
    Equal,
    NotEqual,
    GreaterThan,
    GreaterThanOrEqual,
    LessThan,
    LessThanOrEqual,
}
/// An argument that is passed to a function.
#[derive(Clone)]
pub struct Arg(PhantomData<[u8]>);
native_ref!(&Arg = LLVMValueRef);
sub!{Arg, LLVMIsAArgument}
to_str!{Arg, LLVMPrintValueToString}

/// A value with global scope (eg: Function, Alias, Global variable)
pub struct GlobalValue(PhantomData<[u8]>);
native_ref!(&GlobalValue = LLVMValueRef);
sub!{GlobalValue, LLVMIsAGlobalValue}
to_str!{GlobalValue, LLVMPrintValueToString}
impl GlobalValue {
    /// Set the linkage type for this global
    pub fn set_linkage(&self, linkage: Linkage) {
        unsafe {
            core::LLVMSetLinkage(self.into(), linkage.into());
        }
    }
    /// Returns the linkage type for this global
    pub fn get_linkage(&self) -> Linkage {
        unsafe { core::LLVMGetLinkage(self.into()).into() }
    }
    /// Returns true if this global is a declaration (as opposed to a definition).
    pub fn is_declaration(&self) -> bool {
        unsafe {
            // FIXME: There should be a constant somewhere, instead of '1'
            core::LLVMIsDeclaration(self.into()) == 1
        }
    }
}

/// A global variable
pub struct GlobalVariable(PhantomData<[u8]>);
native_ref!(&GlobalVariable = LLVMValueRef);
sub!{GlobalVariable, LLVMIsAGlobalVariable, GlobalValue}
to_str!{GlobalVariable, LLVMPrintValueToString}
impl GlobalVariable {
    /// Set the initial value of the global
    pub fn set_initializer(&self, val: &Value) {
        unsafe { core::LLVMSetInitializer(self.into(), val.into()) }
    }
    /// Set the initial value of the global
    pub fn get_initializer(&self) -> Option<&Value> {
        unsafe { util::ptr_to_null(core::LLVMGetInitializer(self.into())) }
    }
    /// Set whether this global is a constant.
    pub fn set_constant(&self, is_constant: bool) {
        let llvm_bool = if is_constant { 1 } else { 0 };
        unsafe {
            core::LLVMSetGlobalConstant(self.into(), llvm_bool);
        }
    }
    /// Returns true if this global is a constant.
    pub fn get_constant(&self) -> bool {
        unsafe { core::LLVMIsGlobalConstant(self.into()) != 0 }
    }
}

/// An alias to another global value.
pub struct Alias(PhantomData<[u8]>);
native_ref!(&Alias = LLVMValueRef);
sub!{Alias, LLVMIsAGlobalAlias, GlobalValue}
to_str!{Alias, LLVMPrintValueToString}
/// A function is a kind of value that can be called and contains blocks of code.
///
/// To get the value of each argument to a function, you can use the index operator.
/// For example, `&func[0]` is the value that represents the first argument to the function.
pub struct Function(PhantomData<[u8]>);
native_ref!(&Function = LLVMValueRef);
sub!{Function, LLVMIsAFunction, GlobalValue}
to_str!{Function, LLVMPrintValueToString}
impl Index<usize> for Function {
    type Output = Arg;
    fn index(&self, index: usize) -> &Arg {
        unsafe {
            if index < core::LLVMCountParams(self.into()) as usize {
                core::LLVMGetParam(self.into(), index as c_uint).into()
            } else {
                panic!("no such index {} on {:?}", index, self.get_type())
            }
        }
    }
}
unsafe impl Sub<Value> for Function {
    fn is(value: &Value) -> bool {
        FunctionType::is(value.get_type())
    }
}
impl Function {
    /// Add a basic block with the name given to the function and return it.
    pub fn append<'a>(&'a self, name: &str) -> &'a BasicBlock {
        util::with_cstr(name, |ptr| unsafe {
            core::LLVMAppendBasicBlockInContext(self.get_context().into(), self.into(), ptr).into()
        })
    }
    /// Iterate through this function's basic blocks.
    pub fn blocks(&self) -> BlockIter {
        BlockIter::new(self)
    }
    /// Returns the entry block of this function or `None` if there is none.
    pub fn get_entry(&self) -> Option<&BasicBlock> {
        unsafe { mem::transmute(core::LLVMGetEntryBasicBlock(self.into())) }
    }
    /// Returns the function signature representing this function's signature.
    pub fn get_signature(&self) -> &FunctionType {
        unsafe {
            let ty = core::LLVMTypeOf(self.into());
            core::LLVMGetElementType(ty).into()
        }
    }
    /// Delete the function
    pub fn delete(&self) {
        unsafe {
            core::LLVMDeleteFunction(self.into());
        }
    }
}

impl GetContext for Function {
    fn get_context(&self) -> &Context {
        self.get_type().get_context()
    }
}

/// A way of indicating to LLVM how you want a global to interact during linkage.
#[derive(Copy, Clone, Eq, PartialEq, Debug)]
#[repr(C)]
pub enum Linkage {
    /// Default linkage. The global is externally visible and participates in linkage normally.
    External = 0,
    /// Never emitted to the containing module's object file. Used to allow inlining and/or other optimisations to take place, given knowledge of the definition of the global, which is somewhere outside of the module. Otherwise the same as LinkOnceODR. Only allowed on definitions, not declarations.
    AvailableExternally = 1,
    /// Merged with other globals of the same name during linkage. Unreferenced LinkOnce globals may be discarded.
    LinkOnceAny = 2,
    /// Similar to LinkOnceAny, but indicates that it will only be merged with equivalent globals.
    LinkOnceODR = 3,
    /// Same merging semantics as LinkOnceAny. Unlike LinkOnce, unreference globals will not be discarded.
    WeakAny = 5,
    /// Similar to WeakAny, but indicates that it will only be merged with equivalent globals.
    WeakODR = 6,
    /// Only allowed on global array pointers. When two globals with Appending linkage are merged, they are appended together.
    Appending = 7,
    /// Similar to Private, but shows as a local symbol in the object file.
    Internal = 8,
    /// Only directly accessible by objects in the current module. May be renamed as neccessary to avoid collisions, and all references will be updated. Will not show up in the object file's symbol table.
    Private = 9,
    /// Weak until linked. If not linked, the output symbol is null, instead of undefined.
    ExternalWeak = 12,
    /// Similar to Weak, but may not have an explicit section, must have a zero initializer, and may not be marked constant. Cannot be used on functions or aliases.
    Common = 14,
}
impl From<LLVMLinkage> for Linkage {
    fn from(attr: LLVMLinkage) -> Linkage {
        unsafe { mem::transmute(attr) }
    }
}
impl From<Linkage> for LLVMLinkage {
    fn from(attr: Linkage) -> LLVMLinkage {
        unsafe { mem::transmute(attr) }
    }
}

impl GetContext for Value {
    fn get_context(&self) -> &Context {
        self.get_type().get_context()
    }
}
