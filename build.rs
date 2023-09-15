fn main() {
    println!("cargo:rerun-if-changed=cpp/seh_wrapper.cpp");
    cc::Build::new()
        .cpp(true)
        .file("cpp/seh_wrapper.cpp")
        .compile("seh_wrapper");
}
