#![feature(test)]
extern crate test;

extern crate arrayvec;
extern crate rayon;

use arrayvec::ArrayVec;
use rayon::prelude::*;
use std::sync::atomic::{AtomicBool, Ordering};
use std::cell::Cell;

pub const INPUT: &str = include_str!("input.txt");
pub const TEST_INPUT: &str = "0 <-> 2
1 <-> 1
2 <-> 0, 3, 4
3 <-> 2, 4
4 <-> 2, 3, 6
5 <-> 6
6 <-> 4, 5";

pub fn part1(text: &str) -> usize {
    let graph = text.par_lines()
        .filter_map(|line| {
            let connections = line.split("> ")
                .skip(1)
                .next()?
                .split(", ")
                .filter_map(|n| n.parse().ok())
                .collect();
            Some((Cell::new(false), connections))
        })
        .collect::<Vec<_>>();

    fn count(graph: &[(Cell<bool>, ArrayVec<[u16; 6]>)], idx: usize) -> usize {
        let (ref visited, ref edges) = graph[idx];

        if visited.get() {
            return 0;
        }
        visited.set(true);

        edges
            .iter()
            .map(|&idx| count(graph, idx as usize))
            .sum::<usize>() + 1
    }

    count(&graph, 0)
}

pub fn part1_rayon(text: &str) -> usize {
    let graph = text.par_lines()
        .filter_map(|line| {
            let connections = line.split("> ")
                .skip(1)
                .next()?
                .split(", ")
                .filter_map(|n| n.parse().ok())
                .collect();
            Some((AtomicBool::new(false), connections))
        })
        .collect::<Vec<_>>();

    fn count(graph: &[(AtomicBool, ArrayVec<[u16; 8]>)], idx: usize) -> usize {
        let (ref visited, ref edges) = graph[idx];

        if visited.load(Ordering::Acquire) {
            return 0;
        }
        visited.store(true, Ordering::Release);

        edges
            .par_iter()
            .map(|&idx| count(graph, idx as usize))
            .sum::<usize>() + 1
    }

    count(&graph, 0)
}

pub fn part2(text: &str) -> usize {
    let graph = text.par_lines()
        .filter_map(|line| {
            let connections = line.split("> ")
                .skip(1)
                .next()?
                .split(", ")
                .filter_map(|n| n.parse().ok())
                .collect();
            Some((Cell::new(false), connections))
        })
        .collect::<Vec<_>>();

    fn visit(graph: &[(Cell<bool>, ArrayVec<[u16; 6]>)], idx: usize) {
        let (ref visited, ref edges) = graph[idx];

        if visited.get() {
            return;
        }
        visited.set(true);

        for &idx in edges {
            visit(graph, idx as usize);
        }
    }

    let mut count = 0;
    for (i, &(ref visited, _)) in graph.iter().enumerate() {
        if !visited.get() {
            visit(&graph, i);
            count += 1;
        }
    }

    count
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
    fn bench_part1_rayon(b: &mut Bencher) {
        b.iter(|| part1_rayon(INPUT));
    }

    #[bench]
    fn bench_part2(b: &mut Bencher) {
        b.iter(|| part2(INPUT));
    }

    #[test]
    fn test_part1() {
        assert_eq!(part1(TEST_INPUT), 6);
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2(TEST_INPUT), 2);
    }
}
