#![feature(test)]

extern crate test;

extern crate arrayvec;
extern crate fxhash;
extern crate rayon;

use arrayvec::ArrayVec;
use std::collections::BTreeMap;
use fxhash::FxHashMap;
use std::cell::Cell;
use std::sync::atomic::{AtomicBool, Ordering};
use rayon::prelude::*;

pub mod leah;

pub const INPUT: &str = include_str!("input.txt");

extern "C" {
    fn some_guy_cpp(ptr: *const u8, len: std::os::raw::c_int) -> std::os::raw::c_int;
    fn lunar_cpp(ptr: *const u8, len: std::os::raw::c_int) -> std::os::raw::c_int;
}

pub fn part1(text: &str) -> Option<&str> {
    let mut found = ArrayVec::<[&str; 2048]>::new();
    let mut children = ArrayVec::<[&str; 2048]>::new();

    for line in text.lines() {
        let mut splits = line.splitn(2, "(");
        let word = splits.next()?.trim();
        found.push(word);
        let rem = splits.next()?;
        let mut splits = rem.splitn(2, "-> ").skip(1);
        if let Some(line_children) = splits.next() {
            for child in line_children.split(",") {
                children.push(child.trim());
            }
        }
    }

    let found = found.as_mut_slice();
    let children = children.as_mut_slice();

    found.sort_unstable();
    children.sort_unstable();

    found
        .iter()
        .zip(children.iter())
        .find(|&(f, c)| f != c)
        .map(|(&f, _)| Some(f))
        .unwrap_or_else(|| found.last().cloned())
}

pub fn part1_leah(input: &str) -> &str {
    let mut current = "";
    let mut map = FxHashMap::default();

    for line in input.lines() {
        let mut split = line.splitn(2, " ");
        let name = split.next().unwrap();
        let rem = split.next().unwrap().split("-> ").skip(1).next();
        if let Some(children) = rem {
            for child in children.split(", ") {
                map.insert(child, name);
            }
            current = name;
        }
    }

    while let Some(next) = map.get(current) {
        current = next;
    }

    current
}

pub fn part1_btree(input: &'static str) -> &str {
    let mut current = "";
    let mut map = BTreeMap::default();

    for line in input.lines() {
        let mut split = line.splitn(2, " ");
        let name = split.next().unwrap();
        let rem = split.next().unwrap().split("-> ").skip(1).next();
        if let Some(children) = rem {
            for child in children.split(", ") {
                map.insert(child, name);
            }
            current = name;
        }
    }

    while let Some(next) = map.get(current) {
        current = next;
    }

    current
}

pub fn part2(text: &str) -> usize {
    struct Node<'a> {
        weight: usize,
        children: ArrayVec<[&'a str; 8]>,
    }

    fn balanced_weight(map: &FxHashMap<&str, Node>, key: &str) -> Result<(usize, usize), usize> {
        let node = map.get(key).unwrap();
        let mut sum = 0;
        let (mut value1, mut value2) = (None, None);
        for &child in &node.children {
            let (child_sum, child_weight) = balanced_weight(map, child)?;
            match (child_sum, value1, value2) {
                (n, None, _) => {
                    value1 = Some((n, child_weight, 1));
                }
                (a, Some((b, w, c)), _) if a == b => {
                    value1 = Some((a, w, c + 1));
                }
                (n, _, None) => {
                    value2 = Some((n, child_weight, 1));
                }
                (a, _, Some((b, w, c))) if a == b => {
                    value2 = Some((a, w, c + 1));
                }
                _ => unreachable!(),
            }
            sum += child_sum;
        }

        if let (Some((s1, w1, c1)), Some((s2, w2, c2))) = (value1, value2) {
            if c1 == 1 {
                Err((w1 as isize - (s1 as isize - s2 as isize)) as usize)
            } else if c2 == 1 {
                Err((w2 as isize - (s2 as isize - s1 as isize)) as usize)
            } else {
                unreachable!()
            }
        } else {
            Ok((sum + node.weight, node.weight))
        }
    }

    let mut map = FxHashMap::default();
    let mut base_map = FxHashMap::default();
    let mut current = "";

    for line in text.lines() {
        let mut splits = line.splitn(2, "(");
        let name = splits.next().unwrap().trim();
        let rem = splits.next().unwrap();
        let mut splits = rem.splitn(2, ")");
        let weight = splits.next().unwrap().parse().unwrap();
        let rem = splits.next().unwrap();
        let mut splits = rem.splitn(2, "-> ").skip(1);
        let mut children = ArrayVec::<[&str; 8]>::new();
        if let Some(children_line) = splits.next() {
            for child in children_line.split(", ") {
                children.push(child);
                base_map.insert(child, name);
            }
            current = name;
        }
        map.insert(name, Node { weight, children });
    }

    while let Some(next) = base_map.get(current) {
        current = next;
    }
    balanced_weight(&map, current).unwrap_err()
}

pub fn part2_single_map(text: &str) -> usize {
    struct Node<'a> {
        marked: Cell<bool>,
        weight: usize,
        children: ArrayVec<[&'a str; 8]>,
    }

    fn balanced_weight(map: &FxHashMap<&str, Node>, key: &str) -> Result<(usize, usize), usize> {
        let node = map.get(key).unwrap();
        let mut sum = 0;
        let (mut value1, mut value2) = (None, None);
        for &child in &node.children {
            let (child_sum, child_weight) = balanced_weight(map, child)?;
            match (child_sum, value1, value2) {
                (n, None, _) => {
                    value1 = Some((n, child_weight, 1));
                }
                (a, Some((b, w, c)), _) if a == b => {
                    value1 = Some((a, w, c + 1));
                }
                (n, _, None) => {
                    value2 = Some((n, child_weight, 1));
                }
                (a, _, Some((b, w, c))) if a == b => {
                    value2 = Some((a, w, c + 1));
                }
                _ => unreachable!(),
            }
            sum += child_sum;
        }

        if let (Some((s1, w1, c1)), Some((s2, w2, c2))) = (value1, value2) {
            if c1 == 1 {
                Err((w1 as isize - (s1 as isize - s2 as isize)) as usize)
            } else if c2 == 1 {
                Err((w2 as isize - (s2 as isize - s1 as isize)) as usize)
            } else {
                unreachable!()
            }
        } else {
            Ok((sum + node.weight, node.weight))
        }
    }

    let mut map = FxHashMap::default();

    for line in text.lines() {
        let mut splits = line.splitn(2, "(");
        let name = splits.next().unwrap().trim();
        let rem = splits.next().unwrap();
        let mut splits = rem.splitn(2, ")");
        let weight = splits.next().unwrap().parse().unwrap();
        let rem = splits.next().unwrap();
        let mut splits = rem.splitn(2, "-> ").skip(1);
        let mut children = ArrayVec::<[&str; 8]>::new();
        if let Some(children_line) = splits.next() {
            for child in children_line.split(", ") {
                children.push(child);
            }
        }
        map.insert(
            name,
            Node {
                weight,
                children,
                marked: Cell::new(false),
            },
        );
    }

    for node in map.values() {
        for child in &node.children {
            map[child].marked.set(true);
        }
    }

    let (base, _) = map.iter().find(|&(_, v)| !v.marked.get()).unwrap();

    balanced_weight(&map, base).unwrap_err()
}

pub fn part2_single_map_rayon(text: &str) -> usize {
    struct Node<'a> {
        marked: AtomicBool,
        weight: usize,
        children: ArrayVec<[&'a str; 8]>,
    }

    fn balanced_weight(map: &FxHashMap<&str, Node>, key: &str) -> Result<(usize, usize), usize> {
        let node = map.get(key).unwrap();
        let mut sum = 0;
        let (mut value1, mut value2) = (None, None);
        for &child in &node.children {
            let (child_sum, child_weight) = balanced_weight(map, child)?;
            match (child_sum, value1, value2) {
                (n, None, _) => {
                    value1 = Some((n, child_weight, 1));
                }
                (a, Some((b, w, c)), _) if a == b => {
                    value1 = Some((a, w, c + 1));
                }
                (n, _, None) => {
                    value2 = Some((n, child_weight, 1));
                }
                (a, _, Some((b, w, c))) if a == b => {
                    value2 = Some((a, w, c + 1));
                }
                _ => unreachable!(),
            }
            sum += child_sum;
        }

        if let (Some((s1, w1, c1)), Some((s2, w2, c2))) = (value1, value2) {
            if c1 == 1 {
                Err((w1 as isize - (s1 as isize - s2 as isize)) as usize)
            } else if c2 == 1 {
                Err((w2 as isize - (s2 as isize - s1 as isize)) as usize)
            } else {
                unreachable!()
            }
        } else {
            Ok((sum + node.weight, node.weight))
        }
    }

    let mut map = FxHashMap::default();

    for line in text.lines() {
        let mut splits = line.splitn(2, "(");
        let name = splits.next().unwrap().trim();
        let rem = splits.next().unwrap();
        let mut splits = rem.splitn(2, ")");
        let weight = splits.next().unwrap().parse().unwrap();
        let rem = splits.next().unwrap();
        let mut splits = rem.splitn(2, "-> ").skip(1);
        let mut children = ArrayVec::<[&str; 8]>::new();
        if let Some(children_line) = splits.next() {
            for child in children_line.split(", ") {
                children.push(child);
            }
        }
        map.insert(
            name,
            Node {
                weight,
                children,
                marked: AtomicBool::new(false),
            },
        );
    }

    map.values().for_each(|node| {
        for child in &node.children {
            map[child].marked.store(true, Ordering::Release);
        }
    });

    let (base, _) = map.iter()
        .find(|&(_, v)| !v.marked.load(Ordering::Acquire))
        .unwrap();

    balanced_weight(&map, base).unwrap_err()
}

pub fn part2_cpp(text: &str) -> usize {
    let ptr = text.as_ptr();
    let len = text.len() as _;
    unsafe { some_guy_cpp(ptr, len) as _ }
}

pub fn part2_lunar_cpp(text: &str) -> usize {
    let ptr = text.as_ptr();
    let len = text.len() as _;
    unsafe { lunar_cpp(ptr, len) as _ }
}

#[cfg(test)]
mod tests {
    use super::*;
    use test::Bencher;

    const TEST_INPUT: &str = "pbga (66)
xhth (57)
ebii (61)
havc (66)
ktlj (57)
fwft (72) -> ktlj, cntj, xhth
qoyq (66)
padx (45) -> pbga, havc, qoyq
tknk (41) -> ugml, padx, fwft
jptl (61)
ugml (68) -> gyxo, ebii, jptl
gyxo (61)
cntj (57)";

    #[bench]
    fn bench_part1(b: &mut Bencher) {
        b.iter(|| part1(INPUT));
    }

    #[bench]
    fn bench_part1_leah(b: &mut Bencher) {
        b.iter(|| part1_leah(INPUT));
    }

    #[bench]
    fn bench_part1_btree(b: &mut Bencher) {
        b.iter(|| part1_btree(INPUT));
    }

    #[bench]
    fn bench_part2(b: &mut Bencher) {
        b.iter(|| part2(INPUT));
    }

    #[bench]
    fn bench_part2_single_map(b: &mut Bencher) {
        b.iter(|| part2_single_map(INPUT));
    }

    // #[bench]
    // fn bench_part2_single_map_rayon(b: &mut Bencher) {
    //     b.iter(|| part2_single_map_rayon(INPUT));
    // }

    #[bench]
    fn bench_part2_leah(b: &mut Bencher) {
        b.iter(|| leah::day(INPUT));
    }

    #[bench]
    fn bench_part2_cpp(b: &mut Bencher) {
        b.iter(|| part2_cpp(INPUT));
    }

    #[bench]
    fn bench_part2_lunar_cpp(b: &mut Bencher) {
        b.iter(|| part2_lunar_cpp(INPUT));
    }

    #[test]
    fn test_part1() {
        assert_eq!(part1(TEST_INPUT), Some("tknk"));
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2(TEST_INPUT), 60);
    }

    // #[test]
    // fn test_part2_cpp() {
    //     assert_eq!(part2_cpp(TEST_INPUT), 60);
    // }

    #[test]
    fn test_part2_lunar_cpp() {
        assert_eq!(part2_lunar_cpp(TEST_INPUT), 60);
    }
}
