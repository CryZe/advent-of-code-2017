extern crate cc;

fn main() {
    cc::Build::new()
        .file("src/wlandry.cpp")
        .cpp(true)
        .flag_if_supported("/std:c++17")
        .flag_if_supported("-std=c++17")
        // .flag("-mcpu=native")
        .compile("wlandry");
}
