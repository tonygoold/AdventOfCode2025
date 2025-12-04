use std::{
    ops::{Range, RangeInclusive},
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
