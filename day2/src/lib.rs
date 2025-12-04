#![warn(clippy::pedantic)]

use std::num::ParseIntError;
use std::ops::RangeInclusive;

/// Attempts to parse a string slice into a series of inclusive ranges.
pub fn parse_ranges(s: &str) -> Result<Vec<RangeInclusive<usize>>, ParseIntError> {
    let mut ranges = vec![];

    for pair in s.split(',') {
        // Always exactly two entries if this succeeds.
        let bounds = pair
            .split('-')
            .map(str::parse::<usize>)
            .collect::<Result<Vec<_>, _>>()?;

        ranges.push(RangeInclusive::new(bounds[0], bounds[1]));
    }

    Ok(ranges)
}

#[cfg(test)]
mod tests {
    use super::*;

    fn test_data() -> &'static str {
        "11-22,95-115,998-1012,1188511880-1188511890,222220-222224,1698522-1698528,446443-446449,38593856-38593862,565653-565659,824824821-824824827,2121212118-2121212124"
    }

    #[test]
    fn ranges_are_parsed_correctly() {
        let rs = parse_ranges(test_data()).unwrap();

        assert_eq!(rs.len(), 11);
    }
}
