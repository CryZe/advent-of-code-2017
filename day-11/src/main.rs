extern crate day_11;

use day_11::*;

fn main() {
    println!("{}", part1(INPUT));
    println!("{}", part1_rayon_map_reduce(INPUT));
    println!("{}", part1_rayon_map_reduce_flat(INPUT));
    println!("{}", part1_flat(INPUT));
    println!("{}", part1_dirs(INPUT));

    println!("======================");

    println!("{}", part2_dirs(INPUT));
    println!("{}", part2(INPUT));
}
