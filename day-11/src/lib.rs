#![feature(test)]
extern crate test;

extern crate rayon;

use rayon::prelude::*;
use std::str::FromStr;

pub const INPUT: &str = include_str!("input.txt");
// pub const LEAH_INPUT: &str = include_str!("leah_input.txt");

type Pos = (isize, isize);
pub const ORIGIN: Pos = (0, 0);

#[derive(Copy, Clone, PartialEq)]
enum Dir {
    North,
    NorthEast,
    NorthWest,
    South,
    SouthEast,
    SouthWest,
}

impl FromStr for Dir {
    type Err = ();
    #[inline]
    fn from_str(s: &str) -> Result<Self, ()> {
        match s {
            "n" => Ok(Dir::North),
            "ne" => Ok(Dir::NorthEast),
            "nw" => Ok(Dir::NorthWest),
            "s" => Ok(Dir::South),
            "se" => Ok(Dir::SouthEast),
            "sw" => Ok(Dir::SouthWest),
            _ => Err(()),
        }
    }
}

impl Dir {
    fn apply(self, (x, y): Pos) -> Pos {
        match self {
            Dir::North => (x - 1, y),
            Dir::South => (x + 1, y),
            Dir::NorthEast => (x - 1, y + 1),
            Dir::SouthWest => (x + 1, y - 1),
            Dir::NorthWest => (x, y - 1),
            Dir::SouthEast => (x, y + 1),
        }
    }
}

fn dist((x1, y1): Pos, (x2, y2): Pos) -> usize {
    (x1 - x2)
        .abs()
        .max((y1 - y2).abs())
        .max(((-x1 - y1) - (-x2 - y2)).abs()) as usize
}

pub fn part1(text: &str) -> usize {
    let mut pos = ORIGIN;
    for dir in text.split(',').filter_map(|d| d.parse::<Dir>().ok()) {
        pos = dir.apply(pos);
    }
    dist(pos, ORIGIN)
}

pub fn part2(text: &str) -> usize {
    let mut pos = ORIGIN;
    let mut max_dist = 0;
    for dir in text.split(',').filter_map(|d| d.parse::<Dir>().ok()) {
        pos = dir.apply(pos);
        max_dist = max_dist.max(dist(pos, ORIGIN));
    }
    max_dist
}

pub fn part1_rayon(text: &str) -> usize {
    let pos = text.par_split(',')
        .filter_map(|d| d.parse::<Dir>().ok())
        .fold(|| ORIGIN, |pos, dir| dir.apply(pos))
        .reduce(|| ORIGIN, |(x1, y1), (x2, y2)| (x1 + x2, y1 + y2));

    dist(pos, ORIGIN)
}

pub fn part1_rayon_map_reduce(text: &str) -> usize {
    let pos = text.par_split(',')
        .filter_map(|d| d.parse::<Dir>().ok())
        .map(|dir| dir.apply(ORIGIN))
        .reduce(|| ORIGIN, |(x1, y1), (x2, y2)| (x1 + x2, y1 + y2));

    dist(pos, ORIGIN)
}

pub fn part1_flat(text: &str) -> usize {
    let (mut x, mut y) = (0isize, 0isize);
    for (dx, dy) in text.split(',').filter_map(|d| {
        Some(match d {
            "n" => (-1, 0),
            "s" => (1, 0),
            "ne" => (-1, 1),
            "sw" => (1, -1),
            "nw" => (0, -1),
            "se" => (0, 1),
            _ => return None,
        })
    }) {
        x += dx;
        y += dy;
    }

    x.abs().max(y.abs()).max((-x - y).abs()) as usize
}

pub fn part1_dirs(text: &str) -> usize {
    let (mut n, mut sw, mut se) = (0isize, 0isize, 0isize);
    for s in text.split(',') {
        match s {
            "n" => n += 1,
            "s" => n -= 1,
            "sw" => sw += 1,
            "ne" => sw -= 1,
            "se" => se += 1,
            "nw" => se -= 1,
            _ => {}
        }
    }
    (n.max(sw).max(se) - n.min(sw).min(se)) as usize
}

pub fn part2_dirs(text: &str) -> usize {
    let (mut n, mut sw, mut se) = (0isize, 0isize, 0isize);
    let mut max_dist = 0;
    for s in text.split(',') {
        match s {
            "n" => n += 1,
            "s" => n -= 1,
            "sw" => sw += 1,
            "ne" => sw -= 1,
            "se" => se += 1,
            "nw" => se -= 1,
            _ => {}
        }
        max_dist = max_dist.max(n.max(sw).max(se) - n.min(sw).min(se));
    }
    max_dist as usize
}

pub fn part1_rayon_map_reduce_flat(text: &str) -> usize {
    let (x, y): (isize, isize) = text.par_split(',')
        .filter_map(|d| {
            Some(match d {
                "n" => (-1, 0),
                "s" => (1, 0),
                "ne" => (-1, 1),
                "sw" => (1, -1),
                "nw" => (0, -1),
                "se" => (0, 1),
                _ => return None,
            })
        })
        .reduce(|| (0, 0), |(x1, y1), (x2, y2)| (x1 + x2, y1 + y2));

    x.abs().max(y.abs()).max((x + y).abs()) as usize
}

#[cfg(test)]
mod tests {
    use super::*;
    use test::Bencher;

    #[bench]
    fn bench_part1(b: &mut Bencher) {
        b.iter(|| part1(INPUT));
    }

    // #[bench]
    // fn bench_part1_rayon(b: &mut Bencher) {
    //     b.iter(|| part1_rayon(INPUT));
    // }

    #[bench]
    fn bench_part1_rayon_map_reduce(b: &mut Bencher) {
        b.iter(|| part1_rayon_map_reduce(INPUT));
    }

    #[bench]
    fn bench_part1_rayon_map_reduce_flat(b: &mut Bencher) {
        b.iter(|| part1_rayon_map_reduce_flat(INPUT));
    }

    // #[bench]
    // fn bench_part1_flat(b: &mut Bencher) {
    //     b.iter(|| part1_flat(INPUT));
    // }

    #[bench]
    fn bench_part1_dirs(b: &mut Bencher) {
        b.iter(|| part1_dirs(INPUT));
    }

    #[bench]
    fn bench_part2_dirs(b: &mut Bencher) {
        b.iter(|| part2_dirs(INPUT));
    }

    #[bench]
    fn bench_part2(b: &mut Bencher) {
        b.iter(|| part2(INPUT));
    }

    // #[bench]
    // fn bench_part2_lokathor(b: &mut Bencher) {
    //     b.iter(|| lokathor::part2(INPUT));
    // }

    #[test]
    fn test_part1() {
        assert_eq!(part1("ne,ne,ne"), 3);
        assert_eq!(part1("ne,ne,sw,sw"), 0);
        assert_eq!(part1("ne,ne,s,s"), 2);
        assert_eq!(part1("se,sw,se,sw,sw"), 3);
    }

    #[test]
    fn test_part1_dirs() {
        assert_eq!(part1_dirs("ne,ne,ne"), 3);
        assert_eq!(part1_dirs("ne,ne,sw,sw"), 0);
        assert_eq!(part1_dirs("ne,ne,s,s"), 2);
        assert_eq!(part1_dirs("se,sw,se,sw,sw"), 3);
    }
}
