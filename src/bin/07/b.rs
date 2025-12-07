use std::collections::HashMap;

use aoc::{input_arg, read_char_grid};

fn main() {
    let grid = read_char_grid(&input_arg());
    let (rows, cols) = grid.size();
    let start = (0..cols)
        .find(|col| grid[(0, *col)] == 'S')
        .expect("Start position not found");
    let beams: HashMap<usize, usize> = {
        let mut set = HashMap::new();
        set.insert(start, 1);
        set
    };

    let beams = (1..rows).fold(beams, |beams, row| {
        let mut new_beams = HashMap::new();
        for (beam, count) in beams {
            if grid[(row, beam)] == '^' {
                *new_beams.entry(beam - 1).or_default() += count;
                *new_beams.entry(beam + 1).or_default() += count;
            } else {
                *new_beams.entry(beam).or_default() += count;
            }
        }
        new_beams
    });

    let timelines = beams.into_values().sum::<usize>();
    println!("The beam has {} timelines", timelines);
}
