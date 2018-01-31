#![feature(i128_type)]
#![cfg_attr(test, feature(test))]

extern crate arrayvec;
extern crate fxhash;
extern crate rayon;

use arrayvec::ArrayVec;
use rayon::prelude::*;
use fxhash::FxHashMap;

pub const INPUT: &str = include_str!("input.txt");
pub const TEST_INPUT: &str = "0: 3
1: 2
4: 4
6: 4";

pub fn part1(text: &str) -> usize {
    text.lines()
        .filter_map(|l| {
            let mut splits = l.split(':');
            let depth: usize = splits.next()?.parse().ok()?;
            let range: usize = splits.next()?.get(1..)?.parse().ok()?;
            let mod_range = (2 * range).checked_sub(2)?;
            if mod_range > 0 && depth % mod_range == 0 {
                Some(depth * range)
            } else {
                None
            }
        })
        .sum()
}

pub fn part1_rayon(text: &str) -> usize {
    text.par_lines()
        .filter_map(|l| {
            let mut splits = l.split(':');
            let depth: usize = splits.next()?.parse().ok()?;
            let range: usize = splits.next()?.get(1..)?.parse().ok()?;
            let mod_range = (2 * range).checked_sub(2)?;
            if mod_range > 0 && depth % mod_range == 0 {
                Some(depth * range)
            } else {
                None
            }
        })
        .sum()
}

pub fn part2(text: &str) -> usize {
    let mut delay = 0;
    while text.lines()
        .filter_map(|l| {
            let mut splits = l.split(':');
            let depth: usize = splits.next()?.parse().ok()?;
            let range: usize = splits.next()?.get(1..)?.parse().ok()?;
            Some((depth, (2 * range).checked_sub(2)?))
        })
        .any(|(depth, mod_range)| (depth + delay) % mod_range == 0)
    {
        delay += 1;
    }
    delay
}

pub fn part2_collect(text: &str) -> usize {
    let parsed = text.lines()
        .filter_map(|l| {
            let mut splits = l.split(':');
            let depth: usize = splits.next()?.parse().ok()?;
            let range: usize = splits.next()?.get(1..)?.parse().ok()?;
            Some((depth, (2 * range).checked_sub(2)?))
        })
        .collect::<ArrayVec<[_; 100]>>();

    let mut delay = 0;
    while parsed
        .iter()
        .any(|&(depth, mod_range)| (depth + delay) % mod_range == 0)
    {
        delay += 1;
    }
    delay
}

pub fn part2_collect_u32(text: &str) -> u32 {
    let parsed = text.lines()
        .filter_map(|l| {
            let mut splits = l.split(':');
            let depth: u32 = splits.next()?.parse().ok()?;
            let range: u32 = splits.next()?.get(1..)?.parse().ok()?;
            Some((depth, (2 * range).checked_sub(2)?))
        })
        .collect::<ArrayVec<[_; 100]>>();
    let parsed = parsed.as_slice();

    let mut delay = 0;
    while parsed
        .iter()
        .any(|&(depth, mod_range)| (depth + delay) % mod_range == 0)
    {
        delay += 1;
    }
    delay
}

pub fn part2_collect_u32_sort(text: &str) -> u32 {
    let mut parsed = text.lines()
        .filter_map(|l| {
            let mut splits = l.split(':');
            let depth: u32 = splits.next()?.parse().ok()?;
            let range: u32 = splits.next()?.get(1..)?.parse().ok()?;
            Some((depth, 2 * range - 2))
        })
        .collect::<ArrayVec<[_; 100]>>();

    let parsed = parsed.as_mut_slice();
    parsed.sort_unstable_by_key(|&(_, r)| r);

    let mut delay = 0;
    while parsed
        .iter()
        .any(|&(depth, mod_range)| (depth + delay) % mod_range == 0)
    {
        delay += 1;
    }
    delay
}

pub fn part2_askalski(text: &str) -> i32 {
    let mut map = FxHashMap::<i32, u128>::default();
    for (depth, mod_range) in text.lines().filter_map(|l| {
        let mut splits = l.split(':');
        let depth: i32 = splits.next()?.parse().ok()?;
        let range: i32 = splits.next()?.get(1..)?.parse().ok()?;
        Some((depth, 2 * range - 2))
    }) {
        *map.entry(mod_range).or_insert(0) |=
            1 << (((mod_range - depth % mod_range) % mod_range) as u128);
    }

    for (&first, second) in &mut map {
        let mut shift = first;
        while shift < 128 {
            *second |= *second << shift;
            shift <<= 1;
        }
    }

    let mut delay = 0i32;
    loop {
        let mut m = 0u64;
        for (first, second) in &map {
            m |= (second >> (delay % first)) as u64;
        }
        if !m != 0 {
            delay += (!m).trailing_zeros() as i32;
            return delay;
        }

        delay += 64;
    }
}

pub fn part2_askalski_rayon(text: &str) -> i32 {
    let mut map = FxHashMap::<i32, u128>::default();
    for (depth, mod_range) in text.lines().filter_map(|l| {
        let mut splits = l.split(':');
        let depth: i32 = splits.next()?.parse().ok()?;
        let range: i32 = splits.next()?.get(1..)?.parse().ok()?;
        Some((depth, 2 * range - 2))
    }) {
        *map.entry(mod_range).or_insert(0) |=
            1 << (((mod_range - depth % mod_range) % mod_range) as u128);
    }

    for (&first, second) in &mut map {
        let mut shift = first;
        while shift < 128 {
            *second |= *second << shift;
            shift <<= 1;
        }
    }

    (0..std::i32::MAX)
        .into_par_iter()
        .with_min_len(6000)
        .filter_map(|n| {
            let delay = n * 64;
            let mut m = 0u64;
            for (first, second) in &map {
                m |= (second >> (delay % first)) as u64;
            }
            if !m != 0 {
                Some(delay + (!m).trailing_zeros() as i32)
            } else {
                None
            }
        })
        .find_first(|_| true)
        .unwrap()
}

// pub fn part2_collect_f64(text: &str) -> usize {
//     let parsed = text.lines()
//         .filter_map(|l| {
//             let mut splits = l.split(':');
//             let depth: f64 = splits.next()?.parse().ok()?;
//             let range: f64 = splits.next()?.get(1..)?.parse().ok()?;
//             Some((depth, (2.0 * range - 2.0)))
//         })
//         .collect::<ArrayVec<[_; 100]>>();

//     let mut delay = 0.0;
//     while parsed
//         .iter()
//         .any(|&(depth, mod_range)| (depth + delay) % mod_range == 0.0)
//     {
//         delay += 1.0;
//     }
//     delay as usize
// }

pub fn part2_collect_rayon(text: &str) -> usize {
    let parsed = text.lines()
        .filter_map(|l| {
            let mut splits = l.split(':');
            let depth: usize = splits.next()?.parse().ok()?;
            let range: usize = splits.next()?.get(1..)?.parse().ok()?;
            Some((depth, (2 * range).checked_sub(2)?))
        })
        .collect::<ArrayVec<[_; 100]>>();

    (0..std::usize::MAX)
        .into_par_iter()
        .with_min_len(100000)
        .find_first(|delay| {
            parsed
                .iter()
                .all(|&(depth, mod_range)| (depth + delay) % mod_range != 0)
        })
        .unwrap()
}

#[cfg(test)]
mod tests {
    use super::*;
    extern crate test;
    use self::test::Bencher;

    // #[bench]
    // fn bench_part1(b: &mut Bencher) {
    //     b.iter(|| part1(INPUT));
    // }

    // #[bench]
    // fn bench_part1_rayon(b: &mut Bencher) {
    //     b.iter(|| part1_rayon(INPUT));
    // }

    // #[bench]
    // fn bench_part2(b: &mut Bencher) {
    //     b.iter(|| part2(INPUT));
    // }

    // #[bench]
    // fn bench_part2_collect(b: &mut Bencher) {
    //     b.iter(|| part2_collect(INPUT));
    // }

    // #[bench]
    // fn bench_part2_collect_u32(b: &mut Bencher) {
    //     b.iter(|| part2_collect_u32(INPUT));
    // }

    #[bench]
    fn bench_part2_collect_u32_sort(b: &mut Bencher) {
        b.iter(|| part2_collect_u32_sort(INPUT));
    }

    #[bench]
    fn bench_part2_askalski(b: &mut Bencher) {
        b.iter(|| part2_askalski(INPUT));
    }

    #[bench]
    fn bench_part2_askalski_rayon(b: &mut Bencher) {
        b.iter(|| part2_askalski_rayon(INPUT));
    }

    // #[bench]
    // fn bench_part2_collect_f64(b: &mut Bencher) {
    //     b.iter(|| part2_collect_f64(INPUT));
    // }

    // #[bench]
    // fn bench_part2_collect_rayon(b: &mut Bencher) {
    //     b.iter(|| part2_collect_rayon(INPUT));
    // }

    #[test]
    fn test_part1() {
        assert_eq!(part1(TEST_INPUT), 24);
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2(TEST_INPUT), 10);
    }

    #[test]
    fn test_part2_askalski() {
        assert_eq!(part2_askalski(TEST_INPUT), 10);
    }

    // #[test]
    // fn test_part2_collect_f64() {
    //     assert_eq!(part2_collect_f64(TEST_INPUT), 10);
    // }
}
