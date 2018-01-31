#![feature(test, conservative_impl_trait, generator_trait, generators)]

pub mod rerz;

pub const INPUT: u64 = 289326;

pub fn part1(num: u64) -> u64 {
    let or = (num as f64).sqrt() as u64 - 1;
    let rw = or | 1;
    let br = rw * rw;
    if num == br {
        return or;
    }
    let rw = rw + 2;
    let radius = rw >> 1;
    let mut edge_pos = num - br;
    let rwm1 = rw - 1;
    if edge_pos >= rwm1 {
        edge_pos -= rwm1;
    }
    if edge_pos >= rwm1 {
        edge_pos -= rwm1;
    }
    if edge_pos >= rwm1 {
        edge_pos -= rwm1;
    }
    let edge_pos_from_middle = (radius as i64 - edge_pos as i64).abs() as u64;
    radius + edge_pos_from_middle
}

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

fn part2_iter() -> impl Iterator<Item = u64> {
    GenIter(|| {
        let (mut x, mut y) = (0i8, 0i8);
        let (mut dx, mut dy) = (1, 0);
        let mut cache = [[None; 24]; 24];
        cache[10][10] = Some(1);

        yield 1;

        let dirs = [
            (1, 0),
            (1, 1),
            (0, 1),
            (-1, 1),
            (-1, 0),
            (-1, -1),
            (0, -1),
            (1, -1),
        ];

        loop {
            x += dx;
            y += dy;
            if if dx == 1 && dy == 0 {
                x - 1 == -y
            } else {
                x.abs() == y.abs()
            } {
                let (ndx, ndy) = (-dy, dx);
                dx = ndx;
                dy = ndy;
            }

            let mut sum = 0;
            for &(dx, dy) in &dirs {
                if let Some(val) = cache[(x + dx + 10) as usize][(y + dy + 10) as usize] {
                    sum += val;
                }
            }

            cache[(x + 10) as usize][(y + 10) as usize] = Some(sum);
            yield sum;
        }
    })
}

pub fn part2(num: u64) -> u64 {
    part2_iter().find(|&n| n > num).unwrap()
}

fn part2_iter_u32() -> impl Iterator<Item = u32> {
    GenIter(|| {
        let (mut x, mut y) = (0i8, 0i8);
        let (mut dx, mut dy) = (1, 0);
        let mut cache = [[None; 15]; 15];
        cache[7][7] = Some(1);

        yield 1;

        let dirs = [
            (1, 0),
            (1, 1),
            (0, 1),
            (-1, 1),
            (-1, 0),
            (-1, -1),
            (0, -1),
            (1, -1),
        ];

        loop {
            x += dx;
            y += dy;
            if if dx == 1 && dy == 0 {
                x - 1 == -y
            } else {
                x.abs() == y.abs()
            } {
                let (ndx, ndy) = (-dy, dx);
                dx = ndx;
                dy = ndy;
            }

            let mut sum = 0;
            for &(dx, dy) in &dirs {
                if let Some(val) = cache[(x + dx + 7) as usize][(y + dy + 7) as usize] {
                    sum += val;
                }
            }

            cache[(x + 7) as usize][(y + 7) as usize] = Some(sum);
            yield sum;
        }
    })
}

pub fn part2_u32(num: u32) -> u32 {
    part2_iter_u32().find(|&n| n > num).unwrap()
}

extern crate test;

#[cfg(test)]
mod tests {
    use super::*;
    use test::Bencher;

    #[test]
    fn test_part1() {
        assert_eq!(part1(1), 0);
        assert_eq!(part1(12), 3);
        assert_eq!(part1(23), 2);
        assert_eq!(part1(25), 4);
        assert_eq!(part1(26), 5);
        assert_eq!(part1(27), 4);
        assert_eq!(part1(28), 3);
        assert_eq!(part1(1024), 31);
    }

    // #[test]
    // fn test_part2() {
    //     assert_eq!(part2(1), 1);
    //     assert_eq!(part2(2), 1);
    //     assert_eq!(part2(3), 2);
    //     assert_eq!(part2(4), 4);
    //     assert_eq!(part2(5), 5);
    //     assert_eq!(part2(6), 10);
    //     assert_eq!(part2(7), 11);
    //     assert_eq!(part2(8), 23);
    //     assert_eq!(part2(9), 25);
    //     assert_eq!(part2(10), 26);
    //     assert_eq!(part2(11), 54);
    //     assert_eq!(part2(12), 57);
    //     assert_eq!(part2(13), 59);
    //     assert_eq!(part2(14), 122);
    //     assert_eq!(part2(15), 133);
    //     assert_eq!(part2(16), 142);
    //     assert_eq!(part2(17), 147);
    //     assert_eq!(part2(18), 304);
    //     assert_eq!(part2(19), 330);
    //     assert_eq!(part2(20), 351);
    //     assert_eq!(part2(21), 362);
    //     assert_eq!(part2(22), 747);
    //     assert_eq!(part2(23), 806);
    // }

    #[bench]
    fn bench_part1(b: &mut Bencher) {
        // use std::io::Read;
        // let mut s = String::new();
        // std::fs::File::open("src/input.txt")
        //     .unwrap()
        //     .read_to_string(&mut s)
        //     .unwrap();
        // let n = s.trim().parse().unwrap();

        let n = test::black_box(INPUT);
        b.iter(|| test::black_box(part1(n)));
    }

    #[bench]
    fn bench_part2(b: &mut Bencher) {
        // let n = test::black_box(INPUT);
        b.iter(|| part2(INPUT));
    }

    #[bench]
    fn bench_part2_u32(b: &mut Bencher) {
        // let n = test::black_box(INPUT);
        let n = INPUT as u32;
        b.iter(|| part2_u32(n));
    }

    #[bench]
    fn bench_rerz(b: &mut Bencher) {
        // let n = test::black_box(INPUT);
        b.iter(|| rerz::day3_2(INPUT as _));
    }
}
