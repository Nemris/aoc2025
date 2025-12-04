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

/// Computes the previous power of 10 smaller than `n`.
///
/// Returns 1 if `n` is 0.
#[must_use]
pub fn prev_power_of_10(n: usize) -> usize {
    // The digits in a usize are always smaller than u32::MAX.
    #[allow(clippy::cast_possible_truncation)]
    10usize.pow(digits(n) as u32 - 1)
}

/// Computes the next power of 10 greater than `n`.
#[must_use]
pub fn next_power_of_10(n: usize) -> usize {
    // The digits in a usize are always smaller than u32::MAX.
    #[allow(clippy::cast_possible_truncation)]
    10usize.pow(digits(n) as u32)
}

/// Counts the digits in `n`.
#[must_use]
pub fn digits(n: usize) -> usize {
    match n.checked_ilog10() {
        Some(n) => n as usize + 1,
        None => 1,
    }
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

    #[test]
    fn prev_power_of_10_is_calculated_correctly() {
        assert_eq!(prev_power_of_10(0), 1);
        assert_eq!(prev_power_of_10(11), 10);
        assert_eq!(prev_power_of_10(999), 100);
        assert_eq!(prev_power_of_10(1001), 1000);
    }

    #[test]
    fn next_power_of_10_is_calculated_correctly() {
        assert_eq!(next_power_of_10(0), 10);
        assert_eq!(next_power_of_10(11), 100);
        assert_eq!(next_power_of_10(999), 1000);
        assert_eq!(next_power_of_10(1001), 10000);
    }

    #[test]
    fn digits_are_counted_correctly() {
        assert_eq!(digits(0), 1);
        assert_eq!(digits(1), 1);
        assert_eq!(digits(42), 2);
        assert_eq!(digits(999), 3);
        assert_eq!(digits(1000), 4);
    }
}
