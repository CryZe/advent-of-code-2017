extern crate day_04;

use day_04::*;

fn main() {
    println!("{}", part1_rayon(INPUT));
    println!("{}", part2_rayon(INPUT));
    println!(
        "{}",
        INPUT
            .lines()
            .map(|l| l.split_whitespace().count())
            .max()
            .unwrap()
    );
}
