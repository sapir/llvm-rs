use ffi::{core, LLVMPassManager};
use ffi::prelude::LLVMPassManagerRef;
use cbox::CSemiBox;
use std::marker::PhantomData;
use module::Module;
use ffi::transforms::scalar::*;
use ffi::transforms::vectorize::*;
use ffi::transforms::ipo::*;

pub struct PassManager(PhantomData<[u8]>);
native_ref!{&PassManager = LLVMPassManagerRef}
dispose!{PassManager,LLVMPassManager,core::LLVMDisposePassManager}

impl<'a> PassManager {
    /// Create a new pass manager
    pub fn new() -> CSemiBox<'a, PassManager> {
        unsafe { core::LLVMCreatePassManager() }.into()
    }

    /// Create a new function pass manager for a given module
    pub fn new_func_pass(module: &'a Module) -> &PassManager {
        unsafe { core::LLVMCreateFunctionPassManagerForModule(module.into()) }.into()
    }

    pub fn init_func_pass(&self) {
        unsafe { core::LLVMInitializeFunctionPassManager(self.into()) };
    }

    pub fn finalize_func_pass(&self) {
        unsafe { core::LLVMFinalizeFunctionPassManager(self.into()) };
    }
}

macro_rules! add_pass {
    ($name:ident, $passname:expr) => {
        impl <'a> PassManager {
            pub fn $name(&self) {
                unsafe {$passname(self.into())};
            }
        }
    };
}

// Scalar transformations
add_pass!{add_dce,LLVMAddAggressiveDCEPass}
add_pass!{add_alingmnet_from_assum,LLVMAddAlignmentFromAssumptionsPass}
add_pass!{add_basic_alias_analysis,LLVMAddBasicAliasAnalysisPass}
add_pass!{add_bit_tacking_dce,LLVMAddBitTrackingDCEPass}
add_pass!{add_cfg,LLVMAddCFGSimplificationPass}
add_pass!{add_constant_propagation,LLVMAddConstantPropagationPass}
add_pass!{add_dead_store_elimination,LLVMAddDeadStoreEliminationPass}
add_pass!{add_demote_memory_to_register,LLVMAddDemoteMemoryToRegisterPass}
add_pass!{add_early_cse,LLVMAddEarlyCSEPass}
add_pass!{add_correlated_value_propagation,LLVMAddCorrelatedValuePropagationPass}
add_pass!{add_gvn,LLVMAddGVNPass}
add_pass!{add_ind_var_simplify,LLVMAddIndVarSimplifyPass}
add_pass!{add_instruction_combining,LLVMAddInstructionCombiningPass}
add_pass!{add_licm,LLVMAddLICMPass}
add_pass!{add_loop_deletion,LLVMAddLoopDeletionPass}
add_pass!{add_loop_idiom,LLVMAddLoopIdiomPass}
add_pass!{add_loop_reroll,LLVMAddLoopRerollPass}
add_pass!{add_loop_rotate,LLVMAddLoopRotatePass}
add_pass!{add_loop_unroll,LLVMAddLoopUnrollPass}
add_pass!{add_loop_nswitch,LLVMAddLoopUnswitchPass}
add_pass!{add_lower_expect_intrinsic,LLVMAddLowerExpectIntrinsicPass}
add_pass!{add_lower_swithc,LLVMAddLowerSwitchPass}
add_pass!{add_mem_cpy,LLVMAddMemCpyOptPass}
add_pass!{add_merged_load_store_motion,LLVMAddMergedLoadStoreMotionPass}
add_pass!{add_partially_inline_lib_calls,LLVMAddPartiallyInlineLibCallsPass}
add_pass!{add_promote_memory_to_register,LLVMAddPromoteMemoryToRegisterPass}
add_pass!{add_reassociate,LLVMAddReassociatePass}
add_pass!{add_sccp,LLVMAddSCCPPass}
add_pass!{add_scalar_repl_aggregates,LLVMAddScalarReplAggregatesPass}
add_pass!{add_scalar_repl_aggregates_ssa,LLVMAddScalarReplAggregatesPassSSA}
add_pass!{add_scalarizer,LLVMAddScalarizerPass}
add_pass!{add_scoped_no_alias_aa,LLVMAddScopedNoAliasAAPass}
add_pass!{add_simplify_lib_calls,LLVMAddSimplifyLibCallsPass}
add_pass!{add_add_tail_call_elimination,LLVMAddTailCallEliminationPass}
add_pass!{add_type_based_alias_nalysis,LLVMAddTypeBasedAliasAnalysisPass}
add_pass!{add_verifier,LLVMAddVerifierPass}

// Vectorization transformations
add_pass!{add_bb_vectorize,LLVMAddBBVectorizePass}
add_pass!{add_loop_vectorize,LLVMAddLoopVectorizePass}
add_pass!{add_slp_vectorize,LLVMAddSLPVectorizePass}

// Interprocedural transformations
add_pass!{add_always_inline,LLVMAddAlwaysInlinerPass}
add_pass!{add_argument_promotion,LLVMAddArgumentPromotionPass}
add_pass!{add_constant_merge,LLVMAddConstantMergePass}
add_pass!{add_dead_arg_elimination,LLVMAddDeadArgEliminationPass}
add_pass!{add_function_attrs,LLVMAddFunctionAttrsPass}
add_pass!{add_function_inlining,LLVMAddFunctionInliningPass}
add_pass!{add_global_dce,LLVMAddGlobalDCEPass}
add_pass!{add_global_pptimizer,LLVMAddGlobalOptimizerPass}
add_pass!{add_ip_constant_propagation,LLVMAddIPConstantPropagationPass}
add_pass!{add_ipsccp,LLVMAddIPSCCPPass}
add_pass!{add_prune_eh,LLVMAddPruneEHPass}
add_pass!{add_strip_dead_prototypes,LLVMAddStripDeadPrototypesPass}
add_pass!{add_strip_symbols,LLVMAddStripSymbolsPass}
