#![warn(clippy::pedantic)]

use std::num::ParseIntError;
use std::ops::RangeInclusive;

pub fn sum_invalid_ids(ranges: &[RangeInclusive<usize>]) -> usize {
    ranges.iter().filter_map(find_invalid_ids).flatten().sum()
}

/// Finds the invalid IDs in a range.
#[must_use]
pub fn find_invalid_ids(range: &RangeInclusive<usize>) -> Option<Vec<usize>> {
    // Determine the actual bounds where invalid IDs may be found..
    let start = if digits(*range.start()).is_multiple_of(2) {
        *range.start()
    } else {
        next_power_of_10(*range.start())
    };
    let end = if digits(*range.end()).is_multiple_of(2) {
        *range.end()
    } else {
        prev_power_of_10(*range.end())
    };

    // We don't know if this can happen in the data, but we know it is invalid.
    if start > end {
        return None;
    }

    let invalid_ids = (start..=end)
        .filter(|id| !is_id_valid(*id))
        .collect::<Vec<_>>();

    if invalid_ids.is_empty() {
        None
    } else {
        Some(invalid_ids)
    }
}

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

/// Computes the subranges of a range, such that for all ranges the number of digits of `r.begin()`
/// and`r.end()` are the same.
#[must_use]
pub fn compute_subranges(r: &RangeInclusive<usize>) -> Vec<RangeInclusive<usize>> {
    fn inner(r: RangeInclusive<usize>, rs: &mut Vec<RangeInclusive<usize>>) {
        if digits(*r.start()) == digits(*r.end()) {
            rs.push(r);
            return;
        }

        let midpoint = next_power_of_10(*r.start());
        let r1 = RangeInclusive::new(*r.start(), midpoint - 1);
        let r2 = RangeInclusive::new(midpoint, *r.end());

        inner(r1, rs);
        inner(r2, rs);
    }

    let mut subranges = vec![];
    inner(r.clone(), &mut subranges);
    subranges
}

/// Checks if `id` is valid.
#[must_use]
pub fn is_id_valid(id: usize) -> bool {
    if !digits(id).is_multiple_of(2) {
        return true;
    }

    // The digits in a usize are always smaller than u32::MAX.
    #[allow(clippy::cast_possible_truncation)]
    let divisor = 10usize.pow(digits(id) as u32 / 2);

    id / divisor != id % divisor
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

/// Splits a number into its digits.
#[must_use]
pub fn split_digits(mut n: usize) -> Vec<usize> {
    let mut ds = Vec::with_capacity(digits(n));

    while n > 0 {
        ds.push(n % 10);
        n /= 10;
    }
    ds.reverse();

    ds
}

/// Joins the digits of a number.
#[must_use]
pub fn join_digits(ds: &[usize]) -> usize {
    let mut n = 0;

    // Digits are smaller than i32::MAX.
    #[allow(clippy::cast_possible_truncation)]
    for (i, d) in ds.iter().rev().enumerate() {
        n += d * 10usize.pow(i as u32);
    }

    n
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
    fn invalid_ids_produce_expected_total() {
        let rs = parse_ranges(test_data()).unwrap();

        assert_eq!(sum_invalid_ids(&rs), 1227775554);
    }

    #[test]
    fn ids_are_validated_correctly() {
        assert_eq!(is_id_valid(11), false);
        assert_eq!(is_id_valid(22), false);
        assert_eq!(is_id_valid(99), false);
        assert_eq!(is_id_valid(1010), false);
        assert_eq!(is_id_valid(1188511885), false);
        assert_eq!(is_id_valid(222222), false);
        assert_eq!(is_id_valid(446446), false);
        assert_eq!(is_id_valid(38593859), false);

        assert_eq!(is_id_valid(12), true);
        assert_eq!(is_id_valid(256), true);
        assert_eq!(is_id_valid(707), true);
        assert_eq!(is_id_valid(1001), true);
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

    #[test]
    fn invalid_ids_are_identified_correctly() {
        let rs = parse_ranges(test_data()).unwrap();
        let expected = &[
            Some(vec![11, 22]),
            Some(vec![99]),
            Some(vec![1010]),
            Some(vec![1188511885]),
            Some(vec![222222]),
            None,
            Some(vec![446446]),
            Some(vec![38593859]),
            None,
            None,
            None,
        ];

        for (r, e) in rs.iter().zip(expected) {
            assert_eq!(find_invalid_ids(r), *e);
        }
    }

    #[test]
    fn subranges_are_computed_correctly() {
        let r = RangeInclusive::new(10usize, 20usize);
        let v = vec![r.clone()];
        assert_eq!(compute_subranges(&r), v);

        let r = RangeInclusive::new(95usize, 115usize);
        let v = vec![
            RangeInclusive::new(95usize, 99usize),
            RangeInclusive::new(100usize, 115usize),
        ];
        assert_eq!(compute_subranges(&r), v);

        let r = RangeInclusive::new(99usize, 1001usize);
        let v = vec![
            RangeInclusive::new(99usize, 99usize),
            RangeInclusive::new(100usize, 999usize),
            RangeInclusive::new(1000usize, 1001usize),
        ];
        assert_eq!(compute_subranges(&r), v);
    }

    #[test]
    fn numbers_are_split_correctly() {
        assert_eq!(split_digits(100), vec![1, 0, 0]);
        assert_eq!(split_digits(42), vec![4, 2]);
        assert_eq!(split_digits(1), vec![1]);
    }

    #[test]
    fn digits_are_joined_correctly() {
        assert_eq!(join_digits(&[1, 0, 0]), 100);
        assert_eq!(join_digits(&[4, 2]), 42);
        assert_eq!(join_digits(&[1]), 1);
    }
}
