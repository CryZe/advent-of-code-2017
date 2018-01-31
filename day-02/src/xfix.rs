use std::num::ParseIntError;
use std::result;
use itertools::Itertools;

error_chain! {
    errors {
        NoEvenlyDivisibleNumber {
            description("no evenly divisible number")
        }
    }

    foreign_links {
        ParseInt(ParseIntError);
    }
}

pub fn checksum(input: &str) -> Result<u32> {
    let mut checksum = 0;
    for line in input.lines() {
        let numbers: result::Result<Vec<u32>, _> =
            line.split_whitespace().map(str::parse).collect();
        checksum += line_checksum(&numbers?)?;
    }
    Ok(checksum)
}

fn line_checksum(numbers: &[u32]) -> Result<u32> {
    numbers
        .iter()
        .enumerate()
        .cartesian_product(numbers.iter().enumerate())
        .find(|&((i, a), (j, b))| i != j && a % b == 0)
        .map(|((_, a), (_, b))| a / b)
        .ok_or_else(|| ErrorKind::NoEvenlyDivisibleNumber.into())
}
