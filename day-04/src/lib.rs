#![feature(test)]

extern crate arrayvec;
extern crate fxhash;
extern crate rayon;
extern crate seahash;

use rayon::prelude::*;
use arrayvec::ArrayVec;
use seahash::SeaHasher;
use std::collections::HashSet;
use std::hash::BuildHasherDefault;
use fxhash::FxBuildHasher;

pub fn part1(text: &str) -> usize {
    let mut set = HashSet::with_capacity_and_hasher(11, BuildHasherDefault::<SeaHasher>::default());
    text.lines()
        .filter(|l| {
            set.clear();
            l.split_whitespace().all(|w| set.insert(w))
        })
        .count()
}

pub fn part1_rayon(text: &str) -> usize {
    text.par_lines()
        .map_with(
            HashSet::with_capacity_and_hasher(11, BuildHasherDefault::<SeaHasher>::default()),
            |set, l| {
                set.clear();
                l.split_whitespace().all(|w| set.insert(w))
            },
        )
        .filter(|&b| b)
        .count()
}

pub fn part1_rayon_fx(text: &str) -> usize {
    text.par_lines()
        .map_with(
            HashSet::with_capacity_and_hasher(11, FxBuildHasher::default()),
            |set, l| {
                set.clear();
                l.split_whitespace().all(|w| set.insert(w))
            },
        )
        .filter(|&b| b)
        .count()
}

pub fn part1_rayon_fx_new_hash_set(text: &str) -> usize {
    text.par_lines()
        .filter(|l| {
            let mut set = HashSet::with_capacity_and_hasher(11, FxBuildHasher::default());
            l.split_whitespace().all(|w| set.insert(w))
        })
        .count()
}

pub fn part2(text: &str) -> usize {
    let mut set = HashSet::with_capacity_and_hasher(11, BuildHasherDefault::<SeaHasher>::default());
    text.lines()
        .filter(|l| {
            set.clear();
            l.split_whitespace().all(|w| {
                let mut bytes = w.bytes().collect::<ArrayVec<[u8; 7]>>();
                bytes.sort_unstable();
                set.insert(bytes)
            })
        })
        .count()
}

pub fn part2_rayon(text: &str) -> usize {
    text.par_lines()
        .map_with(
            HashSet::with_capacity_and_hasher(11, BuildHasherDefault::<SeaHasher>::default()),
            |set, l| {
                set.clear();
                l.split_whitespace().all(|w| {
                    let mut bytes = w.bytes().collect::<ArrayVec<[u8; 7]>>();
                    bytes.sort_unstable();
                    set.insert(bytes)
                })
            },
        )
        .filter(|&b| b)
        .count()
}

#[derive(Eq, PartialEq, Hash, Clone)]
struct WordMap([u8; 26]);

impl WordMap {
    fn new<T: AsRef<[u8]>>(s: T) -> Self {
        let mut map = [0u8; 26];
        for u in s.as_ref() {
            map[(u - b'a') as usize] += 1;
        }
        WordMap(map)
    }
}

pub fn part2_archer(text: &str) -> usize {
    text.par_lines()
        .map_with(
            HashSet::with_capacity_and_hasher(11, BuildHasherDefault::<SeaHasher>::default()),
            |set, l| {
                set.clear();
                l.split_whitespace().all(|w| set.insert(WordMap::new(w)))
            },
        )
        .filter(|&b| b)
        .count()
}

pub fn part2_archer_fx(text: &str) -> usize {
    text.par_lines()
        .map_with(
            HashSet::with_capacity_and_hasher(11, FxBuildHasher::default()),
            |set, l| {
                set.clear();
                l.split_whitespace().all(|w| set.insert(WordMap::new(w)))
            },
        )
        .filter(|&b| b)
        .count()
}

pub fn part2_archer_fx_new_hash_set(text: &str) -> usize {
    text.par_lines()
        .filter(|l| {
            let mut set = HashSet::with_capacity_and_hasher(11, FxBuildHasher::default());
            l.split_whitespace().all(|w| set.insert(WordMap::new(w)))
        })
        .count()
}

pub const INPUT: &str = include_str!("input.txt");


extern crate test;

#[cfg(test)]
mod tests {
    use super::*;
    use test::Bencher;

    // #[bench]
    // fn bench_part1(b: &mut Bencher) {
    //     b.iter(|| part1(INPUT));
    // }

    // #[bench]
    // fn bench_part2(b: &mut Bencher) {
    //     b.iter(|| part2(INPUT));
    // }

    // #[bench]
    // fn bench_part1_rayon(b: &mut Bencher) {
    //     b.iter(|| part1_rayon(INPUT));
    // }

    #[bench]
    fn bench_part1_rayon_fx(b: &mut Bencher) {
        b.iter(|| part1_rayon_fx(INPUT));
    }

    #[bench]
    fn bench_part1_rayon_fx_new_hash_set(b: &mut Bencher) {
        b.iter(|| part1_rayon_fx_new_hash_set(INPUT));
    }

    // #[bench]
    // fn bench_part2_rayon(b: &mut Bencher) {
    //     b.iter(|| part2_rayon(INPUT));
    // }

    // #[bench]
    // fn bench_part2_archer(b: &mut Bencher) {
    //     b.iter(|| part2_archer(INPUT));
    // }

    #[bench]
    fn bench_part2_archer_fx(b: &mut Bencher) {
        b.iter(|| part2_archer_fx(INPUT));
    }

    #[bench]
    fn bench_part2_archer_fx_new_hash_set(b: &mut Bencher) {
        b.iter(|| part2_archer_fx_new_hash_set(INPUT));
    }
}
