extern crate llvm_rs as llvm;
use llvm::*;

#[link(name = "ffi")]
extern "C" {}

fn main() {
    let ctx = Context::new();
    let module = Module::new("simple", &ctx);
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

    let main = module.add_function("main", Type::get::<fn(u64) -> u64>(&ctx));
    let main_body = main.append("entry");
    builder.position_at_end(main_body);
    builder.build_ret(builder.build_call(func, &vec![10u64.compile(&ctx)]));

    println!("{:?}", module);

    module
        .write_bitcode("out.bc")
        .expect("Couldn't write to file");

    module.verify().unwrap();
    let ee = JitEngine::new(&module, JitOptions { opt_level: 0 }).unwrap();
    ee.with_function(func, |fib: extern "C" fn(u64) -> u64| {
        for i in 0..10 {
            println!("fib {} = {}", i, fib(i))
        }
    });

    ee.remove_module(&module);
}
