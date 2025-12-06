use aoc::{input_arg, read_lines};

enum Op {
    Add,
    Mul,
}

impl Op {
    pub fn identity(&self) -> u64 {
        match self {
            Op::Add => 0,
            Op::Mul => 1,
        }
    }
}

fn main() {
    let mut lines = read_lines(&input_arg()).collect::<Vec<_>>();
    let ops = lines
        .pop()
        .expect("No input")
        .split_ascii_whitespace()
        .map(|s| match s {
            "+" => Op::Add,
            "*" => Op::Mul,
            _ => panic!("Unexpected operation"),
        })
        .collect::<Vec<_>>();
    let operands = lines
        .into_iter()
        .map(|line| {
            line.split_ascii_whitespace()
                .map(|s| s.parse::<u64>().expect("Not a number"))
                .collect::<Vec<_>>()
        })
        .collect::<Vec<_>>();

    let width = ops.len();
    if !operands.iter().all(|row| row.len() == width) {
        panic!("Input is not a grid");
    }

    let init = ops.iter().map(Op::identity).collect::<Vec<_>>();
    let results = operands.into_iter().fold(init, |mut acc, row| {
        for i in 0..acc.len() {
            match &ops[i] {
                Op::Add => acc[i] += row[i],
                Op::Mul => acc[i] *= row[i],
            }
        }
        acc
    });
    let sum = results.into_iter().sum::<u64>();
    println!("The sum of operations is {}", sum);
}
