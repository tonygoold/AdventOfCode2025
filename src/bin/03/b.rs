// Same strategy as first part, but use a Vec and carry to the next element
// every time the previous one is replaced.

// Solve it recursively! Solve the left-most, then solve substring for n-1.

use aoc::{input_arg, read_lines};

fn first_greatest_pos(ns: &[u32], start: usize, end: usize) -> usize {
    ns.iter()
        .enumerate()
        .skip(start)
        .take(end - start)
        .fold(
            (0, 0),
            |max, cur| if *cur.1 > max.1 { (cur.0, *cur.1) } else { max },
        )
        .0
}

fn max_sequence(s: &str, size: usize) -> Vec<u32> {
    let digits: Vec<u32> = s
        .chars()
        .map(|c| c.to_digit(10).expect("not a digit"))
        .collect();
    let len = digits.len();
    if len < size {
        panic!("input is too short");
    }

    let mut result: Vec<u32> = Vec::with_capacity(size);
    let mut left: usize = 0;
    for i in 0..size {
        left = first_greatest_pos(&digits, left, len - size + i + 1);
        result.push(digits[left]);
        left += 1;
    }
    result
}

fn seq_to_num(ns: &[u32]) -> u64 {
    ns.iter().fold(0, |acc, n| acc * 10 + *n as u64)
}

fn main() {
    let lines = read_lines(&input_arg());
    let sum: u64 = lines
        .map(|line| max_sequence(&line, 12))
        .map(|ns| seq_to_num(&ns))
        .sum();
    println!("Sum: {}", sum);
}
