/// we could simply do this to compile the test lib to a static lib,
/// but that solution might not scale as good as calling a professional
/// build system
#[allow(unused)]
fn main() {
    // Tell Cargo that if the given file changes, to rerun this build script.
    println!("cargo:rerun-if-changed=src/hello.c");
    cc::Build::new()
        .file("lib/test.c")
        .compile("test");
}
