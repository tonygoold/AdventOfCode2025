use super::errors::ParseError;
use regex::Regex;
use std::{cell::LazyCell, str::FromStr};

const TURN_RE: LazyCell<Regex> =
    LazyCell::new(|| Regex::new(r"(L|R)(\d+)").expect("unable to initialize turn regex"));

pub enum Turn {
    Left(i32),
    Right(i32),
}

impl Turn {
    pub fn is_left(&self) -> bool {
        match self {
            Self::Left(_) => true,
            _ => false,
        }
    }

    pub fn is_right(&self) -> bool {
        match self {
            Self::Right(_) => true,
            _ => false,
        }
    }
}

impl FromStr for Turn {
    type Err = ParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let caps = TURN_RE
            .captures(s)
            .ok_or_else(|| ParseError::generic("no match"))?;
        let c_str = caps
            .get(1)
            .ok_or_else(|| ParseError::generic("no match for direction"))?
            .as_str();
        let n: i32 = caps
            .get(2)
            .ok_or_else(|| ParseError::generic("no match for value"))?
            .as_str()
            .parse()
            .map_err(|_| ParseError::generic("value is not a number"))?;
        match c_str {
            "L" => Ok(Self::Left(n)),
            "R" => Ok(Self::Right(n)),
            _ => Err(ParseError::generic("direction is not L or R")),
        }
    }
}

pub struct Safe {
    size: i32,
    pos: i32,
}

impl Safe {
    pub fn new(size: i32, pos: i32) -> Self {
        Self { size, pos }
    }

    pub fn pos(&self) -> i32 {
        self.pos
    }

    pub fn turn(&mut self, t: &Turn) -> i32 {
        let (mut n, is_left) = match t {
            Turn::Left(n) => (*n, true),
            Turn::Right(n) => (*n, false),
        };

        // Reduce to less than a single full rotation
        let mut zeroes = 0;
        while n >= self.size {
            zeroes += 1;
            n -= self.size;
        }

        if is_left {
            let prev = self.pos;
            self.pos -= n;
            if self.pos <= 0 && prev > 0 {
                zeroes += 1;
            }
            if self.pos < 0 {
                self.pos += self.size;
            }
        } else {
            self.pos += n;
            if self.pos >= self.size {
                zeroes += 1;
                self.pos -= self.size;
            }
        }
        zeroes
    }
}

#[cfg(test)]
mod tests {
    use super::{Safe, Turn};

    #[test]
    pub fn test_left_to_zero() {
        let mut safe = Safe::new(100, 99);
        let count = safe.turn(&Turn::Left(99));
        assert_eq!(0, safe.pos());
        assert_eq!(1, count);
    }

    #[test]
    pub fn test_left_no_underflow() {
        let mut safe = Safe::new(100, 99);
        let count = safe.turn(&Turn::Left(98));
        assert_eq!(1, safe.pos());
        assert_eq!(0, count);
    }

    #[test]
    pub fn test_left_underflow() {
        let mut safe = Safe::new(100, 1);
        let count = safe.turn(&Turn::Left(2));
        assert_eq!(99, safe.pos());
        assert_eq!(1, count);
    }

    #[test]
    pub fn test_left_underflow_from_zero() {
        let mut safe = Safe::new(100, 0);
        let count = safe.turn(&Turn::Left(5));
        assert_eq!(95, safe.pos());
        assert_eq!(0, count);
    }

    #[test]
    pub fn test_left_underflow_zero_to_zero() {
        let mut safe = Safe::new(100, 0);
        let count = safe.turn(&Turn::Left(100));
        assert_eq!(0, safe.pos());
        assert_eq!(1, count);
    }

    #[test]
    pub fn test_left_multiple_underflow() {
        let mut safe = Safe::new(100, 1);
        let count = safe.turn(&Turn::Left(302));
        assert_eq!(99, safe.pos());
        assert_eq!(4, count);
    }

    #[test]
    pub fn test_right_to_zero() {
        let mut safe = Safe::new(100, 50);
        let count = safe.turn(&Turn::Right(50));
        assert_eq!(0, safe.pos());
        assert_eq!(1, count);
    }

    #[test]
    pub fn test_right_no_overflow() {
        let mut safe = Safe::new(100, 0);
        let count = safe.turn(&Turn::Right(99));
        assert_eq!(99, safe.pos());
        assert_eq!(0, count);
    }

    #[test]
    pub fn test_right_overflow() {
        let mut safe = Safe::new(100, 99);
        let count = safe.turn(&Turn::Right(2));
        assert_eq!(1, safe.pos());
        assert_eq!(1, count);
    }

    #[test]
    pub fn test_right_overflow_zero_to_zero() {
        let mut safe = Safe::new(100, 0);
        let count = safe.turn(&Turn::Right(100));
        assert_eq!(0, safe.pos());
        assert_eq!(1, count);
    }

    #[test]
    pub fn test_right_multiple_overflow() {
        let mut safe = Safe::new(100, 99);
        let count = safe.turn(&Turn::Right(302));
        assert_eq!(1, safe.pos());
        assert_eq!(4, count);
    }
}
