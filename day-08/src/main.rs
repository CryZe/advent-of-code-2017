extern crate day_08;

use day_08::*;
use std::fs::File;
use std::io::Write;

fn main() {
    let instructions = &parse_parallel(TEST_INPUT);

    // let compiled_fn = compiler::jit_compile_part1(instructions);
    // println!("{}", compiled_fn.call());

    let compiled_fn = compiler::jit_compile_part2(instructions);
    println!("{}", compiled_fn.call());

    File::create("compiled.bin")
        .unwrap()
        .write_all(compiled_fn.as_slice())
        .unwrap();
    // println!("{}", part1(INPUT));
    // println!("{}", part2(INPUT));
}
