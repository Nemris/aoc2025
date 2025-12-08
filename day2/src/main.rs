#![warn(clippy::pedantic)]

use std::env;
use std::error;
use std::fmt;
use std::fs;

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

fn main() -> Result<(), Box<dyn error::Error>> {
    let args = env::args().collect::<Vec<_>>();
    if args.len() != 2 {
        eprintln!("usage: {} file", args[0]);
        return Err(Box::new(Error::MissingInputFile));
    }

    let ranges = day2::parse_ranges(fs::read_to_string(&args[1])?.trim_end())?;
    println!("Sum of invalid IDs: {}", day2::sum_invalid_ids(&ranges));

    Ok(())
}
