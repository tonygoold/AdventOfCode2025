use std::{
    iter::Sum,
    ops::{Range, RangeInclusive, Sub},
    str::FromStr,
};

use super::errors::ParseError;

pub fn try_range_from_str<Idx: FromStr>(s: &str, sep: &str) -> Result<Range<Idx>, ParseError> {
    let mut parts = s.splitn(3, sep);
    let start: Idx = parts
        .next()
        .ok_or_else(|| ParseError::generic("no range start"))?
        .parse()
        .map_err(|_| ParseError::generic("invalid range start"))?;
    let end: Idx = parts
        .next()
        .ok_or_else(|| ParseError::generic("no range end"))?
        .parse()
        .map_err(|_| ParseError::generic("invalid range end"))?;
    if parts.next().is_none() {
        Ok(start..end)
    } else {
        Err(ParseError::generic("range has too many parts"))
    }
}

pub fn try_range_incl_from_str<Idx: FromStr>(
    s: &str,
    sep: &str,
) -> Result<RangeInclusive<Idx>, ParseError> {
    let mut parts = s.splitn(3, sep);
    let start: Idx = parts
        .next()
        .ok_or_else(|| ParseError::generic("no range start"))?
        .parse()
        .map_err(|_| ParseError::generic("invalid range start"))?;
    let end: Idx = parts
        .next()
        .ok_or_else(|| ParseError::generic("no range end"))?
        .parse()
        .map_err(|_| ParseError::generic("invalid range end"))?;
    if parts.next().is_none() {
        Ok(start..=end)
    } else {
        Err(ParseError::generic("range has too many parts"))
    }
}

pub fn ranges_adjacent<Idx: Ord>(r1: &Range<Idx>, r2: &Range<Idx>) -> bool {
    /*
    Ranges r1 and r2 are disjoint, non-adjacent if
    r1.end < r2.start || r1.start > r2.end
     */
    r1.end >= r2.start && r1.start <= r2.end
}

// RangeUnion contains a set of Ranges. It automatically merges any ranges
// that are adjacent or overlapping.
pub struct RangeUnion<Idx: Ord> {
    ranges: Vec<Range<Idx>>,
}

impl<Idx: Ord> RangeUnion<Idx> {
    pub fn new() -> Self {
        Self { ranges: Vec::new() }
    }

    pub fn contains(&self, item: Idx) -> bool {
        self.ranges.iter().any(|range| range.contains(&item))
    }

    pub fn add(&mut self, mut range: Range<Idx>) {
        let adjacent_indices = self
            .ranges
            .iter()
            .enumerate()
            .rev()
            .filter(|(_, r)| ranges_adjacent(&range, *r))
            .map(|(idx, _)| idx)
            .collect::<Vec<_>>();
        for idx in adjacent_indices {
            let r = self.ranges.swap_remove(idx);
            if r.start < range.start {
                range.start = r.start;
            }
            if r.end > range.end {
                range.end = r.end;
            }
        }
        self.ranges.push(range);
    }

    pub fn count(&self) -> usize {
        self.ranges.len()
    }
}

impl<Idx: Ord> Default for RangeUnion<Idx> {
    fn default() -> Self {
        Self::new()
    }
}

impl<Idx> RangeUnion<Idx>
where
    // The bound on Sum is necessary because Sub::Output is unbound.
    Idx: Ord + Copy + Sub + Sum<<Idx as Sub>::Output>,
{
    pub fn size(&self) -> Idx {
        self.ranges.iter().map(|r| r.end - r.start).sum()
    }
}

#[cfg(test)]
mod tests {
    use super::RangeUnion;

    #[test]
    pub fn test_disjoint() {
        let mut set: RangeUnion<u64> = RangeUnion::new();
        set.add(0..5);
        set.add(6..9);
        assert_eq!(2, set.count());
        assert_eq!(5 + 3, set.size());
    }

    #[test]
    pub fn test_adjacent() {
        let mut set: RangeUnion<u64> = RangeUnion::new();
        set.add(0..5);
        set.add(5..9);
        assert_eq!(1, set.count());
        assert_eq!(9, set.size());
    }

    #[test]
    pub fn test_overlapping() {
        let mut set: RangeUnion<u64> = RangeUnion::new();
        set.add(0..5);
        set.add(2..7);
        assert_eq!(1, set.count());
        assert_eq!(7, set.size());
    }

    #[test]
    pub fn test_double_merge() {
        let mut set: RangeUnion<u64> = RangeUnion::new();
        set.add(0..5);
        set.add(6..9);
        assert_eq!(2, set.count());
        set.add(5..6);
        assert_eq!(1, set.count());
        assert_eq!(9, set.size());
    }
}
