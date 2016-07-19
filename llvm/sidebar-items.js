initSidebarItems({"enum":[["AddressSpace",""],["Attribute","These indicate how you want arguments / functions to be handled."],["Linkage","A way of indicating to LLVM how you want a global to interact during linkage."],["Predicate","Comparative operations on values."]],"struct":[["Alias","An alias to another global value."],["Arg","An argument that is passed to a function."],["BasicBlock","A container of instructions that execute sequentially."],["Builder","This provides a uniform API for creating instructions and inserting them into a basic block."],["CBox","A wrapper for pointers made by C that are now completely owned by Rust, so they are not limited by any lifetimes."],["CSemiBox","A wrapper for pointers made by C that are now partially owned in Rust."],["Context","Contains all the LLVM entities - mainly modules."],["Function","A function is a kind of value that can be called and contains blocks of code."],["FunctionType","A function signature type."],["Functions","An iterator through the functions contained in a module."],["GenericValue","A wrapped value that can be passed to an interpreted function or returned from one"],["GlobalValue","A value with global scope (eg: Function, Alias, Global variable)"],["GlobalVariable","A global variable"],["Interpreter","The interpreter backend"],["JitEngine","The MCJIT backend, which compiles functions and values into machine code."],["JitOptions","The options to pass to the MCJIT backend."],["Module","Represents a single compilation unit of code."],["ObjectFile","An external object file that has been parsed by LLVM."],["StructType","A structure type, such as a tuple or struct."],["Symbol",""],["Symbols",""],["Target",""],["TargetData","Represents an LLVM Target"],["Type","Defines how a value should be laid out in memory."],["Value","A typed value that can be used as an operand in instructions."]],"trait":[["Compile","A type that can be represented as a constant in LLVM IR."],["ExecutionEngine","An abstract interface for implementation execution of LLVM modules."],["GenericValueCast","A value that can be cast into a `GenericValue` and that a `GenericValue` can be cast into."],["GetContext","Implemented by everything that is owned by a context."],["Sub","Indicates that this structure is a substructure of another."]]});