#![feature(test)]
extern crate test;

pub const INPUT: &str = include_str!("input.txt");

pub mod kenira;
pub mod leah_chars;
pub mod leah_split;

trait Visitor {
    fn group(&mut self, depth: usize);
    fn garbage(&mut self);
    fn garbage_char(&mut self, c: char);
}

#[derive(Debug)]
pub enum Error {
    CharacterOutsideOfGroups(char),
    UnclosedGroup,
    UnclosedGarbage,
    UnexpectedCharacterInGroup(char),
}

fn parse<V: Visitor>(text: &str, visitor: &mut V) -> Result<(), Error> {
    let mut chars = text.chars();

    match chars.next() {
        Some('{') => visitor.group(1),
        Some(c) => return Err(Error::CharacterOutsideOfGroups(c)),
        None => return Ok(()),
    }

    let mut depth = 1;

    loop {
        match chars.next().ok_or(Error::UnclosedGroup)? {
            '{' => {
                depth += 1;
                visitor.group(depth);
            }
            '}' => {
                depth -= 1;
                if depth == 0 {
                    return Ok(());
                }
            }
            '<' => {
                visitor.garbage();
                loop {
                    match chars.next().ok_or(Error::UnclosedGarbage)? {
                        '!' => drop(chars.next()),
                        '>' => break,
                        c => visitor.garbage_char(c),
                    }
                }
            }
            ',' => {}
            c => return Err(Error::UnexpectedCharacterInGroup(c)),
        }
    }
}

pub fn part1(text: &str) -> Result<usize, Error> {
    struct Counter(usize);
    impl Visitor for Counter {
        fn group(&mut self, depth: usize) {
            self.0 += depth;
        }
        fn garbage(&mut self) {}
        fn garbage_char(&mut self, _: char) {}
    }

    let mut counter = Counter(0);
    parse(text, &mut counter)?;
    Ok(counter.0)
}

pub fn part2(text: &str) -> Result<usize, Error> {
    struct Counter(usize);
    impl Visitor for Counter {
        fn group(&mut self, _: usize) {}
        fn garbage(&mut self) {}
        fn garbage_char(&mut self, _: char) {
            self.0 += 1;
        }
    }

    let mut counter = Counter(0);
    parse(text, &mut counter)?;
    Ok(counter.0)
}

pub fn both(text: &str) -> Result<(usize, usize), Error> {
    struct Counter(usize, usize);
    impl Visitor for Counter {
        fn group(&mut self, depth: usize) {
            self.0 += depth;
        }
        fn garbage(&mut self) {}
        fn garbage_char(&mut self, _: char) {
            self.1 += 1;
        }
    }

    let mut counter = Counter(0, 0);
    parse(text, &mut counter)?;
    Ok((counter.0, counter.1))
}

fn parse_hacky<V: Visitor>(text: &str, visitor: &mut V) {
    let mut chars = text.bytes();

    match chars.next() {
        Some(b'{') => visitor.group(1),
        _ => return,
    }

    let mut depth = 1;

    while let Some(c) = chars.next() {
        match c {
            b'{' => {
                depth += 1;
                visitor.group(depth);
            }
            b'}' => {
                depth -= 1;
                if depth == 0 {
                    return;
                }
            }
            b'<' => {
                visitor.garbage();
                while let Some(c) = chars.next() {
                    match c {
                        b'!' => drop(chars.next()),
                        b'>' => break,
                        _ => visitor.garbage_char(' '),
                    }
                }
            }
            _ => {}
        }
    }
}

pub fn both_hacky(text: &str) -> (usize, usize) {
    struct Counter(usize, usize);
    impl Visitor for Counter {
        fn group(&mut self, depth: usize) {
            self.0 += depth;
        }
        fn garbage(&mut self) {}
        fn garbage_char(&mut self, _: char) {
            self.1 += 1;
        }
    }

    let mut counter = Counter(0, 0);
    parse_hacky(text, &mut counter);
    (counter.0, counter.1)
}

#[cfg(test)]
mod tests {
    use super::*;
    use test::Bencher;

    #[bench]
    fn bench_part1(b: &mut Bencher) {
        b.iter(|| part1(INPUT).unwrap());
    }

    #[bench]
    fn bench_part1_leah_chars(b: &mut Bencher) {
        b.iter(|| leah_chars::part1(INPUT));
    }

    #[bench]
    fn bench_part1_leah_split(b: &mut Bencher) {
        b.iter(|| leah_split::part1(INPUT));
    }

    #[bench]
    fn bench_part2(b: &mut Bencher) {
        b.iter(|| part2(INPUT).unwrap());
    }

    #[bench]
    fn bench_both(b: &mut Bencher) {
        b.iter(|| both(INPUT).unwrap());
    }

    #[bench]
    fn bench_both_hacky(b: &mut Bencher) {
        b.iter(|| both_hacky(INPUT));
    }

    #[bench]
    fn bench_both_kenira(b: &mut Bencher) {
        b.iter(|| kenira::both(INPUT));
    }
}
