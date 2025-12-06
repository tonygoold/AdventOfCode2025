use aoc::{input_arg, read_char_grid};

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
    let grid = read_char_grid(&input_arg());
    // Transpose the input. Every resulting line is either empty, a number
    // with an operation at the end (new calculation), or just a number.
    let (rows, cols) = grid.size();
    let mut lines: Vec<String> = Vec::with_capacity(cols);
    lines.resize_with(cols, Default::default);
    for i in 0..rows {
        for j in 0..cols {
            lines[j].push(grid[(i, j)]);
        }
    }

    // We could reverse the input lines and collect all operands until we hit
    // an operator, but both operators are commutative, so we can evaluate as
    // we go.
    let results = lines
        .into_iter()
        .filter(|line| !line.trim().is_empty())
        .fold((Vec::new(), Op::Add), |(mut nums, mut op), mut line| {
            if line.ends_with('+') {
                op = Op::Add;
                nums.push(op.identity());
                line.pop();
            } else if line.ends_with('*') {
                op = Op::Mul;
                nums.push(op.identity());
                line.pop();
            }
            let n: u64 = line.trim().parse().expect("Not a number");
            match op {
                Op::Add => *nums.last_mut().expect("No numbers") += n,
                Op::Mul => *nums.last_mut().expect("No numbers") *= n,
            }
            (nums, op)
        })
        .0;
    let sum = results.into_iter().sum::<u64>();
    println!("The sum of operations is {}", sum);
}
