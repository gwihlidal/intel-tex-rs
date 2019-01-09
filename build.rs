extern crate ispc;

fn compile_kernel() {
    let mut cfg = ispc::Config::new();
    cfg.file("vendor/ISPC Texture Compressor/ispc_texcomp/kernel.ispc");
    cfg.opt_level(3);
    //cfg.optimization_opt(ispc::OptimizationOpt::FastMath);
    //cfg.quiet();
    cfg.woff();
    //cfg.instrument();
    cfg.compile("kernel");
}

fn compile_kernel_astc() {
    let mut cfg = ispc::Config::new();
    cfg.file("vendor/ISPC Texture Compressor/ispc_texcomp/kernel_astc.ispc");
    cfg.opt_level(3);
    //cfg.optimization_opt(ispc::OptimizationOpt::FastMath);
    //cfg.quiet();
    cfg.woff();
    //cfg.instrument();
    cfg.compile("kernel_astc");
}

fn main() {
    compile_kernel();
    compile_kernel_astc();
}
