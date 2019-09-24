fn main() {
    cc::Build::new()
        .file("src/gl_bridge.c")
        .flag("-lGL")
        .flag("-lGLEW")
        .compile("gl_bridge");
    println!("cargo:rustc-link-lib=dylib=GL");
    println!("cargo:rustc-link-lib=dylib=GLEW");
}