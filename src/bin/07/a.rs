use std::collections::HashSet;

use aoc::{input_arg, read_char_grid};

fn main() {
    let grid = read_char_grid(&input_arg());
    let (rows, cols) = grid.size();
    let start = (0..cols)
        .find(|col| grid[(0, *col)] == 'S')
        .expect("Start position not found");
    let beams: HashSet<usize> = {
        let mut set = HashSet::new();
        set.insert(start);
        set
    };

    let (_, splits) = (1..rows).fold((beams, 0usize), |(beams, count), row| {
        let mut new_count = count;
        let mut new_beams = HashSet::new();
        for beam in beams {
            if grid[(row, beam)] == '^' {
                new_count += 1;
                new_beams.insert(beam - 1);
                new_beams.insert(beam + 1);
            } else {
                new_beams.insert(beam);
            }
        }
        (new_beams, new_count)
    });

    println!("The beam was split {} times", splits);
}
