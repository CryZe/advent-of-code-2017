extern crate cc;

fn main() {
    cc::Build::new()
        .file("src/kenira.cpp")
        .cpp(true)
        .flag_if_supported("/std:c++17")
        .flag_if_supported("-std=c++17")
        // .flag("-mcpu=native")
        .compile("kenira");
}
