use ffi::{core,LLVMPassManager};
use ffi::prelude::LLVMPassManagerRef;
use cbox::{CSemiBox};
use std::marker::PhantomData;
use module::Module;
use ffi::transforms::scalar::*;

pub struct PassManager(PhantomData<[u8]>);
native_ref!{&PassManager = LLVMPassManagerRef}
dispose!{PassManager,LLVMPassManager,core::LLVMDisposePassManager}

impl<'a> PassManager {
    /// Create a new pass manager
    pub fn new() -> CSemiBox<'a,PassManager> {
        unsafe{core::LLVMCreatePassManager()}.into()
    }

    /// Create a new function pass manager for a given module
    pub fn new_func_pass(module:&'a Module) -> &PassManager {
        unsafe{core::LLVMCreateFunctionPassManagerForModule(module.into())}.into()
    }

    pub fn init_func_pass(&self)  {
        unsafe {
            core::LLVMInitializeFunctionPassManager(self.into()) 
        };
    }

    pub fn finalize_func_pass(&self) {
        unsafe {
            core::LLVMFinalizeFunctionPassManager(self.into())
        };
    }
}




macro_rules! add_scalar_pass {
    ($name:ident, $passname:expr) => {
        impl <'a> PassManager {
            pub fn $name(&self) {
                unsafe {$passname(self.into())};
            }
        }
    };
}


add_scalar_pass!{add_dce,LLVMAddAggressiveDCEPass}
add_scalar_pass!{add_alingmnet_from_assum,LLVMAddAlignmentFromAssumptionsPass}
add_scalar_pass!{add_basic_alias_analysis,LLVMAddBasicAliasAnalysisPass}
add_scalar_pass!{add_bit_tacking_dce,LLVMAddBitTrackingDCEPass}
add_scalar_pass!{add_cfg,LLVMAddCFGSimplificationPass}
add_scalar_pass!{add_constant_propagation,LLVMAddConstantPropagationPass}
add_scalar_pass!{add_dead_store_elimination,LLVMAddDeadStoreEliminationPass}
add_scalar_pass!{add_demote_memory_to_register,LLVMAddDemoteMemoryToRegisterPass}
add_scalar_pass!{add_early_cse,LLVMAddEarlyCSEPass}
add_scalar_pass!{add_correlated_value_propagation,LLVMAddCorrelatedValuePropagationPass}
add_scalar_pass!{add_gvn,LLVMAddGVNPass}
add_scalar_pass!{add_ind_var_simplify,LLVMAddIndVarSimplifyPass}
add_scalar_pass!{add_instruction_combining,LLVMAddInstructionCombiningPass}
add_scalar_pass!{add_licm,LLVMAddLICMPass}
add_scalar_pass!{add_loop_deletion,LLVMAddLoopDeletionPass}
add_scalar_pass!{add_loop_idiom,LLVMAddLoopIdiomPass}
add_scalar_pass!{add_loop_reroll,LLVMAddLoopRerollPass}
add_scalar_pass!{add_loop_rotate,LLVMAddLoopRotatePass}
add_scalar_pass!{add_loop_unroll,LLVMAddLoopUnrollPass}
add_scalar_pass!{add_loop_nswitch,LLVMAddLoopUnswitchPass}
add_scalar_pass!{add_lower_expect_intrinsic,LLVMAddLowerExpectIntrinsicPass}
add_scalar_pass!{add_lower_swithc,LLVMAddLowerSwitchPass}
add_scalar_pass!{add_mem_cpy,LLVMAddMemCpyOptPass}
add_scalar_pass!{add_merged_load_store_motion,LLVMAddMergedLoadStoreMotionPass}
add_scalar_pass!{add_partially_inline_lib_calls,LLVMAddPartiallyInlineLibCallsPass}
add_scalar_pass!{add_promote_memory_to_register,LLVMAddPromoteMemoryToRegisterPass}
add_scalar_pass!{add_reassociate,LLVMAddReassociatePass}

add_scalar_pass!{add_sccp,LLVMAddSCCPPass}

add_scalar_pass!{add_scalar_repl_aggregates,LLVMAddScalarReplAggregatesPass}

add_scalar_pass!{add_scalar_repl_aggregates_ssa,LLVMAddScalarReplAggregatesPassSSA}

add_scalar_pass!{add_scalarizer,LLVMAddScalarizerPass}
add_scalar_pass!{add_scoped_no_alias_aa,LLVMAddScopedNoAliasAAPass}
add_scalar_pass!{add_simplify_lib_calls,LLVMAddSimplifyLibCallsPass}

add_scalar_pass!{add_add_tail_call_elimination,LLVMAddTailCallEliminationPass}
add_scalar_pass!{add_type_based_alias_nalysis,LLVMAddTypeBasedAliasAnalysisPass}
add_scalar_pass!{add_verifier,LLVMAddVerifierPass}





