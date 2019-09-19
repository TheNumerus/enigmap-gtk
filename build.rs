fn main() {
    cc::Build::new()
        .file("src/gl_bridge.c")
        .compile("gl_bridge");
        println!("cargo:rustc-link-lib=dylib=GL");
}