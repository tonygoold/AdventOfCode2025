use aoc::{input_arg, point::Point3D, read_lines};

use std::{
    collections::{BinaryHeap, HashMap},
    str::FromStr,
};

struct Pair(Point3D<usize>, Point3D<usize>, usize);

// To implement a min-heap, a pair is "greater" than another pair if its
// distance is smaller.
impl Ord for Pair {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        other.2.cmp(&self.2)
    }
}

impl PartialOrd for Pair {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl PartialEq for Pair {
    fn eq(&self, other: &Self) -> bool {
        self.2 == other.2
    }
}

impl Eq for Pair {}

fn square_dist(p1: &Point3D<usize>, p2: &Point3D<usize>) -> usize {
    let dx = if p1.x > p2.x {
        p1.x - p2.x
    } else {
        p2.x - p1.x
    };
    let dy = if p1.y > p2.y {
        p1.y - p2.y
    } else {
        p2.y - p1.y
    };
    let dz = if p1.z > p2.z {
        p1.z - p2.z
    } else {
        p2.z - p1.z
    };
    dx * dx + dy * dy + dz * dz
}

fn main() {
    let points = read_lines(&input_arg())
        .map(|line| Point3D::<usize>::from_str(&line).expect("Invalid point"))
        .collect::<Vec<_>>();
    let mut sqdist_heap: BinaryHeap<Pair> = BinaryHeap::new();
    points.iter().enumerate().for_each(|(i, p1)| {
        points.iter().skip(i + 1).for_each(|p2| {
            sqdist_heap.push(Pair(*p1, *p2, square_dist(p1, p2)));
        });
    });

    let num_pairings: usize = if points.len() > 20 { 1000 } else { 10 };

    let mut groups: Vec<Vec<Point3D<usize>>> = Vec::new();
    let mut groupings: HashMap<Point3D<usize>, usize> = HashMap::new();
    (0..num_pairings).for_each(|_| {
        let pair = sqdist_heap.pop().expect("Not enough pairs");
        let id0 = groupings.get(&pair.0);
        let id1 = groupings.get(&pair.1);
        match (id0, id1) {
            (Some(id), None) => {
                let id = *id;
                groups[id].push(pair.1);
                groupings.insert(pair.1, id);
            }
            (None, Some(id)) => {
                let id = *id;
                groups[id].push(pair.0);
                groupings.insert(pair.0, id);
            }
            (Some(id0), Some(id1)) => {
                // Pick an arbitrary group to merge into
                let id0 = *id0;
                let id1 = *id1;
                // Skip merging if already in same group
                if id0 != id1 {
                    groups[id1].iter().for_each(|p| {
                        groupings.insert(*p, id0);
                    });
                    // This would be nicer if there were a Vec::replace that
                    // returned the old item. An in place solution exists with
                    // Vec::split_at_mut, but it's more cumbersome.
                    let items = groups[id1].to_vec();
                    groups[id1].clear();
                    groups[id0].extend(items);
                }
            }
            (None, None) => {
                let id = groups.len();
                groups.push(vec![pair.0, pair.1]);
                groupings.insert(pair.0, id);
                groupings.insert(pair.1, id);
            }
        }
    });

    let mut sizes = groups.iter().map(|group| group.len()).collect::<Vec<_>>();
    sizes.sort_unstable();
    sizes.reverse();
    let prod = sizes.iter().take(3).product::<usize>();
    println!("Product of three largest sizes: {}", prod);
}
