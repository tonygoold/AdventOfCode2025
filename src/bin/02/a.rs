use std::ops::RangeInclusive;

use aoc::{input_arg, math::num_digits, range::try_range_incl_from_str, read_line};

fn main() {
    let ranges = read_line(&input_arg())
        .split(",")
        .map(|s| try_range_incl_from_str::<u64>(s, "-"))
        .collect::<Result<Vec<_>, _>>()
        .expect("Failed to parse input");

    let sum: u64 = ranges.iter().map(sum_matches).sum();
    println!("Total sum: {}", sum);
}

fn sum_matches(range: &RangeInclusive<u64>) -> u64 {
    let mut min_digits = num_digits(*range.start());
    if min_digits & 1 != 0 {
        min_digits += 1;
    }
    let mut max_digits = num_digits(*range.end());
    if max_digits & 1 != 0 {
        max_digits -= 1;
    }
    // Range ends both have same number of odd digits
    if min_digits > max_digits {
        return 0;
    }
    let mut sum = 0;
    let mut digits = min_digits;
    while digits <= max_digits {
        sum += sum_matches_with_digits(range, digits);
        digits += 2;
    }
    sum
}

fn sum_matches_with_digits(range: &RangeInclusive<u64>, digits: u32) -> u64 {
    let half_scale = 10u64.pow(digits / 2);
    let mut start_half = half_scale / 10;
    let mut end_half = start_half * 10 - 1;
    let start = start_half * half_scale + start_half;
    let end = end_half * half_scale + end_half;
    let range_start = *range.start();
    let range_end = *range.end();
    if start < range_start {
        // Sanity check: start should only be less than range_start if they
        // have the same number of digits.
        if num_digits(range_start) != digits {
            panic!(
                "start {} < range_start {} and different number of digits",
                start, range_start,
            );
        }

        start_half = first_half(range_start);
        if !range.contains(&(start_half * half_scale + start_half)) {
            start_half += 1;
        }
    }

    if end > range_end {
        // Sanity check: end should only be greater than range_end if they
        // have the same number of digits.
        if num_digits(range_end) != digits {
            panic!(
                "end {} < range_end {} and different number of digits",
                end, range_end,
            );
        }

        end_half = first_half(range_end);
        if !range.contains(&(end_half * half_scale + end_half)) {
            end_half -= 1;
        }
    }

    if start_half <= end_half {
        (start_half..=end_half).map(|n| n * half_scale + n).sum()
    } else {
        0
    }
}

fn first_half(n: u64) -> u64 {
    n / 10u64.pow(num_digits(n) / 2)
}

#[cfg(test)]
mod tests {
    use super::sum_matches;

    #[test]
    pub fn test_known_values() {
        assert_eq!(33, sum_matches(&(11..=22)));
        assert_eq!(99, sum_matches(&(95..=115)));
        assert_eq!(1010, sum_matches(&(998..=1012)));
        assert_eq!(1188511885, sum_matches(&(1188511880..=1188511890)));
        assert_eq!(222222, sum_matches(&(222220..=222224)));
        assert_eq!(0, sum_matches(&(1698522..=1698528)));
        assert_eq!(446446, sum_matches(&(446443..=446449)));
        assert_eq!(38593859, sum_matches(&(38593856..=38593862)));
    }
}
