use aoc::{input_arg, range::try_range_incl_from_str, read_line_groups};

fn main() {
    let mut groups = read_line_groups(&input_arg());
    let ids = groups
        .pop()
        .expect("IDs not found")
        .into_iter()
        .map(|s| s.parse::<u64>().expect("Input is not a number"))
        .collect::<Vec<u64>>();
    let ranges = groups
        .pop()
        .expect("Ranges not found")
        .into_iter()
        .map(|s| try_range_incl_from_str::<u64>(&s, "-").expect("Input is not a range"))
        .collect::<Vec<_>>();

    let good_ids = ids
        .into_iter()
        .filter(|id| ranges.iter().any(|range| range.contains(id)))
        .collect::<Vec<u64>>();
    println!("There are {} good ingredients", good_ids.len());
}
