#![feature(i128_type, conservative_impl_trait, generator_trait, generators)]
#![cfg_attr(test, feature(test))]

extern crate arrayvec;
extern crate rayon;

use arrayvec::{ArrayString, ArrayVec};
use rayon::prelude::*;
use std::fmt::Write;

pub const INPUT: &str = "hfdlxzhv";
pub const TEST_INPUT: &str = "flqrgnkx";

use std::ops::{Generator, GeneratorState};

#[derive(Copy, Clone, Debug)]
pub struct GenIter<T>(pub T)
where
    T: Generator;

impl<T: Generator> Iterator for GenIter<T> {
    type Item = <T as Generator>::Yield;

    fn next(&mut self) -> Option<Self::Item> {
        match self.0.resume() {
            GeneratorState::Yielded(n) => Some(n),
            GeneratorState::Complete(_) => None,
        }
    }
}

impl<G: Generator> From<G> for GenIter<G> {
    fn from(gen: G) -> Self {
        GenIter(gen)
    }
}

fn reverse(elements: &mut [u8; 256], start: u8, len: u8) {
    let (mut left, mut right) = (start, start.wrapping_add(len).wrapping_sub(1));
    for _ in 0..len / 2 {
        elements.swap(left as usize, right as usize);
        left = left.wrapping_add(1);
        right = right.wrapping_sub(1);
    }
}

fn initial_list() -> [u8; 256] {
    let mut list = [0; 256];
    for i in 0..256 {
        list[i] = i as u8;
    }
    list
}

fn round<I>(list: &mut [u8; 256], input: I, index: &mut u8, skip_size: &mut u8)
where
    I: IntoIterator<Item = u8>,
{
    for len in input {
        reverse(list, *index, len);
        *index = index.wrapping_add(len).wrapping_add(*skip_size);
        *skip_size = skip_size.wrapping_add(1);
    }
}

fn hash(text: &str) -> u128 {
    let text = text.trim();
    let mut list = initial_list();
    let (mut index, mut skip_size) = (0, 0);

    for _ in 0..64 {
        round(
            &mut list,
            text.bytes().chain([17, 31, 73, 47, 23].iter().cloned()),
            &mut index,
            &mut skip_size,
        );
    }

    let mut output = 0u128;
    for c in list.chunks(16).map(|c| c.iter().fold(0, |a, b| a ^ b)) {
        output = output << 8 | (c as u128);
    }
    output
}

pub fn part1(text: &str) -> u32 {
    (0..128)
        .map(|i| {
            let mut buf = ArrayString::<[u8; 32]>::new();
            write!(buf, "{}-{}", text, i).unwrap();
            hash(&buf).count_ones()
        })
        .sum()
}

pub fn part1_rayon(text: &str) -> u32 {
    (0..128)
        .into_par_iter()
        .map(|i| {
            let mut buf = ArrayString::<[u8; 32]>::new();
            write!(buf, "{}-{}", text, i).unwrap();
            hash(&buf).count_ones()
        })
        .sum()
}

fn ranges(mut hash: u128) -> impl Iterator<Item = (u32, u32)> {
    GenIter(move || {
        let mut range_start = 0;
        while hash != 0 {
            let offset = hash.trailing_zeros();
            hash = hash >> offset;
            range_start += offset;
            let range_len = (!hash).trailing_zeros();
            let range_end = range_start + range_len;
            yield (range_start, range_end);
            hash = hash >> range_len;
            range_start = range_end;
        }
    })
}

fn unconnected_ranges(hash1: u128, hash2: u128) -> usize {
    let mut iter1 = ranges(hash1).peekable();
    let mut unconnected = 0;
    'outer: for (s2, e2) in ranges(hash2) {
        while let Some(&(s1, e1)) = iter1.peek() {
            if e1 > s2 {
                continue 'outer;
            }
            iter1.next();
        }
        unconnected += 1;
    }
    unconnected
}

pub fn part2(text: &str) -> u32 {
    println!("{:?}", ranges(0xFF_00_00_FF_FF_00_F0).collect::<Vec<_>>());
    unimplemented!()
    // (0..128)
    //     .into_par_iter()
    //     .map(|i| {
    //         let mut buf = ArrayString::<[u8; 32]>::new();
    //         write!(buf, "{}-{}", text, i).unwrap();
    //         let mut hash = hash(&buf);
    //         let mut buf = ArrayVec::<[(u32, u32); 128]>::new();
    //         let mut range_start = 0;
    //         while hash != 0 {
    //             let offset = hash.trailing_zeros();
    //             hash = hash >> offset;
    //             range_start += offset;
    //             let range_len = (!hash).trailing_zeros();
    //             let range_end = range_start + range_len;
    //             buf.push((range_start, range_end));
    //             hash = hash >> range_len;
    //             range_start = range_end;
    //         }
    //         (0, buf)
    //     })
    //     .reduce(|| (0, ArrayVec::new()), |up, down| {
    //         let mut buf = ArrayVec::new();
    //         let (mut up, mut down) =
    //     })
}

#[cfg(test)]
mod tests {
    use super::*;
    extern crate test;
    use self::test::Bencher;

    #[bench]
    fn bench_part1(b: &mut Bencher) {
        b.iter(|| part1(INPUT));
    }

    #[bench]
    fn bench_part1_rayon(b: &mut Bencher) {
        b.iter(|| part1_rayon(INPUT));
    }

    #[test]
    fn test_part1() {
        assert_eq!(part1(TEST_INPUT), 8108);
    }
}
