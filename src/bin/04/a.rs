use aoc::{forklift::is_accessible, input_arg, read_char_grid};

fn main() {
    let tiles = read_char_grid(&input_arg());
    let accessible = tiles
        .iter()
        .filter(|(row, col, tile)| **tile == '@' && is_accessible(&tiles, *row, *col))
        .count();
    println!("There are {} accessible rolls", accessible);
}
