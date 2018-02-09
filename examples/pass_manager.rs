extern crate llvm_rs as llvm;;

use llvm::{Builder, Compile, Context, Module, PassManager, PassRegistry, Type};

fn main() {
    let ctx = Context::new();
    let module = Module::new("main", &ctx);
    let builder = Builder::new(&ctx);

    let pm_reg = PassRegistry::new();
    let fpm = PassManager::new_func_pass(&module);

    pm_reg.init_all();

    fpm.add_instruction_combining()
        .add_reassociate()
        .add_gvn()
        .add_cfg()
        .add_tail_call_elimination();

    let func = module.add_function("fib", Type::get::<fn(u64) -> u64>(&ctx));
    let value = &func[0];
    let entry = func.append("entry");
    let on_zero = func.append("on_zero");
    let on_one = func.append("on_one");
    let default = func.append("default");
    let builder = Builder::new(&ctx);
    let zero = 0u64.compile(&ctx);
    let one = 1u64.compile(&ctx);
    builder.position_at_end(entry);
    builder.build_switch(value, default, &[(zero, on_zero), (one, on_one)]);
    builder.position_at_end(on_zero);
    builder.build_ret(zero);
    builder.position_at_end(on_one);
    builder.build_ret(one);
    builder.position_at_end(default);
    let two = 2u64.compile(&ctx);
    let a = builder.build_sub(value, one);
    let b = builder.build_sub(value, two);
    let fa = builder.build_tail_call(func, &[a]);
    let fb = builder.build_tail_call(func, &[b]);
    builder.build_ret(builder.build_add(fa, fb));

    module
        .write_bitcode("out.bc")
        .expect("Couldn't write to file");

    module.verify().unwrap();

    println!("{:?}", module);
}
