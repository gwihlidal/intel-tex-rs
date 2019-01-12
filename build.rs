extern crate ispc;

/*
    ISPC project file builds the kernels as such:
    <Command Condition="'$(Configuration)|$(Platform)'=='Release|x64'">ispc -O2 "%(Filename).ispc" -o "$(TargetDir)%(Filename).obj" -h "$(ProjectDir)%(Filename)_ispc.h" --target=sse2,sse4,avx,avx2 --opt=fast-math</Command>
    <Outputs Condition="'$(Configuration)|$(Platform)'=='Release|x64'">$(TargetDir)%(Filename).obj;$(TargetDir)%(Filename)_sse2.obj;$(TargetDir)%(Filename)_sse4.obj;$(TargetDir)%(Filename)_avx.obj;$(TargetDir)%(Filename)_avx2.obj;</Outputs>
*/

fn compile_kernel() {
    let mut cfg = ispc::Config::new();
    cfg.file("vendor/ISPC Texture Compressor/ispc_texcomp/kernel.ispc");
    cfg.opt_level(2);
    cfg.optimization_opt(ispc::opt::OptimizationOpt::FastMath);
    //cfg.quiet();
    cfg.woff();
    //cfg.instrument();
    cfg.compile("kernel");
}

fn compile_kernel_astc() {
    let mut cfg = ispc::Config::new();
    cfg.file("vendor/ISPC Texture Compressor/ispc_texcomp/kernel_astc.ispc");
    cfg.opt_level(2);
    cfg.optimization_opt(ispc::opt::OptimizationOpt::FastMath);
    //cfg.quiet();
    cfg.woff();
    //cfg.instrument();
    cfg.compile("kernel_astc");
}

fn main() {
    compile_kernel();
    compile_kernel_astc();
}
