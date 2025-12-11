#![warn(clippy::pedantic)]

use std::num::ParseIntError;
use std::ops::RangeInclusive;

/// Finds the invalid IDs in a range.
#[must_use]
pub fn find_invalid_ids(range: &RangeInclusive<usize>) -> Vec<usize> {
    let mut ids = vec![];

    for r in compute_subranges(range) {
        for l in guess_pattern_lengths(*r.start()) {
            ids.extend_from_slice(&guess_invalid_ids(&r, l));
        }
    }
    ids.sort_unstable();
    ids.dedup();

    ids
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

/// Determines the possible pattern lengths of `n`.
#[must_use]
pub fn guess_pattern_lengths(n: usize) -> Vec<usize> {
    let d = digits(n);
    (1..=d / 2).filter(|&i| d.is_multiple_of(i)).collect()
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

/// Computes the next power of 10 greater than `n`.
#[must_use]
pub fn next_power_of_10(n: usize) -> usize {
    // The digits in a usize are always smaller than u32::MAX.
    #[allow(clippy::cast_possible_truncation)]
    10usize.pow(digits(n) as u32)
}

/// Guesses the invalid IDs in a range.
///
/// Both ends of the range must have the same amount of digits. Moreover, `pattern_length` must
/// divide the number of digits cleanly.
#[must_use]
pub fn guess_invalid_ids(r: &RangeInclusive<usize>, pattern_length: usize) -> Vec<usize> {
    // First pass: identify the starting pattern.
    let mut pattern_digits = split_digits(*r.start())
        .chunks(pattern_length)
        .next()
        .expect("chunks should not be empty")
        .repeat(digits(*r.start()) / pattern_length);
    let start_digits = split_digits(*r.start());
    while pattern_digits < start_digits {
        increment_chunks(&mut pattern_digits, pattern_length);
    }

    // Second pass: collect the patterns.
    let mut patterns = vec![];
    let end_digits = split_digits(*r.end());
    while pattern_digits <= end_digits {
        patterns.push(join_digits(&pattern_digits));
        let r = increment_chunks(&mut pattern_digits, pattern_length);

        // Bail if we cannot increment any further.
        if r.is_none() {
            break;
        }
    }

    patterns
}

/// Divides `digits` in chunks, then increments all the resulting numbers by one in-place.
///
/// In order to ensure `digits.len()` remains the same, the increase is capped at the next power of
/// 10 minus one.
/// If no chunk can be increased, returns `None`.
pub fn increment_chunks(digits: &mut [usize], chunk_size: usize) -> Option<()> {
    if digits.chunks(chunk_size).all(|c| c.iter().all(|&n| n == 9)) {
        return None;
    }

    for c in digits.chunks_mut(chunk_size) {
        if c.iter().all(|&n| n == 9) {
            continue;
        }
        c.copy_from_slice(split_digits(join_digits(c) + 1).as_mut_slice());
    }

    Some(())
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
        // We're testing against the expected part 1 result here, therefore the invalid IDs must be
        // filtered.
        let rs = parse_ranges(test_data()).unwrap();
        let ids = rs
            .iter()
            .flat_map(find_invalid_ids)
            .filter(|&id| {
                let d = digits(id);
                let v = split_digits(id);
                v[..d / 2] == v[d / 2..]
            })
            .collect::<Vec<_>>();

        assert_eq!(ids.iter().sum::<usize>(), 1227775554);
    }

    #[test]
    fn invalid_ids_produce_expected_full_total() {
        let rs = parse_ranges(test_data()).unwrap();
        let ids = rs.iter().flat_map(find_invalid_ids).collect::<Vec<_>>();

        assert_eq!(ids.iter().sum::<usize>(), 4174379265);
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
            vec![11, 22],
            vec![99, 111],
            vec![999, 1010],
            vec![1188511885],
            vec![222222],
            vec![],
            vec![446446],
            vec![38593859],
            vec![565656],
            vec![824824824],
            vec![2121212121],
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
    fn possible_pattern_lengths_are_guessed_correctly() {
        assert_eq!(guess_pattern_lengths(11), vec![1]);
        assert_eq!(guess_pattern_lengths(111), vec![1]);
        assert_eq!(guess_pattern_lengths(1111), vec![1, 2]);
        assert_eq!(guess_pattern_lengths(11111), vec![1]);
        assert_eq!(guess_pattern_lengths(111111), vec![1, 2, 3]);
        assert_eq!(guess_pattern_lengths(111111111), vec![1, 3]);
    }

    #[test]
    fn invalid_ids_are_discovered_correctly() {
        let r = RangeInclusive::new(11usize, 22usize);
        let v = vec![11, 22];
        assert_eq!(guess_invalid_ids(&r, 1), v);

        let r = RangeInclusive::new(99usize, 99usize);
        let v = vec![99];
        assert_eq!(guess_invalid_ids(&r, 1), v);

        let r = RangeInclusive::new(1188511880usize, 1188511890usize);
        let v = vec![1188511885];
        assert_eq!(guess_invalid_ids(&r, 5), v);
    }

    #[test]
    fn increment_by_chunk_is_performed_correctly() {
        let mut ds = split_digits(101112);
        let r = increment_chunks(&mut ds, 2);
        assert_eq!(ds, split_digits(111213));
        assert!(r.is_some());

        let mut ds = split_digits(101112);
        let r = increment_chunks(&mut ds, 3);
        assert_eq!(ds, split_digits(102113));
        assert!(r.is_some());

        let mut ds = split_digits(2020202020);
        let r = increment_chunks(&mut ds, 1);
        assert_eq!(ds, split_digits(3131313131));
        assert!(r.is_some());

        let mut ds = split_digits(9999999998);
        let r = increment_chunks(&mut ds, 5);
        assert_eq!(ds, split_digits(9999999999));
        assert!(r.is_some());

        let mut ds = split_digits(99);
        let r = increment_chunks(&mut ds, 1);
        assert_eq!(ds, split_digits(99));
        assert!(r.is_none());
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
