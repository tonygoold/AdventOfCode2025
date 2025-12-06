use aoc::{
    input_arg,
    range::{try_range_incl_from_str, RangeUnion},
    read_line_groups,
};

fn main() {
    let mut groups = read_line_groups(&input_arg());
    // We ignore this now, but it's still expected in the input.
    groups.pop().expect("IDs not found");

    let ranges = groups
        .pop()
        .expect("Ranges not found")
        .into_iter()
        .map(|s| {
            try_range_incl_from_str::<u64>(&s, "-")
                // Ranges are easier to work with than RangeInclusives.
                .map(|range| (*range.start())..(*range.end() + 1))
                .expect("Input is not a range")
        })
        .fold(RangeUnion::new(), |mut union, range| {
            union.add(range);
            union
        });

    let good_ids = ranges.size();
    println!("There are {} good ingredients", good_ids);
}
