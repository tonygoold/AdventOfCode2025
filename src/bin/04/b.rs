use aoc::{forklift::mark_accessible, input_arg, read_char_grid};

fn main() {
    let mut tiles = read_char_grid(&input_arg());
    let mut total_removed: usize = 0;
    loop {
        tiles = mark_accessible(&tiles);
        let removed: Vec<(usize, usize)> = tiles
            .iter()
            .filter(|(_, _, tile)| **tile == 'x')
            .map(|(row, col, _)| (row, col))
            .collect();
        if removed.is_empty() {
            break;
        }
        total_removed += removed.len();
        for (row, col) in removed {
            tiles[(row, col)] = '.';
        }
    }
    println!("There were {} rolls removed", total_removed);
}
