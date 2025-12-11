#![warn(clippy::pedantic)]

use std::env;
use std::error;
use std::fmt;
use std::fs;
use std::ops::RangeInclusive;

#[derive(Debug)]
enum Error {
    MissingInputFile,
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Self::MissingInputFile => write!(f, "missing input file"),
        }
    }
}

impl error::Error for Error {}

/// Solves part 1 of day 2.
fn solve_part_1(ranges: &[RangeInclusive<usize>]) -> usize {
    // Part 1 considers only IDs with a pattern repeated exactly twice to be invalid.
    ranges
        .iter()
        .flat_map(day2::find_invalid_ids)
        .filter(|&id| {
            let d = day2::digits(id);
            let v = day2::split_digits(id);
            v[..d / 2] == v[d / 2..]
        })
        .sum()
}

/// Solvespart 2 of day 2.
fn solve_part_2(ranges: &[RangeInclusive<usize>]) -> usize {
    ranges.iter().flat_map(day2::find_invalid_ids).sum()
}

fn main() -> Result<(), Box<dyn error::Error>> {
    let args = env::args().collect::<Vec<_>>();
    if args.len() != 2 {
        eprintln!("usage: {} file", args[0]);
        return Err(Box::new(Error::MissingInputFile));
    }

    let ranges = day2::parse_ranges(fs::read_to_string(&args[1])?.trim_end())?;
    println!("Sum of filtered invalid IDs: {}", solve_part_1(&ranges));
    println!("Sum of all invalid IDs: {}", solve_part_2(&ranges));

    Ok(())
}
