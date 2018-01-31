#![feature(test)]

extern crate arrayvec;
#[macro_use]
extern crate error_chain;
extern crate itertools;
extern crate rayon;

use itertools::Itertools;
use rayon::prelude::*;
use arrayvec::ArrayVec;

pub const INPUT: &str = include_str!("large_input.txt");

pub mod xfix;

pub fn part1(text: &str) -> i64 {
    text.lines()
        .filter_map(|l| {
            l.split_whitespace()
                .filter_map(|n| n.parse::<i64>().ok())
                .minmax()
                .into_option()
        })
        .map(|(min, max)| max - min)
        .sum()
}

pub fn part2(text: &str) -> usize {
    let mut numbers = Vec::new();

    text.lines()
        .filter_map(|l| {
            numbers.clear();
            numbers.extend(l.split_whitespace().filter_map(|n| n.parse::<u16>().ok()));
            numbers.sort_unstable();
            let mut iter = numbers.iter();
            while let Some(&divisor) = iter.next() {
                let (mut multiple, mut quotient) = (divisor, 1);
                for &dividend in iter.clone() {
                    while dividend > multiple {
                        multiple += divisor;
                        quotient += 1;
                    }
                    if dividend == multiple {
                        return Some(quotient);
                    }
                }
            }
            None
        })
        .sum()
}

pub fn part2_arrayvec(text: &str) -> usize {
    let mut numbers = ArrayVec::<[u16; 16]>::new();

    text.lines()
        .filter_map(|l| {
            numbers.clear();
            numbers.extend(l.split_whitespace().filter_map(|n| n.parse().ok()));
            numbers.sort_unstable();
            let mut iter = numbers.iter();
            while let Some(&divisor) = iter.next() {
                let (mut multiple, mut quotient) = (divisor, 1);
                for &dividend in iter.clone() {
                    while dividend > multiple {
                        multiple += divisor;
                        quotient += 1;
                    }
                    if dividend == multiple {
                        return Some(quotient);
                    }
                }
            }
            None
        })
        .sum()
}

pub fn part2_simple(text: &str) -> i64 {
    let mut numbers = Vec::new();

    text.lines()
        .filter_map(|l| {
            numbers.clear();
            numbers.extend(l.split_whitespace().filter_map(|n| n.parse::<i64>().ok()));
            numbers.sort_unstable();
            let mut iter = numbers.iter();
            while let Some(&divisor) = iter.next() {
                for &dividend in iter.clone() {
                    if dividend % divisor == 0 {
                        return Some(dividend / divisor);
                    }
                }
            }
            None
        })
        .sum()
}

pub fn xavil(text: &str) -> u64 {
    let mut numbers = ArrayVec::<[u64; 16]>::new();

    text.lines()
        .filter_map(|l| {
            numbers.clear();
            numbers.extend(l.split_whitespace().filter_map(|n| n.parse::<u64>().ok()));
            for (i, &num) in numbers.iter().enumerate() {
                for (j, &denom) in numbers.iter().enumerate() {
                    if num % denom == 0 && i != j {
                        return Some(num / denom);
                    }
                }
            }
            None
        })
        .sum()
}

pub fn leah(text: &str) -> f32 {
    let mut numbers = Vec::new();

    fn evenly_divisible(row: &[f32]) -> f32 {
        for (i, n) in row.iter().enumerate() {
            for (j, m) in row.iter().enumerate() {
                if i == j {
                    continue;
                };
                if n < m {
                    continue;
                };
                if (n / m) % 1f32 == 0f32 {
                    return n / m;
                }
            }
        }
        0f32
    }

    text.lines()
        .map(|l| {
            numbers.clear();
            numbers.extend(l.split_whitespace().filter_map(|n| n.parse::<f32>().ok()));
            evenly_divisible(&numbers)
        })
        .sum()
}

pub fn part2_par_sort(text: &str) -> usize {
    let mut numbers = Vec::new();

    text.lines()
        .filter_map(|l| {
            numbers.clear();
            numbers.extend(l.split_whitespace().filter_map(|n| n.parse::<i64>().ok()));
            numbers.par_sort();
            let mut iter = numbers.iter();
            while let Some(&min) = iter.next() {
                let (mut mul, mut count) = (min, 1);
                for &max in iter.clone() {
                    while max > mul {
                        mul += min;
                        count += 1;
                    }
                    if max == mul {
                        return Some(count);
                    }
                }
            }
            None
        })
        .sum()
}

pub fn part2_rayon(text: &str) -> usize {
    text.par_lines()
        .filter_map(|l| {
            let mut numbers: ArrayVec<[i64; 16]> = l.split_whitespace()
                .filter_map(|n| n.parse().ok())
                .collect();
            numbers.sort_unstable();
            let mut iter = numbers.iter();
            while let Some(&min) = iter.next() {
                let (mut mul, mut count) = (min, 1);
                for &max in iter.clone() {
                    while max > mul {
                        mul += min;
                        count += 1;
                    }
                    if max == mul {
                        return Some(count);
                    }
                }
            }
            None
        })
        .sum()
}


extern crate test;

#[cfg(test)]
mod tests {
    use super::*;
    use test::Bencher;

    #[bench]
    fn bench_mine(b: &mut Bencher) {
        b.iter(|| part2(INPUT));
    }

    #[bench]
    fn bench_mine_arrayvec(b: &mut Bencher) {
        b.iter(|| part2_arrayvec(INPUT));
    }

    #[bench]
    fn bench_mine_simple(b: &mut Bencher) {
        b.iter(|| part2_simple(INPUT));
    }

    #[bench]
    fn bench_xavil(b: &mut Bencher) {
        b.iter(|| xavil(INPUT));
    }

    #[bench]
    fn bench_leah(b: &mut Bencher) {
        b.iter(|| leah(INPUT));
    }

    #[bench]
    fn bench_mine_par_sort(b: &mut Bencher) {
        b.iter(|| part2_par_sort(INPUT));
    }

    #[bench]
    fn bench_mine_rayon(b: &mut Bencher) {
        b.iter(|| part2_rayon(INPUT));
    }

    #[bench]
    fn bench_xfix(b: &mut Bencher) {
        b.iter(|| xfix::checksum(INPUT));
    }
}
