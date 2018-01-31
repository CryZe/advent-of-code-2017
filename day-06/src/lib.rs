#![feature(test)]

extern crate test;

extern crate arrayvec;
extern crate fxhash;

use std::rc::Rc;
use arrayvec::ArrayVec;
use std::collections::{HashMap, HashSet};
use fxhash::FxBuildHasher;

pub const INPUT: &str = include_str!("input.txt");

pub fn part1(text: &str) -> usize {
    type I = u16;
    type V = ArrayVec<[I; 16]>;

    let mut numbers = Rc::new(
        text.split_whitespace()
            .filter_map(|n| n.parse().ok())
            .collect::<V>(),
    );

    let mut set = HashSet::with_hasher(FxBuildHasher::default());
    let mut count = 0;

    while set.insert(numbers.clone()) {
        count += 1;

        let numbers = Rc::make_mut(&mut numbers).as_mut_slice();
        let len = numbers.len();

        let (max_index, max) = numbers
            .iter()
            .cloned()
            .enumerate()
            .max_by_key(|&(i, n)| (n, len - i))
            .unwrap();

        let all = max / len as I;
        let rem = max as usize % len;

        numbers[max_index] = 0;

        for n in numbers.iter_mut() {
            *n += all;
        }

        for n in numbers.iter_mut().skip(max_index + 1).take(rem) {
            *n += 1;
        }

        for n in numbers
            .iter_mut()
            .take((max_index + 1 + rem).saturating_sub(len))
        {
            *n += 1;
        }
    }

    count
}

pub fn part1_simple_loop(text: &str) -> usize {
    type I = u16;
    type V = ArrayVec<[I; 16]>;

    let mut numbers = Rc::new(
        text.split_whitespace()
            .filter_map(|n| n.parse().ok())
            .collect::<V>(),
    );

    let mut set = HashSet::with_hasher(FxBuildHasher::default());
    let mut count = 0;

    while set.insert(numbers.clone()) {
        count += 1;

        let numbers = Rc::make_mut(&mut numbers).as_mut_slice();
        let len = numbers.len();

        let (mut index, rem) = numbers
            .iter()
            .cloned()
            .enumerate()
            .max_by_key(|&(i, n)| (n, len - i))
            .unwrap();

        numbers[index] = 0;

        for _ in 0..rem {
            index += 1;
            if index >= len {
                index -= len;
            }
            numbers[index] += 1;
        }
    }

    count
}

pub fn part1_simple_loop_vec_with_data(mut numbers: Vec<u16>) -> (Vec<u16>, usize) {
    let mut set = HashSet::with_hasher(FxBuildHasher::default());
    let mut count = 0;

    let found = loop {
        if let Some(found) = set.replace(numbers.clone()) {
            break found;
        }
        count += 1;

        let len = numbers.len();

        let (mut index, rem) = numbers
            .iter()
            .cloned()
            .enumerate()
            .rev()
            .max_by_key(|&(_, n)| n)
            .unwrap();

        numbers[index] = 0;

        for _ in 0..rem {
            index += 1;
            if index >= len {
                index -= len;
            }
            numbers[index] += 1;
        }
    };

    (found, count)
}

pub fn part1_simple_loop_array_vec_with_data(
    mut numbers: ArrayVec<[u16; 16]>,
) -> (usize, ArrayVec<[u16; 16]>) {
    let mut set = HashSet::with_hasher(FxBuildHasher::default());
    let mut count = 0;

    loop {
        if let Some(found) = set.replace(numbers.clone()) {
            return (count, found);
        }
        count += 1;

        let len = numbers.len();

        let (mut index, rem) = numbers
            .iter()
            .cloned()
            .enumerate()
            .rev()
            .max_by_key(|&(_, n)| n)
            .unwrap();

        numbers[index] = 0;

        for _ in 0..rem {
            index += 1;
            if index >= len {
                index -= len;
            }
            numbers[index] += 1;
        }
    }
}

pub fn part1_simple_loop_array_with_data(mut numbers: [u16; 16]) -> (usize, [u16; 16]) {
    let mut set = HashSet::with_hasher(FxBuildHasher::default());
    let mut count = 0;

    loop {
        if let Some(found) = set.replace(numbers.clone()) {
            return (count, found);
        }
        count += 1;

        let len = numbers.len();

        let (mut index, mut max) = (0, 0);
        for (i, &n) in numbers.iter().enumerate() {
            if n > max {
                index = i;
                max = n;
            }
        }

        numbers[index] = 0;

        for _ in 0..max {
            index += 1;
            if index >= len {
                index -= len;
            }
            numbers[index] += 1;
        }
    }
}

pub fn part1_simple_loop_vec(text: &str) -> usize {
    part1_simple_loop_vec_with_data(
        text.split_whitespace()
            .filter_map(|n| n.parse().ok())
            .collect(),
    ).1
}

pub fn part1_simple_loop_array_vec(text: &str) -> usize {
    part1_simple_loop_array_vec_with_data(
        text.split_whitespace()
            .filter_map(|n| n.parse().ok())
            .collect(),
    ).0
}

pub fn part1_simple_loop_array(text: &str) -> usize {
    let mut numbers: [u16; 16] = unsafe { std::mem::uninitialized() };
    for (d, n) in numbers
        .iter_mut()
        .zip(text.split_whitespace().filter_map(|n| n.parse().ok()))
    {
        *d = n;
    }

    let mut set = HashSet::with_hasher(FxBuildHasher::default());
    let mut count = 0;

    while set.insert(numbers.clone()) {
        count += 1;

        let len = numbers.len();

        let (mut index, mut max) = (0, 0);
        for (i, &n) in numbers.iter().enumerate() {
            if n > max {
                index = i;
                max = n;
            }
        }

        numbers[index] = 0;

        for _ in 0..max {
            index += 1;
            if index >= len {
                index -= len;
            }
            numbers[index] += 1;
        }
    }

    count
}

pub fn part1_archer(text: &str) -> usize {
    let mut numbers: [u16; 16] = unsafe { std::mem::uninitialized() };
    for (d, n) in numbers
        .iter_mut()
        .zip(text.split_whitespace().filter_map(|n| n.parse().ok()))
    {
        *d = n;
    }

    let mut set = HashSet::with_hasher(FxBuildHasher::default());
    let mut count = 0;

    while set.insert(numbers.clone()) {
        count += 1;

        let (mut index, mut max) = (0, 0);
        for (i, &n) in numbers.iter().enumerate() {
            if n > max {
                index = i;
                max = n;
            }
        }

        numbers[index] = 0;

        for entry in numbers.iter_mut().skip(index + 1) {
            if max > 0 {
                *entry += 1;
                max -= 1;
            } else {
                break;
            }
        }

        while max > 0 {
            for entry in numbers.iter_mut() {
                if max > 0 {
                    *entry += 1;
                    max -= 1;
                } else {
                    break;
                }
            }
        }
    }

    count
}

pub fn part2(text: &str) -> usize {
    let mut numbers: [u16; 16] = unsafe { std::mem::uninitialized() };
    for (d, n) in numbers
        .iter_mut()
        .zip(text.split_whitespace().filter_map(|n| n.parse().ok()))
    {
        *d = n;
    }

    let mut map = HashMap::with_hasher(FxBuildHasher::default());
    let mut count = 0;

    loop {
        if let Some(original_count) = map.insert(numbers.clone(), count) {
            return count - original_count;
        }
        count += 1;

        let len = numbers.len();

        let (mut index, mut max) = (0, 0);
        for (i, &n) in numbers.iter().enumerate() {
            if n > max {
                index = i;
                max = n;
            }
        }

        numbers[index] = 0;

        for _ in 0..max {
            index += 1;
            if index >= len {
                index -= len;
            }
            numbers[index] += 1;
        }
    }
}

pub fn part2_archer(text: &str) -> usize {
    let mut numbers: [u16; 16] = unsafe { std::mem::uninitialized() };
    for (d, n) in numbers
        .iter_mut()
        .zip(text.split_whitespace().filter_map(|n| n.parse().ok()))
    {
        *d = n;
    }

    let mut map = HashMap::with_hasher(FxBuildHasher::default());
    let mut count = 0;

    loop {
        if let Some(original_count) = map.insert(numbers.clone(), count) {
            return count - original_count;
        }
        count += 1;

        let (mut index, mut max) = (0, 0);
        for (i, &n) in numbers.iter().enumerate() {
            if n > max {
                index = i;
                max = n;
            }
        }

        numbers[index] = 0;

        for entry in numbers.iter_mut().skip(index + 1) {
            if max > 0 {
                *entry += 1;
                max -= 1;
            } else {
                break;
            }
        }

        while max > 0 {
            for entry in numbers.iter_mut() {
                if max > 0 {
                    *entry += 1;
                    max -= 1;
                } else {
                    break;
                }
            }
        }
    }
}

pub fn part2_array_set(text: &str) -> usize {
    let mut numbers: [u16; 16] = unsafe { std::mem::uninitialized() };
    for (d, n) in numbers
        .iter_mut()
        .zip(text.split_whitespace().filter_map(|n| n.parse().ok()))
    {
        *d = n;
    }

    let (_, found) = part1_simple_loop_array_with_data(numbers);
    part1_simple_loop_array_with_data(found).0
}

fn redistribute(numbers: &mut [u16; 16]) {
    let (mut index, mut max) = (0, 0);
    for (i, &n) in numbers.iter().enumerate() {
        if n > max {
            index = i;
            max = n;
        }
    }

    numbers[index] = 0;

    for entry in numbers.iter_mut().skip(index + 1) {
        if max > 0 {
            *entry += 1;
            max -= 1;
        } else {
            break;
        }
    }

    while max > 0 {
        for entry in numbers.iter_mut() {
            if max > 0 {
                *entry += 1;
                max -= 1;
            } else {
                break;
            }
        }
    }
}

pub fn floyd<T, F>(mut f: F, x0: T) -> (usize, usize)
where
    T: Clone + PartialEq,
    F: FnMut(&mut T),
{
    let mut tortoise = x0.clone();
    f(&mut tortoise);

    let mut hare = x0.clone();
    f(&mut hare);
    f(&mut hare);

    while tortoise != hare {
        f(&mut tortoise);
        f(&mut hare);
        f(&mut hare);
    }

    let mut mu = 0;
    tortoise = x0;
    while tortoise != hare {
        f(&mut tortoise);
        f(&mut hare);
        mu += 1;
    }

    let mut lam = 1;
    hare = tortoise.clone();
    f(&mut hare);
    while tortoise != hare {
        f(&mut hare);
        lam += 1;
    }

    (lam, mu)
}

pub fn brent<T, F>(mut f: F, x0: T) -> (usize, usize)
where
    T: Clone + PartialEq,
    F: FnMut(&mut T),
{
    let (mut power, mut lam) = (1, 1);

    let mut tortoise = x0.clone();

    let mut hare = x0.clone();
    f(&mut hare);

    while tortoise != hare {
        if power == lam {
            tortoise = hare.clone();
            power <<= 1;
            lam = 0;
        }
        f(&mut hare);
        lam += 1;
    }

    let mut mu = 0;
    tortoise = x0.clone();
    hare = x0;

    for _ in 0..lam {
        f(&mut hare);
    }

    while tortoise != hare {
        f(&mut tortoise);
        f(&mut hare);
        mu += 1;
    }

    (lam, mu)
}

pub fn part1_floyd(text: &str) -> usize {
    let mut numbers: [u16; 16] = unsafe { std::mem::uninitialized() };
    for (d, n) in numbers
        .iter_mut()
        .zip(text.split_whitespace().filter_map(|n| n.parse().ok()))
    {
        *d = n;
    }

    let (lam, mu) = floyd(redistribute, numbers);
    lam + mu
}

pub fn part2_floyd(text: &str) -> usize {
    let mut numbers: [u16; 16] = unsafe { std::mem::uninitialized() };
    for (d, n) in numbers
        .iter_mut()
        .zip(text.split_whitespace().filter_map(|n| n.parse().ok()))
    {
        *d = n;
    }

    let (lam, _) = floyd(redistribute, numbers);
    lam
}

pub fn part1_brent(text: &str) -> usize {
    let mut numbers: [u16; 16] = unsafe { std::mem::uninitialized() };
    for (d, n) in numbers
        .iter_mut()
        .zip(text.split_whitespace().filter_map(|n| n.parse().ok()))
    {
        *d = n;
    }

    let (lam, mu) = brent(redistribute, numbers);
    lam + mu
}

pub fn part2_brent(text: &str) -> usize {
    let mut numbers: [u16; 16] = unsafe { std::mem::uninitialized() };
    for (d, n) in numbers
        .iter_mut()
        .zip(text.split_whitespace().filter_map(|n| n.parse().ok()))
    {
        *d = n;
    }

    let (lam, _) = brent(redistribute, numbers);
    lam
}

#[cfg(test)]
mod tests {
    use super::*;
    use test::Bencher;

    // #[bench]
    // fn bench_part1(b: &mut Bencher) {
    //     b.iter(|| part1(INPUT));
    // }

    // #[bench]
    // fn bench_part1_simple_loop(b: &mut Bencher) {
    //     b.iter(|| part1_simple_loop(INPUT));
    // }

    // #[bench]
    // fn bench_part1_simple_loop_vec(b: &mut Bencher) {
    //     b.iter(|| part1_simple_loop_vec(INPUT));
    // }

    // #[bench]
    // fn bench_part1_simple_loop_array_vec(b: &mut Bencher) {
    //     b.iter(|| part1_simple_loop_array_vec(INPUT));
    // }

    #[bench]
    fn bench_part1_simple_loop_array(b: &mut Bencher) {
        b.iter(|| part1_simple_loop_array(INPUT));
    }

    #[bench]
    fn bench_part1_archer(b: &mut Bencher) {
        b.iter(|| part1_archer(INPUT));
    }

    #[bench]
    fn bench_part2_cryze(b: &mut Bencher) {
        b.iter(|| part2(INPUT));
    }

    #[bench]
    fn bench_part2_archer(b: &mut Bencher) {
        b.iter(|| part2_archer(INPUT));
    }

    #[bench]
    fn bench_part1_floyd(b: &mut Bencher) {
        b.iter(|| part1_floyd(INPUT));
    }

    #[bench]
    fn bench_part2_floyd(b: &mut Bencher) {
        b.iter(|| part2_floyd(INPUT));
    }

    #[bench]
    fn bench_part1_brent(b: &mut Bencher) {
        b.iter(|| part1_brent(INPUT));
    }

    #[bench]
    fn bench_part2_brent(b: &mut Bencher) {
        b.iter(|| part2_brent(INPUT));
    }

    // #[bench]
    // fn bench_part2_array_set(b: &mut Bencher) {
    //     b.iter(|| part2_array_set(INPUT));
    // }
}
