#![feature(test)]

extern crate arrayvec;

use arrayvec::ArrayVec;

pub const INPUT: &str = include_str!("input.txt");

pub fn part1(text: &str) -> usize {
    let mut numbers: ArrayVec<[i16; 2048]> = text.split_whitespace()
        .filter_map(|n| n.parse().ok())
        .collect();

    let mut index = 0;
    let mut count = 0;

    while let Some(cell) = numbers.get_mut(index as usize) {
        index += *cell;
        *cell += 1;
        count += 1;
    }

    count
}

pub fn part2(text: &str) -> usize {
    let mut numbers: ArrayVec<[i16; 2048]> = text.split_whitespace()
        .filter_map(|n| n.parse().ok())
        .collect();

    let mut index = 0;
    let mut count = 0;

    while let Some(cell) = numbers.get_mut(index as usize) {
        index += *cell;
        *cell += if *cell >= 3 { -1 } else { 1 };
        count += 1;
    }

    count
}

extern crate test;

#[cfg(test)]
mod tests {
    use super::*;
    use test::Bencher;

    #[test]
    fn it_works() {
        assert_eq!(part1("0 3  0  1  -3"), 5);
        assert_eq!(part2("0 3  0  1  -3"), 10);
    }

    #[bench]
    fn bench_part1(b: &mut Bencher) {
        b.iter(|| part1(INPUT));
    }

    #[bench]
    fn bench_part2(b: &mut Bencher) {
        b.iter(|| part2(INPUT));
    }
}
