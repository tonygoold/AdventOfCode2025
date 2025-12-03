use std::str::FromStr;

use aoc::{
    input_arg, read_lines,
    safe::{Safe, Turn},
};

fn main() {
    let turns = read_lines(&input_arg())
        .map(|s| Turn::from_str(&s))
        .collect::<Result<Vec<_>, _>>()
        .expect("Failed to parse turns");
    let (_, zeroes) = turns
        .into_iter()
        .fold((Safe::new(100, 50), 0), |(mut safe, count), turn| {
            safe.turn(&turn);
            let pos = safe.pos();
            (safe, count + if pos == 0 { 1 } else { 0 })
        });
    println!("Encountered {} zeroes", zeroes);
}
