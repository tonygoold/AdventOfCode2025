use super::grid::Grid;

pub fn is_accessible(tiles: &Grid<char>, row: usize, col: usize) -> bool {
    // A tile is accessible if there are fewer than 4 inaccessible
    // neighbouring tiles.
    tiles
        .neighbours_with_diagonal((row, col))
        .into_iter()
        .filter(|pos| tiles[*pos] == '@')
        .count()
        < 4
}

pub fn mark_accessible(tiles: &Grid<char>) -> Grid<char> {
    tiles.map(|(row, col), tile| match *tile {
        '@' => {
            if is_accessible(tiles, row, col) {
                'x'
            } else {
                '@'
            }
        }
        _ => *tile,
    })
}
