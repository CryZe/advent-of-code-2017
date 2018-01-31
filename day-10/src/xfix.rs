use enum_map::EnumMap;
use std::num::Wrapping;
use arrayvec::ArrayString;
use std::fmt::Write;

fn reverse(map: &mut EnumMap<u8, u8>, start: u8, length: u8) {
    let mut positions = u32::from(start)..(u32::from(start) + u32::from(length));
    while let (Some(front), Some(back)) = (positions.next(), positions.next_back()) {
        map.swap(front as u8, back as u8);
    }
}

pub fn part1(line: &str) -> u32 {
    let mut array = enum_map! { i => i };
    let mut current_position = Wrapping(0);
    let mut skip = Wrapping(0);
    for length in line.split(',') {
        let length = length.parse().unwrap();
        reverse(&mut array, current_position.0, length);
        current_position += Wrapping(length) + skip;
        skip += Wrapping(1);
    }
    u32::from(array[0]) * u32::from(array[1])
}

pub fn part2(input: &str) -> ArrayString<[u8; 32]> {
    let input = input.trim().as_bytes();
    let mut array = enum_map! { i => i };
    let mut current_position = Wrapping(0);
    let mut skip = Wrapping(0);
    for _round in 0..64 {
        for &byte in input.iter().chain(&[17, 31, 73, 47, 23]) {
            reverse(&mut array, current_position.0, byte);
            current_position += Wrapping(byte) + skip;
            skip += Wrapping(1);
        }
    }
    let mut arr = [0; 16];
    for (chunk, out) in array.as_slice().chunks(16).zip(&mut arr) {
        *out = chunk.iter().fold(0, |a, b| a ^ b);
    }

    let mut output = ArrayString::new();
    for c in arr.iter() {
        write!(output, "{:02x}", c).unwrap();
    }
    output
}
