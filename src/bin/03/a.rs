use aoc::{input_arg, read_lines};

/*
Strategy:

1. Set max_l and max_r to the second last and last characters of the input.
2. Iterate from right to left, from the third last character to the first:
    1. If c >= max_l then
        1. If max_l > max_r, then max_r = max_l
        2. max_l = c

Why c >= max_l? If we had 332, then initially max_l = 3 and max_r = 2.
If we only checked c > max_l, we would end with [3,2] instead of [3,3].

This has the following properties:

1. max_l is the left-most greatest digit, excluding the last.
2. max_r is the greatest digit to the right of max_l.
 */

fn max_ordered_pair(s: &str) -> (u32, u32) {
    let mut digits = s
        .chars()
        .rev()
        .map(|c| c.to_digit(10).expect("not a digit"));
    let mut max_r = digits.next().expect("not enough characters");
    let mut max_l = digits.next().expect("not enough characters");
    for digit in digits {
        if digit >= max_l {
            if max_l > max_r {
                max_r = max_l;
            }
            max_l = digit;
        }
    }
    (max_l, max_r)
}

fn main() {
    let lines = read_lines(&input_arg());
    let sum: u32 = lines
        .map(|line| {
            let (l, r) = max_ordered_pair(&line);
            l * 10 + r
        })
        .sum();
    println!("Sum: {}", sum);
}
