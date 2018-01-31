extern crate cc;

fn main() {
    cc::Build::new()
        .file("src/some_guy.cpp")
        .cpp(true)
        .compile("some_guy");
    cc::Build::new()
        .file("src/lunar.cpp")
        .cpp(true)
        .compile("lunar");
}
