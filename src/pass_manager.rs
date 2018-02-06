use libc::c_int;
use ffi::{core, LLVMPassManager};
use ffi::prelude::LLVMPassManagerRef;
use cbox::CSemiBox;
use std::marker::PhantomData;
use module::Module;
use ffi::transforms::scalar::*;
use ffi::transforms::vectorize::*;
use ffi::transforms::ipo::*;
use value::Value;
use ffi::LLVMPassRegistry;
use ffi::core::LLVMGetGlobalPassRegistry;
use ffi::initialization::*;

/// The struct responsible for setting up optimization sequences
pub struct PassManager(PhantomData<[u8]>);
native_ref!{&PassManager = LLVMPassManagerRef}
dispose!{PassManager,LLVMPassManager,core::LLVMDisposePassManager}

impl<'a> PassManager {
    /// Create a new pass manager
    pub fn new() -> CSemiBox<'a, PassManager> {
        unsafe { core::LLVMCreatePassManager() }.into()
    }

    /// Create a new function pass manager for a given module. It runs the optimizations
    /// on each function immediatly as it is generated
    pub fn new_func_pass(module: &'a Module) -> CSemiBox<'a, PassManager> {
        unsafe { core::LLVMCreateFunctionPassManagerForModule(module.into()) }.into()
    }

    // Run the function pass manager
    pub fn run_func_pass(&self, f: &'a Value) -> bool {
        unsafe { core::LLVMRunFunctionPassManager(self.into(), f.into()) != 0 }
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
            pub fn $name(&self) -> &PassManager {
                unsafe {$passname(self.into())};
                self
            }
        }
    };
}

// Scalar transformations
add_pass!{add_agressive_dce,LLVMAddAggressiveDCEPass}
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
add_pass!{add_tail_call_elimination,LLVMAddTailCallEliminationPass}
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

use ffi::transforms::pass_manager_builder::*;

/// Used to custimize a pass sequence in various ways
/// For more information go to [llvm](http://llvm.org/doxygen/classllvm_1_1PassManagerBuilder.html)
pub struct PassManagerBuilder(PhantomData<[u8]>);
native_ref!{&PassManagerBuilder = LLVMPassManagerBuilderRef}
dispose!{PassManagerBuilder,LLVMOpaquePassManagerBuilder,LLVMPassManagerBuilderDispose}

impl<'a> PassManagerBuilder {
    /// Create a new `PassManagerBuilder`
    pub fn new() -> CSemiBox<'a, PassManagerBuilder> {
        unsafe { LLVMPassManagerBuilderCreate() }.into()
    }

    /// Specify the basic optimization level
    pub fn set_opt(&self, level: u32) {
        unsafe { LLVMPassManagerBuilderSetOptLevel(self.into(), level.into()) };
    }

    /// How much we're optimizing for size
    pub fn set_size(&self, level: u32) {
        unsafe { LLVMPassManagerBuilderSetOptLevel(self.into(), level.into()) };
    }

    ///
    pub fn set_disable_simplify_lib_calls(&self, opt: bool) {
        unsafe { LLVMPassManagerBuilderSetDisableSimplifyLibCalls(self.into(), opt as c_int) }
    }

    pub fn disable_unit_at_a_time(&self, opt: bool) {
        unsafe { LLVMPassManagerBuilderSetDisableUnitAtATime(self.into(), opt as c_int) }
    }

    pub fn disable_unroll_loop(&self, opt: bool) {
        unsafe { LLVMPassManagerBuilderSetDisableUnrollLoops(self.into(), opt as c_int) }
    }

    pub fn populate_lto_pass_manger(
        &self,
        pass_manager: &PassManager,
        internalize: bool,
        run_inliner: bool,
    ) {
        unsafe {
            LLVMPassManagerBuilderPopulateLTOPassManager(
                self.into(),
                pass_manager.into(),
                internalize as c_int,
                run_inliner as c_int,
            )
        }
    }

    pub fn inline_with_threshold(&self, threshold: u32) {
        unsafe { LLVMPassManagerBuilderUseInlinerWithThreshold(self.into(), threshold) }
    }

    pub fn populate_module_pass_manager(&self, pass_manager: &PassManager) {
        unsafe { LLVMPassManagerBuilderPopulateModulePassManager(self.into(), pass_manager.into()) }
    }

    pub fn populate_function_pass_manager(&self, pass_manager: &PassManager) {
        unsafe {
            LLVMPassManagerBuilderPopulateFunctionPassManager(self.into(), pass_manager.into())
        }
    }
}

/// This class manages the registration and intitialization of
/// the pass subsystem as application startup, and assists the PassManager
/// in resolving pass dependencies
pub struct PassRegistry(*mut LLVMPassRegistry);

// Waiting on stabilisation of #13231
//impl !Send for PassRegistry {}
//impl !Sync for PassRegistry{}

impl PassRegistry {
    pub fn new() -> Self {
        PassRegistry(unsafe { LLVMGetGlobalPassRegistry() })
    }

    pub fn init_all(&self) {
        self.init_analyis()
            .init_codegen()
            .init_core()
            .init_ipa()
            .init_ipo()
            .init_inst_combine()
            .init_instrumentation()
            .init_scalar_opts()
            .init_target()
            .init_transfrom_utils()
            .init_vectorization();
    }

    fn get(&self) -> *mut LLVMPassRegistry {
        self.0
    }

    pub fn init_analyis(&self) -> &PassRegistry {
        unsafe { LLVMInitializeAnalysis(self.get()) }
        self
    }

    pub fn init_codegen(&self) -> &PassRegistry {
        unsafe { LLVMInitializeCodeGen(self.get()) }

        self
    }

    pub fn init_core(&self) -> &PassRegistry {
        unsafe { LLVMInitializeCore(self.get()) }

        self
    }

    pub fn init_ipa(&self) -> &PassRegistry {
        unsafe { LLVMInitializeIPA(self.get()) }

        self
    }

    pub fn init_ipo(&self) -> &PassRegistry {
        unsafe { LLVMInitializeIPO(self.get()) }

        self
    }

    pub fn init_inst_combine(&self) -> &PassRegistry {
        unsafe { LLVMInitializeInstCombine(self.get()) }

        self
    }

    pub fn init_instrumentation(&self) -> &PassRegistry {
        unsafe { LLVMInitializeInstrumentation(self.get()) }

        self
    }

    pub fn init_scalar_opts(&self) -> &PassRegistry {
        unsafe { LLVMInitializeScalarOpts(self.get()) }

        self
    }

    pub fn init_target(&self) -> &PassRegistry {
        unsafe { LLVMInitializeTarget(self.get()) }

        self
    }

    pub fn init_transfrom_utils(&self) -> &PassRegistry {
        unsafe { LLVMInitializeTransformUtils(self.get()) }

        self
    }

    pub fn init_vectorization(&self) -> &PassRegistry {
        unsafe { LLVMInitializeVectorization(self.get()) }
        self
    }
}
