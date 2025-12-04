use std::ops::RangeInclusive;

use aoc::{input_arg, math::num_digits, range::try_range_incl_from_str, read_line};

/*
Thoughts:

Need to watch out for counting multiple times:
- 12121212 is both (12) and (1212)
- 111111 is (1), (11), and (111)

Odd numbers of digits are no longer excluded, but only for a single repeating
digit, e.g., 11111.

More specifically, for a k-digit pattern, we only need to consider numbers with
kn digits (n >= 1).
 */

fn main() {
    let ranges = read_line(&input_arg())
        .split(",")
        .map(|s| try_range_incl_from_str::<u64>(s, "-"))
        .collect::<Result<Vec<_>, _>>()
        .expect("Failed to parse input");

    let sum: u64 = ranges.iter().map(sum_matches).sum();
    println!("Total sum: {}", sum);
}

fn is_repeating(n: u64) -> bool {
    let digits = num_digits(n);
    (1..=digits / 2).any(|i| {
        if digits % i == 0 {
            let num_parts = digits / i;
            let factor = 10u64.pow(i);
            let repeated_part = n % factor;
            let mut num = 0;
            for _ in 0..num_parts {
                num = num * factor + repeated_part;
            }
            n == num
        } else {
            false
        }
    })
}

fn sum_matches(range: &RangeInclusive<u64>) -> u64 {
    range.clone().filter(|n| is_repeating(*n)).sum()
}

#[cfg(test)]
mod tests {
    use super::sum_matches;

    #[test]
    pub fn test_known_values() {
        assert_eq!(11 + 22, sum_matches(&(11..=22)));
        assert_eq!(99 + 111, sum_matches(&(95..=115)));
        assert_eq!(999 + 1010, sum_matches(&(998..=1012)));
        assert_eq!(1188511885, sum_matches(&(1188511880..=1188511890)));
        assert_eq!(222222, sum_matches(&(222220..=222224)));
        assert_eq!(0, sum_matches(&(1698522..=1698528)));
        assert_eq!(446446, sum_matches(&(446443..=446449)));
        assert_eq!(38593859, sum_matches(&(38593856..=38593862)));
        assert_eq!(824824824, sum_matches(&(824824821..=824824827)));
        assert_eq!(2121212121, sum_matches(&(2121212118..=2121212124)));
    }
}
