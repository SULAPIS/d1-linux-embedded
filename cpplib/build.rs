use cxx_build::CFG;
fn main() {
    CFG.include_prefix = "";
    cxx_build::bridge("src/lib.rs")
        .file("src/cpp.cc")
        .include("include")
        .compile("cpplib");
}
