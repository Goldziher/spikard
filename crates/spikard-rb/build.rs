use rb_sys_build::RbConfig;

fn main() {
    let mut rbconfig = RbConfig::current();
    rbconfig.link_ruby(false);
    rbconfig.print_cargo_args();
    println!("cargo:rerun-if-changed=build.rs");
}
