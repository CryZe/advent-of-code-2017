#![feature(conservative_impl_trait)]
#![feature(test)]
extern crate test;

extern crate arrayvec;
extern crate cretonne;
extern crate cton_frontend;
extern crate cton_native;
extern crate cton_wasm;
extern crate fxhash;
extern crate mmap;
extern crate rayon;

use fxhash::FxHashMap;
use arrayvec::ArrayVec;
use rayon::prelude::*;

pub const INPUT: &str = include_str!("input.txt");
pub const TEST_INPUT: &str = "b inc 5 if a > 1
a inc 1 if b < 5
c dec -10 if a >= 1
c inc -20 if c == 10";

pub mod compiler;
pub mod leah;

#[derive(Copy, Clone, PartialEq)]
enum Op {
    Dec,
    Inc,
}

impl Op {
    fn exec(self, v: &mut i64, delta: i64) -> i64 {
        *v += match self {
            Op::Dec => -delta,
            Op::Inc => delta,
        };
        *v
    }
}

#[derive(Copy, Clone, PartialEq)]
enum CmpOp {
    Eq,
    Ne,
    Lt,
    Le,
    Gt,
    Ge,
}

impl CmpOp {
    fn cmp(self, a: i64, b: i64) -> bool {
        match self {
            CmpOp::Eq => a == b,
            CmpOp::Ne => a != b,
            CmpOp::Lt => a < b,
            CmpOp::Le => a <= b,
            CmpOp::Gt => a > b,
            CmpOp::Ge => a >= b,
        }
    }
}

pub struct Instruction<'a> {
    dst: &'a str,
    op: Op,
    val: i64,
    src: &'a str,
    cmp_op: CmpOp,
    cmp_val: i64,
}

fn parse<'a>(text: &'a str) -> impl Iterator<Item = Instruction<'a>> {
    text.lines().filter_map(|l| {
        let mut splits = l.split_whitespace();
        let dst = splits.next()?;
        let op = match splits.next()? {
            "inc" => Op::Inc,
            "dec" => Op::Dec,
            _ => return None,
        };
        let val = splits.next()?.parse().ok()?;
        splits.next()?;
        let src = splits.next()?;
        let cmp_op = match splits.next()? {
            "==" => CmpOp::Eq,
            "!=" => CmpOp::Ne,
            "<" => CmpOp::Lt,
            "<=" => CmpOp::Le,
            ">" => CmpOp::Gt,
            ">=" => CmpOp::Ge,
            _ => return None,
        };
        let cmp_val = splits.next()?.parse().ok()?;

        Some(Instruction {
            dst,
            op,
            val,
            src,
            cmp_op,
            cmp_val,
        })
    })
}

#[derive(Debug)]
struct Registers<'a> {
    map: FxHashMap<&'a str, i64>,
}

impl<'a> Registers<'a> {
    fn new() -> Self {
        Self {
            map: FxHashMap::default(),
        }
    }

    fn get(&mut self, x: &'a str) -> &mut i64 {
        self.map.entry(x).or_insert(0)
    }

    fn max(&self) -> i64 {
        self.map.values().cloned().max().unwrap_or_default()
    }
}

fn interpret<'a, I, F>(instructions: I, mut per_instruction: F) -> Registers<'a>
where
    I: IntoIterator<Item = Instruction<'a>>,
    F: FnMut(&mut Registers<'a>, i64),
{
    let mut registers = Registers::new();
    for instruction in instructions {
        let a = *registers.get(instruction.src);
        let b = instruction.cmp_val;
        if instruction.cmp_op.cmp(a, b) {
            let val = instruction
                .op
                .exec(registers.get(instruction.dst), instruction.val);
            per_instruction(&mut registers, val);
        }
    }
    registers
}

pub fn part1(text: &str) -> i64 {
    interpret(parse(text), |_, _| {}).max()
}

pub fn part1_rayon(text: &str) -> i64 {
    interpret(parse_parallel(text), |_, _| {}).max()
}

pub fn part2(text: &str) -> i64 {
    let mut max = 0;
    interpret(parse(text), |_, v| {
        if v > max {
            max = v
        }
    });
    max
}

pub fn part2_arrayvec(text: &str) -> i64 {
    let parsed: ArrayVec<[Instruction; 1024]> = parse(text).collect();

    let mut max = 0;
    interpret(parsed, |_, v| {
        if v > max {
            max = v
        }
    });
    max
}

pub fn parse_parallel<'a>(text: &'a str) -> Vec<Instruction<'a>> {
    let mut instructions = Vec::with_capacity(1024);
    instructions.par_extend(text.par_lines().filter_map(|l| {
        let mut splits = l.split_whitespace();
        let dst = splits.next()?;
        let op = match splits.next()? {
            "inc" => Op::Inc,
            "dec" => Op::Dec,
            _ => return None,
        };
        let val = splits.next()?.parse().ok()?;
        splits.next()?;
        let src = splits.next()?;
        let cmp_op = match splits.next()? {
            "==" => CmpOp::Eq,
            "!=" => CmpOp::Ne,
            "<" => CmpOp::Lt,
            "<=" => CmpOp::Le,
            ">" => CmpOp::Gt,
            ">=" => CmpOp::Ge,
            _ => return None,
        };
        let cmp_val = splits.next()?.parse().ok()?;

        Some(Instruction {
            dst,
            op,
            val,
            src,
            cmp_op,
            cmp_val,
        })
    }));
    instructions
}

pub fn part2_rayon(text: &str) -> i64 {
    let mut max = 0;
    interpret(parse_parallel(text), |_, v| {
        if v > max {
            max = v
        }
    });
    max
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
    fn bench_part1_leah(b: &mut Bencher) {
        b.iter(|| leah::part1(INPUT));
    }

    #[bench]
    fn bench_part1_rayon(b: &mut Bencher) {
        b.iter(|| part1_rayon(INPUT));
    }

    #[bench]
    fn bench_part1_cretonne(b: &mut Bencher) {
        let compiled_fn = compiler::jit_compile_part1(&parse_parallel(INPUT));
        b.iter(|| compiled_fn.call());
    }

    #[bench]
    fn bench_part2(b: &mut Bencher) {
        b.iter(|| part2(INPUT));
    }

    #[bench]
    fn bench_part2_arrayvec(b: &mut Bencher) {
        b.iter(|| part2_arrayvec(INPUT));
    }

    #[bench]
    fn bench_part2_rayon(b: &mut Bencher) {
        b.iter(|| part2_rayon(INPUT));
    }

    #[bench]
    fn bench_part2_cretonne(b: &mut Bencher) {
        let compiled_fn = compiler::jit_compile_part2(&parse_parallel(INPUT));
        b.iter(|| compiled_fn.call());
    }

    #[test]
    fn test_part1() {
        assert_eq!(part1(TEST_INPUT), 1);
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2(TEST_INPUT), 10);
    }
}
