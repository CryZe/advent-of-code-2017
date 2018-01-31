#![feature(conservative_impl_trait, inclusive_range_syntax)]
#![feature(test)]
extern crate test;

extern crate arrayvec;
// #[macro_use]
// extern crate enum_map;
extern crate rayon;

use arrayvec::{ArrayString, ArrayVec};
use std::fmt::Write;
use rayon::prelude::*;
use std::io::{Cursor, Write as IoWrite};

// pub mod xfix;
pub mod wlandry;

pub const INPUT: &str = "97,167,54,178,2,11,209,174,119,248,254,0,255,1,64,190";

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

pub fn part1(text: &str) -> u16 {
    let mut list = initial_list();
    let (mut index, mut skip_size) = (0, 0);

    round(
        &mut list,
        text.split(',').filter_map(|l| l.parse().ok()),
        &mut index,
        &mut skip_size,
    );

    list[0] as u16 * list[1] as u16
}

pub fn part2(text: &str) -> ArrayString<[u8; 32]> {
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

    let mut output = ArrayString::new();
    for c in list.chunks(16).map(|c| c.iter().fold(0, |a, b| a ^ b)) {
        write!(output, "{:02x}", c).unwrap();
    }
    output
}

pub fn part1_no_list(text: &str) -> u16 {
    let mut left_right = ArrayVec::<[(u8, u8); 16]>::new();

    let (mut index, mut skip_size) = (0u8, 0u8);
    for len in text.split(',').filter_map(|l| l.parse().ok()) {
        left_right.push((index, index.wrapping_add(len)));
        index = index.wrapping_add(len).wrapping_add(skip_size);
        skip_size = skip_size.wrapping_add(1);
    }

    let mut tracking = [0, 1];

    for &(left, right) in left_right.iter().rev() {
        if left < right {
            for val in &mut tracking {
                if *val >= left && *val < right {
                    *val = right.wrapping_sub(1).wrapping_sub(val.wrapping_sub(left));
                }
            }
        } else if left > right {
            for val in &mut tracking {
                if *val >= left || *val < right {
                    *val = right.wrapping_sub(1).wrapping_sub(val.wrapping_sub(left));
                }
            }
        }
    }

    tracking[0] as u16 * tracking[1] as u16
}

pub fn part1_no_list_no_array_vec(text: &str) -> u16 {
    let mut left_right = [(0, 0); 16];
    let mut lr_len = 0;

    let (mut index, mut skip_size) = (0u8, 0u8);
    for len in text.split(',').filter_map(|l| l.parse().ok()) {
        left_right[lr_len] = (index, index.wrapping_add(len));
        lr_len += 1;
        index = index.wrapping_add(len).wrapping_add(skip_size);
        skip_size = skip_size.wrapping_add(1);
    }

    let mut tracking = [0, 1];

    for &(left, right) in left_right.iter().rev() {
        if left < right {
            for val in &mut tracking {
                if *val >= left && *val < right {
                    *val = right.wrapping_sub(1).wrapping_sub(val.wrapping_sub(left));
                }
            }
        } else if left > right {
            for val in &mut tracking {
                if *val >= left || *val < right {
                    *val = right.wrapping_sub(1).wrapping_sub(val.wrapping_sub(left));
                }
            }
        }
    }

    tracking[0] as u16 * tracking[1] as u16
}

pub fn part2_no_list_rayon(text: &str) -> ArrayString<[u8; 32]> {
    let mut left_right = ArrayVec::<[(u8, u8); 4096]>::new();
    let mut output = ArrayString::new();
    unsafe { output.set_len(32) };

    let (mut index, mut skip_size) = (0u8, 0u8);
    for _ in 0..64 {
        for len in text.bytes().chain([17, 31, 73, 47, 23].iter().cloned()) {
            left_right.push((index, index.wrapping_add(len)));
            index = index.wrapping_add(len).wrapping_add(skip_size);
            skip_size = skip_size.wrapping_add(1);
        }
    }

    let left_right = left_right.as_slice();

    (0..16)
        .into_par_iter()
        .map(|c| {
            (16 * c..16 * (c + 1))
                .into_par_iter()
                .map(|val| {
                    let mut val = val as u8;
                    for &(left, right) in left_right.iter().rev() {
                        if left < right {
                            if val >= left && val < right {
                                val = right.wrapping_sub(1).wrapping_sub(val.wrapping_sub(left));
                            }
                        } else if left > right {
                            if val >= left || val < right {
                                val = right.wrapping_sub(1).wrapping_sub(val.wrapping_sub(left));
                            }
                        }
                    }
                    val
                })
                .reduce(|| 0, |a, b| a ^ b)
        })
        .zip(unsafe { output.as_bytes_mut().par_chunks_mut(2) })
        .for_each(|(s, d)| {
            write!(Cursor::new(d), "{:02x}", s).unwrap();
        });

    output
}

#[cfg(test)]
mod tests {
    use super::*;
    use test::Bencher;

    #[bench]
    fn bench_part1(b: &mut Bencher) {
        b.iter(|| part1(INPUT));
    }

    #[bench]
    fn bench_part1_no_list(b: &mut Bencher) {
        b.iter(|| part1_no_list(INPUT));
    }

    #[bench]
    fn bench_part1_no_list_no_array_vec(b: &mut Bencher) {
        b.iter(|| part1_no_list_no_array_vec(INPUT));
    }

    #[bench]
    fn bench_part2(b: &mut Bencher) {
        b.iter(|| part2(INPUT));
    }

    // #[bench]
    // fn bench_part1_xfix(b: &mut Bencher) {
    //     b.iter(|| xfix::part1(INPUT));
    // }

    // #[bench]
    // fn bench_part2_xfix(b: &mut Bencher) {
    //     b.iter(|| xfix::part2(INPUT));
    // }

    #[bench]
    fn bench_part2_wlandry(b: &mut Bencher) {
        b.iter(|| wlandry::part2(INPUT));
    }

    // #[bench]
    // fn bench_part2_no_list_rayon(b: &mut Bencher) {
    //     b.iter(|| part2_no_list_rayon(INPUT));
    // }

    #[test]
    fn test_no_list() {
        part1_no_list(INPUT);
    }

    #[test]
    fn test_part2() {
        assert_eq!(&part2(""), "a2582a3a0e66e6e86e3812dcb672a272");
        assert_eq!(&part2("AoC 2017"), "33efeb34ea91902bb2f59c9920caa6cd");
        assert_eq!(&part2("1,2,3"), "3efbe78a8d82f29979031a4aa0b16a9d");
    }
}
